//! A RULE THAT HOLDS AT RUNTIME: each time a thing happens, what must come of it — checked on the real event, as it is written. With
//! `since`, a rule written today asks only about what happens from now on, never every time before it; with `below`, it asks about
//! the thing wherever it happens beneath its node. Until what it expects comes, the happening is open and owed; nothing is a timeout.
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
fn a_rule_from_now_on_asks_only_what_happens_after_it_wherever_beneath() {
    let cc = "Ori › Claude Code";
    let mut log = vec![
        common::first_key("Ori"),
        common::under(901, "Ori › needs"),
        common::under(902, "Ori › needs › heard"),
        common::under(903, "Ori › needs › top"),
        common::under(1, cc),
        common::under(2, "Ori › Claude Code › session a"),
        common::under(3, "Ori › Claude Code › session b"),
        // said before the rule: never heard — and never owed by it
        act(
            4,
            "Ori",
            "Ori › Claude Code › session a",
            "say",
            json!({}),
            json!({ "says": "before" }),
        ),
        // the rule: what you say to any session is heard by it, from now on
        common::signed_by(
            &p("Ori"),
            act(
                5,
                "Ori › needs",
                "Ori › needs › heard",
                "acceptance",
                json!({}),
                json!({ "says": "what you say reaches the session", "when": { "is": "say", "at": p(cc), "by": "Ori", "below": true }, "expect": { "is": "heard" }, "since": true }),
            ),
        ),
    ];
    let t = fold(&log);
    assert_eq!(
        t.holds(5),
        Some(true),
        "a rule from now on was owed for what was said before it"
    );
    assert!(t.asked(5).is_empty());
    // said after it, in two sessions: both open, owed
    log.push(act(
        6,
        "Ori",
        "Ori › Claude Code › session a",
        "say",
        json!({}),
        json!({ "says": "one" }),
    ));
    log.push(act(
        7,
        "Ori",
        "Ori › Claude Code › session b",
        "say",
        json!({}),
        json!({ "says": "two" }),
    ));
    let t = fold(&log);
    assert_eq!(
        t.asked(5),
        vec![6, 7],
        "what is said beneath the node, after the rule, is what it asks about"
    );
    assert_eq!(t.unanswered(5), vec![6, 7]);
    assert_eq!(t.holds(5), Some(false));
    assert!(t
        .owed()
        .iter()
        .any(|(to, g)| *g == 5 && to == "Ori › needs › heard"));
    // a "could not" answers it but is not what it expects: still open, and kept
    log.push(act(
        8,
        "Ori › Claude Code › session b",
        "Ori › Claude Code › session b",
        "unmet",
        json!({ "re": 7 }),
        json!({ "why": "ended before it heard" }),
    ));
    // the session hears the first: that one is met
    log.push(act(
        9,
        "Ori › Claude Code › session a",
        "Ori › Claude Code › session a",
        "heard",
        json!({ "re": 6 }),
        json!({}),
    ));
    let t = fold(&log);
    assert_eq!(
        t.unanswered(5),
        vec![7],
        "the heard one is still open, or the unheard one is not"
    );
    assert_eq!(t.holds(5), Some(false));
    // without `below`, only its own node is asked: nothing happens at Claude Code itself
    log.push(common::signed_by(&p("Ori"), act(10, "Ori › needs", "Ori › needs › top", "acceptance", json!({}), json!({ "when": { "is": "say", "at": p(cc), "by": "Ori" }, "expect": { "is": "heard" }, "since": true }))));
    let t = fold(&log);
    assert!(t.asked(10).is_empty() && t.holds(10) == Some(true));
}
