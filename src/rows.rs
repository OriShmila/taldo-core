//! THE RECORD AS ROWS: what the twin keeps of its entries. Folded from a list (`twin::fold`), every entry is kept in memory, as it
//! always was. Given a SOURCE — the rows the record is stored as, behind the program's own store — only what the current state needs
//! stays in memory: which numbers are entries and of what kind, and the few entries read lately; any other entry is read from its row
//! when asked for. Memory then stays flat as history grows. The core never touches a file: the source is the program's.
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::ops::{Bound, RangeBounds};
use std::sync::{Arc, Mutex};

/// WHERE THE ENTRIES ARE STORED, as rows: read by number, a page at a time, and by signature
pub trait Source: Send + Sync {
    /// the entry numbered `g`, as it was taken
    fn entry(&self, g: i64) -> Option<Value>;
    /// the entries numbered from `from` to `to` (both included), oldest first or newest first, at most `limit` of them
    fn page(&self, from: i64, to: i64, newest_first: bool, limit: usize) -> Vec<(i64, Value)>;
    /// the number of the entry taken with this signature, if one was
    fn signed(&self, sig: &str) -> Option<i64>;
    /// the entries whose value has the detail `field` (not null), oldest first — when the source keeps an index of it; else none, and
    /// the entries are walked
    fn having(&self, _field: &str) -> Option<Vec<i64>> {
        None
    }
}

/// how many entries read from rows are kept at hand
const AT_HAND: usize = 4096;
/// how many rows are read at once when the entries are walked
const PAGE: usize = 256;

#[derive(Default)]
struct Hand {
    map: HashMap<i64, (Arc<Value>, u64)>,
    tick: u64,
}
impl Hand {
    fn get(&mut self, g: i64) -> Option<Arc<Value>> {
        self.tick += 1;
        let t = self.tick;
        self.map.get_mut(&g).map(|(v, at)| {
            *at = t;
            v.clone()
        })
    }
    fn put(&mut self, g: i64, v: Arc<Value>) {
        self.tick += 1;
        self.map.insert(g, (v, self.tick));
        if self.map.len() > AT_HAND {
            // the half read longest ago goes
            let mut ticks: Vec<u64> = self.map.values().map(|(_, t)| *t).collect();
            ticks.sort_unstable();
            let cut = ticks[ticks.len() / 2];
            self.map.retain(|_, (_, t)| *t >= cut);
        }
    }
}

/// THE ENTRIES OF A TWIN: by number; each entry's kind known without reading it
pub struct Entries {
    /// every entry, when there is no source
    mem: BTreeMap<i64, Arc<Value>>,
    source: Option<Arc<dyn Source>>,
    hand: Mutex<Hand>,
    /// for each number, its entry's kind (an index into `names`, plus one) — 0 when that number is no entry
    kind: Vec<u32>,
    names: Vec<String>,
    ids: HashMap<String, u32>,
    /// the entries of each kind, in order
    of_kind: HashMap<u32, Vec<i64>>,
    /// the details indexed: for each, the entries whose value has it, in order
    with: BTreeMap<String, Vec<i64>>,
    count: usize,
    top: i64,
}
impl Default for Entries {
    fn default() -> Self {
        Entries {
            mem: BTreeMap::new(),
            source: None,
            hand: Mutex::new(Hand::default()),
            kind: vec![],
            names: vec![],
            ids: HashMap::new(),
            of_kind: HashMap::new(),
            with: BTreeMap::new(),
            count: 0,
            top: 0,
        }
    }
}
impl Clone for Entries {
    fn clone(&self) -> Self {
        Entries {
            mem: self.mem.clone(),
            source: self.source.clone(),
            hand: Mutex::new(Hand::default()),
            kind: self.kind.clone(),
            names: self.names.clone(),
            ids: self.ids.clone(),
            of_kind: self.of_kind.clone(),
            with: self.with.clone(),
            count: self.count,
            top: self.top,
        }
    }
}
impl std::fmt::Debug for Entries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entries({} entries, top {})", self.count, self.top)
    }
}

fn bounds(r: impl RangeBounds<i64>) -> (i64, i64) {
    let lo = match r.start_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => x.saturating_add(1),
        Bound::Unbounded => i64::MIN,
    };
    let hi = match r.end_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => x.saturating_sub(1),
        Bound::Unbounded => i64::MAX,
    };
    (lo, hi)
}

impl Entries {
    /// entries kept in rows, read from `source`
    pub fn stored(source: Arc<dyn Source>) -> Entries {
        Entries {
            source: Some(source),
            ..Default::default()
        }
    }
    pub fn source(&self) -> Option<&Arc<dyn Source>> {
        self.source.as_ref()
    }
    fn id(&mut self, is: &str) -> u32 {
        if let Some(i) = self.ids.get(is) {
            return *i;
        }
        let i = self.names.len() as u32 + 1;
        self.names.push(is.to_string());
        self.ids.insert(is.to_string(), i);
        i
    }
    /// AN ENTRY TAKEN: known by number and kind; kept whole only without a source (else its row holds it)
    pub(crate) fn insert(&mut self, g: i64, e: &Value) {
        let is = e["is"].as_str().unwrap_or("").to_string();
        let k = self.id(&is);
        if g >= 0 {
            let i = g as usize;
            if self.kind.len() <= i {
                self.kind.resize(i + 1, 0);
            }
            let was = self.kind[i];
            if was != k {
                if was == 0 {
                    self.count += 1;
                } else if let Some(l) = self.of_kind.get_mut(&was) {
                    l.retain(|x| *x != g);
                }
                let l = self.of_kind.entry(k).or_default();
                let at = l.partition_point(|x| *x < g);
                l.insert(at, g);
            }
            self.kind[i] = k;
        }
        self.top = self.top.max(g);
        for (f, l) in self.with.iter_mut() {
            if !e["value"][f.as_str()].is_null() && l.last().is_none_or(|x| *x < g) {
                l.push(g);
            } else if !e["value"][f.as_str()].is_null() && !l.contains(&g) {
                let at = l.partition_point(|x| *x < g);
                l.insert(at, g);
            }
        }
        let v = Arc::new(e.clone());
        if self.source.is_some() {
            self.hand.lock().unwrap_or_else(|x| x.into_inner()).put(g, v);
        } else {
            self.mem.insert(g, v);
        }
    }
    /// WHETHER `g` IS AN ENTRY taken
    pub fn contains_key(&self, g: &i64) -> bool {
        *g >= 0 && self.kind.get(*g as usize).is_some_and(|k| *k != 0)
    }
    /// THE KIND OF ENTRY `g`, without reading it
    pub fn kind_of(&self, g: i64) -> Option<&str> {
        let k = *self.kind.get(usize::try_from(g).ok()?)?;
        (k != 0).then(|| self.names[k as usize - 1].as_str())
    }
    /// THE ENTRIES OF A KIND, oldest first
    pub fn of_kind(&self, is: &str) -> &[i64] {
        self.ids
            .get(is)
            .and_then(|k| self.of_kind.get(k))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
    /// THE ENTRIES OF A KIND, read: oldest first (`.rev()` for newest first)
    pub fn of<'a>(&'a self, is: &str) -> impl DoubleEndedIterator<Item = (i64, Arc<Value>)> + 'a {
        self.of_kind(is).iter().filter_map(move |g| self.get(g).map(|e| (*g, e)))
    }
    /// THE ENTRIES WHOSE VALUE HAS A DETAIL (`value.<field>`, not null), oldest first — kept at hand for each detail indexed
    /// (`index`); any other is asked of the source, else the entries are walked
    pub fn having(&self, field: &str) -> Vec<i64> {
        if let Some(l) = self.with.get(field) {
            return l.iter().copied().filter(|g| self.contains_key(g)).collect();
        }
        if let Some(gs) = self.source.as_ref().and_then(|s| s.having(field)) {
            return gs.into_iter().filter(|g| self.contains_key(g)).collect();
        }
        self.iter().filter(|(_, e)| !e["value"][field].is_null()).map(|(g, _)| g).collect()
    }
    /// KEEP AN INDEX of these details at hand: the entries whose value has each, read once now (from the source, or walked) and kept
    /// as entries land
    pub fn index(&mut self, fields: &[&str]) {
        for f in fields {
            if !self.with.contains_key(*f) {
                let l = self.having(f);
                self.with.insert(f.to_string(), l);
            }
        }
    }
    /// the details indexed, each with its entries (for a snapshot)
    pub(crate) fn indexes(&self) -> &BTreeMap<String, Vec<i64>> {
        &self.with
    }
    pub(crate) fn set_indexes(&mut self, with: BTreeMap<String, Vec<i64>>) {
        self.with = with;
    }
    /// ENTRY `g`, read from its row when it is not at hand
    pub fn get(&self, g: &i64) -> Option<Arc<Value>> {
        if !self.contains_key(g) {
            return None;
        }
        let Some(src) = &self.source else {
            return self.mem.get(g).cloned();
        };
        if let Some(v) = self.hand.lock().unwrap_or_else(|x| x.into_inner()).get(*g) {
            return Some(v);
        }
        let v = Arc::new(src.entry(*g)?);
        self.hand
            .lock()
            .unwrap_or_else(|x| x.into_inner())
            .put(*g, v.clone());
        Some(v)
    }
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// THE NEWEST ENTRY'S NUMBER (0 when there is none)
    pub fn top(&self) -> i64 {
        self.top
    }
    /// every entry's number, oldest first
    pub fn keys(&self) -> impl DoubleEndedIterator<Item = i64> + '_ {
        self.kind
            .iter()
            .enumerate()
            .filter(|(_, k)| **k != 0)
            .map(|(g, _)| g as i64)
    }
    /// THE ENTRIES, oldest first (`.rev()` for newest first) — read from rows a page at a time
    pub fn iter(&self) -> Iter<'_> {
        self.range(..)
    }
    pub fn values(&self) -> impl DoubleEndedIterator<Item = Arc<Value>> + '_ {
        self.iter().map(|(_, e)| e)
    }
    /// the entries numbered in `r`
    pub fn range(&self, r: impl RangeBounds<i64>) -> Iter<'_> {
        let (lo, hi) = bounds(r);
        Iter {
            es: self,
            lo: lo.max(0),
            hi: hi.min(self.top),
            front: Default::default(),
            back: Default::default(),
        }
    }
    /// the kinds' names and each number's kind, as kept (for a snapshot)
    pub(crate) fn kinds(&self) -> (&[String], &[u32]) {
        (&self.names, &self.kind)
    }
    /// the entries known again from a snapshot: their numbers and kinds; their rows are read from the source
    pub(crate) fn restored(source: Arc<dyn Source>, names: Vec<String>, kind: Vec<u32>) -> Entries {
        let mut e = Entries::stored(source);
        e.ids = names
            .iter()
            .enumerate()
            .map(|(i, n)| (n.clone(), i as u32 + 1))
            .collect();
        e.names = names;
        for (g, k) in kind.iter().enumerate() {
            if *k != 0 {
                e.count += 1;
                e.top = g as i64;
                e.of_kind.entry(*k).or_default().push(g as i64);
            }
        }
        e.kind = kind;
        e
    }
}

/// THE ENTRIES WALKED, a page of rows at a time, from either end
pub struct Iter<'a> {
    es: &'a Entries,
    lo: i64,
    hi: i64,
    front: std::collections::VecDeque<(i64, Arc<Value>)>,
    back: std::collections::VecDeque<(i64, Arc<Value>)>,
}
impl Iter<'_> {
    fn fill(&mut self, newest_first: bool) {
        if self.lo > self.hi {
            return;
        }
        let es = self.es;
        let got: Vec<(i64, Arc<Value>)> = match &es.source {
            None => {
                let it = es.mem.range(self.lo..=self.hi);
                if newest_first {
                    it.rev().take(PAGE).map(|(g, v)| (*g, v.clone())).collect()
                } else {
                    it.take(PAGE).map(|(g, v)| (*g, v.clone())).collect()
                }
            }
            Some(src) => {
                let rows = src.page(self.lo, self.hi, newest_first, PAGE);
                // the last row read bounds what was read, entries or not
                let edge = rows.last().map(|(g, _)| *g);
                let out: Vec<(i64, Arc<Value>)> = rows
                    .into_iter()
                    .filter(|(g, _)| es.contains_key(g))
                    .map(|(g, v)| (g, Arc::new(v)))
                    .collect();
                match (edge, newest_first) {
                    (None, _) => {
                        self.lo = self.hi + 1;
                        return;
                    }
                    (Some(e), false) => self.lo = e + 1,
                    (Some(e), true) => self.hi = e - 1,
                }
                if out.is_empty() {
                    return self.fill(newest_first);
                }
                if newest_first {
                    self.back.extend(out);
                } else {
                    self.front.extend(out);
                }
                return;
            }
        };
        match (got.last().map(|(g, _)| *g), newest_first) {
            (None, _) => self.lo = self.hi + 1,
            (Some(e), false) => self.lo = e + 1,
            (Some(e), true) => self.hi = e - 1,
        }
        if newest_first {
            self.back.extend(got);
        } else {
            self.front.extend(got);
        }
    }
}
impl Iterator for Iter<'_> {
    type Item = (i64, Arc<Value>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.front.is_empty() {
            self.fill(false);
        }
        self.front.pop_front().or_else(|| self.back.pop_back())
    }
}
impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back.is_empty() {
            self.fill(true);
        }
        self.back.pop_front().or_else(|| self.front.pop_back())
    }
}
