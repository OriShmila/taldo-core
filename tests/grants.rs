//! GRANTS — an act on what is not yours (reading it, calling it) is taken only under a grant in force on that thing.
mod common;
use taldo_core::twin::{fold, Twin};
use serde_json::{json, Value};

fn p(s: &str) -> String {
    s.to_string()
}
fn act(g: i64, by: &str, at: &str, is: &str, refs: Value, value: Value) -> Value {
    common::signed(json!({ "g": g, "ts": g, "by": p(by), "at": p(at), "is": is, "refs": refs, "value": value }))
}
/// Ori, two agents with their keys, a contract in Ori's files, and Anthropic as a service Ori holds
fn world() -> Vec<Value> {
    vec![
        common::first_key("Ori"),
        common::under(1, "Ori › Claude"),
        common::under(2, "Ori › other"),
        common::under(3, "Ori › files"),
        common::under(4, "Ori › files › contract"),
        common::under(5, "Ori › Anthropic"),
    ]
}
fn with(log: &[Value], more: &[Value]) -> Twin {
    let mut l = log.to_vec();
    l.extend_from_slice(more);
    fold(&l)
}
fn read(g: i64, by: &str, on: &str) -> Value {
    act(g, by, by, "read", json!({ "on": p(on) }), json!({}))
}
fn call(g: i64, usd: f64) -> Value {
    act(g, "Ori › Claude", "Ori › Claude", "call", json!({ "on": p("Ori › Anthropic") }), json!({ "usd": usd }))
}
fn taken(t: &Twin, g: i64) -> bool {
    t.entries.contains_key(&g)
}

#[test]
fn reading_what_is_not_yours_needs_a_grant_naming_you_and_the_act() {
    let w = world();
    // no grant: refused, with why
    let t = with(&w, &[read(10, "Ori › Claude", "Ori › files › contract")]);
    assert!(!taken(&t, 10), "a read with no grant was taken");
    assert!(t.rejected.iter().any(|r| r["why"].as_str().unwrap_or("").contains("no grant lets")), "{:?}", t.rejected);
    // a grant on the folder that reaches below covers what is in it — for the one it names, and the act it names
    let grant = act(10, "Ori", "Ori › files", "grant", json!({}), json!({ "to": p("Ori › Claude"), "action": "read", "below": true }));
    let t = with(&w, &[grant.clone(), read(11, "Ori › Claude", "Ori › files › contract")]);
    assert!(taken(&t, 11), "a granted read was refused: {:?}", t.rejected);
    let t = with(&w, &[grant.clone(), read(11, "Ori › other", "Ori › files › contract")]);
    assert!(!taken(&t, 11), "a read by someone the grant does not name was taken");
    let t = with(&w, &[grant, act(11, "Ori › Claude", "Ori › Claude", "copy", json!({ "on": p("Ori › files › contract") }), json!({}))]);
    assert!(!taken(&t, 11), "an act the grant does not name was taken");
    // what is your own needs no grant
    let t = with(&w, &[act(10, "Ori › Claude", "Ori › Claude", "read", json!({ "on": p("Ori › Claude") }), json!({}))]);
    assert!(taken(&t, 10), "reading your own needed a grant");
}

#[test]
fn a_grant_ends_when_it_runs_out_or_is_revoked() {
    let w = world();
    let grant = act(10, "Ori", "Ori › files", "grant", json!({}), json!({ "to": p("Ori › Claude"), "action": "read", "below": true, "until": 20 }));
    let t = with(&w, &[grant.clone(), read(15, "Ori › Claude", "Ori › files › contract")]);
    assert!(taken(&t, 15), "a read before the grant ran out was refused");
    let t = with(&w, &[grant, read(25, "Ori › Claude", "Ori › files › contract")]);
    assert!(!taken(&t, 25), "a read after the grant ran out was taken");
    assert!(t.rejected.iter().any(|r| r["why"].as_str().unwrap_or("").contains("ran out")), "{:?}", t.rejected);
    // revoked by its writer: no more reads
    let open = act(10, "Ori", "Ori › files", "grant", json!({}), json!({ "to": p("Ori › Claude"), "action": "read", "below": true }));
    let revoke = act(12, "Ori", "Ori › files", "revoke", json!({ "re": 10 }), json!({}));
    let t = with(&w, &[open, read(11, "Ori › Claude", "Ori › files › contract"), revoke, read(13, "Ori › Claude", "Ori › files › contract")]);
    assert!(taken(&t, 11) && !taken(&t, 13), "a revoke did not end the grant: {:?}", t.rejected);
}

// A BUDGET LETS ACTS WHILE WHAT WAS SPENT IS UNDER IT: an act's cost is known only once it happened, so the one that crosses the
// limit is taken whole, and the next is the one refused — and before a movement, the twin answers the same (`may`), writing nothing
#[test]
fn a_grant_with_a_budget_takes_acts_while_what_was_spent_is_under_it() {
    let w = world();
    let grant = act(10, "Ori", "Ori › Anthropic", "grant", json!({}), json!({ "to": p("Ori › Claude"), "action": "call", "budget": { "field": "usd", "max": 10.0 } }));
    let t = with(&w, &[grant.clone(), call(11, 4.0), call(12, 4.0), call(13, 4.0), call(14, 1.0)]);
    assert!(taken(&t, 11) && taken(&t, 12) && taken(&t, 13), "acts while under the budget were refused (the third crosses it, whole): {:?}", t.rejected);
    assert!(!taken(&t, 14), "an act after the budget was spent was taken");
    assert!(t.rejected.iter().any(|r| r["why"].as_str().unwrap_or("").contains("over its budget")), "{:?}", t.rejected);
    let by = p("Ori › Claude");
    assert!(t.may(&by, &p("Ori › Anthropic"), "call", 15).unwrap_err().contains("over its budget"), "may let a movement past a spent budget");
    let t = with(&w, &[grant, call(11, 4.0)]);
    assert!(t.may(&by, &p("Ori › Anthropic"), "call", 12).is_ok(), "may refused a movement under the budget");
    assert!(t.may(&by, &p("Ori › Anthropic"), "read", 12).is_err(), "may let an act the grant does not name");
    assert!(t.may(&p("Ori › other"), &p("Ori › Anthropic"), "call", 12).is_err(), "may let a node the grant does not name");
    assert!(t.may(&by, &p("Ori › Claude"), "write", 12).is_ok(), "may refused a node on what is its own");
    // a window: only what was spent inside it counts
    let windowed = act(10, "Ori", "Ori › Anthropic", "grant", json!({}), json!({ "to": p("Ori › Claude"), "action": "call", "budget": { "field": "usd", "max": 10.0, "window": 50 } }));
    let t = with(&w, &[windowed, call(11, 8.0), call(100, 8.0)]);
    assert!(taken(&t, 100), "a call after the window was counted against it: {:?}", t.rejected);
}

// A GRANT TO WRITE A NODE'S DETAILS IS NEVER A GRANT TO BECOME IT: under a grant of `be`, its plain details are written — its keys, who
// carries it and what stays when it is deleted are taken only from it or a node above it
#[test]
fn a_grant_never_sets_what_governs_a_node() {
    let w = world();
    let grant = act(10, "Ori", "Ori › files", "grant", json!({}), json!({ "to": p("Ori › Claude"), "action": "be", "below": true }));
    let t = with(&w, &[grant.clone(), act(11, "Ori › Claude", "Ori › files › contract", "be", json!({}), json!({ "status": "signed" }))]);
    assert!(taken(&t, 11), "a granted detail was refused: {:?}", t.rejected);
    for (g, k) in [(12, "keys"), (13, "carried_by"), (14, "keep_deleted")] {
        let t = with(&w, &[grant.clone(), act(g, "Ori › Claude", "Ori › files › contract", "be", json!({}), json!({ k: [] }))]);
        assert!(!taken(&t, g), "{k} was set under a grant");
        assert!(t.rejected.iter().any(|r| r["why"].as_str().unwrap_or("").contains("never under a grant")), "{:?}", t.rejected);
    }
    // the node itself, and a node above it, set them as ever
    let t = with(&w, &[act(10, "Ori", "Ori › files › contract", "be", json!({}), json!({ "keep_deleted": true }))]);
    assert!(taken(&t, 10), "a node above could not set what governs below it: {:?}", t.rejected);
}
