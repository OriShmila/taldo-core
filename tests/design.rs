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
    let t = with(&[invite(10, ORI, "Dana"), joins(11, "Dana", 10)]);
    assert!(taken(&t, 11) && t.nodes.contains_key("Dana"), "{}", why(&t, 11));
    let t = with(&[invite(10, ORI, "Dana"), joins(11, "Dana", 10), joins(12, "Mallory", 10)]);
    assert!(!taken(&t, 12), "one way in let two people in");
    let t = with(&[invite(10, ORI, "Dana"), act(11, ORI, ORI, "revoke", json!({ "re": 10 }), json!({})), joins(12, "Dana", 10)]);
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
    let log: Vec<Value> = world().into_iter().chain([invite(10, ORI, "Dana"), joins(11, "Dana", 10)]).collect();
    let store = std::sync::Arc::new(taldo_core::fakes::FakeStore::default());
    store.lease("t").unwrap();
    for e in &log {
        store.keep("t", std::slice::from_ref(e), &[], b"").unwrap();
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
    let mut early = act(12, ORI, ENGINE, "purge", json!({ "re": 11 }), json!({ "lines": [] }));
    early["arrived"] = json!(5_500);
    let t = with(&[keep, d, early]);
    assert!(!taken(&t, 12), "a purge was taken by its delete's writer's time, before the delete's own wait from when it arrived");
}

#[test]
fn whoever_joins_proves_the_key_they_name() {
    let unsigned = json!({ "g": 11, "ts": 11, "arrived": 11, "by": "Dana", "at": "Dana", "is": "be", "refs": { "re": 10 }, "value": { "keys": [{ "key": public("Dana") }] } });
    let t = with(&[invite(10, ORI, "Dana"), unsigned]);
    assert!(!taken(&t, 11), "someone joined without proving any key");
    let mut other = json!({ "g": 11, "ts": 11, "by": "Dana", "at": "Dana", "is": "be", "refs": { "re": 10 }, "value": { "keys": [{ "key": public("Dana") }] } });
    other = signed_by("Mallory", other);
    let t = with(&[invite(10, ORI, "Dana"), other]);
    assert!(!taken(&t, 11), "someone joined signing with a key they do not name");
}

#[test]
fn code_never_decides_who_anyone_is_nor_what_runs() {
    let whole = json!({ "program": { "x86_64-linux": "a".repeat(64) } });
    let sealed = act(10, TALDO, ENGINE, "seal", json!({}), json!({ "wasm": "a".repeat(64), "cwasm": { "x86_64": "b".repeat(64) }, "contract": "c", "reach": [] }));
    let part = "Ori › Taldo › engine › part";
    for (g, e) in [
        (12, act(12, ENGINE, part, "be", json!({}), json!({ "keys": [{ "key": public("an outsider") }] }))),
        (12, act(12, ENGINE, part, "seal", json!({}), whole.clone())),
        (12, act(12, ENGINE, ENGINE, "grant", json!({}), json!({ "action": "join", "as": "Ori › Taldo › engine › joiner" }))),
        (12, act(12, ENGINE, "Ori › Taldo › engine › new", "make", json!({ "in": ENGINE }), json!({ "kind": "agent", "keys": [] }))),
    ] {
        let t = with(&[sealed.clone(), under(11, part), e.clone()]);
        assert!(!taken(&t, g), "code decided who or what runs: {e}");
    }
}

// ─── THE HOLES A REVIEW FOUND (26 Oct): each by what it would do, closed where the rule lives ───

#[test]
fn a_grant_to_grant_never_reaches_who_a_person_is() {
    // Mallory, let to write grants at Ori, writes herself a way back to Ori: refused — a way back is set only from above
    let m = "Ori › beside";
    let t = with(&[act(10, ORI, ORI, "grant", json!({}), json!({ "to": m, "action": "grant" })), act(11, m, ORI, "grant", json!({}), json!({ "to": m, "action": "take over" }))]);
    assert!(!taken(&t, 11), "a grant to grant gave a way back into a person");
}

#[test]
fn a_way_in_lets_in_only_the_one_it_names_and_no_one_burns_it() {
    let t = with(&[invite(10, ORI, "Dana"), joins(11, "Mallory", 10), joins(12, "Dana", 10)]);
    assert!(!taken(&t, 11), "a way in for Dana let Mallory in");
    assert!(taken(&t, 12), "Dana's way in was burnt by someone else: {}", why(&t, 12));
    let t = with(&[invite(10, ORI, "Dana"), act(11, ORI, ORI, "be", json!({ "re": 10 }), json!({ "says": "seen" })), joins(12, "Dana", 10)]);
    assert!(!taken(&t, 11) && taken(&t, 12), "an answer by another used up Dana's way in");
}

#[test]
fn what_runs_is_never_ended_under_a_grant() {
    let keep = act(9, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 1_000 }));
    let sealed = act(10, TALDO, ENGINE, "seal", json!({}), json!({ "wasm": "a".repeat(64), "cwasm": { "x86_64": "b".repeat(64) }, "contract": "c", "reach": [] }));
    let grant = act(11, TALDO, ENGINE, "grant", json!({}), json!({ "to": BESIDE, "action": "delete" }));
    let t = with(&[keep, sealed, grant, act(12, BESIDE, ENGINE, "delete", json!({}), json!({}))]);
    assert!(!taken(&t, 12) && t.sealed(ENGINE).is_some(), "a grant to delete stopped code that runs");
}

#[test]
fn a_person_saying_their_keys_never_starts_nor_ends_a_take_over() {
    // a way back to a device of Ori's, a take-over through it — then Ori says his keys again, naming only his own
    const PHONE: &str = "Ori › phone";
    let way = [make(4, ORI, PHONE, ORI, "device"), act(10, ORI, ORI, "grant", json!({}), json!({ "to": PHONE, "action": "take over" }))];
    let over = signed_by(PHONE, json!({ "g": 11, "ts": 100, "arrived": 100, "by": ORI, "at": ORI, "is": "takeover", "refs": { "proof": PHONE }, "value": { "keys": [{ "key": public("a new phone") }] } }));
    let said = signed(json!({ "g": 12, "ts": 101, "arrived": 101, "by": ORI, "at": ORI, "is": "be", "refs": {}, "value": { "keys": [{ "key": public(ORI) }] } }));
    let t = with(&[way[0].clone(), way[1].clone(), over, said]);
    assert!(taken(&t, 11) && taken(&t, 12), "{} / {}", why(&t, 11), why(&t, 12));
    let at = |when: i64| -> Vec<String> { taldo_core::identity::keys_for(&t, ORI, when).into_iter().map(|(k, _)| k).collect() };
    assert!(!at(200).contains(&public("a new phone")), "a take-over's key signed before its day was out");
    assert!(at(100 + taldo_core::twin::TAKEOVER_WAITS).contains(&public("a new phone")), "a take-over ended without a no, by a list of keys");
}

#[test]
fn a_grant_is_taken_only_whole() {
    for v in [json!({ "to": BESIDE, "action": "be", "until": "never" }), json!({ "to": BESIDE, "action": "be", "budget": { "max": 3 } }), json!({ "action": "be" }), json!({ "to": BESIDE })] {
        assert!(!taken(&with(&[act(10, TALDO, ENGINE, "grant", json!({}), v.clone())]), 10), "a grant not whole was taken: {v}");
    }
}

// ─── THE SECOND REVIEW'S PROBES (26 Oct): who signed is recorded; a way back reaches only what it names; a no counts only from
// whose it is ───

#[test]
fn a_way_back_reaches_only_the_node_it_names() {
    let es = vec![
        first_key(ORI),
        act(1, ORI, ORI, "grant", json!({}), json!({ "action": "join", "as": "Bob" })),
        signed(json!({ "g": 2, "ts": 2, "arrived": 2, "by": "Bob", "at": "Bob", "is": "be", "refs": { "re": 1 }, "value": { "keys": [{ "key": public("Bob") }] } })),
        act(3, ORI, ORI, "grant", json!({}), json!({ "action": "take over", "to": "Bob" })),
        make(4, "Bob", "Bob › agent", "Bob", "agent"),
        signed_by("Bob › agent", json!({ "g": 5, "ts": 5, "by": ORI, "at": ORI, "is": "takeover", "refs": { "proof": "Bob › agent" }, "value": { "keys": [{ "key": public("thief") }] } })),
        signed_by("Bob › agent", json!({ "g": 6, "ts": 6, "by": ORI, "at": ORI, "is": "takeover", "refs": { "proof": "Bob" }, "value": { "keys": [{ "key": public("thief") }] } })),
    ];
    let t = fold(&es);
    assert!(!t.entries.contains_key(&5) && !t.entries.contains_key(&6), "a node under the way back took Ori over");
}

#[test]
fn a_move_stays_made_when_its_signer_changes_keys() {
    let mut es = vec![first_key(ORI), make(1, ORI, "T", ORI, "thing")];
    es.push(act(2, "T", "T", "ask", json!({ "about": "T" }), json!({ "label": "go", "moves": [{ "says": "ok", "act": { "is": "approve" }, "to": { "x": 1 } }] })));
    es.push(act(3, ORI, "T", "approve", json!({ "re": 2 }), json!({})));
    assert!(fold(&es).moved(2).is_some());
    es.push(act(4, ORI, ORI, "be", json!({}), json!({ "keys": [{ "key": public("Ori2") }] })));
    let t = fold(&es);
    assert!(t.moved(2).is_some() && !t.ask_open(2), "a move came undone when its signer's keys changed: {:?}", t.rejected);
}

#[test]
fn a_no_is_taken_only_from_whose_it_is() {
    let es = vec![
        first_key(ORI),
        make(1, ORI, "T", ORI, "thing"),
        make(2, ORI, "C", ORI, "thing"),
        act(3, "T", "T", "ask", json!({ "about": "T" }), json!({ "label": "go", "moves": [{ "says": "ok", "to": { "x": 1 } }] })),
        act(4, "C", "C", "decline", json!({ "re": 3 }), json!({})),
    ];
    let t = fold(&es);
    assert!(!t.entries.contains_key(&4) && t.ask_open(3), "a no from beside the holder was taken");
}

#[test]
fn code_never_decides_by_signing_as_a_node_of_its_own() {
    let h = "a".repeat(64);
    let es = vec![
        first_key(ORI),
        make(1, ORI, "S", ORI, "level"),
        act(2, ORI, "S", "seal", json!({}), json!({ "wasm": h, "cwasm": { "m": h }, "contract": "c", "reach": [] })),
        act(3, "S", "S › c", "make", json!({ "in": "S" }), json!({ "kind": "x" })),
        signed_by("S", json!({ "g": 4, "ts": 4, "by": "S › c", "at": "S › c › d", "is": "make", "refs": { "in": "S › c" }, "value": { "kind": "x", "keys": [{ "key": public("anyone") }] } })),
    ];
    let t = fold(&es);
    assert!(!t.nodes.contains_key("S › c › d"), "code gave a node its keys by signing as its own child");
    assert!(!fold(&[first_key(ORI), act(1, ORI, "K", "make", json!({ "in": ORI }), json!({}))]).nodes.contains_key("K"), "a make that says no kind was taken");
}

// ─── THE THIRD REVIEW'S PROBES (26 Oct): the signer gated as the node it signs for; a move frozen as taken; a purge erases what was
// said and keeps what decided, judging nothing again; the record's own kinds written by no one ───

#[test]
fn a_blocked_carrier_signs_as_no_one() {
    let es = vec![
        first_key(ORI),
        make(1, ORI, "D", ORI, "device"),
        act(2, ORI, ORI, "be", json!({}), json!({ "carried_by": ["D"] })),
        act(3, ORI, "D", "block", json!({}), json!({})),
        signed_by("D", json!({ "g": 5, "ts": 5, "arrived": 5, "by": ORI, "at": ORI, "is": "be", "refs": {}, "value": { "name": "set by a blocked device" } })),
    ];
    assert!(!fold(&es).entries.contains_key(&5), "a blocked device signed as the person it carries");
}

#[test]
fn a_move_stays_made_when_who_carried_its_holder_changes() {
    let mut es = vec![
        first_key(ORI),
        act(1, ORI, ORI, "grant", json!({}), json!({ "action": "join", "as": "Bob" })),
        signed(json!({ "g": 2, "ts": 2, "arrived": 2, "by": "Bob", "at": "Bob", "is": "be", "refs": { "re": 1 }, "value": { "keys": [{ "key": public("Bob") }] } })),
        act(3, ORI, ORI, "be", json!({}), json!({ "carried_by": ["Bob"] })),
        make(4, ORI, "T", ORI, "thing"),
        act(5, "T", "T", "ask", json!({ "about": "T" }), json!({ "label": "go", "moves": [{ "says": "ok", "act": { "is": "approve" }, "to": { "x": 1 } }] })),
        signed_by("Bob", json!({ "g": 6, "ts": 6, "arrived": 6, "by": ORI, "at": "T", "is": "approve", "refs": { "re": 5 }, "value": {} })),
    ];
    assert!(fold(&es).moved(5).is_some(), "{:?}", fold(&es).rejected);
    es.push(act(7, ORI, ORI, "be", json!({}), json!({ "carried_by": [] })));
    assert!(fold(&es).moved(5).is_some(), "a move came undone when who carried its holder changed");
}

#[test]
fn a_purge_erases_what_was_said_and_keeps_what_decided() {
    let mut es = vec![
        first_key(ORI),
        act(1, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 10 })),
        make(2, ORI, "P", ORI, "thing"),
        make(3, ORI, "Q", ORI, "thing"),
        act(4, ORI, "P", "grant", json!({}), json!({ "action": "note", "to": "Q" })),
        act(5, "Q", "Q", "note", json!({ "on": "P" }), json!({ "x": 1 })),
        act(6, ORI, "P", "be", json!({}), json!({ "name": "Pat", "keys": [{ "key": public("P") }] })),
        act(7, "P", "P", "say", json!({}), json!({ "secret": "what P said" })),
        act(8, ORI, "P", "delete", json!({}), json!({})),
    ];
    let t0 = fold(&es);
    assert!(t0.rejected.is_empty(), "{:?}", t0.rejected);
    es.push(act(100, ORI, "P", "purge", json!({ "re": 8 }), json!({ "lines": t0.purging(8) })));
    let t = fold(&es);
    assert!(t.entries.contains_key(&100), "{:?}", t.rejected);
    assert!(t.rejected.is_empty() && t.entries.contains_key(&5), "an act taken under a grant that stays was lost: {:?}", t.rejected);
    assert!(t.entries.contains_key(&4) && t.entry(4)["is"] == "grant", "the grant it stood on was erased");
    assert!(!t.entries.contains_key(&6) && !t.entries.contains_key(&7), "an erased line is still a line");
    assert!(t.name("P").is_none() && t.nodes["P"].who["keys"][0]["key"] == public("P"), "what P said stayed, or who P is went");
    // THE RECORD AFTER THE STORE FORGOT THE ERASED ROWS folds the same
    let forgotten: Vec<Value> = es.iter().filter(|e| !taldo_core::twin::Twin::forgets(&es[9]).contains(&e["g"].as_i64().unwrap())).cloned().collect();
    let again = fold(&forgotten);
    assert!(again.rejected.is_empty() && again.nodes["P"].who == t.nodes["P"].who, "the record without its erased rows folds otherwise: {:?}", again.rejected);
}

// ─── THE FOURTH REVIEW'S PROBES (26 Oct): one answer to who signs for a node, for entries and requests alike; what stays of an
// erased entry decides no one it did not decide; a twin after a purge is the twin its kept rows fold to; whoever deleted itself
// answers its own delete; where a program runs is set from above ───

const EMPTY: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

#[test]
fn a_blocked_carrier_asks_as_no_one() {
    let t = fold(&[
        first_key(ORI),
        make(1, ORI, "D", ORI, "device"),
        act(2, ORI, ORI, "be", json!({}), json!({ "carried_by": ["D"] })),
        act(3, ORI, "D", "block", json!({}), json!({})),
    ]);
    let r = signed_by("D", json!({ "g": 0, "ts": 10, "by": ORI, "at": ORI, "is": "request", "refs": {}, "value": { "method": "GET", "path": "/at", "body": EMPTY } }));
    assert!(taldo_core::identity::requested_by(&t, &r, "GET", "/at", b"", 10, 60_000, &|_| false).is_err(), "a blocked device asked as the person it carries");
    assert!(!taldo_core::identity::keys_for(&t, ORI, 10).iter().any(|(k, _)| *k == public("D")), "a blocked device's key still signs for whom it carries");
}

/// the twin after a purge, and the twin its kept rows fold to: the same — nodes, who each is, what each holds, nothing refused
fn same_as_its_rows(live: &Twin) {
    let rows: Vec<Value> = live.entries.iter().map(|(_, e)| (*e).clone()).collect();
    let again = fold(&rows);
    assert!(again.rejected.is_empty(), "its kept rows do not fold: {:?}", again.rejected);
    for (k, n) in &live.nodes {
        let m = &again.nodes[k];
        assert_eq!((&n.who, &n.value, &n.accepted), (&m.who, &m.value, &m.accepted), "{k} differs from what its rows fold to");
    }
    let open = |t: &Twin| t.entries.of("ask").filter(|(g, _)| t.ask_open(*g)).map(|(g, _)| g).collect::<Vec<_>>();
    assert_eq!(open(live), open(&again), "the asks open differ from what its rows fold to");
}

#[test]
fn what_stays_of_an_erased_entry_decides_no_one_it_did_not() {
    let mut es = vec![
        first_key(ORI),
        act(1, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 10 })),
        make(2, ORI, "P", ORI, "thing"),
        make(3, ORI, "Q", ORI, "thing"),
        act(4, ORI, "P", "grant", json!({}), json!({ "action": "note", "to": "Q" })),
        act(5, "Q", "P", "note", json!({}), json!({ "keys": [{ "key": public("X") }], "carried_by": ["Q"] })),
        act(6, "P", "P", "ask", json!({ "about": "P" }), json!({ "label": "go", "moves": [{ "says": "ok", "to": { "x": 1 } }] })),
        act(7, ORI, "P", "decline", json!({ "re": 6 }), json!({})),
        act(8, ORI, "P", "delete", json!({}), json!({})),
    ];
    let t0 = fold(&es);
    assert!(t0.rejected.is_empty(), "{:?}", t0.rejected);
    let lines = t0.purging(8);
    es.push(act(100, ORI, "P", "purge", json!({ "re": 8 }), json!({ "lines": lines })));
    let t = fold(&es);
    assert!(t.entries.contains_key(&100), "{:?}", t.rejected);
    assert!(t.nodes["P"].who.get("carried_by").is_none() && t.nodes["P"].who.get("keys").is_some_and(|k| !k.to_string().contains(&public("X"))), "a note, erased, decided who P is: {:?}", t.nodes["P"].who);
    same_as_its_rows(&t);
}

#[test]
fn whoever_deleted_itself_answers_its_own_delete() {
    let mut es = vec![
        first_key(ORI),
        act(1, ORI, ORI, "grant", json!({}), json!({ "action": "join", "as": "Bob" })),
        signed(json!({ "g": 2, "ts": 2, "arrived": 2, "by": "Bob", "at": "Bob", "is": "be", "refs": { "re": 1 }, "value": { "keys": [{ "key": public("Bob") }], "keep_deleted": 10 } })),
        act(3, "Bob", "Bob", "say", json!({}), json!({ "says": "hi" })),
        act(4, "Bob", "Bob", "delete", json!({}), json!({})),
    ];
    let mut undone = es.clone();
    undone.push(act(5, "Bob", "Bob", "restore", json!({ "re": 4 }), json!({})));
    let t = fold(&undone);
    assert!(t.entries.contains_key(&5) && !t.deleted("Bob"), "Bob could not undo his own delete: {:?}", t.rejected);
    let t0 = fold(&es);
    let lines = t0.purging(4);
    es.push(act(100, "Bob", "Bob", "purge", json!({ "re": 4 }), json!({ "lines": lines })));
    let t = fold(&es);
    assert!(t.entries.contains_key(&100), "Bob could not carry out his own delete: {:?}", t.rejected);
    same_as_its_rows(&t);
}

#[test]
fn a_purge_in_a_long_kept_record_lands_before_its_row_is_kept() {
    use taldo_core::seams::Store;
    let store = std::sync::Arc::new(taldo_core::fakes::FakeStore::default());
    store.lease("k").unwrap();
    let mut t = Twin::stored(store.clone());
    // AS THE KEEPER: landed on a copy, kept as one, then taken
    let keep = |t: &mut Twin, e: Value| {
        let mut e = e;
        arrives(&mut e);
        let s = t.admit(&e, e["arrived"].as_i64().unwrap()).unwrap_or_else(|w| panic!("{w}"));
        let mut next = t.clone();
        next.land(&e, &s, e["arrived"].as_i64().unwrap());
        let row = taldo_core::twin::kept(&e, e["arrived"].as_i64().unwrap());
        store.keep("k", std::slice::from_ref(&row), &Twin::forgets(&e), &next.snapshot()).unwrap();
        next.rows_kept();
        *t = next;
    };
    keep(&mut t, first_key(ORI));
    keep(&mut t, act(1, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 10 })));
    keep(&mut t, make(2, ORI, "P", ORI, "thing"));
    for g in 3..5_003 {
        keep(&mut t, act(g, ORI, "P", "say", json!({}), json!({ "says": format!("line {g}") })));
    }
    keep(&mut t, act(5_003, ORI, "P", "delete", json!({}), json!({})));
    let lines = t.purging(5_003);
    let mut purge = act(5_004, ORI, "P", "purge", json!({ "re": 5_003 }), json!({ "lines": lines }));
    purge["arrived"] = json!(6_000);
    keep(&mut t, purge);
    assert!(t.entries.contains_key(&5_004));
    assert!(taldo_core::rows::Source::entry(&*store, 4).is_none(), "the store still holds what was erased");
    assert!(!t.entries.contains_key(&4));
    let again = Twin::refold(store.clone());
    assert!(again.rejected.is_empty() && again.entries.contains_key(&5_004), "the kept rows do not fold again: {:?}", again.rejected.iter().take(3).collect::<Vec<_>>());
}

#[test]
fn where_a_program_runs_is_set_from_above() {
    let place = |host: &str| json!({ "place": { "host": host, "user": "u", "key": "k", "vm": "a", "machine": "x86_64-linux" } });
    let es = vec![first_key(ORI), make(1, ORI, "N", ORI, "taldo"), act(2, ORI, "N", "be", json!({}), place("h1")), act(3, "N", "N", "be", json!({}), place("elsewhere")), act(4, ORI, "N", "be", json!({}), json!({ "place": { "host": "h2" } })), act(5, ORI, "N", "be", json!({}), json!({ "host": "a detail, as any" }))];
    let t = fold(&es);
    assert!(t.entries.contains_key(&2) && !t.entries.contains_key(&3), "a program's node said where it runs itself: {:?}", t.rejected);
    assert!(!t.entries.contains_key(&4), "a place not whole was taken");
    assert_eq!(t.nodes["N"].who["place"]["host"], "h1");
    assert_eq!(t.nodes["N"].value["host"], "a detail, as any", "a detail named like part of a place is not an everyday detail any more");
}

// ─── THE FIFTH REVIEW'S PROBES (26 Oct): a deleted node answers no delete but a person's own of themselves ───

#[test]
fn nothing_deleted_answers_its_own_delete_but_a_person_of_themselves() {
    let keep = act(1, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 10 }));
    // a device that carries Ori, deleted by Ori: its key, as Ori, neither undoes nor carries out that delete
    let base = vec![first_key(ORI), keep.clone(), make(2, ORI, "D", ORI, "device"), act(3, ORI, ORI, "be", json!({}), json!({ "carried_by": ["D"] })), act(4, ORI, "D", "delete", json!({}), json!({}))];
    let mut r = base.clone();
    r.push(signed_by("D", json!({ "g": 5, "ts": 5, "arrived": 5, "by": ORI, "at": "D", "is": "restore", "refs": { "re": 4 }, "value": {} })));
    let t = fold(&r);
    assert!(!t.entries.contains_key(&5) && t.deleted("D"), "a deleted device brought itself back");
    let mut p = base.clone();
    p.push(signed_by("D", json!({ "g": 5, "ts": 50, "arrived": 50, "by": ORI, "at": "D", "is": "purge", "refs": { "re": 4 }, "value": { "lines": [] } })));
    assert!(!fold(&p).entries.contains_key(&5), "a deleted device erased its own trail");
    // an agent Ori deleted purges nothing of itself
    let mut a = vec![first_key(ORI), keep, make(2, ORI, "A", ORI, "agent"), act(3, "A", "A", "say", json!({}), json!({ "says": "x" })), act(4, ORI, "A", "delete", json!({}), json!({}))];
    let t0 = fold(&a);
    let lines = t0.purging(4);
    a.push(signed(json!({ "g": 5, "ts": 50, "arrived": 50, "by": "A", "at": "A", "is": "purge", "refs": { "re": 4 }, "value": { "lines": lines } })));
    assert!(!fold(&a).entries.contains_key(&5), "a deleted agent purged itself");
}

#[test]
fn an_entry_taken_and_not_kept_yet_reads_alike_by_number_and_walked() {
    use taldo_core::seams::Store;
    let store = std::sync::Arc::new(taldo_core::fakes::FakeStore::default());
    store.lease("k").unwrap();
    let mut t = Twin::stored(store.clone());
    for e in [first_key(ORI), act(1, ORI, ORI, "be", json!({}), json!({ "n": 1 }))] {
        let mut e = e;
        arrives(&mut e);
        let s = t.admit(&e, e["arrived"].as_i64().unwrap()).unwrap();
        t.land(&e, &s, e["arrived"].as_i64().unwrap());
    }
    assert!(t.entries.get(&1).is_some());
    assert_eq!(t.entries.iter().map(|(g, _)| g).collect::<Vec<_>>(), vec![0, 1], "walked, what is taken and not kept is missing");
    assert_eq!(t.entries.iter().rev().map(|(g, _)| g).collect::<Vec<_>>(), vec![1, 0]);
    assert_eq!(t.entries.having("n"), vec![1], "by a detail, what is taken and not kept is missing");
}

// ─── NO LINE IS EVER CHANGED (26 Oct): a line is taken out by a later line that points to it — a stop, or a purge ───

#[test]
fn an_answer_stays_taken_when_what_it_answered_is_erased() {
    use taldo_core::seams::Store;
    // Dana asks; Ori says no; a year later her node is deleted, and erased
    let store = std::sync::Arc::new(taldo_core::fakes::FakeStore::default());
    store.lease("k").unwrap();
    let mut t = Twin::stored(store.clone());
    let keep = |t: &mut Twin, mut e: Value| {
        arrives(&mut e);
        let s = t.admit(&e, e["arrived"].as_i64().unwrap()).unwrap_or_else(|w| panic!("{w}"));
        let mut next = t.clone();
        next.land(&e, &s, e["arrived"].as_i64().unwrap());
        let row = taldo_core::twin::kept(&e, e["arrived"].as_i64().unwrap());
        store.keep("k", std::slice::from_ref(&row), &Twin::forgets(&e), &next.snapshot()).unwrap();
        next.rows_kept();
        *t = next;
    };
    for e in [
        first_key(ORI),
        act(1, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 10 })),
        make(3, ORI, "Dana", ORI, "client"),
        act(5, "Dana", "Dana", "ask", json!({ "about": "Dana" }), json!({ "label": "go", "moves": [{ "says": "send it", "to": { "sent": true } }] })),
        act(6, ORI, ORI, "decline", json!({ "re": 5 }), json!({})),
        act(7, ORI, "Dana", "delete", json!({}), json!({})),
    ] {
        keep(&mut t, e);
    }
    let lines = t.purging(7);
    let mut purge = act(8, ORI, "Dana", "purge", json!({ "re": 7 }), json!({ "lines": lines }));
    purge["arrived"] = json!(100);
    keep(&mut t, purge);
    assert!(t.entries.contains_key(&8));
    // STARTED AGAIN FROM NOTHING BUT ITS ROWS, the erased ones forgotten: every line judged again, none refused
    let again = Twin::refold(store.clone());
    assert!(again.rejected.is_empty(), "a cold start refused lines taken before: {:?}", again.rejected);
    for k in t.nodes.keys() {
        assert_eq!((&t.nodes[k].who, &t.nodes[k].value), (&again.nodes[k].who, &again.nodes[k].value), "{k} differs after a cold start");
    }
}

#[test]
fn a_stop_takes_out_a_line_and_whatever_stood_on_it() {
    use taldo_core::seams::Store;
    // a stolen device that carries Ori gives Ori a thief's key, and the thief signs as Ori with it
    let mut es = vec![
        first_key(ORI),
        make(1, ORI, "D", ORI, "device"),
        act(2, ORI, ORI, "be", json!({}), json!({ "carried_by": ["D"] })),
        signed_by("D", json!({ "g": 3, "ts": 3, "arrived": 3, "by": ORI, "at": ORI, "is": "be", "refs": {}, "value": { "keys": [{ "key": public(ORI) }, { "key": public("thief") }] } })),
        signed_by("thief", json!({ "g": 4, "ts": 4, "arrived": 4, "by": ORI, "at": ORI, "is": "be", "refs": {}, "value": { "name": "the thief's" } })),
        act(5, ORI, "D", "block", json!({}), json!({})),
    ];
    let t0 = fold(&es);
    assert!(t0.rejected.is_empty() && signs(&t0, ORI).contains(&public("thief")), "{:?}", t0.rejected);
    // WHAT FALLS IS NAMED: the thief's own line stood on the key, and is named with it
    let mut alone = es.clone();
    alone.push(act(6, ORI, ORI, "stop", json!({}), json!({ "lines": t0.stopping(&[3]) })));
    assert!(t0.admit(&alone[6], 6).is_err_and(|w| w["falls"] == 4), "a stop left what stood on its line refused in the rows");
    es.push(act(6, ORI, ORI, "stop", json!({}), json!({ "lines": t0.stopping(&[3, 4]) })));
    let t = fold(&es);
    assert!(t.entries.contains_key(&6), "{}", why(&t, 6));
    assert!(!t.entries.contains_key(&3) && !t.entries.contains_key(&4), "a line stopped, or what stood on it, still counts");
    assert!(!signs(&t, ORI).contains(&public("thief")), "the thief's key still signs for Ori");
    assert_eq!(t.name(ORI), None);
    // THE SAME FROM THE ROWS, the stopped line's row kept: it is passed over, never read
    let store = std::sync::Arc::new(taldo_core::fakes::FakeStore::default());
    store.lease("k").unwrap();
    store.keep("k", &es, &[], b"").unwrap();
    let again = Twin::refold(store);
    assert!(again.entries.contains_key(&6) && !again.entries.contains_key(&3) && !signs(&again, ORI).contains(&public("thief")));
    // TAKEN OUT FOR GOOD: the stopped line, sent again under a new number, is not taken
    let mut replay = (*t0.entry(3)).clone();
    replay["g"] = json!(7);
    es.push(replay);
    assert!(!fold(&es).entries.contains_key(&7), "a stopped line came back under a new number");
}

#[test]
fn a_stop_is_set_from_above_and_never_takes_out_a_coming_to_be() {
    let es = vec![
        first_key(ORI),
        make(1, ORI, "P", ORI, "thing"),
        make(2, ORI, "Q", ORI, "thing"),
        act(3, ORI, "P", "be", json!({}), json!({ "name": "Pat" })),
        act(4, "P", "P", "be", json!({}), json!({ "said": "by P" })),
    ];
    let t0 = fold(&es);
    let stop = |g: i64, by: &str, at: &str, lines: &[i64]| act(g, by, at, "stop", json!({}), json!({ "lines": t0.stopping(lines) }));
    let with = |e: Value| {
        let mut l = es.clone();
        l.push(e);
        fold(&l)
    };
    assert!(!with(stop(10, "P", "P", &[4])).entries.contains_key(&10), "a node stopped its own line");
    assert!(!with(stop(10, "Q", "Q", &[4])).entries.contains_key(&10), "a node beside stopped a line");
    assert!(!with(stop(10, ORI, "P", &[1])).entries.contains_key(&10), "a node's coming to be was stopped");
    assert!(!with(stop(10, ORI, "Q", &[3])).entries.contains_key(&10), "a stop at one node took out a line at another");
    let t = with(stop(10, ORI, "P", &[3, 4]));
    assert!(t.entries.contains_key(&10) && t.name("P").is_none(), "{}", why(&t, 10));
    // WHAT A STOP SAYS STAYS IS THE LINE: one that says otherwise is not taken
    let mut lying = t0.stopping(&[4]);
    lying[0]["by"] = json!(ORI);
    assert!(!with(act(10, ORI, "P", "stop", json!({}), json!({ "lines": lying }))).entries.contains_key(&10), "a stop said a line was someone else's");
}

#[test]
fn a_stop_not_taken_stops_nothing() {
    // a stop written into the rows by hand, signed by no one: it is judged in its place, and what it names counts as ever
    let es = vec![first_key(ORI), make(1, ORI, "P", ORI, "thing"), act(2, ORI, "P", "be", json!({}), json!({ "name": "Pat" }))];
    let t0 = fold(&es);
    let mut forged = act(3, ORI, "P", "stop", json!({}), json!({ "lines": t0.stopping(&[2]) }));
    forged["sig"]["sig"] = json!("00".repeat(64));
    let mut l = es.clone();
    l.push(forged);
    let t = fold(&l);
    assert!(!t.entries.contains_key(&3) && t.entries.contains_key(&2) && t.name("P").as_deref() == Some("Pat"), "a stop not taken stopped a line");
}

#[test]
fn nothing_answers_a_line_once_it_is_taken_out() {
    let mut es = vec![
        first_key(ORI),
        act(1, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 10 })),
        make(2, ORI, "Dana", ORI, "client"),
        act(3, "Dana", "Dana", "ask", json!({ "about": "Dana" }), json!({ "label": "go", "moves": [{ "says": "ok", "to": { "x": 1 } }] })),
        act(4, ORI, "Dana", "delete", json!({}), json!({})),
    ];
    let t0 = fold(&es);
    es.push(act(100, ORI, "Dana", "purge", json!({ "re": 4 }), json!({ "lines": t0.purging(4) })));
    es.push(act(101, ORI, ORI, "decline", json!({ "re": 3 }), json!({})));
    let t = fold(&es);
    assert!(t.entries.contains_key(&100) && !t.entries.contains_key(&101), "an erased line was answered after it was erased");
}
