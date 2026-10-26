//! THE CORE'S DESIGN, AS ORI SAID IT (26 Oct) — each test one of his rules, written before the code that keeps it:
//! 1. WHO A NODE IS, AND WHAT IT RUNS, ARE DECIDED FROM ABOVE IT: set when it is made, by its maker above it, and changed only by a
//!    node above it — never by itself, never under a grant, never by any other kind of entry; a person, with nothing above, says who
//!    they are with their own key, and is taken over only by their own way back.
//! 2. NOTHING EXISTS BY CLAIMING ITSELF: every node is made by a node that exists (itself, or under that node's grant — a link, a
//!    sign-up); only the record's beginning is a person by their own first key; every node an entry names exists.
//! 3. AN ANSWER COUNTS ONLY THROUGH WHAT IT ANSWERS (`refs.re`): an entry that exists, of the kind it answers, before it.
//! 4. TIME IS WHEN AN ENTRY ARRIVED, never the time its writer wrote in it.
mod common;
use common::*;
use serde_json::{json, Value};
use taldo_core::twin::{fold, Twin};

const ORI: &str = "Ori";
const TALDO: &str = "Ori › Taldo";
const ENGINE: &str = "Ori › Taldo › engine";
const BESIDE: &str = "Ori › beside";

fn world() -> Vec<Value> {
    vec![first_key(ORI), under(1, TALDO), under(2, ENGINE), under(3, BESIDE)]
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
    t.rejected.iter().find(|r| r["g"] == g).map(|r| r["why"].as_str().unwrap_or("").to_string()).unwrap_or_else(|| "taken".into())
}
/// who signs for a node now: its own keys
fn signs(t: &Twin, node: &str) -> Vec<String> {
    let mut ks: Vec<String> = taldo_core::identity::keys_for(t, node, i64::MAX).into_iter().map(|(k, _)| k).collect();
    ks.sort();
    ks
}

// ─── 1. WHO A NODE IS ───

#[test]
fn every_kind_of_entry_but_one_from_above_leaves_who_a_node_is_as_it_was() {
    let before = signs(&with(&[]), ENGINE);
    let grant_be = act(10, TALDO, ENGINE, "grant", json!({}), json!({ "to": BESIDE, "action": "be" }));
    // from itself, from beside it (under a grant), and through every other kind of entry: nothing changes who signs for it
    let tries: Vec<(&str, Value)> = vec![
        ("itself, as a detail", act(11, ENGINE, ENGINE, "be", json!({}), json!({ "keys": [{ "key": public("an outsider") }] }))),
        ("beside it, under a grant to write its details", act(11, BESIDE, ENGINE, "be", json!({}), json!({ "keys": [{ "key": public("an outsider") }] }))),
        ("itself, as who carries it", act(11, ENGINE, ENGINE, "be", json!({}), json!({ "carried_by": BESIDE }))),
        ("itself, by a take-over", act(11, ENGINE, ENGINE, "takeover", json!({}), json!({ "keys": [{ "key": public("an outsider") }] }))),
        ("itself, in an entry of its own kind", act(11, ENGINE, ENGINE, "said", json!({}), json!({ "keys": [{ "key": public("an outsider") }] }))),
    ];
    for (how, e) in tries {
        let t = with(&[grant_be.clone(), e]);
        assert_eq!(signs(&t, ENGINE), before, "who signs for the engine changed: {how}");
    }
    // a node made beside, under a grant to make there, is given no keys by its maker
    let grant_make = act(10, TALDO, TALDO, "grant", json!({}), json!({ "to": BESIDE, "action": "make" }));
    let t = with(&[grant_make, act(11, BESIDE, "Ori › Taldo › new", "make", json!({ "in": TALDO }), json!({ "kind": "agent", "keys": [{ "key": public("an outsider") }] }))]);
    assert!(!signs(&t, "Ori › Taldo › new").contains(&public("an outsider")), "a maker beside gave the node it made its keys");
}

#[test]
fn who_a_node_is_is_set_by_its_maker_and_changed_from_above() {
    let t = with(&[]);
    assert!(signs(&t, ENGINE).contains(&public(ENGINE)), "its maker's keys for it");
    let t = with(&[act(10, TALDO, ENGINE, "be", json!({}), json!({ "keys": [{ "key": public("its new key") }] }))]);
    assert!(taken(&t, 10), "{}", why(&t, 10));
    assert!(signs(&t, ENGINE).contains(&public("its new key")) && !signs(&t, ENGINE).contains(&public(ENGINE)), "the level above did not change who it is");
}

#[test]
fn a_person_says_who_they_are_with_their_own_key() {
    let t = with(&[act(10, ORI, ORI, "be", json!({}), json!({ "keys": [{ "key": public(ORI) }, { "key": public("Ori's phone") }] }))]);
    assert!(taken(&t, 10), "{}", why(&t, 10));
    assert!(signs(&t, ORI).contains(&public("Ori's phone")));
}

#[test]
fn what_a_node_runs_is_sealed_from_above_only() {
    let whole = json!({ "wasm": "a".repeat(64), "cwasm": { "x86_64": "b".repeat(64) }, "contract": "taldo:level@0.3.0", "reach": [] });
    for by in [ENGINE, BESIDE] {
        let t = with(&[act(10, by, ENGINE, "seal", json!({}), whole.clone())]);
        assert!(!taken(&t, 10) && t.sealed(ENGINE).is_none(), "{by} sealed the engine");
    }
    let t = with(&[act(10, TALDO, ENGINE, "seal", json!({}), whole)]);
    assert_eq!(t.sealed(ENGINE), Some(10), "{}", why(&t, 10));
}

// ─── 2. NOTHING EXISTS BY CLAIMING ITSELF ───

#[test]
fn no_one_claims_to_be_a_person_after_the_beginning() {
    let claim = json!({ "g": 10, "ts": 10, "arrived": 10, "by": "Mallory", "at": "Mallory", "is": "be", "refs": {}, "value": { "keys": [{ "key": public("Mallory") }] } });
    for e in [claim.clone(), signed(claim)] {
        let t = with(&[e]);
        assert!(!taken(&t, 10) && !t.nodes.contains_key("Mallory"), "someone became a person by claiming it: {}", why(&t, 10));
    }
}

#[test]
fn a_new_node_is_made_by_one_that_exists_or_under_its_grant() {
    let t = with(&[act(10, TALDO, TALDO, "grant", json!({}), json!({ "to": BESIDE, "action": "make" })), act(11, BESIDE, "Ori › Taldo › sign-up", "make", json!({ "in": TALDO }), json!({ "kind": "person" }))]);
    assert!(taken(&t, 11), "{}", why(&t, 11));
    let t = with(&[act(10, BESIDE, "Ori › Taldo › sign-up", "make", json!({ "in": TALDO }), json!({ "kind": "person" }))]);
    assert!(!taken(&t, 10), "a node was made where its maker was given nothing");
}

#[test]
fn every_node_an_entry_names_exists() {
    for (g, e) in [
        (10, act(10, TALDO, TALDO, "grant", json!({}), json!({ "to": "Ori › nobody", "action": "be" }))),
        (10, act(10, TALDO, ENGINE, "be", json!({}), json!({ "carried_by": "Ori › nobody" }))),
    ] {
        let t = with(&[e]);
        assert!(!taken(&t, g), "an entry naming a node never made was taken");
    }
}

// ─── 3. AN ANSWER COUNTS ONLY THROUGH WHAT IT ANSWERS ───

#[test]
fn an_answer_counts_only_through_its_re_to_what_exists_of_its_kind() {
    let keep = act(10, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 0 }));
    let d1 = act(11, ORI, ENGINE, "delete", json!({}), json!({}));
    let d2 = act(12, ORI, BESIDE, "delete", json!({}), json!({}));
    // a restore of one delete carrying the other's number elsewhere restores only the one it answers
    let r = act(13, ORI, ENGINE, "restore", json!({ "re": 11, "also": 12 }), json!({}));
    let t = with(&[keep.clone(), d1.clone(), d2.clone(), r]);
    assert!(t.rejected.is_empty(), "{:?}", t.rejected);
    assert!(!t.deleted(ENGINE) && t.deleted(BESIDE), "a number beside its re answered another delete");
    // an answer to what is not there, or not of its kind, or not yet written, is no answer
    for (g, e) in [
        (13, act(13, ORI, ENGINE, "restore", json!({ "re": 99 }), json!({}))),
        (13, act(13, ORI, ENGINE, "restore", json!({ "re": 10 }), json!({}))),
    ] {
        let t = with(&[keep.clone(), d1.clone(), e]);
        assert!(!taken(&t, g) && t.deleted(ENGINE), "an answer to what it does not answer was taken");
    }
}

// ─── 4. TIME IS WHEN AN ENTRY ARRIVED ───

#[test]
fn a_grant_runs_out_by_when_an_act_arrived_never_by_its_writers_clock() {
    let mut t = with(&[act(10, TALDO, ENGINE, "grant", json!({}), json!({ "to": BESIDE, "action": "be", "until": 1_000 }))]);
    // written as if early, arriving late: its writer's clock says nothing
    let late = signed(json!({ "g": 11, "ts": 500, "arrived": 500, "by": BESIDE, "at": ENGINE, "is": "be", "refs": {}, "value": { "says": "x" } }));
    t.apply_arrived(&late, 2_000);
    assert!(!t.entries.contains_key(&11), "an act was taken under a grant run out, by the time its writer wrote in it");
}

#[test]
fn a_person_comes_in_once_through_a_way_in_that_exists() {
    let t = with(&[invite(10, ORI), joins(11, "Dana", 10)]);
    assert!(taken(&t, 11) && t.nodes.contains_key("Dana"), "{}", why(&t, 11));
    let t = with(&[invite(10, ORI), joins(11, "Dana", 10), joins(12, "Mallory", 10)]);
    assert!(!taken(&t, 12), "one way in let two people in");
    let t = with(&[invite(10, ORI), act(11, ORI, ORI, "revoke", json!({ "re": 10 }), json!({})), joins(12, "Dana", 10)]);
    assert!(!taken(&t, 12), "a way in taken back still let someone in");
}

#[test]
fn a_record_folds_by_when_each_row_arrived() {
    let grant = act(10, TALDO, ENGINE, "grant", json!({}), json!({ "to": BESIDE, "action": "be", "until": 1_000 }));
    let mut late = act(11, BESIDE, ENGINE, "be", json!({}), json!({ "says": "x" }));
    late["arrived"] = json!(2_000);
    assert!(!taken(&with(&[grant.clone(), late]), 11), "a row that arrived after its grant ran out was taken, by its writer's time");
    let mut on_time = act(11, BESIDE, ENGINE, "be", json!({}), json!({ "says": "x" }));
    on_time["ts"] = json!(5_000);
    on_time = signed(on_time);
    on_time["arrived"] = json!(900);
    assert!(taken(&with(&[grant, on_time]), 11), "a row that arrived in time was refused by its writer's time");
}

#[test]
fn a_twin_folded_again_from_its_rows_is_the_same() {
    use taldo_core::seams::Store;
    let log: Vec<Value> = world().into_iter().chain([invite(10, ORI), joins(11, "Dana", 10)]).collect();
    let store = std::sync::Arc::new(taldo_core::fakes::FakeStore::default());
    store.lease("t").unwrap();
    for e in &log {
        store.append("t", std::slice::from_ref(e)).unwrap();
    }
    let (a, b) = (fold(&log), Twin::refold(store));
    assert!(b.rejected.is_empty(), "{:?}", b.rejected);
    assert_eq!(a.nodes.keys().collect::<Vec<_>>(), b.nodes.keys().collect::<Vec<_>>());
    assert_eq!(signs(&a, "Dana"), signs(&b, "Dana"));
}

#[test]
fn a_delete_waits_from_when_it_arrived_never_its_writers_clock() {
    let keep = act(10, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 1_000 }));
    let mut d = act(11, ORI, ENGINE, "delete", json!({}), json!({}));
    d["ts"] = json!(-5_000);
    let d = signed(d);
    let mut d = d;
    d["arrived"] = json!(5_000);
    let mut early = act(12, ORI, ENGINE, "purge", json!({ "re": 11 }), json!({ "sigs": [] }));
    early["arrived"] = json!(5_500);
    let t = with(&[keep, d, early]);
    assert!(!taken(&t, 12), "a purge was taken by its delete's writer's time, before the delete's own wait from when it arrived");
}

#[test]
fn whoever_joins_proves_the_key_they_name() {
    let unsigned = json!({ "g": 11, "ts": 11, "arrived": 11, "by": "Dana", "at": "Dana", "is": "be", "refs": { "re": 10 }, "value": { "keys": [{ "key": public("Dana") }] } });
    let t = with(&[invite(10, ORI), unsigned]);
    assert!(!taken(&t, 11), "someone joined without proving any key");
    let mut other = json!({ "g": 11, "ts": 11, "by": "Dana", "at": "Dana", "is": "be", "refs": { "re": 10 }, "value": { "keys": [{ "key": public("Dana") }] } });
    other = signed_by("Mallory", other);
    let t = with(&[invite(10, ORI), other]);
    assert!(!taken(&t, 11), "someone joined signing with a key they do not name");
}
