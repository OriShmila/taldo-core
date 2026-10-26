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
    let sigs: Vec<Value> = erased.iter().map(|g| t.entry(*g)["sig"]["sig"].clone()).collect();
    let purge = |g: i64, ts: i64| common::signed(json!({ "g": g, "ts": ts, "arrived": ts, "by": "taxorg", "at": "dana", "is": "purge", "refs": { "re": 20 }, "value": { "sigs": sigs } }));
    // before the waiting is over: refused
    let t = with(&[keep(19), delete(20, 20, "dana"), purge(21, 20 + KEEP - 1)]);
    assert!(!taken(&t, 21), "a purge was taken before the waiting was over");
    // after it, taken — and the record without what it erased folds to the skeleton
    let t = with(&[keep(19), delete(20, 20, "dana"), purge(21, 20 + KEEP)]);
    assert!(taken(&t, 21), "{}", why(&t, 21));
    let mut after: Vec<Value> = world().into_iter().filter(|e| !erased.contains(&e["g"].as_i64().unwrap())).collect();
    after.extend([keep(19), delete(20, 20, "dana"), purge(21, 20 + KEEP)]);
    let t = fold(&after);
    assert!(t.rejected.is_empty(), "the record after a purge does not fold: {:?}", t.rejected);
    assert_eq!(t.nodes["dana"].kind, "outside");
    assert_eq!(t.name("dana"), None, "who she was is still there");
    assert!(t.nodes["form"].value.get("id").is_none());
    assert!(taken(&t, 8), "what the org wrote pointing to her was lost");
    // an erased act given again: never taken
    let mut replay = after.clone();
    replay.push(world()[6].clone());
    let t = fold(&replay);
    assert!(!taken(&t, 6), "an erased act was taken again");
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
