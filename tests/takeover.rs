//! GETTING BACK IN: a synced passkey is the main way — the same key on a new device, nothing
//! to do. For the rest, a person grants their own ways back: a take-over grant to a person they trust, a device of theirs, or a
//! proof of theirs (an email, a code) — never to an org, an agent, a machine or a sandbox, so whoever runs their Taldo can never
//! become them. A key a granted way brings in (`takeover`) signs for them only a day after it arrived, and they can stop it before
//! from any key of theirs.
mod common;
use common::{act, make};
use taldo_core::twin::{fold, Twin, TAKEOVER_WAITS};
use serde_json::{json, Value};

const T0: i64 = 1_000;

fn world() -> Vec<Value> {
    vec![
        common::first_key("noa"),
        // the others came in by ways in Noa gave
        common::invite(901, "noa", "sister"),
        common::joins(902, "sister", 901),
        common::invite(903, "noa", "stranger"),
        common::joins(904, "stranger", 903),
        make(1, "noa", "org", "noa", "org"),
        make(2, "org", "agent", "org", "agent"),
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
fn grant(g: i64, to: &str) -> Value {
    act(g, "noa", "noa", "grant", json!({}), json!({ "to": to, "action": "take over" }))
}
/// the key a new device brings, taken over for Noa by `proof`, signed by it
fn takeover(g: i64, ts: i64, proof: &str) -> Value {
    common::signed_by(proof, json!({ "g": g, "ts": ts, "arrived": ts, "by": "noa", "at": "noa", "is": "takeover", "refs": { "proof": proof }, "value": { "keys": [{ "key": common::public("noa's new phone") }] } }))
}
/// an act of Noa's signed with the new phone's key
fn as_new_phone(g: i64, ts: i64) -> Value {
    common::signed_by("noa's new phone", json!({ "g": g, "ts": ts, "arrived": ts, "by": "noa", "at": "noa", "is": "say", "refs": {}, "value": { "says": "I'm back" } }))
}

#[test]
fn a_take_over_goes_only_to_a_person_a_device_or_a_proof_never_to_who_runs_her_taldo() {
    let t = with(&[grant(10, "sister")]);
    assert!(taken(&t, 10), "{}", why(&t, 10));
    for (g, to) in [(11, "org"), (12, "agent")] {
        let t = with(&[grant(g, to)]);
        assert!(!taken(&t, g), "a take-over was granted to {to}");
        assert!(why(&t, g).contains("a way back is a person"), "{}", why(&t, g));
    }
}

#[test]
fn a_key_taken_over_signs_only_a_day_after_it_arrived() {
    let log = [grant(10, "sister"), takeover(20, T0, "sister")];
    let t = with(&log);
    assert!(taken(&t, 20), "the take-over by a granted way was refused: {}", why(&t, 20));
    let mut early = log.to_vec();
    early.push(as_new_phone(30, T0 + TAKEOVER_WAITS - 1));
    let t = with(&early);
    assert!(!taken(&t, 30), "the new key signed before its day was over");
    let mut later = log.to_vec();
    later.push(as_new_phone(30, T0 + TAKEOVER_WAITS));
    let t = with(&later);
    assert!(taken(&t, 30), "the new key did not sign after its day: {}", why(&t, 30));
}

#[test]
fn she_stops_a_take_over_from_any_key_of_hers() {
    let t = with(&[
        grant(10, "sister"),
        takeover(20, T0, "sister"),
        act(21, "noa", "noa", "decline", json!({ "re": 20 }), json!({})),
        as_new_phone(30, T0 + TAKEOVER_WAITS),
    ]);
    assert!(taken(&t, 21), "{}", why(&t, 21));
    assert!(!taken(&t, 30), "a take-over she stopped still signs for her");
}

#[test]
fn no_one_else_stops_her_take_over() {
    let t = with(&[
        grant(10, "sister"),
        takeover(20, T0, "sister"),
        act(21, "stranger", "stranger", "decline", json!({ "re": 20 }), json!({})),
        as_new_phone(30, T0 + TAKEOVER_WAITS),
    ]);
    assert!(taken(&t, 30), "someone else's decline undid her take-over: {}", why(&t, 30));
}

#[test]
fn only_a_granted_way_takes_over_and_only_keys() {
    let t = with(&[grant(10, "sister"), takeover(20, T0, "stranger")]);
    assert!(!taken(&t, 20), "someone she never granted took over");
    let t = with(&[takeover(20, T0, "sister")]);
    assert!(!taken(&t, 20), "a take-over with no grant was taken");
    let more = common::signed_by("sister", json!({ "g": 20, "ts": T0, "arrived": T0, "by": "noa", "at": "noa", "is": "takeover", "refs": { "proof": "sister" }, "value": { "keys": [{ "key": common::public("x") }], "name": "not Noa" } }));
    let t = with(&[grant(10, "sister"), more]);
    assert!(!taken(&t, 20), "a take-over carried more than keys");
}

#[test]
fn a_recovery_code_is_a_way_back_too() {
    // the code's node under her, holding the code's key — Taldo's own kind, never described as data
    let code = common::signed(json!({ "g": 10, "ts": 10, "arrived": 10, "by": "noa", "at": "code", "is": "make", "refs": { "in": "noa" }, "value": { "kind": "recovery code", "keys": [{ "key": common::public("code") }] } }));
    let t = with(&[code.clone(), grant(11, "code"), takeover(20, T0, "code")]);
    assert!(taken(&t, 11), "a take-over to her recovery code was refused: {}", why(&t, 11));
    assert!(taken(&t, 20), "her recovery code could not bring a key back: {}", why(&t, 20));
}
