//! A TWIN'S CURRENT STATE, KEPT: everything the twin holds in memory, written as bytes so a start reads it back and applies only the
//! entries that came after it — never the whole history again. The entries themselves are not in it: they are the rows (`rows`).
//! Only a twin that keeps rows is kept this way; one folded from a list is folded again.
use crate::rows::{Entries, Source};
use crate::twin::{Node, Twin};
use serde_json::{Map, Value};
use imbl::{OrdMap, Vector};
use std::sync::Arc;

const MAGIC: &[u8] = b"taldo twin state 12\n";

struct W(Vec<u8>);
impl W {
    fn u(&mut self, mut x: u64) {
        loop {
            let b = (x & 0x7f) as u8;
            x >>= 7;
            if x == 0 {
                self.0.push(b);
                return;
            }
            self.0.push(b | 0x80);
        }
    }
    fn i(&mut self, x: i64) {
        self.u(((x << 1) ^ (x >> 63)) as u64)
    }
    fn s(&mut self, s: &str) {
        self.u(s.len() as u64);
        self.0.extend_from_slice(s.as_bytes());
    }
    fn map(&mut self, m: &Map<String, Value>) {
        self.s(&Value::Object(m.clone()).to_string())
    }
    fn json(&mut self, v: &Value) {
        self.s(&v.to_string())
    }
    /// numbers, mostly rising: each as its distance from the one before
    fn gs<'a>(&mut self, l: impl ExactSizeIterator<Item = &'a i64>) {
        self.u(l.len() as u64);
        let mut last = 0i64;
        for x in l {
            self.i(x.wrapping_sub(last));
            last = *x;
        }
    }
    fn broken(&mut self, l: &Vector<(String, i64)>) {
        self.u(l.len() as u64);
        for (h, a) in l {
            self.s(h);
            self.i(*a);
        }
    }
}
struct R<'a>(&'a [u8], usize);
impl R<'_> {
    fn u(&mut self) -> Result<u64, String> {
        let (mut x, mut shift) = (0u64, 0);
        loop {
            let b = *self.0.get(self.1).ok_or("the state ends early")?;
            self.1 += 1;
            x |= ((b & 0x7f) as u64) << shift;
            if b & 0x80 == 0 {
                return Ok(x);
            }
            shift += 7;
            if shift > 63 {
                return Err("a number in the state is too long".into());
            }
        }
    }
    fn n(&mut self) -> Result<usize, String> {
        Ok(self.u()? as usize)
    }
    fn i(&mut self) -> Result<i64, String> {
        let z = self.u()?;
        Ok((z >> 1) as i64 ^ -((z & 1) as i64))
    }
    fn s(&mut self) -> Result<String, String> {
        let n = self.n()?;
        let b = self.0.get(self.1..self.1.saturating_add(n)).ok_or("the state ends early")?;
        self.1 += n;
        String::from_utf8(b.to_vec()).map_err(|_| "a name in the state is not text".to_string())
    }
    fn json(&mut self) -> Result<Value, String> {
        serde_json::from_str(&self.s()?).map_err(|e| format!("the state holds bad JSON: {e}"))
    }
    fn map(&mut self) -> Result<Map<String, Value>, String> {
        match self.json()? {
            Value::Object(m) => Ok(m),
            _ => Err("the state holds details that are not an object".into()),
        }
    }
    fn gs(&mut self) -> Result<Vector<i64>, String> {
        let n = self.n()?;
        let mut out = Vector::new();
        let mut last = 0i64;
        for _ in 0..n {
            last = last.wrapping_add(self.i()?);
            out.push_back(last);
        }
        Ok(out)
    }
    fn broken(&mut self) -> Result<Vector<(String, i64)>, String> {
        let n = self.n()?;
        (0..n).map(|_| Ok((self.s()?, self.i()?))).collect()
    }
}

impl Twin {
    /// THE CURRENT STATE, as bytes: every node with its details and what is at it, by it and naming it; what answers what; the holds in
    /// force and what waits; which numbers are entries and of what kind. Never an entry itself: those are the rows
    pub fn snapshot(&self) -> Vec<u8> {
        let mut w = W(MAGIC.to_vec());
        w.u(self.nodes.len() as u64);
        for n in self.nodes.values() {
            w.s(&n.id);
            w.s(&n.kind);
            w.s(n.parent.as_deref().unwrap_or(""));
            w.map(&n.who);
            w.map(&n.value);
            w.map(&n.accepted);
            w.gs(n.at.iter());
            w.gs(n.by.iter());
            w.gs(n.named.iter());
        }
        w.u(self.answers.len() as u64);
        for (g, l) in &self.answers {
            w.i(*g);
            w.gs(l.iter());
        }
        w.u(self.rejected.len() as u64);
        for r in &self.rejected {
            w.json(r);
        }
        for m in [&self.history, &self.force] {
            w.u(m.len() as u64);
            for (k, l) in m {
                w.s(k);
                w.gs(l.iter());
            }
        }
        w.u(self.readers.len() as u64);
        for (k, l) in &self.readers {
            w.s(k);
            w.gs(l.iter());
        }
        for m in [&self.out, &self.pending] {
            w.u(m.len() as u64);
            for (g, l) in m {
                w.i(*g);
                w.broken(l);
            }
        }
        w.u(self.rewritten.len() as u64);
        for (g, l) in &self.rewritten {
            w.i(*g);
            w.gs(l.iter());
        }
        w.gs(self.present.iter());
        w.u(self.taken_out.len() as u64);
        for (g, o) in &self.taken_out {
            w.i(*g);
            w.i(o.by);
            w.s(&o.at);
            w.s(&o.writer);
            w.s(&o.is);
        }
        w.u(self.erased.len() as u64);
        for (g, p) in &self.erased {
            w.i(*g);
            w.i(*p);
        }
        w.s(&self.head);
        let (names, kinds) = self.entries.kinds();
        w.u(names.len() as u64);
        for n in names {
            w.s(n);
        }
        w.u(kinds.len() as u64);
        // runs of one kind, as (kind, how many): a record is mostly long runs of the same few kinds
        let mut runs: Vec<(u32, u64)> = vec![];
        for k in kinds {
            match runs.last_mut() {
                Some((x, n)) if x == k => *n += 1,
                _ => runs.push((*k, 1)),
            }
        }
        w.u(runs.len() as u64);
        for (k, n) in runs {
            w.u(k as u64);
            w.u(n);
        }
        let with = self.entries.indexes();
        w.u(with.len() as u64);
        for (f, l) in with {
            w.s(f);
            w.gs(l.iter());
        }
        w.0
    }

    /// WHETHER A STATE IS THIS TWIN'S OWN: written by a twin of this shape — one an older twin wrote is folded again from the rows
    pub fn current(state: &[u8]) -> bool {
        state.starts_with(MAGIC)
    }
    /// A TWIN AGAIN FROM ITS STATE (`snapshot`), its entries read from `source`, and every row kept after it folded on — or why the
    /// state cannot be read
    pub fn restore(state: &[u8], source: Arc<dyn Source>) -> Result<Twin, String> {
        if !state.starts_with(MAGIC) {
            return Err("not a twin's state".into());
        }
        let mut r = R(state, MAGIC.len());
        let mut t = Twin::default();
        for _ in 0..r.n()? {
            let (id, kind, parent) = (r.s()?, r.s()?, r.s()?);
            let n = Node {
                id,
                kind,
                parent: (!parent.is_empty()).then_some(parent),
                who: r.map()?,
                value: r.map()?,
                accepted: r.map()?,
                at: r.gs()?,
                by: r.gs()?,
                named: r.gs()?,
            };
            let k = n.id.clone();
            if let Some(p) = n.parent.clone() {
                t.kids.entry(p).or_default().insert(k.clone());
            }
            t.nodes.insert(k, n);
        }
        for _ in 0..r.n()? {
            let g = r.i()?;
            t.answers.insert(g, r.gs()?);
        }
        for _ in 0..r.n()? {
            t.rejected.push_back(r.json()?);
        }
        let mut lists: Vec<OrdMap<String, Vector<i64>>> = vec![];
        for _ in 0..2 {
            let mut m = OrdMap::new();
            for _ in 0..r.n()? {
                let k = r.s()?;
                m.insert(k, r.gs()?);
            }
            lists.push(m);
        }
        t.force = lists.pop().unwrap();
        t.history = lists.pop().unwrap();
        for _ in 0..r.n()? {
            let k = r.s()?;
            t.readers.insert(k, r.gs()?.into_iter().collect());
        }
        let mut waits: Vec<OrdMap<i64, Vector<(String, i64)>>> = vec![];
        for _ in 0..2 {
            let mut m = OrdMap::new();
            for _ in 0..r.n()? {
                let g = r.i()?;
                m.insert(g, r.broken()?);
            }
            waits.push(m);
        }
        t.pending = waits.pop().unwrap();
        t.out = waits.pop().unwrap();
        for _ in 0..r.n()? {
            let g = r.i()?;
            t.rewritten.insert(g, r.gs()?);
        }
        t.present = r.gs()?.into_iter().collect();
        // the lines taken out — each with what took it out, where it was, by whom and its kind; the lines erased, each with its purge;
        // the last mark's hash
        for _ in 0..r.n()? {
            let g = r.i()?;
            let by = r.i()?;
            t.taken_out.insert(g, crate::twin::Out { by, at: r.s()?, writer: r.s()?, is: r.s()?, line: None });
        }
        for _ in 0..r.n()? {
            let g = r.i()?;
            t.erased.insert(g, r.i()?);
        }
        t.head = r.s()?;
        let names: Vec<String> = (0..r.n()?).map(|_| r.s()).collect::<Result<_, _>>()?;
        let len = r.n()?;
        let mut kinds: Vec<u32> = Vec::with_capacity(len.min(1 << 26));
        for _ in 0..r.n()? {
            let k = r.u()? as u32;
            let n = r.n()?;
            if k as usize > names.len() || kinds.len() + n > len {
                return Err("the state names a kind it does not hold".into());
            }
            kinds.extend(std::iter::repeat(k).take(n));
        }
        let mut with = OrdMap::new();
        for _ in 0..r.n()? {
            let f = r.s()?;
            with.insert(f, r.gs()?);
        }
        if kinds.len() != len || r.1 != state.len() {
            return Err("the state is not whole".into());
        }
        t.entries = Entries::restored(source.clone(), names, kinds);
        t.entries.set_indexes(with);
        // ROWS KEPT AFTER THE STATE: every one read on it as any row is (`Twin::read`), so a state behind its rows never reads the
        // record short
        let mut reading = crate::twin::Reading::default();
        let mut from = t.entries.top().max(t.taken_out.keys().next_back().copied().unwrap_or(0)) + 1;
        loop {
            let page = source.page(from, i64::MAX, false, 1024);
            let Some(last) = page.last().map(|(g, _)| *g) else { break };
            for (_, e) in &page {
                t.read(e, &mut reading).map_err(|why| format!("a row kept after the state: {why}"))?;
            }
            from = last + 1;
        }
        reading.done()?;
        t.entries.rows_kept();
        Ok(t)
    }
}
