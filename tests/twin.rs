//! The twin as a pure function of the log: entries fold into nodes, and the same log always folds to the same twin.
mod common;
use taldo_core::entry::check;
use taldo_core::twin::fold;
use serde_json::{json, Value};

fn e(g: i64, by: &str, at: &str, is: &str, refs: Value, value: Value) -> Value {
    common::signed(json!({ "g": g, "ts": g * 1000, "by": by, "at": at, "is": is, "refs": refs, "value": value }))
}

#[test]
fn a_make_brings_a_node_under_its_parent() {
    let t = fold(&[
        common::first_key("Ori"),
        common::under(1, "Ori › Mac"),
        common::under(2, "Ori › Mac › file"),
        e(3, "Ori", "Ori › Mac › file", "say", json!({}), json!({})),
    ]);
    assert!(t.rejected.is_empty(), "{:?}", t.rejected);
    assert_eq!(t.parent("Ori › Mac › file").as_deref(), Some("Ori › Mac"));
    assert_eq!(t.children("Ori › Mac"), vec!["Ori › Mac › file".to_string()]);
    assert_eq!(t.nodes["Ori › Mac › file"].at, vec![2, 3]);
}

#[test]
fn an_invalid_entry_is_rejected_not_folded() {
    let bad = json!({ "g": 1, "by": 7, "at": "Ori", "is": "say" });
    assert!(check(&bad).is_err());
    let t = fold(&[bad]);
    assert!(t.entries.is_empty());
    assert_eq!(t.rejected.len(), 1);
}

#[test]
fn folding_is_deterministic() {
    let log = [
        common::first_key("Ori"),
        common::under(1, "Ori › a"),
        e(2, "Ori", "Ori › a", "be", json!({}), json!({ "kind": "file" })),
        e(3, "Ori", "Ori › a", "be", json!({}), json!({ "state": "gone" })),
    ];
    let (a, b) = (fold(&log), fold(&log));
    assert_eq!(
        (&a.nodes["Ori › a"].value, &a.nodes["Ori › a"].accepted),
        (&b.nodes["Ori › a"].value, &b.nodes["Ori › a"].accepted)
    );
    assert_eq!(a.nodes["Ori › a"].value["state"], "gone");
}
