//! THE RECORD AS ROWS: what the twin keeps of its entries. Folded from a list (`twin::fold`), every entry is kept in memory, as it
//! always was. Given a SOURCE — the rows the record is stored as, behind the program's own store — only what the current state needs
//! stays in memory: which numbers are entries and of what kind, and the few entries read lately; any other entry is read from its row
//! when asked for. Memory then stays flat as history grows. The core never touches a file: the source is the program's.
use serde_json::Value;
use imbl::{OrdMap, Vector};
use std::collections::HashMap;
use std::ops::{Bound, RangeBounds};
use std::sync::{Arc, Mutex};

/// WHERE THE ENTRIES ARE STORED, as rows: read by number, a page at a time, and by signature
pub trait Source: Send + Sync {
    /// the entry numbered `g`, as it was taken
    fn entry(&self, g: i64) -> Option<Value>;
    /// the entries numbered from `from` to `to` (both included), oldest first or newest first, at most `limit` of them
    fn page(&self, from: i64, to: i64, newest_first: bool, limit: usize) -> Vec<(i64, Value)>;
    /// the number of the entry taken with this signature — in its standard spelling (`identity::canonical_sig`): a source keeps each
    /// entry findable by that spelling — if one was
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
    mem: OrdMap<i64, Arc<Value>>,
    source: Option<Arc<dyn Source>>,
    hand: Mutex<Hand>,
    /// for each number, its entry's kind (an index into `names`, plus one) — 0 when that number is no entry
    kind: Vector<u32>,
    names: Vec<String>,
    ids: HashMap<String, u32>,
    /// the entries of each kind, in order
    of_kind: HashMap<u32, Vector<i64>>,
    /// the details indexed: for each, the entries whose value has it, in order
    with: OrdMap<String, Vector<i64>>,
    count: usize,
    top: i64,
    /// AN OVERLAY'S: its own entries have no rows — they are numbered above every row it was made on (`rows_to`), kept in memory
    overlay: bool,
    rows_to: i64,
}
impl Default for Entries {
    fn default() -> Self {
        Entries {
            mem: OrdMap::new(),
            source: None,
            hand: Mutex::new(Hand::default()),
            kind: Vector::new(),
            names: vec![],
            ids: HashMap::new(),
            of_kind: HashMap::new(),
            with: OrdMap::new(),
            count: 0,
            top: 0,
            overlay: false,
            rows_to: 0,
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
            overlay: self.overlay,
            rows_to: self.rows_to,
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
    /// BECOME AN OVERLAY'S ENTRIES: whatever lands from now on is kept here, never as a row
    pub(crate) fn overlay(&mut self) {
        if !self.overlay {
            self.overlay = true;
            // with no entry yet, no row is below it
            self.rows_to = if self.count == 0 { -1 } else { self.top };
        }
    }
    pub(crate) fn is_overlay(&self) -> bool {
        self.overlay
    }
    /// AN OVERLAY OF ROWS: the last number in its rows — every entry it takes is numbered above it
    pub(crate) fn overlay_of_rows(&self) -> Option<i64> {
        (self.overlay && self.source.is_some()).then_some(self.rows_to)
    }
    /// whether entry `g` is read from the rows: kept as rows, and not an overlay's own
    fn in_rows(&self, g: i64) -> bool {
        self.source.is_some() && !(self.overlay && g > self.rows_to)
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
            while self.kind.len() <= i {
                self.kind.push_back(0);
            }
            let was = self.kind[i];
            if was != k {
                if was == 0 {
                    self.count += 1;
                } else if let Some(l) = self.of_kind.get_mut(&was) {
                    l.retain(|x| *x != g);
                }
                let l = self.of_kind.entry(k).or_default();
                let at = l.binary_search(&g).unwrap_or_else(|at| at);
                l.insert(at, g);
            }
            self.kind[i] = k;
        }
        self.top = self.top.max(g);
        for f in self.with.keys().cloned().collect::<Vec<_>>() {
            if e["value"][f.as_str()].is_null() {
                continue;
            }
            let l = self.with.get_mut(&f).expect("an index kept");
            if let Err(at) = l.binary_search(&g) {
                l.insert(at, g);
            }
        }
        assert!(!self.overlay || self.source.is_none() || g > self.rows_to, "an overlay's entry {g} is not above the rows it was made on ({})", self.rows_to);
        let v = Arc::new(e.clone());
        if self.in_rows(g) {
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
    pub fn of_kind<'a>(&'a self, is: &str) -> impl DoubleEndedIterator<Item = i64> + 'a {
        self.ids.get(is).and_then(|k| self.of_kind.get(k)).into_iter().flat_map(|l| l.iter().copied())
    }
    /// THE ENTRIES OF A KIND, read: oldest first (`.rev()` for newest first)
    pub fn of<'a>(&'a self, is: &str) -> impl DoubleEndedIterator<Item = (i64, Arc<Value>)> + 'a {
        self.of_kind(is).filter_map(move |g| self.get(&g).map(|e| (g, e)))
    }
    /// THE ENTRIES WHOSE VALUE HAS A DETAIL (`value.<field>`, not null), oldest first — kept at hand for each detail indexed
    /// (`index`); any other is asked of the source, else the entries are walked
    pub fn having(&self, field: &str) -> Vec<i64> {
        if let Some(l) = self.with.get(field) {
            return l.iter().copied().filter(|g| self.contains_key(g)).collect();
        }
        // the rows' own index holds no overlay's entries: an overlay walks them
        if let Some(gs) = self.source.as_ref().filter(|_| !self.overlay).and_then(|s| s.having(field)) {
            return gs.into_iter().filter(|g| self.contains_key(g)).collect();
        }
        self.iter().filter(|(_, e)| !e["value"][field].is_null()).map(|(g, _)| g).collect()
    }
    /// KEEP AN INDEX of these details at hand: the entries whose value has each, read once now (from the source, or walked) and kept
    /// as entries land
    pub fn index(&mut self, fields: &[&str]) {
        for f in fields {
            if !self.with.contains_key(*f) {
                let l = self.having(f).into_iter().collect();
                self.with.insert(f.to_string(), l);
            }
        }
    }
    /// the details indexed, each with its entries (for a snapshot)
    pub(crate) fn indexes(&self) -> &OrdMap<String, Vector<i64>> {
        &self.with
    }
    pub(crate) fn set_indexes(&mut self, with: OrdMap<String, Vector<i64>>) {
        self.with = with;
    }
    /// ENTRY `g`, read from its row when it is not at hand
    pub fn get(&self, g: &i64) -> Option<Arc<Value>> {
        if !self.contains_key(g) {
            return None;
        }
        let Some(src) = self.source.as_ref().filter(|_| self.in_rows(*g)) else {
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
    pub(crate) fn kinds(&self) -> (&[String], &Vector<u32>) {
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
                e.of_kind.entry(*k).or_default().push_back(g as i64);
            }
        }
        e.kind = kind.into_iter().collect();
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
        // what is read from the rows: up to the last row (all of it, but an overlay's own entries); the rest is in memory
        let rows_to = match (&es.source, es.overlay) {
            (None, _) => i64::MIN,
            (Some(_), false) => i64::MAX,
            (Some(_), true) => es.rows_to,
        };
        if if newest_first { self.hi <= rows_to } else { self.lo <= rows_to } {
            let src = es.source.as_ref().expect("rows are read only from a source");
            let hi = self.hi.min(rows_to);
            let rows = src.page(self.lo, hi, newest_first, PAGE);
            // the last row read bounds what was read, entries or not
            let edge = rows.last().map(|(g, _)| *g);
            let out: Vec<(i64, Arc<Value>)> = rows
                .into_iter()
                .filter(|(g, _)| es.contains_key(g))
                .map(|(g, v)| (g, Arc::new(v)))
                .collect();
            match (edge, newest_first) {
                (None, false) => self.lo = hi + 1,
                (None, true) => self.hi = self.lo - 1,
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
        let lo = self.lo.max(rows_to.saturating_add(1));
        let it = es.mem.range(lo..=self.hi);
        let got: Vec<(i64, Arc<Value>)> = if newest_first {
            it.rev().take(PAGE).map(|(g, v)| (*g, v.clone())).collect()
        } else {
            it.take(PAGE).map(|(g, v)| (*g, v.clone())).collect()
        };
        match (got.last().map(|(g, _)| *g), newest_first) {
            (None, false) => self.lo = self.hi + 1,
            // nothing more in memory going back: the rows below it, if any, are next
            (None, true) => {
                self.hi = lo - 1;
                return self.fill(true);
            }
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
