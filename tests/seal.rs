//! SEALS — what code a node runs, and who it is, are set from above it: never by the node itself, by what is under it or beside it,
//! and no grant opens them. A seal is whole, and the newest one is what runs; nothing runs at a node deleted or blocked.
mod common;
use taldo_core::twin::{fold, Twin};
use serde_json::{json, Value};

fn p(s: &str) -> String {
    s.to_string()
}
fn act(g: i64, by: &str, at: &str, is: &str, refs: Value, value: Value) -> Value {
    common::signed(json!({ "g": g, "ts": g, "arrived": g, "by": p(by), "at": p(at), "is": is, "refs": refs, "value": value }))
}
const ENGINE: &str = "Ori › Taldo › engine";
/// Ori; Taldo's level, holding its engine (the code sealed) with a part under it, and a sibling beside it
fn world() -> Vec<Value> {
    vec![
        common::first_key("Ori"),
        common::under(1, "Ori › Taldo"),
        common::under(2, ENGINE),
        common::under(3, "Ori › Taldo › engine › part"),
        common::under(4, "Ori › Taldo › beside"),
    ]
}
fn h(c: char) -> String {
    c.to_string().repeat(64)
}
fn whole(c: char) -> Value {
    json!({ "wasm": h(c), "cwasm": { "x86_64": h('b'), "aarch64": h('c') }, "contract": "taldo:level@0.1.0", "reach": ["taldo:level/twin", "taldo:level/acts"] })
}
fn seal(g: i64, by: &str, value: Value) -> Value {
    act(g, by, ENGINE, "seal", json!({}), value)
}
fn with(more: &[Value]) -> Twin {
    let mut l = world();
    l.extend_from_slice(more);
    fold(&l)
}
fn why(t: &Twin, g: i64) -> String {
    t.rejected.iter().find(|r| r["g"] == g).map(|r| r["why"].as_str().unwrap_or("").to_string()).unwrap_or_else(|| "taken".into())
}

#[test]
fn a_level_above_seals_and_the_newest_seal_is_what_runs() {
    let t = with(&[seal(10, "Ori › Taldo", whole('a'))]);
    assert_eq!(t.sealed(ENGINE), Some(10));
    assert_eq!(t.entry(10)["value"], whole('a'));
    // from further up too; the newest replaces it — going back is sealing the one before again
    let t = with(&[seal(10, "Ori › Taldo", whole('a')), seal(11, "Ori", whole('d')), seal(12, "Ori › Taldo", whole('a'))]);
    assert_eq!(t.sealed(ENGINE), Some(12));
    assert_eq!(t.sealed("Ori › Taldo"), None, "nothing sealed there");
}

#[test]
fn a_level_never_seals_itself_nor_what_is_beside_or_above_it_and_no_grant_opens_it() {
    for by in [ENGINE, "Ori › Taldo › engine › part", "Ori › Taldo › beside"] {
        let t = with(&[seal(10, by, whole('a'))]);
        assert_eq!(why(&t, 10), format!("the code of {ENGINE} is set only from above it — never by itself, and never under a grant"), "sealed by {by}");
        assert_eq!(t.sealed(ENGINE), None);
    }
    // a grant to seal, from the very top, opens nothing
    let grant = act(10, "Ori", ENGINE, "grant", json!({}), json!({ "to": p("Ori › Taldo › beside"), "action": "seal" }));
    let t = with(&[grant, seal(11, "Ori › Taldo › beside", whole('a'))]);
    assert!(t.entries.contains_key(&10), "{:?}", t.rejected);
    assert_eq!(why(&t, 11), format!("the code of {ENGINE} is set only from above it — never by itself, and never under a grant"));
}

#[test]
fn a_seal_is_whole() {
    let mut cases: Vec<(Value, &str)> = vec![];
    let mut v = whole('a');
    v["wasm"] = json!("abc");
    cases.push((v, "a seal's wasm is its module's sha-256, in hex"));
    let mut v = whole('a');
    v["wasm"] = json!(h('A'));
    cases.push((v, "a seal's wasm is its module's sha-256, in hex"));
    let mut v = whole('a');
    v["cwasm"] = json!({});
    cases.push((v, "a seal's cwasm is, for each machine, its ready code's sha-256 in hex"));
    let mut v = whole('a');
    v["cwasm"]["x86_64"] = json!("nope");
    cases.push((v, "a seal's cwasm is, for each machine, its ready code's sha-256 in hex"));
    let mut v = whole('a');
    v["contract"] = json!(" ");
    cases.push((v, "a seal names the contract it was built to"));
    let mut v = whole('a');
    v.as_object_mut().unwrap().remove("reach");
    cases.push((v, "a seal's reach is the list of what it may call"));
    let mut v = whole('a');
    v["reach"] = json!(["taldo:level/twin", ""]);
    cases.push((v, "a seal's reach is the list of what it may call"));
    let mut v = whole('a');
    v["source"] = json!("fn main() {}");
    cases.push((v, "a seal says only its module, machine code, contract and reach — not source"));
    cases.push((json!("a"), "a seal says its module, its machine code, its contract and its reach"));
    for (v, said) in cases {
        let t = with(&[seal(10, "Ori › Taldo", v.clone())]);
        assert_eq!(why(&t, 10), said, "{v}");
        assert_eq!(t.sealed(ENGINE), None);
    }
}

#[test]
fn nothing_runs_at_a_node_deleted_or_blocked() {
    let keep = act(10, "Ori", "Ori", "be", json!({}), json!({ "keep_deleted": 1000 }));
    let s = seal(11, "Ori › Taldo", whole('a'));
    let t = with(&[keep.clone(), s.clone(), act(12, "Ori", "Ori › Taldo", "delete", json!({}), json!({}))]);
    assert!(t.entries.contains_key(&12), "{:?}", t.rejected);
    assert_eq!(t.sealed(ENGINE), None, "deleted above it");
    let t = with(&[s, act(12, "Ori", ENGINE, "block", json!({}), json!({}))]);
    assert!(t.entries.contains_key(&12), "{:?}", t.rejected);
    assert_eq!(t.sealed(ENGINE), None, "blocked");
}

// WHO A NODE IS, LIKE WHAT IT RUNS, IS SET FROM ABOVE: its keys, who carries it, what stays when it is deleted — by a node above it,
// never by itself (sealed or not), nor under a grant; a person, with nothing above them, says who they are; and a level sets who the
// nodes under it are, and what they run, as any node above them does
#[test]
fn who_a_node_is_is_set_from_above() {
    for (g, by, at, k) in [(12, ENGINE, ENGINE, "keys"), (13, ENGINE, ENGINE, "carried_by"), (14, ENGINE, ENGINE, "keep_deleted"), (15, "Ori › Taldo › beside", ENGINE, "keys")] {
        let t = with(&[act(g, by, at, "be", json!({}), json!({ k: [] }))]);
        assert_eq!(why(&t, g), format!("the {k} of {at} is set only from above it — never by itself, and never under a grant"), "{by} set the {k} of {at}");
    }
    // a grant to write its details is never a grant to say who it is
    let grant = act(10, "Ori", ENGINE, "grant", json!({}), json!({ "to": p("Ori › Taldo › beside"), "action": "be" }));
    let t = with(&[grant, act(11, "Ori › Taldo › beside", ENGINE, "be", json!({}), json!({ "says": "ready" })), act(12, "Ori › Taldo › beside", ENGINE, "be", json!({}), json!({ "keys": [] }))]);
    assert!(t.entries.contains_key(&11), "{}", why(&t, 11));
    assert!(!t.entries.contains_key(&12), "a grant to write details let its holder say who {ENGINE} is");
    // from above: the level above it, and a level for the nodes under it — keys and code alike; a person for themselves
    let t = with(&[act(10, ENGINE, "Ori › Taldo › engine › part", "be", json!({}), json!({ "keys": [] })), act(11, ENGINE, "Ori › Taldo › engine › part", "seal", json!({}), whole('b')), act(12, "Ori", "Ori", "be", json!({}), json!({ "keep_deleted": 1000 })), act(13, "Ori › Taldo", ENGINE, "be", json!({}), json!({ "keys": [] }))]);
    assert!(t.rejected.is_empty(), "{:?}", t.rejected);
}

// NOT TAKEN OVER: a node with something above it is never taken over, nor granted to be — who it is comes from above
#[test]
fn only_a_person_is_taken_over() {
    for (g, is, value) in [(10, "takeover", json!({ "keys": [] })), (11, "grant", json!({ "to": "Ori", "action": "take over" }))] {
        let t = with(&[act(g, ENGINE, ENGINE, is, json!({}), value)]);
        assert_eq!(why(&t, g), format!("{ENGINE} is not a person: who it is is set from above it, never taken over"), "{is}");
    }
}
