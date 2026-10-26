//! THE TWIN: ONE GRAPH. A NODE has its own id, a kind and a name (names need not be unique). It comes to be by a `make` under its
//! parent — `{ by, at: <new id>, is: "make", refs: { in: <parent> }, value: { kind, keys? } }` — and only there; a person is a
//! root, made by its first key. A make says only what kind it is: its name and every other detail
//! are work after it. The parent holds what is under it; the OWNER is the person at the top of the chain; a node's LEVEL is its depth
//! from that person. AN ENTRY is an act by one node at another, numbered (g), naming nodes by id and other entries by their
//! numbers. Naming never creates: every node an entry names was made before.
//!
//! EACH LEVEL HOLDS THE LEVEL BELOW IT. Nothing is kept as a verdict: what is accepted is what the acceptances in force NOW let in.
//!   1. A node works freely at itself and below. At or on anything else — beside it, above it, another owner's — only under a grant.
//!   2. An ACCEPTANCE at a node is written by the level above that node; only that level changes it (`re`). A change to an
//!      acceptance is a change like any other: it comes into force only if every acceptance in force at its writer and above that
//!      holds still holds with it.
//!   3. WHAT IS ACCEPTED is all work (`be`: details of a node), in order, against the acceptances in force now: a piece of work is in
//!      when every acceptance in force at its maker and above that holds at that point still holds with it.
//!   4. A change that is not in WAITS at the nearest holder of what it breaks — until that holder writes that acceptance after it,
//!      changed (to let it in) or as it is (to keep it out); then it is decided there, and no longer waits.
//!   5. Whatever is open is OWED: an acceptance in force that does not hold, to its node — or, when work at or below that node that
//!      would meet it is decided out, to its holder, to change it; a change that waits, to the holder it waits at.
//! ONLY WHAT IS RELEVANT IS READ: work changes only the node it is at, and an acceptance reads only the nodes its condition names; so
//! whether work is in depends only on that node's own history and the acceptances that read it, and an acceptance change reads again
//! only the history of the nodes it reads.
//! WHAT AN ACCEPTANCE SAYS (its `value`) is only its condition — a note beside it (`means`) is read by nobody here — one of:
//!   STATE  `expect: { at, has }` — the node at `at` has, as accepted, every detail in `has`;
//!   EVENT  `when: { is, at, by?, below? }, expect: { is?, from? }, since?` — every entry at `at` (or, with `below`, at any node
//!          beneath it too) that is `is` (and by `by`) has, among what came of it (what answers it, and what answers that), one that is
//!          `is` and is from `from` or below it. With `since`, only what happened after the acceptance itself is asked: a rule written
//!          today is not owed for every time before it.
//! Work (`be`) can break a STATE; an EVENT is never broken by work — it is false until what it expects comes, and owed until then.
//! A hold that reads, of each act, what it acts on (`of: "re"`) and is `strict` is never broken at all: an act it forbids is refused
//! when it is written — whatever program wrote it. That is how a level keeps a rule over the one below that no program can skip.
//! A node's details as written are `value`; as accepted, `accepted`. Whether an acceptance holds is read off what is accepted.
//!
//! A DECISION IS A THING MOVING. What a level needs from above it is an ASK: written by the asker at itself, about a thing (`refs.about`),
//! with a `label` and the moves that would settle it (`moves`: each `{ says, to }`, `to` the details the thing would then have). It is
//! HELD — open, and owed to its holder — until the thing has, as accepted, every detail of one of its moves (after the ask), or its
//! holder declines it (`decline`, `re` it). An answer is not the end; the move is.
//!   ONE ASK PER THING AND LABEL: an ask about a thing, with a label, while one like it is open, is evidence on that one — never a
//!   second decision.
//!   WHO HOLDS IT is read from the tree, never from code: a FLOW — a node's detail `flow:<label>`, so setting one is a move like any
//!   other — says, for what is beneath it, who moves asks of that label (`by`), and whether a person must be there (`present`). The nearest flow for the label from the
//!   thing up decides — but a flow with `mine` above it keeps that label with its writer, whatever is below; and a hand that already
//!   moved this thing for this label, with the same ask raised again since, is passed over: it goes up. With no flow, the root holds it.
//!   A HAND MAY MAKE ONLY THE MOVE: the holder of an open ask may write, at its thing — even beside itself — exactly one of its moves,
//!   `re` the ask; nothing else, nowhere else.
//! A GRANT LETS ONE NODE ACT ON WHAT IS NOT ITS OWN. An act at a node that is not its writer's, or on one (`refs.on`: reading it,
//! calling it), is taken only under a `grant` in force on that node: written at it by its holder or one above,
//! `{ to, action, below?, until?, budget? }` — to whom (that node and below), which act (the act's kind), whether it reaches what is
//! below that node too (`below`; without it, the node itself and nothing under it — your key on a client's org node opens that node
//! only), until when, and how much (`budget`: the sum of one of the act's values, over a window). A `revoke` answering it, by its
//! writer or above, ends it. A mention is never a grant: running something, or naming it, gives no reach into it.
//! A SEAL SAYS WHAT CODE A NODE RUNS: `{ wasm, cwasm: { <machine>: hash }, contract, reach }`, written at the node by a level above
//! it — never by the node itself, by what is under it or beside it, and no grant opens it, so a level never seals itself. The newest
//! seal at a node is what runs there (`sealed`); going back is sealing the one before again. Nothing runs at a node deleted or blocked.
//! AN OVERLAY of the twin (`overlay`) takes entries that are never kept: it shares everything the twin holds and copies only what an
//! entry lands on, so what a change would do is read exactly as from the twin, at the cost of what it touches.
//! DELETING: a `delete` at a node hides it and everything under it at once — no act lands at or on it,
//! nothing about it is asked or owed — and a `restore` answering it brings it back. It is taken only once the owner said how long a
//! deletion waits (`keep_deleted`, ms, a detail of the node or one above it). When that is over, a `purge` answering it erases what it held: every entry at it and under it but the makes, the delete and its answers (`erasing`).
//! What stays is the skeleton: ids and kinds, never a name. The purge keeps the signatures it erased, so none is taken again.
//! IDENTITY KINDS: how something outside proves who it is is the Identity seam's (`seams`); what is let in by a kind names it
//! (`refs.as` on its make). A `block` at a node means it, and all under it, signs nothing from then on.
//! GETTING BACK IN: a synced passkey is the same key on a new device — nothing to do. For the rest, a person grants their own ways
//! back: a `grant` at themselves, `action: "take over"`, to a person they trust, a device of theirs, their recovery code (a node of
//! that kind, Taldo's own), or a proof of theirs (a node let in by an identity kind: an email) — never an org, an agent, a machine or a sandbox, so whoever runs their Taldo never
//! becomes them. A `takeover` at the person, signed by a granted way and carrying only `keys`, brings a key that signs for them a day
//! after it arrived (`TAKEOVER_WAITS`) — unless they `decline` it first, from any key of theirs.
//! A RESPONSIBILITY is a node like any other; who carries it now is its detail `carried_by` (people's ids). Whoever carries it signs
//! as it, and an ask it holds is asked of them — when it moves to someone else, its stages go with it. Reading is an act
//! like any other: whoever gets an item's key gets it in answer to an act the core took, so no program hands a key out unseen.
use crate::entry::{check, ids_of, parse_id};
use crate::rows::{Entries, Source};
use serde_json::{json, Map, Value};
use imbl::{OrdMap, OrdSet, Vector};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

/// HOW LONG A KEY TAKEN OVER WAITS before it signs: a day, so the person sees it on their other devices and can stop it
pub const TAKEOVER_WAITS: i64 = 24 * 60 * 60 * 1000;

#[derive(Debug, Clone, Default)]
pub struct Node {
    pub id: String,
    pub kind: String,
    /// what holds it: none for a person
    pub parent: Option<String>,
    /// its details as written, the latest of each
    pub value: Map<String, Value>,
    /// its details as accepted
    pub accepted: Map<String, Value>,
    /// the entries at it, by it, and naming it
    pub at: Vector<i64>,
    pub by: Vector<i64>,
    pub named: Vector<i64>,
}

#[derive(Clone, Default)]
pub struct Twin {
    pub nodes: OrdMap<String, Node>,
    pub kids: OrdMap<String, OrdSet<String>>,
    /// the entries taken, by number: all in memory when folded from a list, else read from their rows (`rows`)
    pub entries: Entries,
    /// the entries that point to an entry, by any role
    pub answers: OrdMap<i64, Vector<i64>>,
    /// entries that are not entries, and why
    pub rejected: Vector<Value>,
    /// each node's own work, in order
    pub(crate) history: OrdMap<String, Vector<i64>>,
    /// the acceptances in force at each node
    pub(crate) force: OrdMap<String, Vector<i64>>,
    /// for each node, the acceptances in force that read it
    pub(crate) readers: OrdMap<String, OrdSet<i64>>,
    /// work that is not in, with what it breaks
    pub(crate) out: OrdMap<i64, Vector<(String, i64)>>,
    /// acceptance changes not in force, with what each breaks
    pub(crate) pending: OrdMap<i64, Vector<(String, i64)>>,
    /// for each acceptance, the acceptances written since in its place (`re` it), in force or not
    pub(crate) rewritten: OrdMap<i64, Vector<i64>>,
    /// every signature taken: an act signed once is taken once
    pub(crate) signatures: OrdSet<String>,
    /// the entries signed with a person there (`identity`) — a fact for holds to read, never required here
    pub(crate) present: OrdSet<i64>,
    /// the signatures of every act a purge erased, in their standard spelling: never taken again
    pub(crate) erased: OrdSet<String>,
    /// when each take-over arrived: its key signs a day after
    pub(crate) arrivals: OrdMap<i64, i64>,
}

impl Twin {
    /// AN EMPTY TWIN whose entries are kept as rows in `source`: only what the current state needs stays in memory
    pub fn stored(source: Arc<dyn Source>) -> Twin {
        Twin {
            entries: Entries::stored(source),
            ..Default::default()
        }
    }
}

impl Twin {
    /// AN OVERLAY: this twin as it is now, for entries that are never kept — landed on it, they show what they would do, read exactly
    /// as from a twin. It shares everything it holds with this twin and copies only what an entry lands on: making one costs nothing,
    /// each entry costs only what it touches, and this twin is never changed by it. Its entries are new — numbered above every entry
    /// this twin has — and stay in memory, since no row of them exists
    pub fn overlay(&self) -> Twin {
        let mut o = self.clone();
        o.entries.overlay();
        o
    }
}

pub fn fold(entries: &[Value]) -> Twin {
    let mut t = Twin::default();
    for e in entries {
        t.apply(e);
    }
    t
}

/// the entry numbers an entry points to
pub fn numbers(e: &Value) -> Vec<i64> {
    e["refs"]
        .as_object()
        .into_iter()
        .flat_map(|o| o.values())
        .filter_map(Value::as_i64)
        .collect()
}
fn id_of(v: &Value) -> String {
    parse_id(v).unwrap_or_default()
}
/// THE NODES A CONDITION READS: the node its state is about, or the node its events happen at
fn reads(c: &Value) -> Vec<String> {
    vec![id_of(if is_event(c) { &c["when"]["at"] } else { &c["expect"]["at"] })]
}
fn is_event(c: &Value) -> bool {
    c["when"].is_object()
}
/// A STATE: the node at `at` has, as accepted, every detail in `has` (an event is not read here: work never breaks it)
fn meets(c: &Value, details: &dyn Fn(&str) -> Map<String, Value>) -> bool {
    if is_event(c) {
        return true;
    }
    let Some(has) = c["expect"]["has"].as_object() else {
        return false;
    };
    let d = details(&id_of(&c["expect"]["at"]));
    has.iter().all(|(k, x)| d.get(k) == Some(x))
}
/// WHAT GOVERNS A NODE, as a detail: the keys it signs with, who carries it, what stays when it is deleted — set only by it or above it
pub const GOVERNING: [&str; 3] = ["keys", "carried_by", "keep_deleted"];
/// A SEAL IS WHOLE: `wasm`, the module's hash; `cwasm`, its ready machine code's hash for each machine; `contract`, what it was built
/// to; `reach`, what it may call — and nothing else
fn seal_whole(v: &Value) -> Result<(), String> {
    let hash = |x: &Value| x.as_str().is_some_and(|h| h.len() == 64 && h.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
    let Some(o) = v.as_object() else { return Err("a seal says its module, its machine code, its contract and its reach".into()) };
    if let Some(k) = o.keys().find(|k| !matches!(k.as_str(), "wasm" | "cwasm" | "contract" | "reach")) {
        return Err(format!("a seal says only its module, machine code, contract and reach — not {k}"));
    }
    if !hash(&v["wasm"]) {
        return Err("a seal's wasm is its module's sha-256, in hex".into());
    }
    let machines = v["cwasm"].as_object().filter(|m| !m.is_empty() && m.iter().all(|(k, h)| !k.is_empty() && hash(h)));
    if machines.is_none() {
        return Err("a seal's cwasm is, for each machine, its ready code's sha-256 in hex".into());
    }
    if v["contract"].as_str().is_none_or(|c| c.trim().is_empty()) {
        return Err("a seal names the contract it was built to".into());
    }
    if !v["reach"].as_array().is_some_and(|r| r.iter().all(|x| x.as_str().is_some_and(|x| !x.trim().is_empty()))) {
        return Err("a seal's reach is the list of what it may call".into());
    }
    Ok(())
}

fn laid(mut d: Map<String, Value>, e: &Value) -> Map<String, Value> {
    for (k, x) in e["value"].as_object().cloned().unwrap_or_default() {
        d.insert(k, x);
    }
    d
}

impl Twin {
    /// WHAT HOLDS A NODE: its parent — none for a person, or a node never made
    pub fn parent(&self, id: &str) -> Option<String> {
        self.nodes.get(id)?.parent.clone()
    }
    /// A NODE AND EVERYTHING ABOVE IT, nearest first, up to its person
    pub fn chain(&self, id: &str) -> Vec<String> {
        let mut out = vec![];
        let mut at = self.nodes.get(id).map(|n| n.id.clone());
        while let Some(x) = at {
            if out.contains(&x) {
                break;
            }
            at = self.parent(&x);
            out.push(x);
        }
        out
    }
    /// WHETHER `id` IS `top` OR UNDER IT
    pub fn within(&self, id: &str, top: &str) -> bool {
        id == top || self.chain(id).iter().any(|x| x == top)
    }
    /// THE OWNER: the person at the top of the chain
    pub fn owner(&self, id: &str) -> Option<String> {
        self.chain(id).pop()
    }
    /// A NODE'S LEVEL: its depth from its person (a person is 0)
    pub fn level(&self, id: &str) -> Option<usize> {
        self.nodes.contains_key(id).then(|| self.chain(id).len() - 1)
    }
    /// A NODE'S NAME: its detail `name` — never in its make, so erasing what it held leaves no name
    pub fn name(&self, id: &str) -> Option<String> {
        self.nodes.get(id)?.value.get("name")?.as_str().map(String::from)
    }
    /// a delete brought back: a `restore` answering it
    fn restored(&self, d: i64) -> bool {
        self.answers.get(&d).into_iter().flatten().any(|x| self.is(*x) == "restore")
    }
    /// THE DELETES IN FORCE AT A NODE: not brought back
    fn deletes_at(&self, id: &str) -> Vec<i64> {
        self.at_kind(id, "delete").map(|(g, _)| g).filter(|g| !self.restored(*g)).collect()
    }
    /// WHETHER A NODE IS DELETED: it, or a node above it, has a delete in force
    pub fn deleted(&self, id: &str) -> bool {
        self.chain(id).iter().any(|x| !self.deletes_at(x).is_empty())
    }
    /// HOW LONG A DELETION WAITS before it is erased: the nearest `keep_deleted` from the node up
    fn keep_for(&self, id: &str) -> Option<i64> {
        self.chain(id).iter().find_map(|x| self.nodes.get(x)?.value.get("keep_deleted")?.as_i64())
    }
    /// A NODE AND EVERYTHING UNDER IT
    fn subtree(&self, id: &str) -> Vec<String> {
        let (mut out, mut next) = (vec![], vec![id.to_string()]);
        while let Some(x) = next.pop() {
            next.extend(self.children(&x));
            out.push(x);
        }
        out
    }
    /// WHAT A PURGE OF DELETE `d` ERASES: every entry at its node and under it, but the makes, the delete and what answers it
    pub fn erasing(&self, d: i64) -> Vec<i64> {
        let Some(e) = self.entries.get(&d) else { return vec![] };
        let mut out: Vec<i64> = self
            .subtree(&id_of(&e["at"]))
            .iter()
            .filter_map(|x| self.nodes.get(x))
            .flat_map(|n| n.at.iter().copied())
            .filter(|g| !matches!(self.is(*g), "make" | "delete" | "restore" | "purge"))
            .collect();
        out.sort();
        out
    }
    /// THE CODE A NODE RUNS: its newest seal — said by a level above it — or none; nothing runs at a node deleted or blocked
    pub fn sealed(&self, node: &str) -> Option<i64> {
        if self.deleted(node) || self.blocked(node) {
            return None;
        }
        self.at_kind(node, "seal").next_back().map(|(g, _)| g)
    }
    /// WHETHER A NODE IS BLOCKED: a `block` at it or above it
    pub fn blocked(&self, id: &str) -> bool {
        self.chain(id).iter().any(|x| self.at_kind(x, "block").next().is_some())
    }
    /// WHETHER `proof` IS A WAY BACK `person` GRANTED: a take-over grant at them, to it, not revoked
    pub(crate) fn takeover_granted(&self, person: &str, proof: &str) -> bool {
        self.at_kind(person, "grant").any(|(g, e)| e["value"]["action"] == "take over" && id_of(&e["value"]["to"]) == proof && !self.revoked(g))
    }
    /// THE KEYS TAKEN OVER FOR A NODE that sign by `arrived`: each take-over at it a day old by then, not declined
    pub(crate) fn taken_over(&self, id: &str, arrived: i64) -> Vec<Value> {
        self.at_kind(id, "takeover")
            .filter(|(g, _)| self.arrivals.get(g).is_some_and(|a| a + TAKEOVER_WAITS <= arrived))
            .filter(|(g, _)| !self.answers.get(g).into_iter().flatten().any(|x| self.is(*x) == "decline"))
            .flat_map(|(_, e)| e["value"]["keys"].as_array().cloned().unwrap_or_default())
            .collect()
    }
    /// THE KIND A NODE WAS LET IN BY: the `identity` node its make names (`refs.as`)
    fn let_in_as(&self, id: &str) -> Option<String> {
        let g = *self.nodes.get(id)?.at.front()?;
        parse_id(&self.e(g)["refs"]["as"])
    }
    /// WHO CARRIES A RESPONSIBILITY NOW: its detail `carried_by`
    pub fn carriers(&self, id: &str) -> Vec<String> {
        self.nodes.get(id).and_then(|n| n.value.get("carried_by")).map(ids_of).unwrap_or_default()
    }
    /// ENTRY `g`, which is taken (read from its row when the twin keeps rows)
    fn e(&self, g: i64) -> Arc<Value> {
        self.entries.get(&g).expect("an entry the twin took")
    }
    /// WHAT ENTRY `g` IS (its kind), without reading it — "" when it is no entry
    pub fn is(&self, g: i64) -> &str {
        self.entries.kind_of(g).unwrap_or("")
    }
    /// THE ENTRIES BY A NODE (`by` exactly that node) of a kind, read: oldest first (`.rev()` for newest first)
    pub fn by_kind<'a>(&'a self, node: &str, is: &'a str) -> impl DoubleEndedIterator<Item = (i64, Arc<Value>)> + 'a {
        let gs = self.nodes.get(node).into_iter().flat_map(|n| n.by.iter().copied());
        gs.filter(move |g| self.is(*g) == is).filter_map(move |g| self.entries.get(&g).map(|e| (g, e)))
    }
    /// THE ENTRIES AT A NODE (`at` exactly that node) of a kind, read: oldest first (`.rev()` for newest first)
    pub fn at_kind<'a>(&'a self, node: &str, is: &'a str) -> impl DoubleEndedIterator<Item = (i64, Arc<Value>)> + 'a {
        let gs = self.nodes.get(node).into_iter().flat_map(|n| n.at.iter().copied());
        gs.filter(move |g| self.is(*g) == is).filter_map(move |g| self.entries.get(&g).map(|e| (g, e)))
    }
    /// ENTRY `g`, known to be taken — read from its row when the twin keeps rows (`entries.get` when it may not be)
    pub fn entry(&self, g: i64) -> Arc<Value> {
        self.e(g)
    }
    /// an acceptance's condition: all it says
    fn expect(&self, a: i64) -> Value {
        self.e(a)["value"].clone()
    }
    /// AN EVENT: every entry at `when.at` that is `when.is` (by `when.by`) has, among what came of it, one that is `expect.is` from
    /// `expect.from` or below (and, when `expect.present` is named, signed with a person there or not). With `of: "re"`, it is the
    /// entry each one acts on (its `re`) that must be so, not what came of it
    fn happened(&self, a: i64) -> bool {
        self.unmet(a).is_empty()
    }
    /// WHAT AN EVENT ACCEPTANCE ASKS ABOUT: every entry at its node (or beneath it, with `below`) of its kind and writer — with
    /// `since`, only those after the acceptance itself — oldest first
    pub fn asked(&self, a: i64) -> Vec<i64> {
        let Some(e) = self
            .entries
            .get(&a)
            .filter(|e| e["is"] == "acceptance" && is_event(&e["value"]))
        else {
            return vec![];
        };
        let w = &e["value"]["when"];
        let after = if e["value"]["since"] == true {
            a
        } else {
            i64::MIN
        };
        let top = id_of(&w["at"]);
        let (mut nodes, mut next) = (vec![], vec![top]);
        while let Some(k) = next.pop() {
            if w["below"] == true {
                next.extend(self.children(&k));
            }
            nodes.push(k);
        }
        let mut out: Vec<i64> = nodes
            .iter()
            .filter_map(|k| self.nodes.get(k))
            .flat_map(|n| n.at.iter().copied())
            .filter(|g| {
                *g > after
                    && self.entries.kind_of(*g) == w["is"].as_str()
                    && (w["by"].is_null() || self.e(*g)["by"] == w["by"])
            })
            .collect();
        out.sort();
        out
    }
    /// the entries an event acceptance asks about that have not yet had what it expects come of them
    fn unmet(&self, a: i64) -> Vec<i64> {
        let c = self.expect(a);
        let x = &c["expect"];
        let asked = self.asked(a);
        let asked = asked.iter();
        let came = |g: i64| {
            let (mut seen, mut next) = (BTreeSet::new(), vec![g]);
            while let Some(y) = next.pop() {
                for a in self.answers.get(&y).into_iter().flatten() {
                    if seen.insert(*a) {
                        next.push(*a);
                    }
                }
            }
            seen
        };
        if c["of"] == "re" {
            return asked
                .copied()
                .filter(|g| {
                    !self.e(*g)["refs"]["re"]
                        .as_i64()
                        .is_some_and(|r| self.fits(x, r))
                })
                .collect();
        }
        asked
            .copied()
            .filter(|g| !came(*g).iter().any(|a| self.fits(x, *a)))
            .collect()
    }
    /// an entry fits what a condition expects: its kind, who it is from, and whether a person was there
    fn fits(&self, x: &Value, g: i64) -> bool {
        let Some(e) = self.entries.get(&g) else {
            return false;
        };
        (x["is"].is_null() || e["is"] == x["is"])
            && (x["from"].is_null() || self.within(&id_of(&e["by"]), &id_of(&x["from"])))
            && (x["present"].is_null() || x["present"] == json!(self.present.contains(&g)))
    }
    /// WHETHER A PERSON WAS THERE when this entry was signed
    pub fn present(&self, g: i64) -> bool {
        self.present.contains(&g)
    }
    /// BEFORE ACTING: the holds over `e`'s writer that it would make false, were it written — each that reads, of the entry it acts on,
    /// what it must be
    pub fn would_break(&self, e: &Value) -> Vec<i64> {
        let by = id_of(&e["by"]);
        let at = id_of(&e["at"]);
        self.force
            .values()
            .flatten()
            .copied()
            .filter(|a| self.over(*a, &by))
            .filter(|a| {
                let c = self.expect(*a);
                let w = &c["when"];
                let here = if w["below"] == true {
                    self.within(&at, &id_of(&w["at"]))
                } else {
                    id_of(&w["at"]) == at
                };
                c["of"] == "re"
                    && w["is"] == e["is"]
                    && here
                    && (w["by"].is_null() || w["by"] == e["by"])
                    && !e["refs"]["re"]
                        .as_i64()
                        .is_some_and(|r| self.fits(&c["expect"], r))
            })
            .collect()
    }
    /// WHAT AN EVENT ACCEPTANCE STILL WAITS FOR: the entries it asks about that nothing it expects has come of yet
    pub fn unanswered(&self, a: i64) -> Vec<i64> {
        self.entries
            .get(&a)
            .filter(|e| e["is"] == "acceptance" && is_event(&e["value"]))
            .map(|_| self.unmet(a))
            .unwrap_or_default()
    }
    fn accepted_of(&self, node: &str) -> Map<String, Value> {
        self.nodes
            .get(node)
            .map(|n| n.accepted.clone())
            .unwrap_or_default()
    }
    /// whether acceptance `a` stands over work by `maker`: it is in force at the maker or above
    fn over(&self, a: i64, maker: &str) -> bool {
        self.within(maker, &id_of(&self.e(a)["at"]))
    }

    /// ONE ENTRY LANDS: at its node, by its node, naming what it names — returns what it touched. Folded from a list, an entry
    /// arrives when its writer says it was written (`ts`)
    pub fn apply(&mut self, e: &Value) -> BTreeSet<String> {
        self.apply_arrived(e, e["ts"].as_i64().unwrap_or(0))
    }
    /// ONE ENTRY LANDS, arrived at `arrived` (ms, the receiver's clock): taken, or kept among the rejected with why
    pub fn apply_arrived(&mut self, e: &Value, arrived: i64) -> BTreeSet<String> {
        match self.admit(e, arrived) {
            Ok(present) => self.land(e, present, arrived),
            Err(why) => {
                self.rejected.push_back(why);
                BTreeSet::new()
            }
        }
    }
    /// WHETHER AN ENTRY WOULD BE TAKEN, arrived at `arrived` — every check made before anything changes: taken, with whether a person
    /// was there, or refused with why (`{ g, why, … }`). The twin is untouched either way: an entry refused leaves no trace, its
    /// signature included
    pub fn admit(&self, e: &Value, arrived: i64) -> Result<bool, Value> {
        // the entry is well formed, at its writer or below (`check`), and from its writer (`identity`) — and taken once
        let present = match check(e).and_then(|_| crate::identity::verify(self, e, arrived)) {
            Ok(p) => p,
            Err(why) => return Err(json!({ "g": e["g"], "why": why })),
        };
        // AN OVERLAY OF ROWS takes only new acts: numbered above every row it was made on
        if let Some(to) = self.entries.overlay_of_rows() {
            if e["g"].as_i64().is_none_or(|g| g <= to) {
                return Err(json!({ "g": e["g"], "why": format!("an overlay takes only new acts, numbered above {to}") }));
            }
        }
        // TAKEN ONCE, recognised by its signature's standard spelling: the same act signed once, written another way, is the same act
        if let Some(s) = e["sig"]["sig"].as_str().and_then(crate::identity::canonical_sig).as_deref() {
            let taken = match self.entries.source() {
                None => self.signatures.contains(s),
                // an overlay's own acts have no rows: it knows their signatures itself
                Some(src) => {
                    src.signed(s).is_some_and(|x| Some(x) != e["g"].as_i64() && self.entries.contains_key(&x))
                        || self.signatures.contains(s)
                }
            };
            if taken {
                return Err(json!({ "g": e["g"], "why": "this signed act was already taken" }));
            }
            if self.erased.contains(s) {
                return Err(json!({ "g": e["g"], "why": "this act was erased: it is never taken again" }));
            }
        }
        // A MOVE A PERSON MUST MAKE: work settling an open ask whose flow says a person must be there is taken only signed with one
        if e["is"] != "decline" && e["is"] != "ask" && !present {
            if let Some(g) = e["refs"]["re"]
                .as_i64()
                .filter(|g| self.ask_open(*g) && self.needs_present(*g) && self.tries_move(e, *g))
            {
                return Err(json!({ "g": e["g"], "why": "this move needs you there — your passkey", "ask": g }));
            }
        }
        // A STRICT HOLD IS NEVER BROKEN: an act a strict hold over its writer forbids is not taken at all — whoever wrote the program that
        // sent it. Not owed later, not waiting: refused here, with which hold refused it
        if e["is"] != "acceptance" {
            let strict: Vec<i64> = self
                .would_break(e)
                .into_iter()
                .filter(|a| self.expect(*a)["strict"] == true)
                .collect();
            if let Some(a) = strict.first() {
                return Err(json!({ "g": e["g"], "why": format!("a strict hold forbids it: {}", self.expect(*a)["says"].as_str().unwrap_or("a hold of the level above")), "hold": a }));
            }
        }
        // NAMING NEVER CREATES: every node an entry names was made before — by a `make` under its parent. A person comes to be by its
        // first key (`identity`)
        let (by, at) = (id_of(&e["by"]), id_of(&e["at"]));
        let root = !self.nodes.contains_key(&by) && by == at && e["is"] == "be";
        if !root && !self.nodes.contains_key(&by) {
            return Err(json!({ "g": e["g"], "why": format!("{by} was never made") }));
        }
        let mut named: Vec<String> = e["refs"].as_object().into_iter().flatten().flat_map(|(_, v)| ids_of(v)).collect();
        if e["is"] == "make" {
            // A MAKE brings a new id under a parent that is: the writer's own, or one a grant to make there reaches
            if self.nodes.contains_key(&at) {
                return Err(json!({ "g": e["g"], "why": format!("{at} is already in the graph") }));
            }
            let Some(parent) = parse_id(&e["refs"]["in"]) else {
                return Err(json!({ "g": e["g"], "why": "a make names what it is made in (refs.in)" }));
            };
            if !self.nodes.contains_key(&parent) {
                return Err(json!({ "g": e["g"], "why": format!("{parent} was never made — make what it belongs to first") }));
            }
            if !self.within(&parent, &by) {
                if let Err(why) = self.granted(e, &by, &parent, arrived) {
                    return Err(json!({ "g": e["g"], "why": why }));
                }
            }
        } else if !root {
            named.push(at.clone());
        }
        if let Some(x) = named.iter().find(|x| !self.nodes.contains_key(*x)) {
            return Err(json!({ "g": e["g"], "why": format!("{x} was never made — make it under what it belongs to first") }));
        }
        // DELETING: with a person there, once the owner said how long a deletion waits; nothing lands at or on what is deleted
        match e["is"].as_str().unwrap_or("") {
            "delete" if self.keep_for(&at).is_none() => {
                return Err(json!({ "g": e["g"], "why": format!("say how long a deletion waits before it is erased (keep_deleted, on {at} or above it)") }));
            }
            "restore" | "purge" => {
                let d = e["refs"]["re"].as_i64().filter(|d| self.is(*d) == "delete" && self.entries.get(d).is_some_and(|x| id_of(&x["at"]) == at));
                let Some(d) = d.filter(|d| !self.restored(*d) && !self.answers.get(d).into_iter().flatten().any(|x| self.is(*x) == "purge")) else {
                    return Err(json!({ "g": e["g"], "why": format!("no delete in force at {at} for it to answer") }));
                };
                if e["is"] == "purge" {
                    let due = self.e(d)["ts"].as_i64().unwrap_or(0) + self.keep_for(&at).unwrap_or(i64::MAX - self.e(d)["ts"].as_i64().unwrap_or(0));
                    if arrived < due {
                        return Err(json!({ "g": e["g"], "why": format!("the deletion of {at} waits until {due}") }));
                    }
                    let said: BTreeSet<&str> = e["value"]["sigs"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
                    if let Some(g) = self.erasing(d).into_iter().find(|g| !self.e(*g)["sig"]["sig"].as_str().is_some_and(|s| said.contains(s))) {
                        return Err(json!({ "g": e["g"], "why": format!("the purge does not name #{g}, which it erases") }));
                    }
                }
            }
            _ => {
                let mut hit: Vec<String> = parse_id(&e["refs"]["on"]).into_iter().collect();
                hit.push(if e["is"] == "make" { id_of(&e["refs"]["in"]) } else { at.clone() });
                if let Some(x) = hit.iter().find(|x| self.nodes.contains_key(*x) && self.deleted(x)) {
                    return Err(json!({ "g": e["g"], "why": format!("{x} is deleted") }));
                }
            }
        }
        // BLOCKED: what is blocked signs nothing from then on
        if self.nodes.contains_key(&by) && self.blocked(&by) {
            return Err(json!({ "g": e["g"], "why": format!("{by} is blocked: it signs nothing from then on") }));
        }
        if e["is"] == "grant" && e["value"]["action"] == "take over" {
            let to = id_of(&e["value"]["to"]);
            let ok = self.nodes.get(&to).is_some_and(|n| matches!(n.kind.as_str(), "person" | "device" | "recovery code")) || self.let_in_as(&to).is_some();
            if !ok {
                return Err(json!({ "g": e["g"], "why": format!("a take-over goes only to a person, a device, a recovery code or a proof of yours — never an org, an agent, a machine or a sandbox ({to})") }));
            }
        }
        if e["is"] == "takeover" {
            let only_keys = e["value"].as_object().is_some_and(|v| v.len() == 1 && v.get("keys").is_some_and(Value::is_array));
            if at != by || !only_keys {
                return Err(json!({ "g": e["g"], "why": "a take-over brings only keys, at the one it takes over for" }));
            }
        }
        // A SEAL says what code a node runs. Only a level above the node says it — never the node itself, what is under it or beside
        // it, and no grant opens it: a level never seals itself. It is whole: the module's hash, its ready machine code's hash for each
        // machine, the contract it was built to, and what it reaches
        if e["is"] == "seal" {
            if at == by || !self.within(&at, &by) {
                return Err(json!({ "g": e["g"], "why": format!("only a level above {at} seals it") }));
            }
            if let Err(why) = seal_whole(&e["value"]) {
                return Err(json!({ "g": e["g"], "why": why }));
            }
        }
        if e["is"] == "make" {
            if let Some(k) = e["value"].as_object().into_iter().flatten().map(|(k, _)| k).find(|k| *k != "kind" && *k != "keys") {
                return Err(json!({ "g": e["g"], "why": format!("a make says only its kind (and keys) — its {k} is a detail after it") }));
            }
        }
        // WHAT GOVERNS A NODE — the keys it signs with, who carries it, what stays when it is deleted — is changed only by the node itself
        // or a node above it, never under a grant: a grant to write a node's details is never a grant to become it (a node made under a
        // grant is given its first keys by its maker, as ever)
        if e["is"] == "be" {
            if let Some(k) = GOVERNING.iter().find(|k| e["value"].get(**k).is_some()) {
                if !root && !self.within(&at, &by) {
                    return Err(json!({ "g": e["g"], "why": format!("the {k} of {at} is set only by it or a node above it — never under a grant") }));
                }
            }
        }
        // CODE NEVER GOVERNS: a node with code sealed at it never changes what governs a node — not even its own, so no outside key ever
        // signs as it — never makes who is who (a person, a device, a recovery code) and never seals: people and the levels above them do
        if self.sealed(&by).is_some() {
            if let Some(k) = (e["is"] == "be").then(|| GOVERNING.iter().find(|k| e["value"].get(**k).is_some())).flatten() {
                return Err(json!({ "g": e["g"], "why": format!("{by} runs sealed code: it never sets the {k} of {at}") }));
            }
            if e["is"] == "make" && e["value"]["kind"].as_str().is_some_and(|k| matches!(k, "person" | "device" | "recovery code")) {
                return Err(json!({ "g": e["g"], "why": format!("{by} runs sealed code: it never makes a {}", e["value"]["kind"]) }));
            }
            // nor gives keys any other way — a take-over, a node made with them, a grant to take over — nor seals code: what runs is
            // said by a level above, never by code
            let keys = e["is"] == "takeover" || (e["is"] == "make" && e["value"].get("keys").is_some()) || (e["is"] == "grant" && e["value"]["action"] == "take over");
            if keys || e["is"] == "seal" {
                return Err(json!({ "g": e["g"], "why": format!("{by} runs sealed code: it never writes a {} that {}", e["is"], if keys { "gives keys" } else { "says what code runs" }) }));
            }
        }
        // AN ACT AT, OR ON, WHAT IS NOT YOURS (beside you, above you, another owner's): taken only under a grant in force on that node
        // that names this act and its writer, not run out, within its budget — or refused, with why
        let mut not_own: Vec<String> = parse_id(&e["refs"]["on"]).into_iter().collect();
        if e["is"] != "make" && !root {
            not_own.push(at.clone());
        }
        for x in not_own.iter().filter(|x| !self.within(x, &by)) {
            if let Err(why) = self.granted(e, &by, x, arrived) {
                return Err(json!({ "g": e["g"], "why": why }));
            }
        }
        Ok(present)
    }
    /// THE GRANTS IN FORCE ON A NODE: each `grant` at it, and each at a node above it that reaches below (`below`) — not revoked since
    /// by its writer or a node above them
    pub fn grants_on(&self, on: &str) -> Vec<i64> {
        self.chain(on)
            .iter()
            .enumerate()
            .flat_map(|(i, x)| self.at_kind(x, "grant").filter(move |(_, e)| i == 0 || e["value"]["below"] == true).map(|(g, _)| g).collect::<Vec<_>>())
            .filter(|g| !self.revoked(*g))
            .collect()
    }
    /// a grant revoked: a `revoke` answering it, by its writer or a node above them
    fn revoked(&self, g: i64) -> bool {
        let writer = id_of(&self.e(g)["by"]);
        self.answers
            .get(&g)
            .into_iter()
            .flatten()
            .any(|x| self.is(*x) == "revoke" && self.within(&writer, &id_of(&self.e(*x)["by"])))
    }
    /// HOW MUCH OF A BUDGET IS SPENT: `field` added over every act of kind `is` by `to` or under it on `on`, since `from` (by the acts'
    /// own time) — what a grant's budget is held against, and what a screen says of it
    pub fn spent(&self, is: &str, to: &str, on: &str, field: &str, from: Option<i64>) -> f64 {
        self.nodes
            .get(on)
            .map(|n| n.named.clone())
            .unwrap_or_default()
            .into_iter()
            .filter(|x| self.is(*x) == is)
            .map(|x| self.e(x))
            .filter(|x| self.within(&id_of(&x["by"]), to) && id_of(&x["refs"]["on"]) == on)
            .filter(|x| from.map_or(true, |f| x["ts"].as_i64().is_some_and(|t| t >= f)))
            .filter_map(|x| x["value"][field].as_f64())
            .sum()
    }
    /// MAY `by` DO `is` ON `on`, at `arrived` — the twin's answer before a movement, writing nothing: on what is its own, yes; on what is
    /// not, only under a grant (`granted`) — or why not. The movement that then happens is written as that act, and taken the same way
    pub fn may(&self, by: &str, on: &str, is: &str, arrived: i64) -> Result<(), String> {
        if self.within(on, by) {
            return Ok(());
        }
        self.granted(&json!({ "is": is, "value": {} }), by, on, arrived)
    }
    /// WHETHER A GRANT LETS `by` DO THIS ACT ON `on`, arrived at `arrived`: one in force names the act (`action` = its kind) and a
    /// node at or above its writer (`to`); has not run out (`until`, read against when the act arrived, never its writer's clock);
    /// and, with a `budget` `{ field, max, window? }`, what every earlier act under it on that thing spent of `field` (in the window,
    /// by the acts' own time) is still under `max` — the act that crosses it is taken whole, for it already happened, and the next
    /// is the one refused — or why not
    fn granted(&self, e: &Value, by: &str, on: &str, arrived: i64) -> Result<(), String> {
        let is = e["is"].as_str().unwrap_or("");
        let mut why = format!("no grant lets {by} {is} {on}");
        for g in self.grants_on(on) {
            let v = self.e(g)["value"].clone();
            let to = id_of(&v["to"]);
            if to.is_empty() || !self.within(by, &to) || v["action"] != is {
                continue;
            }
            if v["until"].as_i64().is_some_and(|u| arrived >= u) {
                why = format!("the grant ({g}) that let {to} {is} {on} ran out");
                continue;
            }
            if let Some(b) = v["budget"].as_object() {
                let field = b.get("field").and_then(Value::as_str).unwrap_or("");
                let max = b.get("max").and_then(Value::as_f64).unwrap_or(0.0);
                let from = b.get("window").and_then(Value::as_i64).map(|w| arrived - w);
                let earlier = self.spent(is, &to, on, field, from);
                if earlier >= max {
                    why = format!("the grant ({g}) that let {to} {is} {on} is over its budget: {earlier} of {max} {field}");
                    continue;
                }
            }
            return Ok(());
        }
        Err(why)
    }
    /// AN ENTRY ADMITTED LANDS, arrived at `arrived`: at its node, by its node, naming what it names — returns what it touched. Only
    /// after `admit` took it
    pub fn land(&mut self, e: &Value, present: bool, arrived: i64) -> BTreeSet<String> {
        let mut touched = BTreeSet::new();
        if e["is"] == "takeover" {
            self.arrivals.insert(e["g"].as_i64().unwrap_or(0), arrived);
        }
        if self.entries.source().is_none() || self.entries.is_overlay() {
            if let Some(s) = e["sig"]["sig"].as_str().and_then(crate::identity::canonical_sig) {
                self.signatures.insert(s);
            }
        }
        let (by, at) = (id_of(&e["by"]), id_of(&e["at"]));
        let g = e["g"].as_i64().unwrap();
        if present {
            self.present.insert(g);
        }
        // A NODE COMES TO BE: a person by its first key, anything else by its make, under its parent
        if !self.nodes.contains_key(&at) {
            let parent = (e["is"] == "make").then(|| id_of(&e["refs"]["in"]));
            let v = &e["value"];
            let kind = v["kind"].as_str().unwrap_or(if parent.is_none() { "person" } else { "" }).to_string();
            if let Some(p) = &parent {
                self.kids.entry(p.clone()).or_default().insert(at.clone());
                touched.insert(p.clone());
            }
            self.nodes.insert(at.clone(), Node { id: at.clone(), kind, parent, ..Default::default() });
        }
        touched.insert(at.clone());
        touched.insert(by.clone());
        self.entries.insert(g, e);
        if e["is"] == "purge" {
            self.erased.extend(e["value"]["sigs"].as_array().into_iter().flatten().filter_map(Value::as_str).filter_map(crate::identity::canonical_sig));
        }
        self.nodes.get_mut(&by).unwrap().by.push_back(g);
        self.nodes.get_mut(&at).unwrap().at.push_back(g);
        for (role, v) in e["refs"].as_object().cloned().unwrap_or_default() {
            if e["is"] == "make" && role == "in" {
                continue;
            }
            for k in ids_of(&v) {
                touched.insert(k.clone());
                self.nodes.get_mut(&k).unwrap().named.push_back(g);
            }
        }
        for x in numbers(e) {
            self.answers.entry(x).or_default().push_back(g);
            if let Some(a) = self.entries.get(&x).and_then(|y| parse_id(&y["at"])) {
                touched.insert(a);
            }
        }
        match e["is"].as_str().unwrap_or("") {
            "be" | "make" => {
                let n = self.nodes.get_mut(&at).unwrap();
                n.value = laid(std::mem::take(&mut n.value), e);
                // 3. work comes last in its node's history: it is read on top of what is in there now
                self.history.entry(at.clone()).or_default().push_back(g);
                let readers = self.readers.get(&at).cloned().unwrap_or_default();
                let broken = self.breaks(g, &readers, &self.accepted_of(&at));
                if broken.is_empty() {
                    let n = self.nodes.get_mut(&at).unwrap();
                    n.accepted = laid(std::mem::take(&mut n.accepted), e);
                } else {
                    self.out.insert(g, broken.into_iter().collect());
                }
            }
            // 2. an acceptance its node's level above wrote is a change, to come into force
            "acceptance" if self.parent(&at).as_deref() == Some(by.as_str()) => {
                if let Some(re) = e["refs"]["re"].as_i64() {
                    self.rewritten.entry(re).or_default().push_back(g);
                }
                self.pending.insert(g, Vector::new());
                touched.insert(at.clone());
            }
            _ => {}
        }
        self.settle(&mut touched);
        touched
    }

    /// what work `w` breaks, laid on `before` (its node's accepted details at that point): each acceptance among `readers` over its
    /// maker that holds before and not with it, with its holder
    fn breaks(
        &self,
        w: i64,
        readers: &OrdSet<i64>,
        before: &Map<String, Value>,
    ) -> Vec<(String, i64)> {
        let e = self.e(w);
        let (maker, node) = (id_of(&e["by"]), id_of(&e["at"]));
        let after = laid(before.clone(), &e);
        let mut out = vec![];
        let was = |k: &str| {
            if k == node {
                before.clone()
            } else {
                self.accepted_of(k)
            }
        };
        let with = |k: &str| {
            if k == node {
                after.clone()
            } else {
                self.accepted_of(k)
            }
        };
        for a in readers.iter().copied().filter(|a| self.over(*a, &maker)) {
            if meets(&self.expect(a), &was) && !meets(&self.expect(a), &with) {
                out.push((self.holder(&id_of(&self.e(a)["at"])), a));
            }
        }
        out
    }
    /// a node's history read again under `readers`: its accepted details, and its work left out with what each breaks
    fn reread(
        &self,
        node: &str,
        readers: &OrdSet<i64>,
    ) -> (Map<String, Value>, BTreeMap<i64, Vec<(String, i64)>>) {
        let (mut d, mut out) = (Map::new(), BTreeMap::new());
        for w in self.history.get(node).into_iter().flatten() {
            let broken = self.breaks(*w, readers, &d);
            if broken.is_empty() {
                d = laid(d, &self.e(*w));
            } else {
                out.insert(*w, broken);
            }
        }
        (d, out)
    }
    /// 2. ACCEPTANCE CHANGES COME INTO FORCE, in order, each when nothing in force at its writer and above that holds breaks with it.
    /// Only the nodes a change reads, and the one it replaces read, are read again
    fn settle(&mut self, touched: &mut BTreeSet<String>) {
        loop {
            let mut moved = false;
            for c in self.pending.keys().copied().collect::<Vec<_>>() {
                let e = self.e(c);
                let (node, writer) = (id_of(&e["at"]), id_of(&e["by"]));
                let old = e["refs"]["re"]
                    .as_i64()
                    .filter(|re| self.force.get(&node).is_some_and(|f| f.contains(re)));
                let mut nodes: BTreeSet<String> = reads(&self.expect(c)).into_iter().collect();
                if let Some(o) = old {
                    nodes.extend(reads(&self.expect(o)));
                }
                // looked at again only when something of what it reads changed, or it is new
                let fresh = self.pending[&c].is_empty();
                if !fresh && nodes.is_disjoint(touched) {
                    continue;
                }
                let mut next: BTreeMap<
                    String,
                    (
                        OrdSet<i64>,
                        Map<String, Value>,
                        BTreeMap<i64, Vec<(String, i64)>>,
                    ),
                > = BTreeMap::new();
                for x in &nodes {
                    let mut r = self.readers.get(x).cloned().unwrap_or_default();
                    if let Some(o) = old {
                        r.remove(&o);
                    }
                    if reads(&self.expect(c)).contains(x) {
                        r.insert(c);
                    }
                    let (d, out) = self.reread(x, &r);
                    next.insert(x.clone(), (r, d, out));
                }
                let now = |k: &str| self.accepted_of(k);
                let then = |k: &str| {
                    next.get(k)
                        .map(|(_, d, _)| d.clone())
                        .unwrap_or_else(|| self.accepted_of(k))
                };
                let mut broken = vec![];
                for x in &nodes {
                    for a in self
                        .readers
                        .get(x)
                        .into_iter()
                        .flatten()
                        .copied()
                        .filter(|a| Some(*a) != old && self.over(*a, &writer))
                    {
                        if meets(&self.expect(a), &now) && !meets(&self.expect(a), &then) {
                            broken.push((self.holder(&id_of(&self.e(a)["at"])), a));
                        }
                    }
                }
                if broken.is_empty() {
                    let f = self.force.entry(node.clone()).or_default();
                    if let Some(o) = old {
                        f.retain(|x| *x != o);
                    }
                    f.push_back(c);
                    for (x, (r, d, out)) in next {
                        for w in self.history.get(&x).into_iter().flatten() {
                            self.out.remove(w);
                        }
                        self.out.extend(out.into_iter().map(|(w, b)| (w, b.into_iter().collect::<Vector<_>>())));
                        self.readers.insert(x.clone(), r);
                        self.nodes.get_mut(&x).unwrap().accepted = d;
                        touched.insert(x);
                    }
                    self.pending.remove(&c);
                    moved = true;
                    break;
                }
                self.pending.insert(c, broken.into_iter().collect());
            }
            if !moved {
                break;
            }
        }
    }
    /// 4. where a change that is not in waits: the nearest holder of what it breaks that has not written that acceptance since
    fn waits_at(&self, c: i64, broken: &Vector<(String, i64)>) -> Vec<(String, i64)> {
        let depth = |h: &str| self.level(h).unwrap_or(0);
        let nearest = broken.iter().map(|(h, _)| depth(h)).max().unwrap_or(0);
        let decided = |h: &str, a: i64| {
            a > c
                || self
                    .rewritten
                    .get(&a)
                    .into_iter()
                    .flatten()
                    .any(|x| *x > c && id_of(&self.e(*x)["by"]) == h)
        };
        broken
            .iter()
            .filter(|(h, a)| depth(h) == nearest && !decided(h, *a))
            .cloned()
            .collect()
    }
    fn open(&self) -> Vec<(i64, Vec<(String, i64)>)> {
        self.out
            .iter()
            .chain(self.pending.iter())
            .map(|(c, b)| (*c, self.waits_at(*c, b)))
            .collect()
    }

    // ─── ASKS: a decision is a thing moving ───

    /// an ask: its thing, its label, its moves
    fn ask_of(&self, g: i64) -> Option<(String, String, Vec<Map<String, Value>>)> {
        if self.entries.kind_of(g) != Some("ask") {
            return None;
        }
        let e = self.entries.get(&g)?;
        let about = parse_id(&e["refs"]["about"])?;
        let label = e["value"]["label"].as_str().unwrap_or("").to_string();
        let moves = e["value"]["moves"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|m| m["to"].as_object().cloned())
            .filter(|m| !m.is_empty())
            .collect();
        Some((about, label, moves))
    }
    /// THE MOVE MADE, if one was: accepted work at its thing, after it, carrying every detail of one of its moves
    pub fn moved(&self, g: i64) -> Option<i64> {
        self.row(g).and_then(|r| r.moved)
    }
    /// the move made on an ask, its holder known
    fn moved_by(&self, g: i64, holder: &str) -> Option<i64> {
        let (about, _, moves) = self.ask_of(g)?;
        // only its holder's move, or a level above it: a session writing the details on itself does not move what it asked (it would
        // answer its own question)
        self.history
            .get(&about)
            .into_iter()
            .flatten()
            .copied()
            .filter(|w| *w > g && !self.out.contains_key(w))
            .find(|w| {
                let by = id_of(&self.e(*w)["by"]);
                let v = self.e(*w)["value"]
                    .as_object()
                    .cloned()
                    .unwrap_or_default();
                self.within(holder, &by)
                    && moves
                        .iter()
                        .any(|m| m.iter().all(|(k, x)| v.get(k) == Some(x)))
            })
            .or_else(|| self.acted(g, holder))
    }
    /// whether an entry is trying to make one of an ask's moves: work at its thing, or an act of a kind one of its moves names
    fn tries_move(&self, e: &Value, g: i64) -> bool {
        let Some((about, _, _)) = self.ask_of(g) else {
            return false;
        };
        (e["is"] == "be" && id_of(&e["at"]) == about)
            || self.e(g)["value"]["moves"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|m| m["act"]["is"] == e["is"])
    }
    /// A MOVE THAT IS AN ACT of its own kind (an approval, a version taken, a rule accepted): an act answering the ask, of the kind and
    /// value one of its moves names, in force — signed by its holder or a level above it: whoever writes it (a session, a thing of
    /// yours with no key of its own), the one who signed is the one who moved
    fn acted(&self, g: i64, holder: &str) -> Option<i64> {
        let acts: Vec<Value> = self.e(g)["value"]["moves"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|m| m["act"]["is"].is_string())
            .map(|m| m["act"].clone())
            .collect();
        if acts.is_empty() {
            return None;
        }
        let theirs: BTreeSet<String> = crate::identity::keys_for(self, holder, i64::MAX)
            .into_iter()
            .map(|k| k.0)
            .collect();
        self.answers
            .get(&g)
            .into_iter()
            .flatten()
            .copied()
            .filter(|w| *w > g && self.accepted(*w))
            .find(|w| {
                let e = self.e(*w);
                e["sig"]["key"].as_str().is_some_and(|k| theirs.contains(k))
                    && acts.iter().any(|a| {
                        a["is"] == e["is"]
                            && (a["value"].is_null() || a["value"] == e["value"])
                            && (a["at"].is_null() || parse_id(&a["at"]) == parse_id(&e["at"]))
                    })
            })
    }
    /// declined by whoever holds it
    fn declined_by(&self, g: i64, h: &str) -> bool {
        // by its holder, or a level above it — you are above every hand
        self.answers.get(&g).into_iter().flatten().any(|x| {
            self.e(*x)["is"] == "decline" && self.within(h, &id_of(&self.e(*x)["by"]))
        })
    }
    /// AN ASK IS OPEN until its thing moved or its holder declined it — and an ask that is evidence on another is never open itself
    pub fn ask_open(&self, g: i64) -> bool {
        self.row(g).is_some_and(|r| r.joins.is_none() && r.moved.is_none() && !r.declined)
            && self.ask_of(g).is_some_and(|(about, _, _)| !self.deleted(&about))
    }
    /// WHERE AN ASK'S MOVES LAND: the places its acts would move — two asks about one thing are one question only if they land alike
    fn lands(&self, g: i64) -> Vec<Value> {
        self.e(g)["value"]["moves"].as_array().into_iter().flatten().map(|m| m["act"]["at"].clone()).collect()
    }
    /// THE ASK THIS ONE IS EVIDENCE ON: an earlier one about the same thing, with the same label, whose moves land in the same places,
    /// open when this one was raised
    pub fn joins(&self, g: i64) -> Option<i64> {
        self.row(g).and_then(|r| r.joins)
    }
    /// ONE KIND OF ASK AT A THING, read in one pass: every ask about `about` with `label` before `before`, oldest first — each with its
    /// holder (from the hands that moved the ones before it), the move made, whether its holder declined it, and the earlier one it is
    /// evidence on. Each is read from the ones before it, already read — read again for every ask, the same kind took twice as long for
    /// every ask more (a few dozen versions of Taldo asked made Now take minutes)
    fn kind(&self, about: &str, label: &str, before: i64) -> Vec<Row> {
        let mut rows: Vec<Row> = vec![];
        let mut tried: BTreeSet<String> = BTreeSet::new();
        let Some(n) = self.nodes.get(about) else {
            return rows;
        };
        for x in n.named.iter().copied().filter(|x| *x < before && self.entries.kind_of(*x) == Some("ask")) {
            if rows.last().is_some_and(|r| r.g == x)
                || !self
                    .ask_of(x)
                    .is_some_and(|(a, l, _)| a == about && l == label)
            {
                continue;
            }
            let r = self.read_row(about, label, x, &rows, &tried);
            if let Some(w) = r.moved {
                tried.insert(id_of(&self.e(w)["by"]));
            }
            rows.push(r);
        }
        rows
    }
    /// one ask of a kind, read from the ones before it (`before`) and the hands that moved them (`tried`)
    fn read_row(&self, about: &str, label: &str, x: i64, before: &[Row], tried: &BTreeSet<String>) -> Row {
        let holder = self.holder_with(about, label, x, tried);
        let moved = self.moved_by(x, &holder);
        let declined = self.declined_by(x, &holder);
        let lands = self.lands(x);
        // THE ASK IT IS EVIDENCE ON: an earlier one like it, landing alike, itself no evidence, open when this one was raised
        let joins = before
            .iter()
            .filter(|y| y.lands == lands && y.joins.is_none())
            .find(|y| y.moved.map_or(true, |m| m > x) && !y.declined)
            .map(|y| y.g);
        Row { g: x, holder, moved, declined, lands, joins }
    }
    /// an ask as its kind reads it
    fn row(&self, g: i64) -> Option<Row> {
        let (about, label, _) = self.ask_of(g)?;
        let mut rows = self.kind(&about, &label, g + 1);
        if rows.last().is_some_and(|r| r.g == g) {
            return rows.pop();
        }
        // not among the asks its thing is named by: read alone, from the ones before it
        let tried: BTreeSet<String> = rows
            .iter()
            .filter_map(|r| r.moved)
            .map(|w| id_of(&self.e(w)["by"]))
            .collect();
        Some(self.read_row(&about, &label, g, &rows, &tried))
    }
    /// WHAT CAME TO AN ASK AS EVIDENCE: every later ask about the same thing and label, landing alike, raised while it was open
    pub fn evidence(&self, g: i64) -> Vec<i64> {
        let Some((about, label, _)) = self.ask_of(g) else {
            return vec![];
        };
        let on: BTreeSet<i64> = self
            .kind(&about, &label, i64::MAX)
            .into_iter()
            .filter(|r| r.joins == Some(g))
            .map(|r| r.g)
            .collect();
        self.nodes
            .get(&about)
            .map(|n| n.named.iter().copied().filter(|x| on.contains(x)).collect())
            .unwrap_or_default()
    }
    /// THE FLOWS AT A NODE, per label: (by, present, mine) — its accepted details `flow:<label>`, so setting a flow is a move like any
    /// other, and a flow can itself be asked for
    fn flows_at(&self, node: &str, before: i64) -> BTreeMap<String, (Option<String>, bool, bool)> {
        // AS THEY STOOD WHEN IT WAS ASKED: a flow set later never takes over a decision already waiting on its holder
        let mut d = Map::new();
        for w in self
            .history
            .get(node)
            .into_iter()
            .flatten()
            .copied()
            // and only as set with a person there: handing off a decision of yours is yours to do, in person
            .filter(|w| *w < before && !self.out.contains_key(w) && self.present.contains(w))
        {
            d = laid(d, &self.e(w));
        }
        d.iter()
            .filter_map(|(k, v)| {
                let label = k.strip_prefix("flow:")?;
                Some((
                    label.to_string(),
                    (
                        parse_id(&v["by"]),
                        v["present"] == true,
                        v["mine"] == true,
                    ),
                ))
            })
            .collect()
    }
    /// THE FLOWS IN FORCE AT A NODE NOW: the labels handed off or kept there, as set with a person there
    pub fn flows(&self, node: &str) -> Vec<String> {
        self.flows_at(node, i64::MAX)
            .into_iter()
            .filter(|(_, f)| f.0.is_some() || f.2)
            .map(|(l, _)| l)
            .collect()
    }
    /// WHO HOLDS AN ASK: from its thing up, the nearest flow for its label — unless a flow above keeps that label (`mine`), and passing
    /// over a hand that already moved this thing for this label since which it was raised again. With none, the root
    pub fn holder_of(&self, g: i64) -> Option<String> {
        self.row(g).map(|r| r.holder)
    }
    /// who holds an ask, given the hands that already moved this thing for this label before it
    fn holder_with(&self, about: &str, label: &str, g: i64, tried: &BTreeSet<String>) -> String {
        let levels = self.chain(about);
        // a level that keeps the label: the highest such keeps it
        if let Some(top) = levels
            .iter()
            .rev()
            .find(|l| self.flows_at(l, g).get(label).is_some_and(|f| f.2))
        {
            return top.clone();
        }
        for l in &levels {
            if let Some((Some(by), _, _)) = self.flows_at(l, g).get(label) {
                if !tried.contains(by) {
                    return by.clone();
                }
            }
        }
        levels.last().cloned().unwrap_or_default()
    }
    /// whether a person must make this move: a flow on the way up says so for the label
    fn needs_present(&self, g: i64) -> bool {
        let Some((about, label, _)) = self.ask_of(g) else {
            return false;
        };
        // the ask itself may say a person must make its move — stricter only, never looser
        if self.e(g)["value"]["present"] == true {
            return true;
        }
        self.chain(&about).iter().any(|x| self.flows_at(x, g).get(&label).is_some_and(|f| f.1))
    }
    /// WHAT IS ASKED OF A HOLDER: every open ask it holds, or a responsibility it carries now holds, oldest first — with whether a
    /// person must make the move
    pub fn asks_for(&self, holder: &str) -> Vec<(i64, bool)> {
        self.entries
            .of_kind("ask")
            .filter(|g| {
                self.ask_open(*g)
                    && self.holder_of(*g).is_some_and(|h| h == holder || self.carriers(&h).iter().any(|c| c == holder))
            })
            .map(|g| (g, self.needs_present(g)))
            .collect()
    }

    /// THE ACCEPTANCES IN FORCE AT A NODE
    pub fn in_force(&self, node: &str) -> Vec<i64> {
        self.force.get(node).map(|f| f.iter().copied().collect()).unwrap_or_default()
    }
    /// WHETHER AN ENTRY IS ACCEPTED: work, when the acceptances in force let it in; an acceptance change, when it came into force
    pub fn accepted(&self, g: i64) -> bool {
        self.entries.contains_key(&g)
            && !self.out.contains_key(&g)
            && !self.pending.contains_key(&g)
    }
    /// WHETHER AN ACCEPTANCE HOLDS, on what is accepted
    pub fn holds(&self, a: i64) -> Option<bool> {
        let e = self.entries.get(&a).filter(|e| e["is"] == "acceptance")?;
        let c = &e["value"];
        Some(if is_event(c) {
            self.happened(a)
        } else {
            meets(c, &|k: &str| self.accepted_of(k))
        })
    }
    /// WHAT WAITS AT A HOLDER: each change, with the acceptance of this holder's it breaks
    pub fn waiting(&self, holder: &str) -> Vec<(i64, i64)> {
        self.open()
            .into_iter()
            .flat_map(|(c, at)| {
                at.into_iter()
                    .filter(|(h, _)| h == holder)
                    .map(move |(_, a)| (c, a))
            })
            .collect()
    }
    /// 5. TO WHOM AN ACCEPTANCE IN FORCE THAT DOES NOT HOLD IS OWED: its node — or, when work at or below it that would meet it is
    /// decided out, its holder, to change it
    fn owed_to(&self, a: i64, decided_out: &[i64]) -> String {
        let n = id_of(&self.e(a)["at"]);
        let blocked = !is_event(&self.expect(a))
            && decided_out.iter().any(|w| {
                let e = self.e(*w);
                let at = id_of(&e["at"]);
                self.within(&id_of(&e["by"]), &n)
                    && meets(&self.expect(a), &|k: &str| {
                        if k == at {
                            laid(self.accepted_of(k), &e)
                        } else {
                            self.accepted_of(k)
                        }
                    })
            });
        if blocked {
            self.holder(&n)
        } else {
            n
        }
    }
    fn decided_out(&self) -> Vec<i64> {
        self.out
            .iter()
            .filter(|(c, b)| self.waits_at(**c, b).is_empty())
            .map(|(c, _)| *c)
            .collect()
    }
    /// 5. WHAT IS OWED, AND TO WHOM
    pub fn owed(&self) -> Vec<(String, i64)> {
        let mut out = vec![];
        let open = self.open();
        let decided_out = self.decided_out();
        let gone = |a: i64| {
            let e = self.e(a);
            self.deleted(&id_of(&e["at"])) || reads(&e["value"]).iter().any(|x| self.nodes.contains_key(x) && self.deleted(x))
        };
        for acc in self.force.values() {
            for a in acc.iter().filter(|a| self.holds(**a) != Some(true) && !gone(**a)) {
                out.push((self.owed_to(*a, &decided_out), *a));
            }
        }
        for (c, at) in open {
            for h in at.iter().map(|(h, _)| h).collect::<BTreeSet<_>>() {
                out.push((h.clone(), c));
            }
        }
        out
    }

    /// 8. REACH: whether what is owed to `who` has reached it — an entry by it, or below it, points to it
    pub fn reached(&self, who: &str, what: i64) -> bool {
        self.answers
            .get(&what)
            .into_iter()
            .flatten()
            .any(|x| self.within(&id_of(&self.e(*x)["by"]), who))
    }
    /// what holds a node, as an address: its parent, or "" for a person
    fn holder(&self, node: &str) -> String {
        self.parent(node).unwrap_or_default()
    }
    pub fn children(&self, r: &str) -> Vec<String> {
        self.kids
            .get(r)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }
}

/// an ask, as its kind reads it (`Twin::kind`)
#[derive(Clone)]
struct Row {
    g: i64,
    holder: String,
    moved: Option<i64>,
    declined: bool,
    lands: Vec<Value>,
    joins: Option<i64>,
}
