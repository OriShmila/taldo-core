//! DELETING: a delete hides a node and everything under it at once — no act lands there, nothing about it is
//! asked or owed — and is undone with one act. It takes the waiting the owner said (`keep_deleted`, on the node or above it): until
//! then, a delete is refused, and says why. When the waiting is over, a `purge` erases what it held:
//! every entry at it and under it goes, but the skeleton stays (each make: an id and a kind, never a name) and so does the delete,
//! so whatever another owner wrote pointing to it still reads. An erased act is never taken again: the purge keeps its signature.
mod common;
use common::{act, make};
use taldo_core::twin::{fold, Twin};
use serde_json::{json, Value};

const KEEP: i64 = 1_000;

fn world() -> Vec<Value> {
    vec![
        common::first_key("taxowner"),
        common::signed(json!({ "g": 1, "ts": 1, "arrived": 1, "by": "taxowner", "at": "taxowner", "is": "be", "refs": {}, "value": { "keys": [{ "key": common::public("taxowner") }, { "key": common::finger_public() }] } })),
        make(2, "taxowner", "taxorg", "taxowner", "org"),
        make(3, "taxorg", "dana", "taxorg", "outside"),
        make(4, "taxorg", "case", "dana", "thing"),
        make(5, "taxorg", "form", "case", "document"),
        act(6, "taxorg", "dana", "be", json!({}), json!({ "name": "Dana", "phone": "050" })),
        act(7, "taxorg", "form", "be", json!({}), json!({ "name": "Form 106", "id": "123" })),
        // something of the org's own that points to Dana: it stays, pointing to an empty node
        act(8, "taxorg", "taxorg", "say", json!({ "about": "dana" }), json!({ "says": "a new client" })),
    ]
}
fn with(more: &[Value]) -> Twin {
    let mut l = world();
    l.extend_from_slice(more);
    fold(&l)
}
fn taken(t: &Twin, g: i64) -> bool {
    t.entries.contains_key(&g)
}
fn why(t: &Twin, g: i64) -> String {
    t.rejected.iter().find(|r| r["g"] == g).map(|r| r["why"].to_string()).unwrap_or_default()
}
fn keep(g: i64) -> Value {
    act(g, "taxowner", "taxorg", "be", json!({}), json!({ "keep_deleted": KEEP }))
}
fn delete(g: i64, ts: i64, at: &str) -> Value {
    common::touched(json!({ "g": g, "ts": ts, "arrived": ts, "by": "taxowner", "at": at, "is": "delete", "refs": {}, "value": {} }))
}

#[test]
fn a_delete_takes_the_waiting_the_owner_said() {
    let t = with(&[delete(20, 20, "dana")]);
    assert!(!taken(&t, 20), "a delete was taken with no waiting said");
    assert!(why(&t, 20).contains("keep_deleted"), "{}", why(&t, 20));
    let t = with(&[keep(19), delete(20, 20, "dana")]);
    assert!(taken(&t, 20), "{}", why(&t, 20));
}

#[test]
fn a_deleted_node_is_hidden_at_once_and_comes_back_when_undone() {
    let ask = act(21, "taxorg", "taxorg", "ask", json!({ "about": "case" }), json!({ "label": "intake", "says": "intake?", "moves": [{ "says": "Done", "act": { "is": "intake" } }] }));
    let t = with(&[keep(19), ask.clone()]);
    assert!(t.ask_open(21));
    let mut log = vec![keep(19), ask, delete(22, 22, "dana"), act(23, "taxorg", "form", "be", json!({}), json!({ "x": 1 }))];
    let t = with(&log);
    assert!(t.deleted("form") && t.deleted("dana") && !t.deleted("taxorg"));
    assert!(!taken(&t, 23), "an act landed under a deleted node");
    assert!(!t.ask_open(21), "a stage about a deleted node is still asked");
    // CLOSED, IT SAYS WHY: whoever asked is told, never left waiting
    assert_eq!(t.closed_why(21).as_deref(), Some("case was deleted"));
    // undone, with a person there: back, whole
    log.pop();
    log.push(common::touched(json!({ "g": 23, "ts": 23, "arrived": 23, "by": "taxowner", "at": "dana", "is": "restore", "refs": { "re": 22 }, "value": {} })));
    log.push(act(24, "taxorg", "form", "be", json!({}), json!({ "x": 1 })));
    let t = with(&log);
    assert!(!t.deleted("form") && taken(&t, 24), "{}", why(&t, 24));
    assert!(t.ask_open(21));
    assert_eq!(t.name("dana").as_deref(), Some("Dana"));
}

#[test]
fn a_purge_erases_what_it_held_after_the_waiting_and_keeps_the_skeleton() {
    let log = vec![keep(19), delete(20, 20, "dana")];
    let t = with(&log);
    let erased = t.erasing(20);
    assert_eq!(erased, vec![6, 7], "what the purge erases: every entry at Dana and under her but the makes and the delete");
    let purge = |g: i64, ts: i64| common::signed(json!({ "g": g, "ts": ts, "arrived": ts, "by": "taxorg", "at": "dana", "is": "purge", "refs": { "re": 20 }, "value": { "lines": [6, 7] } }));
    // before the waiting is over: refused
    let t = with(&[keep(19), delete(20, 20, "dana"), purge(21, 20 + KEEP - 1)]);
    assert!(!taken(&t, 21), "a purge was taken before the waiting was over");
    // after it, taken: what it held says nothing any more, and what stays of each line is all its keeper keeps (`erased_row`)
    let t = with(&[keep(19), delete(20, 20, "dana"), purge(21, 20 + KEEP)]);
    assert!(taken(&t, 21), "{}", why(&t, 21));
    for g in [6, 7] {
        assert!(t.entries.get(&g).is_some_and(|e| e["value"] == json!({}) && e["erased_by"] == 21), "entry {g} still says what it said");
    }
    assert_eq!((t.name("dana"), t.nodes["form"].value.get("id")), (None, None), "what was erased is still read off its node");
    assert_eq!(Twin::lines_named(&purge(21, 20 + KEEP)), vec![6, 7]);
    assert_eq!(t.nodes["dana"].kind, "outside");
    assert!(taken(&t, 8), "what the org wrote pointing to her was lost");
    // an erased act given again, under a new number: never taken
    let mut again = world()[6].clone();
    again["g"] = json!(30);
    assert!(t.admit(&again, 30).is_err(), "an erased act was taken again");
    // undone before the purge: no purge
    let undone = common::touched(json!({ "g": 21, "ts": 21, "arrived": 21, "by": "taxowner", "at": "dana", "is": "restore", "refs": { "re": 20 }, "value": {} }));
    let t = with(&[keep(19), delete(20, 20, "dana"), undone, purge(22, 20 + KEEP)]);
    assert!(!taken(&t, 22), "a purge erased what was brought back");
}

#[test]
fn only_who_deleted_undoes_it() {
    // the owner deleted the case: the org under them, though the case is its own, never brings it back nor erases it
    let below = act(21, "taxorg", "case", "restore", json!({ "re": 20 }), json!({}));
    let t = with(&[keep(19), delete(20, 20, "case"), below]);
    assert!(!taken(&t, 21) && why(&t, 21).contains("only it, or a node above it"), "what was under the deleter undid its delete: {}", why(&t, 21));
    assert!(t.deleted("case"), "the case came back by a restore from under who deleted it");
    let own = common::touched(json!({ "g": 21, "ts": 21, "arrived": 21, "by": "taxowner", "at": "case", "is": "restore", "refs": { "re": 20 }, "value": {} }));
    let t = with(&[keep(19), delete(20, 20, "case"), own]);
    assert!(taken(&t, 21) && !t.deleted("case"), "who deleted it could not bring it back: {}", why(&t, 21));
}

// WHAT KEPT AN ASK TRUE NO LONGER HOLDS: its `while` (details its thing must have) is read by the core — closed and owed to no one,
// and it says why; an ask whose thing moved, or was declined, says that
#[test]
fn an_ask_is_open_only_while_what_keeps_it_true_holds() {
    let ask = act(21, "taxorg", "taxorg", "ask", json!({ "about": "case" }), json!({ "label": "intake", "says": "intake?", "moves": [{ "says": "Done", "to": { "intake": "done" } }], "while": { "stage": "intake" } }));
    let at_intake = act(20, "taxorg", "case", "be", json!({}), json!({ "stage": "intake" }));
    let t = with(&[at_intake.clone(), ask.clone()]);
    assert!(t.ask_open(21) && t.closed_why(21).is_none());
    let t = with(&[at_intake.clone(), ask.clone(), act(22, "taxorg", "case", "be", json!({}), json!({ "stage": "calculation" }))]);
    assert!(!t.ask_open(21), "an ask whose reason ended is still open");
    assert_eq!(t.closed_why(21).as_deref(), Some("what kept it true no longer holds"));
    assert!(t.asks_for("taxowner").iter().chain(t.asks_for("taxorg").iter()).all(|(g, _)| *g != 21), "an ask whose reason ended is still owed");
    let t = with(&[at_intake, ask, common::touched(json!({ "g": 22, "ts": 22, "arrived": 22, "by": "taxowner", "at": "case", "is": "decline", "refs": { "re": 21 }, "value": { "says": "not this year" } }))]);
    assert_eq!(t.closed_why(21).as_deref(), Some("declined: not this year"));
}

// AN ASK WHOSE REASON ENDED IS OVER (29 Oct): one raised after it, like it, stands on its own — never evidence on the one that ended,
// so its thing is never left with no ask open
#[test]
fn an_ask_raised_after_one_whose_reason_ended_stands_on_its_own() {
    let ask = |g: i64, says: &str, stage: &str| act(g, "taxorg", "taxorg", "ask", json!({ "about": "case" }), json!({ "label": "intake", "says": says, "moves": [{ "says": "Done", "to": { "intake": "done" } }], "while": { "stage": stage } }));
    let t = with(&[act(20, "taxorg", "case", "be", json!({}), json!({ "stage": "intake" })), ask(21, "intake?", "intake"), act(22, "taxorg", "case", "be", json!({}), json!({ "stage": "again" })), ask(23, "intake, again?", "again")]);
    assert!(!t.ask_open(21), "the ask whose reason ended is still open");
    assert!(t.joins(23).is_none(), "the new ask was taken as evidence on the one that ended");
    assert!(t.ask_open(23), "the new ask is not open");
    // while the first one's reason holds, one like it is evidence on it, as ever
    let t = with(&[act(20, "taxorg", "case", "be", json!({}), json!({ "stage": "intake" })), ask(21, "intake?", "intake"), ask(22, "intake, again?", "intake")]);
    assert_eq!(t.joins(22), Some(21));
}
