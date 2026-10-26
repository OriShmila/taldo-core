//! BLOCKING: one act, and what was blocked signs nothing from then on.
mod common;
use common::{act, make};
use taldo_core::twin::{fold, Twin};
use serde_json::{json, Value};

fn world() -> Vec<Value> {
    vec![
        common::first_key("ori"),
        common::signed(json!({ "g": 1, "ts": 1, "arrived": 1, "by": "ori", "at": "ori", "is": "be", "refs": {}, "value": { "keys": [{ "key": common::public("ori") }, { "key": common::finger_public() }] } })),
        make(2, "ori", "org", "ori", "org"),
        // an open kind, as data: OVH's three fields, one of them a secret
        make(3, "org", "ovh", "org", "identity"),
        act(4, "org", "ovh", "be", json!({}), json!({ "way": "fields", "fields": [{ "name": "app_key" }, { "name": "app_secret", "secret": true }, { "name": "consumer_key", "secret": true }] })),
        make(5, "org", "dana", "org", "outside"),
        act(6, "org", "dana", "be", json!({}), json!({ "channel": "whatsapp:+972500000000" })),
        make(7, "org", "taldo", "org", "taldo"),
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

#[test]
fn what_is_blocked_signs_nothing_from_then_on() {
    let base = [
        common::signed(json!({ "g": 20, "ts": 20, "arrived": 20, "by": "org", "at": "agent", "is": "make", "refs": { "in": "org" }, "value": { "kind": "agent", "keys": [{ "key": common::public("agent") }] } })),
        act(21, "agent", "agent", "say", json!({}), json!({ "says": "working" })),
        act(22, "ori", "agent", "block", json!({}), json!({})),
        act(23, "agent", "agent", "say", json!({}), json!({ "says": "still here" })),
    ];
    let t = with(&base);
    assert!(taken(&t, 21) && taken(&t, 22), "{:?}", t.rejected);
    assert!(t.blocked("agent"));
    assert!(!taken(&t, 23), "a blocked agent's act was taken");
    assert!(why(&t, 23).contains("blocked"), "{}", why(&t, 23));
}
