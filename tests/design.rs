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
    let r = act(13, ORI, ENGINE, "restore", json!({ "re": 11, "on": 12 }), json!({}));
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
    let log: Vec<Value> = world().into_iter().chain([taldo(4, ORI), invite(10, ORI, "Dana"), joins(11, "Dana", 10)]).collect();
    let store = store();
    let mut t = Twin::stored(store.clone());
    for e in &log {
        keep(&store, &mut t, e.clone()).unwrap_or_else(|w| panic!("{w}"));
    }
    let (a, b) = (fold(&log), Twin::refold(store).unwrap());
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
        act(6, ORI, "P", "be", json!({}), json!({ "name": "Pat" })),
        act(7, "P", "P", "say", json!({}), json!({ "secret": "what P said" })),
        act(8, ORI, "P", "delete", json!({}), json!({})),
    ];
    let t0 = fold(&es);
    assert!(t0.rejected.is_empty(), "{:?}", t0.rejected);
    assert_eq!(t0.erasing(8), vec![6, 7]);
    es.push(act(100, ORI, "P", "purge", json!({ "re": 8 }), json!({ "lines": [6, 7] })));
    let t = fold(&es);
    assert!(t.rejected.is_empty(), "{:?}", t.rejected);
    assert!(t.entries.contains_key(&5) && t.entry(4)["is"] == "grant", "what was taken under what stays was lost");
    for g in [6, 7] {
        assert!(t.entry(g)["value"] == json!({}) && t.entry(g)["erased_by"] == 100, "#{g} still says what it said");
    }
    assert!(t.name("P").is_none(), "what P said is still read off P");
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

#[test]
fn a_purge_names_exactly_what_its_delete_erases() {
    let es = vec![
        first_key(ORI),
        act(1, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 10 })),
        make(2, ORI, "P", ORI, "thing"),
        make(3, ORI, "Q", ORI, "thing"),
        act(4, ORI, "P", "say", json!({}), json!({ "x": 1 })),
        act(5, ORI, "Q", "be", json!({}), json!({ "name": "Quinn" })),
        act(6, ORI, "P", "delete", json!({}), json!({})),
    ];
    let with = |lines: Value| {
        let mut l = es.clone();
        l.push(act(100, ORI, "P", "purge", json!({ "re": 6 }), json!({ "lines": lines })));
        fold(&l)
    };
    assert!(!with(json!([4, 5])).entries.contains_key(&100), "a purge erased a line outside what was deleted");
    assert!(!with(json!([])).entries.contains_key(&100), "a purge left what it erases");
    let t = with(json!([4]));
    assert!(t.entries.contains_key(&100) && t.name("Q").as_deref() == Some("Quinn"), "{:?}", t.rejected);
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
    let lines = fold(&es).erasing(4);
    es.push(act(100, "Bob", "Bob", "purge", json!({ "re": 4 }), json!({ "lines": lines })));
    let t = fold(&es);
    assert!(t.entries.contains_key(&100), "Bob could not carry out his own delete: {:?}", t.rejected);
}

#[test]
fn a_purge_in_a_long_kept_record_is_read_again_the_same() {
    let store = store();
    let mut t = Twin::stored(store.clone());
    for e in [first_key(ORI), act(1, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 10 })), make(2, ORI, "P", ORI, "thing"), taldo(3, ORI)] {
        keep(&store, &mut t, e).unwrap_or_else(|w| panic!("{w}"));
    }
    for g in 4..2_004 {
        keep(&store, &mut t, act(g, ORI, "P", "say", json!({}), json!({ "says": format!("line {g}") }))).unwrap();
    }
    keep(&store, &mut t, act(2_004, ORI, "P", "delete", json!({}), json!({}))).unwrap();
    let lines = t.erasing(2_004);
    let mut purge = act(2_005, ORI, "P", "purge", json!({ "re": 2_004 }), json!({ "lines": lines }));
    purge["arrived"] = json!(6_000);
    keep(&store, &mut t, purge).unwrap_or_else(|w| panic!("{w}"));
    assert!(taldo_core::rows::Source::entry(&*store, 4).is_some_and(|r| r["value"] == json!({}) && r["erased_by"] == 2_005), "the store still holds what was erased");
    let again = Twin::refold(store.clone()).unwrap_or_else(|w| panic!("{w}"));
    assert!(again.entries.contains_key(&2_005) && again.entry(4)["value"] == json!({}));
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
    let lines = fold(&a).erasing(4);
    a.push(signed(json!({ "g": 5, "ts": 50, "arrived": 50, "by": "A", "at": "A", "is": "purge", "refs": { "re": 4 }, "value": { "lines": lines } })));
    assert!(!fold(&a).entries.contains_key(&5), "a deleted agent purged itself");
}

#[test]
fn an_entry_taken_and_not_kept_yet_reads_alike_by_number_and_walked() {
    let store = store();
    let mut t = Twin::stored(store.clone());
    for e in [first_key(ORI), act(1, ORI, ORI, "be", json!({}), json!({ "n": 1 }))] {
        let mut e = e;
        arrives(&mut e);
        { let made = t.entries.top(); t.take(&e, e["arrived"].as_i64().unwrap(), common::TALDO, made, &mark_sign) }.unwrap();
    }
    assert!(t.entries.get(&1).is_some());
    assert_eq!(t.entries.iter().map(|(g, _)| g).collect::<Vec<_>>(), vec![0, 1], "walked, what is taken and not kept is missing");
    assert_eq!(t.entries.iter().rev().map(|(g, _)| g).collect::<Vec<_>>(), vec![1, 0]);
    assert_eq!(t.entries.having("n"), vec![1], "by a detail, what is taken and not kept is missing");
}

// ─── EACH LINE IS JUDGED ONCE, WHEN IT ARRIVES (26 Oct): its keeper keeps it with Taldo's mark — what admission found, chained to
// the mark before — and every start reads the marks and lands the lines, judging none again. Taking out acts from then on; a purge
// erases words, and nothing taken under them is judged again ───

/// the twin as it is, and the twin its kept rows are read to: the same
fn same_after_restart(store: &std::sync::Arc<taldo_core::fakes::FakeStore>, live: &Twin) {
    let again = Twin::refold(store.clone()).unwrap_or_else(|w| panic!("the kept rows do not read again: {w}"));
    assert_eq!(live.entries.keys().collect::<Vec<_>>(), again.entries.keys().collect::<Vec<_>>(), "a restart counts other lines");
    for (k, n) in &live.nodes {
        assert_eq!((&n.who, &n.value, &n.accepted), (&again.nodes[k].who, &again.nodes[k].value, &again.nodes[k].accepted), "{k} differs after a restart");
    }
    let open = |t: &Twin| t.entries.of("ask").filter(|(g, _)| t.ask_open(*g)).map(|(g, _)| g).collect::<Vec<_>>();
    assert_eq!(open(live), open(&again), "the asks open differ after a restart");
}

#[test]
fn an_answer_stays_taken_when_what_it_answered_is_erased() {
    // Dana asks; Ori says no; a year later her node is deleted, and erased — Ori's no stands, judged when it was written
    let store = store();
    let mut t = Twin::stored(store.clone());
    for e in [
        first_key(ORI),
        act(1, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 10 })),
        taldo(2, ORI),
        make(3, ORI, "Dana", ORI, "client"),
        act(4, ORI, "Dana", "grant", json!({}), json!({ "action": "ask", "to": "Dana" })),
        act(5, "Dana", "Dana", "ask", json!({ "about": "Dana" }), json!({ "label": "go", "moves": [{ "says": "send it", "to": { "sent": true } }] })),
        act(6, ORI, ORI, "decline", json!({ "re": 5 }), json!({})),
        act(7, ORI, "Dana", "delete", json!({}), json!({})),
    ] {
        keep(&store, &mut t, e).unwrap_or_else(|w| panic!("{w}"));
    }
    let lines = t.erasing(7);
    assert!(lines.contains(&5));
    let mut purge = act(8, ORI, "Dana", "purge", json!({ "re": 7 }), json!({ "lines": lines }));
    purge["arrived"] = json!(100);
    keep(&store, &mut t, purge).unwrap_or_else(|w| panic!("{w}"));
    assert!(t.entries.contains_key(&6), "Ori's no fell with what it answered");
    same_after_restart(&store, &t);
}

#[test]
fn a_take_out_ends_a_lines_effect_from_then_on_and_takes_out_nothing_unnamed() {
    // a stolen device that carries Ori gives Ori a thief's key, and the thief signs as Ori with it
    let store = store();
    let mut t = Twin::stored(store.clone());
    for e in [
        first_key(ORI),
        taldo(1, ORI),
        make(2, ORI, "D", ORI, "device"),
        act(3, ORI, ORI, "be", json!({}), json!({ "carried_by": ["D"] })),
        signed_by("D", json!({ "g": 4, "ts": 4, "arrived": 4, "by": ORI, "at": ORI, "is": "be", "refs": {}, "value": { "keys": [{ "key": public(ORI) }, { "key": public("thief") }] } })),
        signed_by("thief", json!({ "g": 5, "ts": 5, "arrived": 5, "by": ORI, "at": ORI, "is": "be", "refs": {}, "value": { "name": "the thief's" } })),
        signed_by("thief", json!({ "g": 6, "ts": 6, "arrived": 6, "by": ORI, "at": "D", "is": "be", "refs": {}, "value": { "name": "mine now" } })),
        act(7, ORI, "D", "block", json!({}), json!({})),
    ] {
        keep(&store, &mut t, e).unwrap_or_else(|w| panic!("{w}"));
    }
    assert!(signs(&t, ORI).contains(&public("thief")));
    // ORI TAKES OUT THE KEY AND THE NAME — never the line that named D, which stays as he chose
    keep(&store, &mut t, act(8, ORI, ORI, "take out", json!({}), json!({ "lines": [4, 5] }))).unwrap_or_else(|w| panic!("{w}"));
    assert!(!signs(&t, ORI).contains(&public("thief")), "the thief's key still signs for Ori");
    assert_eq!(t.name(ORI), None);
    assert_eq!(t.name("D").as_deref(), Some("mine now"), "a line no one named was taken out");
    assert!(t.entries.contains_key(&6), "a line no one named fell");
    // THE THIEF'S KEY SIGNS NOTHING NEW
    let late = signed_by("thief", json!({ "g": 9, "ts": 9, "arrived": 9, "by": ORI, "at": ORI, "is": "be", "refs": {}, "value": { "name": "again" } }));
    assert!(keep(&store, &mut t, late).is_err(), "the thief's key signed after it was taken out");
    // A TAKE-OUT TAKEN OUT brings back what it took out
    keep(&store, &mut t, act(10, ORI, ORI, "take out", json!({}), json!({ "lines": [8] }))).unwrap_or_else(|w| panic!("{w}"));
    assert_eq!(t.name(ORI).as_deref(), Some("the thief's"));
    same_after_restart(&store, &t);
    // TAKEN OUT FOR GOOD: a line taken out, sent again under a new number, is not taken
    let mut replay = (*t.entry(5)).clone();
    replay["g"] = json!(11);
    replay.as_object_mut().unwrap().remove("mark");
    assert!(t.admit(&replay, 11).is_err(), "a line came back under a new number");
}

#[test]
fn a_take_out_is_set_from_above_and_never_takes_out_a_coming_to_be() {
    let es = vec![
        first_key(ORI),
        make(1, ORI, "P", ORI, "thing"),
        make(2, ORI, "Q", ORI, "thing"),
        act(3, ORI, "P", "be", json!({}), json!({ "name": "Pat" })),
        act(4, "P", "P", "be", json!({}), json!({ "said": "by P" })),
        act(5, ORI, "P", "delete", json!({}), json!({})),
    ];
    let out = |g: i64, by: &str, at: &str, lines: Value| act(g, by, at, "take out", json!({}), json!({ "lines": lines }));
    let with = |e: Value| {
        let mut l = es[..5].to_vec();
        l.push(e);
        fold(&l)
    };
    assert!(!with(out(10, "P", "P", json!([4]))).entries.contains_key(&10), "a node took out its own line");
    assert!(!with(out(10, "Q", "Q", json!([4]))).entries.contains_key(&10), "a node beside took out a line");
    assert!(!with(out(10, ORI, "P", json!([1]))).entries.contains_key(&10), "a node's coming to be was taken out");
    assert!(!with(out(10, ORI, "Q", json!([3]))).entries.contains_key(&10), "a take-out at one node took out a line at another");
    assert!(!with(out(10, ORI, "P", json!([4, 4]))).entries.contains_key(&10), "a line was named twice");
    let mut deleted = es.clone();
    deleted.push(out(10, ORI, "P", json!([5])));
    assert!(!fold(&deleted).entries.contains_key(&10), "a delete was taken out, not restored");
    let t = with(out(10, ORI, "P", json!([3, 4])));
    assert!(t.entries.contains_key(&10) && t.name("P").is_none(), "{}", why(&t, 10));
}

#[test]
fn a_row_changed_put_in_or_taken_away_is_never_read() {
    let store = store();
    let mut t = Twin::stored(store.clone());
    for e in [first_key(ORI), taldo(1, ORI), make(2, ORI, "P", ORI, "thing"), act(3, ORI, "P", "be", json!({}), json!({ "name": "Pat" })), act(4, ORI, "P", "be", json!({}), json!({ "x": 1 }))] {
        keep(&store, &mut t, e).unwrap_or_else(|w| panic!("{w}"));
    }
    Twin::refold(store.clone()).unwrap();
    let rows: Vec<Value> = taldo_core::rows::Source::page(&*store, 0, i64::MAX, false, 100).into_iter().map(|(_, e)| e).collect();
    let read = |rows: Vec<Value>| {
        let s = store_of(&rows);
        Twin::refold(s)
    };
    // a word changed
    let mut changed = rows.clone();
    changed[3]["value"]["name"] = json!("Mallory");
    assert!(read(changed).is_err(), "a changed row was read");
    // a row taken away
    let mut gone = rows.clone();
    gone.remove(3);
    assert!(read(gone).is_err(), "a row taken away was not seen");
    // a row put in, marked by no one, or by a key that is not Taldo's
    let mut put = rows.clone();
    let mut junk = act(5, ORI, "P", "be", json!({}), json!({ "name": "junk" }));
    junk["mark"] = json!({ "g": 5, "prev": "", "by": TALDO });
    put.push(junk);
    assert!(read(put).is_err(), "a row marked by no one was read");
    // a row erased by no purge
    let mut erased = rows.clone();
    erased[3] = taldo_core::twin::erased_row(&rows[3], 9);
    assert!(read(erased).is_err(), "a row erased by no purge was read");
}

/// a store holding exactly these rows
fn store_of(rows: &[Value]) -> std::sync::Arc<taldo_core::fakes::FakeStore> {
    let s = store();
    for r in rows {
        taldo_core::seams::Store::keep(&*s, "k", std::slice::from_ref(r), &[], b"").unwrap();
    }
    s
}

#[test]
fn a_programs_own_kind_is_never_read_as_the_cores() {
    assert!(taldo_core::twin::core_kind("take out") && taldo_core::twin::core_kind("purge"));
    assert!(!taldo_core::twin::core_kind("stop"), "the app's stop of a session is a core kind");
    let t = with(&[act(10, ORI, ENGINE, "stop", json!({}), json!({}))]);
    assert!(taken(&t, 10), "a session's stop was read as the core's: {}", why(&t, 10));
}

#[test]
fn take_outs_and_purges_interleaved_read_again_the_same() {
    let store = store();
    let mut t = Twin::stored(store.clone());
    for e in [
        first_key(ORI),
        act(1, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 10 })),
        taldo(2, ORI),
        make(3, ORI, "P", ORI, "thing"),
        make(4, ORI, "Q", ORI, "thing"),
        act(5, ORI, "P", "be", json!({}), json!({ "name": "bad" })),
        act(6, ORI, "Q", "be", json!({}), json!({ "name": "also bad" })),
        act(7, ORI, "Q", "say", json!({}), json!({ "secret": "medical record" })),
        act(8, ORI, "P", "take out", json!({}), json!({ "lines": [5] })),
        act(9, ORI, "Q", "take out", json!({}), json!({ "lines": [6] })),
        act(10, ORI, "Q", "delete", json!({}), json!({})),
    ] {
        keep(&store, &mut t, e).unwrap_or_else(|w| panic!("{w}"));
    }
    assert!(t.name("P").is_none() && t.name("Q").is_none());
    // A PURGE ERASES WHAT WAS TAKEN OUT AT Q TOO: its words leave the store
    assert_eq!(t.erasing(10), vec![6, 7]);
    let mut purge = act(11, ORI, "Q", "purge", json!({ "re": 10 }), json!({ "lines": [6, 7] }));
    purge["arrived"] = json!(100);
    keep(&store, &mut t, purge).unwrap_or_else(|w| panic!("{w}"));
    for g in [6, 7] {
        assert!(taldo_core::rows::Source::entry(&*store, g).is_some_and(|r| r["value"] == json!({})), "row {g} still holds what was said");
    }
    same_after_restart(&store, &t);
    // A STATE KEPT, AND ROWS KEPT AFTER IT: read on it as any row
    let state = taldo_core::seams::Store::state(&*store).unwrap().value.unwrap();
    keep(&store, &mut t, act(12, ORI, "P", "be", json!({}), json!({ "name": "good" }))).unwrap();
    let restored = Twin::restore(&state, store.clone()).unwrap_or_else(|w| panic!("{w}"));
    assert_eq!(restored.name("P").as_deref(), Some("good"));
}

#[test]
fn a_mark_is_checked_against_the_key_its_taldo_had_when_it_was_made() {
    // Taldo's key changes midway — in two steps: the new key given beside the old, then the old let go, marked by the new — and a
    // record kept before marks is marked once, at its end, with Taldo's key then
    let lines = vec![
        first_key(ORI),
        taldo(1, ORI),
        make(2, ORI, "P", ORI, "thing"),
        act(3, ORI, common::TALDO, "be", json!({}), json!({ "keys": [{ "key": public(common::TALDO) }, { "key": public("Taldo 2") }] })),
        act(4, ORI, common::TALDO, "be", json!({}), json!({ "keys": [{ "key": public("Taldo 2") }] })),
        act(5, ORI, "P", "be", json!({}), json!({ "name": "Pat" })),
    ];
    let sign2 = |b: &[u8]| -> Result<Value, String> {
        use p256::ecdsa::signature::Signer;
        let s: p256::ecdsa::Signature = key_of("Taldo 2").sign(b);
        Ok(json!({ "key": public("Taldo 2"), "sig": s.to_bytes().iter().map(|x| format!("{x:02x}")).collect::<String>() }))
    };
    // LINE BY LINE: each mark made with the key Taldo marks with as it arrives — the new one from the line after it was given
    let kept = store();
    let mut t = Twin::stored(kept.clone());
    for e in lines.iter().cloned() {
        let mut e = e;
        arrives(&mut e);
        let a = e["arrived"].as_i64().unwrap();
        let made = t.entries.top();
        let row = if made >= 3 { t.take(&e, a, common::TALDO, made, &sign2) } else { t.take(&e, a, common::TALDO, made, &mark_sign) }.unwrap_or_else(|w| panic!("{w}")).row;
        taldo_core::seams::Store::keep(&*kept, "k", &[row], &[], b"").unwrap();
    }
    Twin::refold(kept.clone()).unwrap_or_else(|w| panic!("a record whose Taldo's key changed does not read: {w}"));
    // MARKED ONCE, AT ITS END: every mark made with the key Taldo has then
    let once = store_of(&[]);
    let mut t = Twin::default();
    for e in lines.iter().cloned() {
        let mut e = e;
        arrives(&mut e);
        let row = t.take(&e, e["arrived"].as_i64().unwrap(), common::TALDO, 5, &sign2).unwrap_or_else(|w| panic!("{w}")).row;
        taldo_core::seams::Store::keep(&*once, "k", &[row], &[], b"").unwrap();
    }
    Twin::refold(once).unwrap_or_else(|w| panic!("a record marked once at its end does not read: {w}"));
    // THE OLD KEY, ONCE LET GO, MARKS NOTHING: its keeper is refused the line that lets it go, and a row it marked after is never read
    let mut t = Twin::default();
    let mut refused = None;
    for e in lines.iter().cloned() {
        let mut e = e;
        arrives(&mut e);
        let made = t.entries.top();
        if let Err(w) = t.take(&e, e["arrived"].as_i64().unwrap(), common::TALDO, made, &mark_sign) {
            refused = Some(w);
            break;
        }
    }
    assert!(refused.is_some_and(|w| w["g"] == 4), "the old key marked the line that let it go");
    // A NEW KEY NEVER MARKS ITS OWN GIVING: the line is marked by a key that marks the record as it comes, or not kept
    let mut t = Twin::default();
    for e in lines[..3].iter().cloned() {
        let mut e = e;
        arrives(&mut e);
        let made = t.entries.top();
        t.take(&e, e["arrived"].as_i64().unwrap(), common::TALDO, made, &mark_sign).unwrap();
    }
    let mut giving = lines[3].clone();
    arrives(&mut giving);
    assert!(t.take(&giving, 3, common::TALDO, 2, &sign2).is_err(), "a new key marked its own giving");
}

// ─── THE NINTH REVIEW'S PROBES (27 Oct): one unsigned beginning; the record anchored outside its store, every row read again held to
// what was read; what stays of an erased line bound by its purge's mark; one Taldo marks the record; a line taken out by many ───

fn kept_record(lines: Vec<Value>) -> (std::sync::Arc<taldo_core::fakes::FakeStore>, Twin) {
    let store = store();
    let mut t = Twin::stored(store.clone());
    for e in lines {
        keep(&store, &mut t, e).unwrap_or_else(|w| panic!("{w}"));
    }
    (store, t)
}
fn state_of(store: &taldo_core::fakes::FakeStore) -> Vec<u8> {
    taldo_core::seams::Store::state(store).unwrap().value.unwrap_or_default()
}

#[test]
fn a_record_begun_at_any_number_reads_again() {
    let mut first = first_key(ORI);
    first["g"] = json!(1);
    let (store, _) = kept_record(vec![first, taldo(2, ORI), act(3, ORI, ORI, "be", json!({}), json!({ "name": "Ori" }))]);
    Twin::refold(store).unwrap_or_else(|w| panic!("a record begun at 1 does not read: {w}"));
}

#[test]
fn the_record_is_read_only_where_taldo_last_left_it() {
    let (store, t) = kept_record(vec![first_key(ORI), taldo(1, ORI), make(2, ORI, "D", ORI, "device"), act(3, ORI, ORI, "grant", json!({}), json!({ "action": "read", "to": "D" })), act(4, ORI, "D", "block", json!({}), json!({}))]);
    let anchor = anchor_of(&t, &state_of(&store));
    let again = Twin::refold(store.clone()).unwrap();
    again.anchored(&state_of(&store), &anchor).unwrap_or_else(|w| panic!("{w}"));
    // CUT SHORT: the block taken away
    let rows: Vec<Value> = taldo_core::rows::Source::page(&*store, 0, i64::MAX, false, 100).into_iter().map(|(_, e)| e).collect();
    let short = store_of(&rows[..4]);
    let cut = Twin::refold(short.clone()).unwrap();
    assert!(cut.anchored(&state_of(&store), &anchor).is_err(), "a record cut short was read as where Taldo left it");
    // THE STATE CHANGED
    let mut changed = state_of(&store);
    let last = changed.len() - 1;
    changed[last] ^= 1;
    assert!(again.anchored(&changed, &anchor).is_err(), "a changed state was read as Taldo's");
    // AN ANCHOR SIGNED BY ANOTHER KEY
    let mut forged = t.anchor(&state_of(&store));
    forged["sig"] = {
        use p256::ecdsa::signature::Signer;
        let s: p256::ecdsa::Signature = key_of("thief").sign(taldo_core::rows::canonical_of(&forged).as_bytes());
        json!({ "key": public("thief"), "sig": s.to_bytes().iter().map(|x| format!("{x:02x}")).collect::<String>() })
    };
    assert!(again.anchored(&state_of(&store), &forged).is_err(), "an anchor by a key that does not mark the record was taken");
}

#[test]
fn a_row_changed_after_it_was_read_is_never_read() {
    let (store, _) = kept_record(vec![first_key(ORI), taldo(1, ORI), make(2, ORI, "P", ORI, "thing"), act(3, ORI, "P", "be", json!({}), json!({ "name": "Pat" }))]);
    let t = Twin::refold(store.clone()).unwrap();
    let mut row = taldo_core::rows::Source::entry(&*store, 3).unwrap();
    row["value"]["name"] = json!("Mallory");
    store.put_row(row);
    let read = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| t.entry(3)["value"]["name"].clone()));
    assert!(read.is_err(), "a row changed in the store after it was read was read");
}

#[test]
fn what_stays_of_an_erased_line_is_bound_by_its_purge() {
    let (store, mut t) = kept_record(vec![first_key(ORI), act(1, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 10 })), taldo(2, ORI), make(3, ORI, "P", ORI, "thing"), act(4, ORI, "P", "say", json!({}), json!({ "says": "a secret" })), act(5, ORI, "P", "delete", json!({}), json!({}))]);
    let mut purge = act(6, ORI, "P", "purge", json!({ "re": 5 }), json!({ "lines": [4] }));
    purge["arrived"] = json!(100);
    keep(&store, &mut t, purge).unwrap();
    let stays = taldo_core::rows::Source::entry(&*store, 4).unwrap();
    assert!(stays.get("salt").is_none() && stays["sig"].is_null() && stays["sig_hash"].is_string() && stays["value"] == json!({}), "what stays of #4 holds more than it may: {stays}");
    Twin::refold(store.clone()).unwrap();
    // turned into a block of Taldo
    let rows: Vec<Value> = taldo_core::rows::Source::page(&*store, 0, i64::MAX, false, 100).into_iter().map(|(_, e)| e).collect();
    let mut made = rows.clone();
    made[4]["is"] = json!("block");
    made[4]["at"] = json!(TALDO);
    assert!(Twin::refold(store_of(&made)).is_err(), "what stays of an erased line was made into another act");
}

#[test]
fn only_the_records_taldo_marks_it_and_never_while_blocked() {
    // another person's node of kind taldo marks a line in Ori's record
    let (store, _) = kept_record(vec![first_key(ORI), taldo(1, ORI), act(2, ORI, ORI, "grant", json!({}), json!({ "action": "join", "as": "Eve" }))]);
    let mut rows: Vec<Value> = taldo_core::rows::Source::page(&*store, 0, i64::MAX, false, 100).into_iter().map(|(_, e)| e).collect();
    let mut m = rows[2]["mark"].clone();
    m["by"] = json!("Eve › T");
    rows[2]["mark"] = m;
    assert!(Twin::refold(store_of(&rows)).is_err(), "a mark by a Taldo not the record's was read");
    // TALDO IS NEVER BLOCKED OR DELETED OUT FROM UNDER ITS RECORD: such a line is not kept
    let (store, mut t) = kept_record(vec![first_key(ORI), act(1, ORI, ORI, "be", json!({}), json!({ "keep_deleted": 10 })), taldo(2, ORI)]);
    assert!(keep(&store, &mut t, act(3, ORI, TALDO_NODE, "block", json!({}), json!({}))).is_err(), "Taldo was blocked out from under its record");
    assert!(keep(&store, &mut t, act(3, ORI, TALDO_NODE, "delete", json!({}), json!({}))).is_err(), "Taldo was deleted out from under its record");
    assert!(keep(&store, &mut t, act(3, ORI, TALDO_NODE, "be", json!({}), json!({ "keys": [{ "key": public("a key Taldo does not hold") }] }))).is_err(), "Taldo was given a key it does not hold");
    keep(&store, &mut t, act(3, ORI, ORI, "be", json!({}), json!({ "name": "still read" }))).unwrap();
    Twin::refold(store).unwrap_or_else(|w| panic!("{w}"));
}
const TALDO_NODE: &str = common::TALDO;

#[test]
fn a_line_taken_out_by_many_is_out_while_any_counts() {
    let lines = vec![
        first_key(ORI),
        taldo(1, ORI),
        act(2, ORI, ORI, "be", json!({}), json!({ "name": "x" })),
        act(3, ORI, ORI, "take out", json!({}), json!({ "lines": [2] })),
        act(4, ORI, ORI, "take out", json!({}), json!({ "lines": [3] })),
        act(5, ORI, ORI, "take out", json!({}), json!({ "lines": [2] })),
        act(6, ORI, ORI, "take out", json!({}), json!({ "lines": [4] })),
    ];
    let t = fold(&lines);
    assert!(t.rejected.is_empty(), "{:?}", t.rejected);
    assert_eq!(t.name(ORI), None, "a line taken out by two take-outs in force came back");
    let (store, t) = kept_record(lines);
    assert_eq!(t.name(ORI), None);
    same_after_restart(&store, &t);
}

#[test]
fn a_row_taken_away_after_it_was_read_is_never_read_as_not_there() {
    let (store, _) = kept_record(vec![first_key(ORI), taldo(1, ORI), make(2, ORI, "D", ORI, "device"), act(3, ORI, "D", "block", json!({}), json!({})), act(4, ORI, ORI, "be", json!({}), json!({ "name": "Ori" }))]);
    let t = Twin::refold(store.clone()).unwrap();
    store.drop_row(3);
    let read = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| t.blocked("D")));
    assert!(read.is_err(), "a block taken away from the store was read as never written");
    let walked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| t.entries.iter().count()));
    assert!(walked.is_err(), "the entries walked passed over a row taken away");
}
