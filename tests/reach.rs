//! 7. OWED and 8. REACH — whatever doesn't hold is owed to an address; it has reached it when that address, or below it, answers.
mod common;
use taldo_core::twin::fold;
use serde_json::{json, Value};

fn p(s: &str) -> String {
    s.to_string()
}
fn act(g: i64, by: &str, at: &str, is: &str, refs: Value, value: Value) -> Value {
    common::signed(
        json!({ "g": g, "ts": g, "by": p(by), "at": p(at), "is": is, "refs": refs, "value": value }),
    )
}
#[test]
fn what_is_owed_has_an_address_and_reaching_it_is_recorded() {
    let mut log = vec![
        common::first_key("Ori"),
        common::under(1, "Ori › Taldo"),
        common::under(2, "Ori › Taldo › agent"),
        act(
            3,
            "Ori",
            "Ori › Taldo",
            "acceptance",
            json!({}),
            json!({ "when": { "is": "say", "at": "Ori", "by": "Ori" }, "expect": { "is": "say", "from": "Ori › Taldo" } }),
        ),
        act(
            4,
            "Ori",
            "Ori",
            "say",
            json!({}),
            json!({ "says": "hello" }),
        ),
    ];
    let t = fold(&log);
    let owed = t.owed();
    assert_eq!(
        owed,
        vec![("Ori › Taldo".to_string(), 3)],
        "the hold that does not hold is not owed to Taldo"
    );
    assert!(
        !t.reached("Ori › Taldo", 3),
        "it reached Taldo before anything there answered"
    );
    // the agent, under Taldo, answers what Ori said: it has reached Taldo's branch, and the hold holds
    log.push(act(
        5,
        "Ori › Taldo › agent",
        "Ori › Taldo › agent",
        "say",
        json!({ "re": 4, "for": 3 }),
        json!({ "says": "hi" }),
    ));
    let t = fold(&log);
    assert!(
        t.reached("Ori › Taldo", 3),
        "the answer from under Taldo did not count as reaching it"
    );
    assert!(t.owed().is_empty(), "still owed after it was met");
}
