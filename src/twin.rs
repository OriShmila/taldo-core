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
//! deletion waits (`keep_deleted`, ms, a detail of the node or one above it). When that is over, a `purge` answering it erases what it held: every entry at it and under it but what decided who may do what (`erasing`).
//! JUDGED ONCE: a line is judged when it arrives (`admit`), and its keeper keeps it with Taldo's MARK (`take`) — what admission found
//! (who signed it, with which key, whether a person was there), the line's fingerprint and when it arrived, chained to the mark
//! before and signed by a Taldo of the record's person. A start reads the rows (`refold`): every mark checked, every line landed as
//! it was taken, none judged again — so what a line stood on may change, or be erased, and it stays what it was. A row changed, put
//! in or taken away breaks the chain: the record is not read.
//! TAKING OUT acts from then on: a `take out` names lines — at its node or under it, by their writer or a node under them, from above
//! — and from it on they have no effect; they stay in the rows as what happened, and nothing else is taken out unless named. A take-out
//! taken out brings back what it took out. A node's coming to be is never taken out (the node is deleted), nor a delete (it is
//! restored) nor a purge.
//! ERASING, by law: a `purge` names exactly what its delete erases (`erasing`); its keeper keeps, in place of each of those rows, what
//! stays of it (`erased_row`) — where it was, by whom, its kind, its signature's own and its mark, never a word — and the store keeps
//! no byte of what it said. Nothing answers what was erased.
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
    /// WHO IT IS — the keys it signs with, who carries it, what stays when it is deleted — as decided from above it (`from_above`):
    /// never one of its details, so nothing that writes details reaches it
    pub who: Map<String, Value>,
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
    /// the entries that point to an entry, by any role — read through `answers_from`, which says from whom
    pub(crate) answers: OrdMap<i64, Vector<i64>>,
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
    /// THE LINES TAKEN OUT (`take out`): from the line that took each out, it has no effect — it stays in the rows, as what happened
    pub(crate) taken_out: OrdMap<i64, Out>,
    /// THE LINES A PURGE ERASED, each with its purge: their words are gone from the rows, what stays of each is `erased_row`
    pub(crate) erased: OrdMap<i64, i64>,
    /// THE LAST MARK'S HASH: the next mark is chained to it (`mark`) — empty before the first
    pub(crate) head: String,
    /// THE LAST LINE KEPT OR READ: with `head`, where the record is (`anchor`)
    pub(crate) last: i64,
}

/// A LINE TAKEN OUT: the take-outs that took it out, and where it was, by whom and of what kind — never what it said. A twin folded
/// from a list, with no rows, holds the line itself, so a take-out taken out brings it back
#[derive(Clone, Debug)]
pub struct Out {
    /// every take-out that named it: it is out while one of them counts (`out_now`)
    pub by: Vector<i64>,
    pub at: String,
    pub writer: String,
    pub is: String,
    /// its row's hash, as it was taken: read again from the rows, it is that row
    pub(crate) hash: String,
    pub(crate) line: Option<Arc<Value>>,
}

impl Twin {
    /// AN EMPTY TWIN whose entries are kept as rows in `source`: only what the current state needs stays in memory
    pub fn stored(source: Arc<dyn Source>) -> Twin {
        Twin {
            entries: Entries::stored(source),
            ..Default::default()
        }
    }
    /// THE TWIN READ FROM ITS ROWS, oldest first: each row's mark checked — chained to the one before, signed by a Taldo of the record's
    /// person, the line as it was marked and signed by the key the mark names — and its effect landed; no line is judged again: it was
    /// judged once, when it arrived (`take`). Or why the rows are not what was kept
    pub fn refold(source: Arc<dyn Source>) -> Result<Twin, String> {
        let mut t = Twin::stored(source.clone());
        let mut reading = Reading::default();
        let mut from = 0;
        loop {
            let page = source.page(from, i64::MAX, false, 1024);
            let Some(last) = page.last().map(|(g, _)| *g) else { break };
            for (_, e) in &page {
                t.read(e, &mut reading)?;
            }
            from = last + 1;
        }
        reading.done()?;
        t.entries.rows_kept();
        Ok(t)
    }
}

/// WHAT A READING OF ROWS STILL OWES: marks made before their Taldo's node was (checked once it is), and rows erased whose purge
/// is still to come
pub(crate) struct Reading {
    early: Vec<Value>,
    erased: BTreeMap<i64, i64>,
    /// the last line read
    last: i64,
}
impl Default for Reading {
    fn default() -> Self {
        Reading { early: vec![], erased: BTreeMap::new(), last: -1 }
    }
}
impl Reading {
    /// reading on from line `last`, read already (a state caught up)
    pub(crate) fn after(last: i64) -> Reading {
        Reading { last, ..Default::default() }
    }
    pub(crate) fn done(self) -> Result<(), String> {
        if let Some(m) = self.early.first() {
            return Err(format!("#{}'s mark is by {}, which never was a Taldo of the record's person", m["g"], m["by"]));
        }
        if let Some((g, p)) = self.erased.iter().next() {
            return Err(format!("#{g} has no words, and #{p}, the purge it says erased it, never did"));
        }
        Ok(())
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

/// LINES ARRIVING IN ORDER, held in memory and never kept: each judged as it arrives (`admit`) and landed — what a list of lines
/// comes to. Kept lines are read with `refold`
pub fn fold(entries: &[Value]) -> Twin {
    let mut t = Twin::default();
    for e in entries {
        t.apply(e);
    }
    t
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
/// WHO A NODE IS, as its details: the keys it signs with, who carries it, what stays when it is deleted — and, for a node a whole
/// program runs as, where it runs (`place`: `place_whole`) — set from above (`from_above`), never by the node itself nor under a grant
pub const GOVERNING: [&str; 4] = ["keys", "carried_by", "keep_deleted", "place"];
/// WHERE A WHOLE PROGRAM RUNS, WHOLE: its server (`host`) and how it is reached (`user`, `key`: the key that reaches it), its box there
/// (`vm`), and which of its sealed programs runs (`machine`) — each said, and nothing else
fn place_whole(p: &Value) -> Result<(), String> {
    let o = p.as_object().ok_or("a place is its host, user, key, vm and machine")?;
    for k in ["host", "user", "key", "vm", "machine"] {
        if o.get(k).and_then(Value::as_str).is_none_or(str::is_empty) {
            return Err(format!("a place says its {k}"));
        }
    }
    if let Some(k) = o.keys().find(|k| !["host", "user", "key", "vm", "machine"].contains(&k.as_str())) {
        return Err(format!("a place says nothing but where a program runs — not {k}"));
    }
    Ok(())
}
/// A SEAL IS WHOLE: level code — `wasm`, the module's hash; `cwasm`, its ready machine code's hash for each machine; `contract`, what it
/// was built to; `reach`, what it may call — or a whole program, `program`: its hash for each machine; and nothing else
fn seal_whole(v: &Value) -> Result<(), String> {
    let hash = |x: &Value| x.as_str().is_some_and(|h| h.len() == 64 && h.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
    let Some(o) = v.as_object() else { return Err("a seal says its module, its machine code, its contract and its reach".into()) };
    // A WHOLE PROGRAM, run in a sandbox of its own (a person's Taldo in its microVM): only its sha-256 for each machine
    if let Some(p) = o.get("program") {
        if o.len() != 1 || !p.as_object().is_some_and(|m| !m.is_empty() && m.iter().all(|(k, h)| !k.is_empty() && hash(h))) {
            return Err("a program's seal says only, for each machine, its program's sha-256 in hex".into());
        }
        return Ok(());
    }
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

/// a node's details, with an entry's laid on them — never who it is (`GOVERNING`), which lands only in `Node::who`
fn laid(mut d: Map<String, Value>, e: &Value) -> Map<String, Value> {
    for (k, x) in e["value"].as_object().cloned().unwrap_or_default() {
        if !GOVERNING.contains(&k.as_str()) {
            d.insert(k, x);
        }
    }
    d
}
/// WHO A NODE IS, with what an entry decided of it from above laid on it
fn said_who(mut w: Map<String, Value>, e: &Value) -> Map<String, Value> {
    for (k, x) in e["value"].as_object().cloned().unwrap_or_default() {
        if GOVERNING.contains(&k.as_str()) {
            w.insert(k, x);
        }
    }
    w
}
/// THE LINE A ROW HOLDS: the row without what keeping adds to it (`arrived`, its `salt`, its `mark`) — the one answer to it, so a
/// store holds a row marked to the line it kept
pub fn line_of(row: &Value) -> Value {
    let mut l = row.clone();
    if let Some(o) = l.as_object_mut() {
        for k in ["arrived", "salt", "mark"] {
            o.remove(k);
        }
    }
    l
}
/// AN ENTRY AS IT IS KEPT: its row, carrying — outside what its writer signed — when it arrived (the receiver's time, the one time every
/// rule reads). Nothing else is added to a line, and no line is changed once kept
pub fn kept(e: &Value, arrived: i64) -> Value {
    let mut row = e.clone();
    row["arrived"] = json!(arrived);
    row
}
/// WHOSE ANSWER COUNTS, for an answer the table names: by the answered entry's writer or a node above it; at the answered entry's
/// node or above it; only the one a way in names (`as`); or anyone's
#[derive(Clone, Copy, PartialEq)]
enum Whose {
    Writer,
    Node,
    As,
    Holder,
    Any,
}
/// THE KINDS THE CORE GIVES A MEANING — the one list of them: a program's own kinds never take one of these names, so no line is
/// read by the core as something its writer did not mean
pub const CORE_KINDS: [&str; 15] = ["make", "be", "grant", "revoke", "delete", "restore", "purge", "take out", "seal", "block", "takeover", "decline", "acceptance", "ask", "request"];
/// WHETHER THE CORE GIVES THIS KIND A MEANING (`CORE_KINDS`)
pub fn core_kind(is: &str) -> bool {
    CORE_KINDS.contains(&is)
}
/// WHAT DECIDED WHO MAY DO WHAT: never erased by a purge, never taken out — acts elsewhere were taken under them, and none holds a
/// word anyone said
const DECIDING: [&str; 11] = ["make", "delete", "restore", "purge", "take out", "grant", "revoke", "seal", "block", "takeover", "decline"];
/// WHAT STAYS OF A LINE A PURGE ERASED: its number, where it was, by whom, its kind, what it answered or named (by the record's own
/// roles: ids and numbers), when it arrived, a hash of its signature (so the same act is never taken again, and no guess at its
/// words can be checked against it) and its mark — never a word it said, nor the salt its fingerprint was made with — and the purge
pub fn erased_row(row: &Value, purge: i64) -> Value {
    let sig = row["sig"]["sig"].as_str().and_then(crate::identity::canonical_sig).map(|s| sha(&s));
    json!({ "g": row["g"], "by": row["by"], "at": row["at"], "is": row["is"], "refs": row["refs"], "value": {}, "arrived": row["arrived"], "sig_hash": sig, "mark": row["mark"], "erased_by": purge })
}
/// HOW A STORE FINDS A ROW BY ITS SIGNATURE (`Source::signed`): its signature's standard spelling — or, for what stays of an erased
/// line, `h:` and the hash of it. Nothing else is looked up
pub fn signed_as(row: &Value) -> Option<String> {
    match row["sig_hash"].as_str() {
        Some(h) => Some(format!("h:{h}")),
        None => row["sig"]["sig"].as_str().and_then(crate::identity::canonical_sig),
    }
}
fn sha(s: &str) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(s.as_bytes()).iter().map(|b| format!("{b:02x}")).collect()
}
/// A LINE'S FINGERPRINT, as its mark holds it: its salt — Taldo's own, kept in the row and erased with the words, so no guess at
/// the words can be checked against what stays — the act as signed, and its signature in its standard spelling
fn line_hash(e: &Value) -> String {
    let sig = e["sig"]["sig"].as_str().and_then(crate::identity::canonical_sig).unwrap_or_default();
    sha(&format!("{}\n{}\n{sig}", e["salt"].as_str().unwrap_or(""), crate::identity::signed_bytes(e)))
}
/// THE MARK, before Taldo signs it: which line (its number and fingerprint), when it arrived, who signed it with which key and
/// whether a person was there — as admission found it — chained to the mark before (`prev`), by which Taldo (`by`), made when the
/// record's last line was `made`: its key is one that Taldo had then. A purge's mark holds, too, the hash of what stays of each line
/// it erases (`erased`): what stays is bound by it
fn mark_of(row: &Value, signed: &crate::identity::Signed, prev: &str, by: &str, made: i64, erased: &[Value]) -> Value {
    let mut m = json!({ "g": row["g"], "line": line_hash(row), "arrived": row["arrived"], "signer": signed.signer, "key": signed.key, "present": signed.present, "prev": prev, "by": by, "made": made });
    if row["is"] == "purge" {
        m["erased"] = Value::Object(erased.iter().map(|r| (r["g"].to_string(), json!(crate::rows::row_hash(r)))).collect());
    }
    m
}
/// WHAT TALDO SIGNS OF A MARK: the mark without its signature, in one spelling
pub fn mark_bytes(mark: &Value) -> String {
    let mut m = mark.clone();
    if let Some(o) = m.as_object_mut() {
        o.remove("sig");
    }
    crate::rows::canonical(&m)
}
/// a mark's hash, signature and all: what the next mark is chained to
fn mark_hash(mark: &Value) -> String {
    crate::rows::row_hash(mark)
}
/// A LINE TAKEN: its row to keep, what stays of each line it erases (a purge's keeper keeps those in place of their rows, in the
/// same keep), and what admission found
pub struct Taken {
    pub row: Value,
    pub erased: Vec<Value>,
    pub signed: crate::identity::Signed,
}
/// WHAT EACH KIND ANSWERS, AND WHOSE ANSWER COUNTS — the one table of it, held at admission, so every answer in the twin is one that
/// counts: an undo, a carrying out, a taking back and a no answer only what they name; a delete, a grant and a take-over are answered
/// only as named here; anything else answers any entry before it, as its writer says. None: it answers no such entry
fn answering(is: &str, of: &str) -> Option<Whose> {
    match (is, of) {
        ("restore", "delete") => Some(Whose::Writer),
        ("purge", "delete") => Some(Whose::Any),
        ("revoke", "grant") => Some(Whose::Writer),
        // AN ASK IS MOVED, ANSWERED OR DECLINED only by whoever holds it, or a level above it
        (x, "ask") if !matches!(x, "restore" | "purge" | "revoke") => Some(Whose::Holder),
        ("decline", "takeover") => Some(Whose::Node),
        // a person come in by a way in: their first key, answering the join grant that named them
        ("be", "grant") => Some(Whose::As),
        ("restore" | "purge" | "revoke" | "decline", _) | (_, "delete" | "grant" | "takeover") => None,
        // anything else answering anything else only follows it (a start's word, a step's): it decides nothing, so anyone's counts
        _ => Some(Whose::Any),
    }
}
/// A GRANT, WHOLE: what it lets (`action`); to whom — a node that is (`to`), or for a way in, the one new id it lets come to be
/// (`as`); until when (`until`, a time), whether it reaches below (`below`), and a budget (`{ field, max, window? }`) — or why not.
/// Nothing in it is read as a default
fn grant_whole(v: &Value) -> Result<(), String> {
    let action = v["action"].as_str().filter(|a| !a.is_empty()).ok_or("a grant says what it lets (action)")?;
    if action == "join" {
        if v["as"].as_str().is_none_or(str::is_empty) || v.get("to").is_some() {
            return Err("a way in names the one who comes in by it (as), and gives nothing to a node that is".into());
        }
    } else if parse_id(&v["to"]).is_none() {
        return Err(format!("a grant to {action} names whom it lets (to)"));
    }
    if v.get("until").is_some_and(|u| !u.is_i64()) {
        return Err("a grant's until is a time".into());
    }
    if v.get("below").is_some_and(|b| !b.is_boolean()) {
        return Err("a grant's below is yes or no".into());
    }
    if let Some(b) = v.get("budget") {
        let whole = b["field"].as_str().is_some_and(|f| !f.is_empty()) && b["max"].is_number() && b.get("window").is_none_or(Value::is_i64);
        if !whole {
            return Err("a grant's budget says its field, its max, and maybe its window (a time)".into());
        }
    }
    Ok(())
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
    /// EVERY ANSWER TO AN ENTRY, each one that counts — `answering` held it as it was taken: no reader says whose again
    pub fn answers_to(&self, g: i64) -> Vec<i64> {
        self.answers.get(&g).into_iter().flatten().copied().collect()
    }
    /// THE ANSWERS THAT DECIDE NOTHING, BY WHOSE THEY ARE: each entry answering `g` — of kind `is`, or any when it is "" — written by
    /// `who` or a node above them. A decision (an ask moved or declined, a delete undone, a grant taken back, a take-over said no to) is
    /// held by `answering` as it is taken, and read through `answers_to`; this reads only the words that follow an entry and decide
    /// nothing — a start's `started`, a step's `done` — where the reader says whose it reads
    pub fn answers_from(&self, g: i64, is: &str, who: &str) -> Vec<i64> {
        let of = |x: &i64| (is.is_empty() || self.is(*x) == is) && self.within(who, &id_of(&self.e(*x)["by"]));
        self.answers.get(&g).into_iter().flatten().copied().filter(of).collect()
    }
    /// a delete brought back: a `restore` answering it, by whoever deleted or a node above them
    fn restored(&self, d: i64) -> bool {
        self.answers.get(&d).is_some_and(|a| a.iter().any(|x| self.is(*x) == "restore"))
    }
    /// a delete carried out: a `purge` answering it — any taken, since one is taken only once the waiting is over and naming what it
    /// erases; it undoes nothing
    fn purged(&self, d: i64) -> bool {
        self.answers.get(&d).into_iter().flatten().any(|x| self.is(*x) == "purge")
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
        self.chain(id).iter().find_map(|x| self.nodes.get(x)?.who.get("keep_deleted")?.as_i64())
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
    /// WHAT A PURGE OF DELETE `d` ERASES: every line at its node and under it — that counts, or was taken out — not erased yet, but a
    /// node's coming to be and what decided who may do what (`DECIDING`)
    pub fn erasing(&self, d: i64) -> Vec<i64> {
        let Some(e) = self.entries.get(&d) else { return vec![] };
        let under: BTreeSet<String> = self.subtree(&id_of(&e["at"])).into_iter().collect();
        let first: BTreeSet<i64> = under.iter().filter_map(|x| self.nodes.get(x)?.at.front().copied()).collect();
        let counted = under.iter().filter_map(|x| self.nodes.get(x)).flat_map(|n| n.at.iter().copied()).map(|g| (g, self.is(g).to_string()));
        let out = self.taken_out.iter().filter(|(_, o)| under.contains(&o.at)).map(|(g, o)| (*g, o.is.clone()));
        let mut gs: Vec<i64> = counted.chain(out).filter(|(g, is)| !first.contains(g) && !self.erased.contains_key(g) && !DECIDING.contains(&is.as_str())).map(|(g, _)| g).collect();
        gs.sort();
        gs.dedup();
        gs
    }
    /// THE LINES A TAKE-OUT OR A PURGE NAMES (`value.lines`, their numbers)
    pub fn lines_named(e: &Value) -> Vec<i64> {
        if !matches!(e["is"].as_str(), Some("take out" | "purge")) {
            return vec![];
        }
        e["value"]["lines"].as_array().into_iter().flatten().filter_map(Value::as_i64).collect()
    }
    /// LINE `g` AS IT WAS TAKEN: counted, taken out, or read from the rows
    fn line_row(&self, g: i64) -> Option<Arc<Value>> {
        if let Some(l) = self.entries.get(&g) {
            return Some(l);
        }
        let o = self.taken_out.get(&g)?;
        if let Some(l) = &o.line {
            return Some(l.clone());
        }
        let row = self.entries.source()?.entry(g)?;
        assert!(crate::rows::row_hash(&row) == o.hash, "row {g} of the record is not the row Taldo took out: it was changed in the store");
        Some(Arc::new(row))
    }
    /// EVERY LINE AN ACT TAKES OUT OR BRINGS BACK, as it was taken — the one answer to what an act undoes: the lines a take-out names,
    /// and every line a take-out among them names in turn (taking out a take-out brings its lines back), at any depth — taken out or not
    pub fn lines_changed(&self, e: &Value) -> Vec<Arc<Value>> {
        let mut out = vec![];
        let mut todo = if e["is"] == "take out" { Self::lines_named(e) } else { vec![] };
        let mut seen = BTreeSet::new();
        while let Some(g) = todo.pop() {
            if !seen.insert(g) {
                continue;
            }
            let Some(l) = self.line_row(g) else { continue };
            if l["is"] == "take out" {
                todo.extend(Self::lines_named(&l));
            }
            out.push(l);
        }
        out
    }
    /// THE RECORD'S TALDO — the one that marks it: the first node of kind taldo made under the record's first person
    pub fn record_taldo(&self) -> Option<String> {
        let first = |n: &&Node| n.at.front().copied().unwrap_or(i64::MAX);
        let person = self.nodes.values().filter(|n| n.parent.is_none()).min_by_key(first)?;
        self.children(&person.id).iter().filter_map(|c| self.nodes.get(c)).filter(|n| n.kind == "taldo").min_by_key(first).map(|n| n.id.clone())
    }
    /// THE KEYS THAT MARK THE RECORD NOW: its Taldo's own — none while it is blocked or deleted
    fn marking_keys(&self) -> Vec<String> {
        let Some(t) = self.record_taldo().filter(|t| !self.blocked(t) && !self.deleted(t)) else { return vec![] };
        self.nodes[&t].who.get("keys").and_then(Value::as_array).into_iter().flatten().filter_map(|k| k["key"].as_str().map(String::from)).collect()
    }
    /// WHETHER KEY `k` MARKS THE RECORD as it is (`marking_keys`) — the one answer, for a mark kept and a mark read
    fn marks_with(&self, k: &str) -> bool {
        self.marking_keys().iter().any(|x| x == k)
    }
    /// WHERE THE RECORD IS: its last line, its last mark's hash and its state's hash — and whether it is where the record's first keep
    /// leaves it (`first`) — what Taldo signs and keeps outside the store (`anchored`), so a record cut short, put back or changed is
    /// never read
    pub fn anchor(&self, state: &[u8], first: bool) -> Value {
        use sha2::{Digest, Sha256};
        json!({ "g": self.last, "head": self.head, "state": Sha256::digest(state).iter().map(|b| format!("{b:02x}")).collect::<String>(), "first": first })
    }
    /// WHETHER THE RECORD READ IS WHERE TALDO LAST LEFT IT: `anchor` is Taldo's signed word of it (`{ g, head, state, sig }`), signed
    /// by a key that marks the record now — or why not
    pub fn anchored(&self, state: &[u8], anchor: &Value) -> Result<(), String> {
        let mut said = anchor.clone();
        let sig = said.as_object_mut().and_then(|o| o.remove("sig")).ok_or("the anchor is not signed")?;
        if said != self.anchor(state, said["first"] == true) {
            return Err(format!("the record is not where Taldo last left it: it is at #{} ({}), Taldo left it at #{} ({})", self.last, self.head, said["g"], said["head"]));
        }
        if !self.marks_with(sig["key"].as_str().unwrap_or("")) {
            return Err("the anchor is signed by a key that does not mark the record".into());
        }
        crate::identity::holds(crate::rows::canonical(&said).as_bytes(), &sig).map_err(|w| format!("the anchor: {w}"))
    }
    /// WHAT THIS TWIN TOOK, KEPT BY ITS KEEPER: from now on read from the rows
    pub fn rows_kept(&mut self) {
        self.entries.rows_kept();
    }
    /// WHETHER LINE `g` IS TAKEN OUT NOW: a take-out names it, and that take-out is not taken out itself
    fn out_now(&self, g: i64) -> bool {
        // a take-out is always after what it names: this ends
        self.taken_out.get(&g).is_some_and(|o| o.by.iter().any(|t| !self.out_now(*t)))
    }
    /// THIS TWIN AGAIN FROM ITS LINES, each landed as it was taken — none judged again — but what is taken out now (`out_now`), and
    /// of what a purge erased only what stays of it (`erased_row`): what is, after a take-out or a purge
    fn replay(&self) -> Twin {
        let mut t = Twin { entries: self.entries.fresh(), taken_out: self.taken_out.clone(), erased: self.erased.clone(), head: self.head.clone(), last: self.last, rejected: self.rejected.clone(), signatures: self.signatures.clone(), ..Default::default() };
        let mut lines: BTreeSet<i64> = self.entries.keys().collect();
        lines.extend(self.taken_out.keys());
        for g in lines {
            if t.out_now(g) {
                continue;
            }
            let line = self.line_row(g).expect("a line taken, or taken out, is in the rows");
            let line = match self.erased.get(&g) {
                Some(p) if line.get("erased_by").is_none() => Arc::new(erased_row(&line, *p)),
                _ => line,
            };
            let arrived = line["arrived"].as_i64().expect("a line taken says when it arrived");
            t.land_line(&line, self.present.contains(&g), arrived);
        }
        t
    }
    /// A PERSON ANSWERING THEIR OWN DELETE OF THEMSELVES — the one thing a deleted node writes: a restore or purge of a delete a person
    /// wrote at themselves, by that person. Nothing deleted answers any other delete, nor a delete of itself by anyone above it
    pub fn own_undo(&self, e: &Value) -> bool {
        let by = id_of(&e["by"]);
        matches!(e["is"].as_str(), Some("restore" | "purge"))
            && self.parent(&by).is_none()
            && e["refs"]["re"].as_i64().and_then(|d| self.entries.get(&d)).is_some_and(|d| d["is"] == "delete" && id_of(&d["by"]) == by && id_of(&d["at"]) == by)
    }
    /// THE CODE A NODE RUNS: its newest seal — said by a level above it — or none; nothing runs at a node deleted or blocked
    pub fn sealed(&self, node: &str) -> Option<i64> {
        if self.deleted(node) || self.blocked(node) {
            return None;
        }
        self.at_kind(node, "seal").next_back().map(|(g, _)| g)
    }
    /// A WAY IN, OPEN: the `join` grant this first key answers (`refs.re`), naming it (`as`) — in force, not run out at `arrived`,
    /// and not used yet
    fn way_in(&self, e: &Value, arrived: i64) -> bool {
        let Some(g) = e["refs"]["re"].as_i64() else { return false };
        let Some(j) = self.entries.get(&g).filter(|j| j["is"] == "grant" && j["value"]["action"] == "join" && j["value"]["as"] == e["by"]) else { return false };
        let used = self.answers.get(&g).into_iter().flatten().any(|x| self.is(*x) == "be");
        self.grant_in_force(g, &j, arrived) && !used
    }
    /// A GRANT IN FORCE when an act arrived: not revoked, and not run out (`until`, against when the act arrived) — the one test of it
    fn grant_in_force(&self, g: i64, grant: &Value, arrived: i64) -> bool {
        !self.revoked(g) && grant["value"]["until"].as_i64().is_none_or(|u| arrived < u)
    }
    /// THE ONE TEST OF WHO AND WHAT RUNS (`decides`): always from above — its writer above the node (a person for themselves), or for
    /// a node being made, at or above where it is made — and never by a node that runs code: code is what runs, never who decides.
    /// Each kind may only narrow it: a take-over brings only keys, by a person for themselves; a way back is a person's, to a person,
    /// a device, a recovery code or a proof of theirs
    fn may_decide(&self, e: &Value, what: &str, by: &str, at: &str, root: bool, signer: &str) -> Result<(), String> {
        // NEVER BY CODE: neither the node it is written as, nor the node whose key signed it — code signing as a node of its own
        // decides no more than code itself
        if let Some(x) = [by, signer].into_iter().find(|x| self.sealed(x).is_some()) {
            return Err(format!("{x} runs code: it never decides {what}"));
        }
        let above = match e["is"].as_str().unwrap_or("") {
            "make" => self.within(&id_of(&e["refs"]["in"]), by),
            _ => root || self.from_above(at, by),
        };
        if !above {
            return Err(format!("the {what} of {at} is set only from above it — never by itself, and never under a grant"));
        }
        let person = |n: &str| self.parent(n).is_none();
        match e["is"].as_str().unwrap_or("") {
            "takeover" => {
                let only_keys = e["value"].as_object().is_some_and(|v| v.len() == 1 && v.get("keys").is_some_and(Value::is_array));
                if at != by || !only_keys || !person(at) {
                    return Err(format!("a take-over brings only keys, by a person for themselves — {at} is not taken over"));
                }
            }
            "grant" if what == "way back" => {
                let to = id_of(&e["value"]["to"]);
                let ok = self.nodes.get(&to).is_some_and(|n| matches!(n.kind.as_str(), "person" | "device" | "recovery code")) || self.let_in_as(&to).is_some();
                if !person(at) || !ok {
                    return Err(format!("a way back is a person's, to a person, a device, a recovery code or a proof of theirs — not {to} at {at}"));
                }
            }
            _ => {}
        }
        Ok(())
    }
    /// WHAT AN ENTRY DECIDES OF A NODE, BY WHAT IT DOES: what runs (a seal; a delete, a block or an undo of one over code that runs),
    /// who it is (a make or a `be` saying its keys, who carries it, what stays when it is deleted; a take-over bringing keys, or a
    /// no to one), who may come to be or come back (a way in, a way back, or taking one back) — so `may_decide` is the one test of
    /// every entry that changes `sealed` or `keys_for`
    fn decides(&self, e: &Value) -> Option<&'static str> {
        let at = id_of(&e["at"]);
        let answered = e["refs"]["re"].as_i64().and_then(|g| self.entries.get(&g));
        let way = |g: &Value| match g["value"]["action"].as_str() {
            Some("join") => Some("way in"),
            Some("take over") => Some("way back"),
            _ => None,
        };
        match e["is"].as_str().unwrap_or("") {
            "seal" => Some("code"),
            "take out" => Some("lines"),
            "takeover" => Some("keys"),
            "grant" => way(e),
            "be" | "make" => GOVERNING.iter().find(|k| e["value"].get(**k).is_some()).copied(),
            "delete" | "block" | "restore" if self.nodes.contains_key(&at) && self.subtree(&at).iter().any(|x| self.at_kind(x, "seal").next().is_some()) => Some("code"),
            "revoke" => answered.filter(|g| g["is"] == "grant").and_then(|g| way(&g)),
            "decline" if answered.is_some_and(|g| g["is"] == "takeover") => Some("keys"),
            _ => None,
        }
    }
    /// SET FROM ABOVE: `by` is a node above `at` — or `at` itself when nothing is above it (a person says who they are)
    fn from_above(&self, at: &str, by: &str) -> bool {
        if at == by {
            return self.parent(at).is_none();
        }
        self.within(at, by)
    }
    /// WHETHER A NODE IS BLOCKED: a `block` at it or above it
    pub fn blocked(&self, id: &str) -> bool {
        self.chain(id).iter().any(|x| self.at_kind(x, "block").next().is_some())
    }
    /// THE KEYS A TAKE-OVER BRINGS A PERSON, read from the record whenever they are asked: each take-over at them not declined, its
    /// keys signing from a day after it arrived (`TAKEOVER_WAITS`) — never written into who they are, so nothing a person writes of
    /// their keys starts or ends one
    pub fn taken_over(&self, person: &str, arrived: i64) -> Vec<Value> {
        self.at_kind(person, "takeover")
            .filter(|(g, t)| self.answers.get(g).is_none_or(|a| !a.iter().any(|x| self.is(*x) == "decline")) && t["arrived"].as_i64().expect("an entry taken carries when it arrived") + TAKEOVER_WAITS <= arrived)
            .flat_map(|(_, t)| t["value"]["keys"].as_array().cloned().unwrap_or_default())
            .collect()
    }
    /// WHETHER `proof` IS A WAY BACK `person` GRANTED, in force when `arrived`: a take-over grant at them, to it
    pub(crate) fn takeover_granted(&self, person: &str, proof: &str, arrived: i64) -> bool {
        !self.grants_for(person, "take over", proof, arrived).is_empty()
    }
    /// THE KIND A NODE WAS LET IN BY: the `identity` node its make names (`refs.as`)
    fn let_in_as(&self, id: &str) -> Option<String> {
        let g = *self.nodes.get(id)?.at.front()?;
        parse_id(&self.e(g)["refs"]["as"])
    }
    /// WHO CARRIES A RESPONSIBILITY NOW: its detail `carried_by`
    pub fn carriers(&self, id: &str) -> Vec<String> {
        self.nodes.get(id).and_then(|n| n.who.get("carried_by")).map(ids_of).unwrap_or_default()
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

    /// ONE ENTRY LANDS, as its row was kept: at its node, by its node, naming what it names, arrived when its row says (`arrived`) —
    /// returns what it touched
    pub fn apply(&mut self, e: &Value) -> BTreeSet<String> {
        // WHEN IT ARRIVED is the receiver's — never the time its writer wrote in it
        let Some(arrived) = e["arrived"].as_i64() else {
            self.rejected.push_back(json!({ "g": e["g"], "why": "it says no time it arrived" }));
            return BTreeSet::new();
        };
        self.apply_arrived(e, arrived)
    }
    /// ONE ENTRY LANDS, arrived at `arrived` (ms, the receiver's clock): taken, or kept among the rejected with why
    pub fn apply_arrived(&mut self, e: &Value, arrived: i64) -> BTreeSet<String> {
        match self.admit(e, arrived) {
            Ok(signed) => self.land_line(&kept(e, arrived), signed.present, arrived),
            Err(why) => {
                self.rejected.push_back(why);
                BTreeSet::new()
            }
        }
    }
    /// A LINE TAKEN, AS ITS KEEPER TAKES IT: judged once (`admit`), salted and marked — Taldo's word of what admission found, signed by
    /// `sign` as the Taldo `marker`, chained to the mark before — and landed. What to keep (`Taken`); or why not, and this twin as it
    /// was. `made`: the record's last line when the mark is made — the line before this one as it arrives; a record marked once after
    /// it was kept, its last
    pub fn take(&mut self, e: &Value, arrived: i64, marker: &str, made: i64, sign: &dyn Fn(&[u8]) -> Result<Value, String>) -> Result<Taken, Value> {
        let signed = self.admit(e, arrived)?;
        let refused = |why: String| json!({ "g": e["g"], "why": format!("Taldo could not mark it: {why}") });
        let g = e["g"].as_i64().expect("admitted: it has its number");
        let mut row = kept(e, arrived);
        // ITS SALT: Taldo's own signature of the line, hashed — made by no one else, erased with the words
        let salt = sign(crate::identity::signed_bytes(e).as_bytes()).map_err(refused)?;
        row["salt"] = json!(sha(&salt["sig"].to_string()));
        let erased: Vec<Value> = if e["is"] == "purge" {
            Self::lines_named(e).into_iter().map(|x| self.line_row(x).map(|r| erased_row(&r, g)).ok_or_else(|| refused(format!("#{x} is in no row")))).collect::<Result<_, _>>()?
        } else {
            vec![]
        };
        let mut mark = mark_of(&row, &signed, &self.head, marker, made, &erased);
        mark["sig"] = sign(mark_bytes(&mark).as_bytes()).map_err(refused)?;
        row["mark"] = mark;
        let mut after = self.clone();
        after.head = mark_hash(&row["mark"]);
        after.last = g;
        after.land_line(&row, signed.present, arrived);
        // NOTHING KEPT THAT TALDO COULD NOT MARK AFTER IT: once the record has its Taldo, the key that marks this line still marks the
        // record after it — never Taldo deleted or blocked (or a node above it), nor its keys given to a key it does not hold: every
        // line after would never be read. (A record marked once, at its end, is held to its end instead: `made` past this line)
        let marks_with = row["mark"]["sig"]["key"].as_str().unwrap_or("");
        // ITS OWN MARK HOLDS AS THE READER HOLDS IT: by a key that marks the record as it is when the line comes (`marks_hold`)
        if made < g && self.record_taldo().is_some() && !self.marks_with(marks_with) {
            return Err(json!({ "g": e["g"], "why": "Taldo marks with a key that does not mark the record: it marks nothing" }));
        }
        if made < g && after.record_taldo().is_some() && !after.marks_with(marks_with) {
            return Err(json!({ "g": e["g"], "why": "after it, the key Taldo marks with would not mark the record: Taldo is never deleted, blocked or given keys it does not hold out from under its record" }));
        }
        *self = after;
        Ok(Taken { row, erased, signed })
    }
    /// ONE KEPT ROW READ: its mark checked (`refold`), and the line landed as it was taken — or why the row is not what was kept
    pub(crate) fn read(&mut self, row: &Value, reading: &mut Reading) -> Result<(), String> {
        let g = row["g"].as_i64().ok_or_else(|| format!("a row with no number: {row}"))?;
        let m = &row["mark"];
        let why = |w: &str| format!("#{g}: {w}");
        if m["g"] != row["g"] || m["arrived"] != row["arrived"] {
            return Err(why("its mark is not this line's"));
        }
        if m["prev"].as_str() != Some(self.head.as_str()) {
            return Err(why("its mark is not chained to the mark before it: a row was put in, taken away or changed"));
        }
        match row.get("erased_by") {
            // ERASED: no word to hold to its mark — held to the purge it names, which must erase it
            Some(p) => {
                let p = p.as_i64().filter(|p| *p > g).ok_or_else(|| why("it is erased by no purge after it"))?;
                reading.erased.insert(g, p);
            }
            None => {
                if m["line"].as_str() != Some(line_hash(row).as_str()) {
                    return Err(why("it is not the line that was marked"));
                }
                let key = m["key"].as_str().unwrap_or("");
                if !(key.is_empty() && crate::identity::unsigned_beginning(self, row)) {
                    let (k, _) = crate::identity::signer(row).map_err(|w| why(&w))?;
                    if k != key {
                        return Err(why("it is signed by another key than its mark says"));
                    }
                }
            }
        }
        // MARKED BY A TALDO OF THE RECORD'S PERSON, with a key it had when the mark was made (`made`) — a mark waits until the rows
        // read reach that line
        if m["made"].as_i64().is_none() {
            return Err(why("its mark says not when it was made"));
        }
        reading.early.push(m.clone());
        self.marks_hold(reading)?;
        let present = m["present"].as_bool().ok_or_else(|| why("its mark says not whether a person was there"))?;
        let arrived = row["arrived"].as_i64().ok_or_else(|| why("it says no time it arrived"))?;
        self.land_line(row, present, arrived);
        self.head = mark_hash(m);
        reading.last = g;
        self.last = g;
        self.marks_hold(reading)?;
        // A PURGE READ: every line it names is erased, by it — its row holds no word, and is what stays of it as the purge's mark says
        for x in Self::lines_named(row).into_iter().filter(|_| row["is"] == "purge") {
            reading.erased.remove(&x);
            let said = &m["erased"][x.to_string()];
            let kept = self.entries.source().and_then(|s| s.entry(x));
            if !kept.as_ref().is_some_and(|r| r["erased_by"] == g && said.as_str() == Some(crate::rows::row_hash(r).as_str())) {
                return Err(why(&format!("it erases #{x}, whose row is not what stays of it as the purge says")));
            }
        }
        Ok(())
    }
    /// THE MARKS WAITING, CHECKED once the rows read reach the line each was made at (`made`): by the record's Taldo, with a key that
    /// marked the record then (`marking_keys`: never while that Taldo is blocked or deleted)
    fn marks_hold(&self, reading: &mut Reading) -> Result<(), String> {
        for m in std::mem::take(&mut reading.early) {
            let made = m["made"].as_i64().unwrap_or(i64::MAX);
            let Some(t) = self.record_taldo().filter(|_| made <= reading.last) else {
                reading.early.push(m);
                continue;
            };
            if m["by"] != t.as_str() {
                return Err(format!("#{}'s mark is by {}, not the record's Taldo ({t})", m["g"], m["by"]));
            }
            if !self.marks_with(m["sig"]["key"].as_str().unwrap_or("")) {
                return Err(format!("#{}'s mark is signed by a key that did not mark the record then", m["g"]));
            }
            crate::identity::holds(mark_bytes(&m).as_bytes(), &m["sig"]).map_err(|w| format!("#{}'s mark: {w}", m["g"]))?;
        }
        Ok(())
    }
    /// WHETHER AN ENTRY WOULD BE TAKEN, arrived at `arrived` — every check made before anything changes: taken, with who signed it, with
    /// which key, and whether a person was there, or refused with why (`{ g, why, … }`). The twin is untouched either way: an entry
    /// refused leaves no trace, its signature included. Judged once: a line taken is never judged again
    pub fn admit(&self, e: &Value, arrived: i64) -> Result<crate::identity::Signed, Value> {
        // the entry is well formed, at its writer or below (`check`), and from its writer (`identity`) — and taken once
        let signed = match check(e).and_then(|_| crate::identity::verify(self, e, arrived)) {
            Ok(s) => s,
            Err(why) => return Err(json!({ "g": e["g"], "why": why })),
        };
        let present = signed.present;
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
                // every row kept is a line taken — counted, taken out, or erased (found by its signature's hash): none is taken again
                Some(src) => [s.to_string(), format!("h:{}", sha(s))].iter().any(|k| src.signed(k).is_some_and(|x| Some(x) != e["g"].as_i64())) || self.signatures.contains(s),
            };
            if taken {
                return Err(json!({ "g": e["g"], "why": "this signed act was already taken" }));
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
        // A PERSON COMES TO BE BY THEIR OWN FIRST KEY only at the record's beginning, or through a way in a node that exists gave (a
        // `join` grant: a link, a sign-up) — once; nothing else exists by claiming itself
        let root = !self.nodes.contains_key(&by) && by == at && e["is"] == "be" && (self.nodes.is_empty() || self.way_in(e, arrived));
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
                let Some(d) = d.filter(|d| !self.restored(*d) && !self.purged(*d)) else {
                    return Err(json!({ "g": e["g"], "why": format!("no delete in force at {at} for it to answer") }));
                };
                if e["is"] == "purge" {
                    let deleted_at = self.e(d)["arrived"].as_i64().expect("an entry taken carries when it arrived");
                    let Some(keep) = self.keep_for(&at) else {
                        return Err(json!({ "g": e["g"], "why": format!("nothing says how long the deletion of {at} waits (keep_deleted): it is not erased") }));
                    };
                    let due = deleted_at.saturating_add(keep);
                    if arrived < due {
                        return Err(json!({ "g": e["g"], "why": format!("the deletion of {at} waits until {due}") }));
                    }
                    // IT NAMES EXACTLY WHAT ITS DELETE ERASES (`erasing`), so its keeper erases exactly that
                    let named: BTreeSet<i64> = Self::lines_named(e).into_iter().collect();
                    let want: BTreeSet<i64> = self.erasing(d).into_iter().collect();
                    if named.len() != Self::lines_named(e).len() || named != want {
                        return Err(json!({ "g": e["g"], "why": format!("a purge names exactly what its delete erases: {want:?}") }));
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
        // A TAKE-OUT takes out lines at its node or under it, each written by its own writer or a node under it — set from above that
        // node (`may_decide`): nothing is taken out by a node beside or under whoever wrote it
        if e["is"] == "take out" {
            let g = e["g"].as_i64().expect("check: an entry has its number");
            let lines = Self::lines_named(e);
            if lines.is_empty() || lines.len() != lines.iter().collect::<BTreeSet<_>>().len() {
                return Err(json!({ "g": e["g"], "why": "a take-out names the lines it takes out, each once (value.lines)" }));
            }
            for x in lines {
                // a line that counts, before it — never a node's coming to be (a node is deleted), a delete or its undo (a delete is
                // undone by its restore), nor a purge (what the law erased stays erased)
                let Some(l) = (x < g).then(|| self.entries.get(&x)).flatten().filter(|_| !self.erased.contains_key(&x)) else {
                    return Err(json!({ "g": e["g"], "why": format!("#{x} is no line before it that counts") }));
                };
                let first = self.nodes.get(&id_of(&l["at"])).and_then(|n| n.at.front().copied()) == Some(x);
                if first || matches!(l["is"].as_str(), Some("delete" | "restore" | "purge")) {
                    return Err(json!({ "g": e["g"], "why": format!("#{x} is not taken out: a node is deleted, a delete restored, and a purge stands") }));
                }
                if !self.within(&id_of(&l["at"]), &at) {
                    return Err(json!({ "g": e["g"], "why": format!("#{x} is not at {at} or under it") }));
                }
                if !self.within(&id_of(&l["by"]), &by) {
                    return Err(json!({ "g": e["g"], "why": format!("#{x} was written by {}: only it, or a node above it, takes it out", id_of(&l["by"])) }));
                }
            }
        }
        // BLOCKED OR DELETED, NOTHING IS WRITTEN AS IT — but, by whoever deleted, the delete's own undo or carrying out. Who signs for a
        // node leaves the blocked and deleted out already (`identity::signing_nodes`)
        if self.nodes.contains_key(&by) && self.blocked(&by) {
            return Err(json!({ "g": e["g"], "why": format!("{by} is blocked: it signs nothing from then on") }));
        }
        if self.nodes.contains_key(&by) && self.deleted(&by) && !self.own_undo(e) {
            return Err(json!({ "g": e["g"], "why": format!("{by} is deleted: it signs nothing") }));
        }
        // WHO A NODE IS AND WHAT IT RUNS: every door to them (`decides`) through the one test (`may_decide`)
        if let Some(what) = self.decides(e) {
            if let Err(why) = self.may_decide(e, what, &by, &at, root, &signed.signer) {
                return Err(json!({ "g": e["g"], "why": why }));
            }
        }
        // WHAT IT NAMES EXISTS: a grant's holder, who carries a node — nothing is given to a node that was never made
        let named_in_value: Vec<String> = parse_id(&e["value"]["to"]).filter(|_| e["is"] == "grant").into_iter().chain(e["value"].get("carried_by").map(ids_of).unwrap_or_default()).collect();
        if let Some(x) = named_in_value.iter().find(|x| !self.nodes.contains_key(*x)) {
            return Err(json!({ "g": e["g"], "why": format!("{x} was never made") }));
        }
        // AN ANSWER ANSWERS WHAT IT NAMES IN `re`: an entry before it, of the kind it answers
        // nothing answers what was erased: its words are gone
        if let Some(re) = e["refs"]["re"].as_i64() {
            if self.erased.contains_key(&re) {
                return Err(json!({ "g": e["g"], "why": format!("it answers #{re}, which was erased") }));
            }
            let Some(x) = (re < e["g"].as_i64().expect("check: an entry has its number")).then(|| self.entries.get(&re)).flatten() else {
                return Err(json!({ "g": e["g"], "why": format!("it answers #{re}, which is no entry before it") }));
            };
            let (is, kind) = (e["is"].as_str().unwrap_or(""), x["is"].as_str().unwrap_or(""));
            let not = match answering(is, kind) {
                None => Some(format!("a {is} does not answer a {kind} (#{re})")),
                Some(Whose::Writer) => (!self.within(&id_of(&x["by"]), &by)).then(|| format!("#{re} was written by {}: only it, or a node above it, answers it with a {is}", id_of(&x["by"]))),
                Some(Whose::Node) => (!self.within(&id_of(&x["at"]), &by)).then(|| format!("#{re} is at {}: only it, or a node above it, answers it with a {is}", id_of(&x["at"]))),
                Some(Whose::As) => (x["value"]["action"] != "join" || x["value"]["as"] != json!(by)).then(|| format!("#{re} is no way in for {by}")),
                Some(Whose::Holder) => match self.holder_of(re) {
                    Some(h) if self.within(&h, &by) => None,
                    Some(h) => Some(format!("#{re} is held by {h}: only it, or a node above it, answers it with a {is}")),
                    None => Some(format!("#{re} is held by no one")),
                },
                Some(Whose::Any) => None,
            };
            if let Some(why) = not {
                return Err(json!({ "g": e["g"], "why": why }));
            }
        }
        if let Some(p) = e["value"].get("place").filter(|_| matches!(e["is"].as_str(), Some("be" | "make"))) {
            if let Err(why) = place_whole(p) {
                return Err(json!({ "g": e["g"], "why": why }));
            }
        }
        if e["is"] == "grant" {
            if let Err(why) = grant_whole(&e["value"]) {
                return Err(json!({ "g": e["g"], "why": why }));
            }
        }
        if e["is"] == "seal" {
            if let Err(why) = seal_whole(&e["value"]) {
                return Err(json!({ "g": e["g"], "why": why }));
            }
        }
        if matches!(e["is"].as_str(), Some("purge" | "take out")) && self.entries.is_overlay() {
            return Err(json!({ "g": e["g"], "why": "an overlay shows what acts would do: it takes nothing out" }));
        }
        if e["is"] == "make" {
            if e["value"]["kind"].as_str().is_none_or(str::is_empty) {
                return Err(json!({ "g": e["g"], "why": "a make says what it makes (kind)" }));
            }
            if let Some(k) = e["value"].as_object().into_iter().flatten().map(|(k, _)| k).find(|k| *k != "kind" && *k != "keys") {
                return Err(json!({ "g": e["g"], "why": format!("a make says only its kind (and keys) — its {k} is a detail after it") }));
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
        Ok(signed)
    }
    /// THE GRANTS IN FORCE ON A NODE when `arrived`: each `grant` at it, and each at a node above it that reaches below (`below`) — in
    /// force (`grant_in_force`: not revoked, not run out). The one way any grant is read: no reader tests a grant again
    pub fn grants_on(&self, on: &str, arrived: i64) -> Vec<i64> {
        self.grants_reaching(on).into_iter().filter(|(g, e)| self.grant_in_force(*g, e, arrived)).map(|(g, _)| g).collect()
    }
    /// EVERY GRANT REACHING A NODE, in force or not: at it, or at a node above it reaching below (`below`)
    fn grants_reaching(&self, on: &str) -> Vec<(i64, Arc<Value>)> {
        self.chain(on).iter().enumerate().flat_map(|(i, x)| self.at_kind(x, "grant").filter(move |(_, e)| i == 0 || e["value"]["below"] == true).collect::<Vec<_>>()).collect()
    }
    /// WHOM A GRANT LETS, by what it lets — the one table of it: a way in only the one it names (`as`); a way back only the very node
    /// it names (`to`) — never a node under it; anything else the node it names and every node under it
    fn lets(&self, grant: &Value, by: &str) -> bool {
        let v = &grant["value"];
        match v["action"].as_str() {
            Some("join") => v["as"] == json!(by),
            Some("take over") => parse_id(&v["to"]).as_deref() == Some(by),
            _ => parse_id(&v["to"]).is_some_and(|to| self.within(by, &to)),
        }
    }
    /// THE GRANTS IN FORCE LETTING `by` DO `action` ON `on`, as of `arrived` (`lets`) — the one query of who a grant lets do what
    pub fn grants_for(&self, on: &str, action: &str, by: &str, arrived: i64) -> Vec<i64> {
        self.grants_on(on, arrived).into_iter().filter(|g| { let e = self.e(*g); e["value"]["action"] == action && self.lets(&e, by) }).collect()
    }
    /// a grant revoked: a `revoke` answering it, by its writer or a node above them
    fn revoked(&self, g: i64) -> bool {
        self.answers.get(&g).is_some_and(|a| a.iter().any(|x| self.is(*x) == "revoke"))
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
            .filter(|x| from.map_or(true, |f| x["arrived"].as_i64().is_some_and(|t| t >= f)))
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
        // WHY NOT, said as it is: a grant that named this once and is no longer in force, or none ever
        let once = self.grants_reaching(on).into_iter().any(|(_, e)| e["value"]["action"] == is && self.lets(&e, by));
        let mut why = if once { format!("the grant that let {by} {is} {on} ran out, or was taken back") } else { format!("no grant lets {by} {is} {on}") };
        for g in self.grants_for(on, is, by, arrived) {
            let v = self.e(g)["value"].clone();
            let to = id_of(&v["to"]);
            // its budget was taken whole (`grant_whole`): its field, its max, its window
            if let Some(b) = v["budget"].as_object() {
                let field = b["field"].as_str().expect("a grant's budget, taken whole, says its field");
                let max = b["max"].as_f64().expect("a grant's budget, taken whole, says its max");
                let from = b.get("window").map(|w| arrived - w.as_i64().expect("a grant's window, taken whole, is a time"));
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
    fn land_line(&mut self, e: &Value, present: bool, arrived: i64) -> BTreeSet<String> {
        let e = &kept(e, arrived);
        let mut touched = BTreeSet::new();
        let g = e["g"].as_i64().expect("an entry has its number");
        // A TAKE-OUT OR A PURGE, NEW: from here, what it names has no effect, or only what stays of it — the twin is made again from
        // its lines without that (`replay`), none judged again; then it lands on that. Made again, it finds what it names done
        let named = Self::lines_named(e);
        let new: Vec<i64> = match e["is"].as_str() {
            Some("take out") => named.iter().copied().filter(|x| self.taken_out.get(x).is_none_or(|o| !o.by.contains(&g))).collect(),
            Some("purge") => named.iter().copied().filter(|x| !self.erased.contains_key(x)).collect(),
            _ => vec![],
        };
        if !new.is_empty() {
            for x in &new {
                if e["is"] == "purge" {
                    self.erased.insert(*x, g);
                    // A LINE TAKEN OUT, ERASED: its row is what stays of it from now on, and nothing more is held of it
                    if let Some(line) = self.taken_out.get(x).and_then(|_| self.line_row(*x)) {
                        let stays = erased_row(&line, g);
                        let o = self.taken_out.get_mut(x).expect("taken out");
                        o.hash = crate::rows::row_hash(&stays);
                        o.line = o.line.as_ref().map(|_| Arc::new(stays));
                    }
                    continue;
                }
                if let Some(o) = self.taken_out.get_mut(x) {
                    o.by.push_back(g);
                    continue;
                }
                let line = self.entries.get(x).expect("a line a take-out names when it is taken counts");
                let held = self.entries.source().is_none().then(|| line.clone());
                self.taken_out.insert(*x, Out { by: Vector::unit(g), at: id_of(&line["at"]), writer: id_of(&line["by"]), is: line["is"].as_str().unwrap_or("").to_string(), hash: crate::rows::row_hash(&line), line: held });
            }
            let again = self.replay();
            touched.extend(new.iter().filter_map(|x| self.entries.get(x).map(|l| id_of(&l["at"]))).filter(|x| again.nodes.contains_key(x)));
            *self = again;
        }
        if self.entries.source().is_none() || self.entries.is_overlay() {
            if let Some(s) = e["sig"]["sig"].as_str().and_then(crate::identity::canonical_sig) {
                self.signatures.insert(s);
            }
        }
        let (by, at) = (id_of(&e["by"]), id_of(&e["at"]));
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
        if let Some(x) = e["refs"]["re"].as_i64() {
            self.answers.entry(x).or_default().push_back(g);
            if let Some(a) = self.entries.get(&x).and_then(|y| parse_id(&y["at"])) {
                touched.insert(a);
            }
        }
        match e["is"].as_str().unwrap_or("") {
            "be" | "make" => {
                let n = self.nodes.get_mut(&at).unwrap();
                n.who = said_who(std::mem::take(&mut n.who), e);
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
                let v = self.e(*w)["value"]
                    .as_object()
                    .cloned()
                    .unwrap_or_default();
                self.moved_as(*w, holder)
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
        self.answers
            .get(&g)
            .into_iter()
            .flatten()
            .copied()
            .filter(|w| *w > g && self.accepted(*w))
            .find(|w| {
                let e = self.e(*w);
                self.moved_as(*w, holder)
                    && acts.iter().any(|a| {
                        a["is"] == e["is"]
                            && (a["value"].is_null() || a["value"] == e["value"])
                            && (a["at"].is_null() || parse_id(&a["at"]) == parse_id(&e["at"]))
                    })
            })
    }
    /// WHETHER ENTRY `w` IS A MOVE OF `holder`'S: written as the holder or a level above it — as admission proved it
    fn moved_as(&self, w: i64, holder: &str) -> bool {
        // what it is written as was proven at admission (its signer signed for it then); where a node is never changes — so nothing
        // here reads who carries whom today
        self.within(holder, &id_of(&self.e(w)["by"]))
    }
    /// declined by whoever holds it
    fn declined(&self, g: i64) -> bool {
        // by its holder, or a level above it — you are above every hand: the only no an ask takes (`answering`)
        self.answers.get(&g).is_some_and(|a| a.iter().any(|x| self.is(*x) == "decline"))
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
        let declined = self.declined(x);
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
        // A HOLD IS REACHED when what it asked is answered (`re`) from its address or below; anything else, when it is answered so
        let asked = if self.is(what) == "acceptance" { self.asked(what) } else { vec![what] };
        asked.iter().any(|g| self.answers.get(g).into_iter().flatten().any(|x| self.within(&id_of(&self.e(*x)["by"]), who)))
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
