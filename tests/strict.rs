//! A STRICT HOLD IS NEVER BROKEN: the level above says, once, what an act below must answer — and an act that does not is refused when
//! it is written, whatever program wrote it. No session answers a decision; Taldo on this Mac answers only failures. The level below
//! cannot write or loosen the rule over itself.
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
fn a_strict_hold_refuses_what_it_forbids_whoever_wrote_it() {
    let (cc, s, mac) = (
        "Ori › Claude Code",
        "Ori › Claude Code › session a",
        "Ori › this Mac › Taldo",
    );
    let mut log = vec![
        common::first_key("Ori"),
        common::under(1, cc),
        common::under(2, s),
        common::under(3, "Ori › this Mac"),
        common::under(4, mac),
        // a session asks; it fails at something; your rules, strict
        act(5, s, s, "question", json!({}), json!({ "says": "Deploy?" })),
        act(
            6,
            s,
            s,
            "failure",
            json!({}),
            json!({ "says": "could not reach it" }),
        ),
        common::signed_by(
            &p("Ori"),
            act(
                7,
                "Ori",
                cc,
                "acceptance",
                json!({}),
                json!({ "strict": true, "says": "no session answers a decision", "when": { "is": "answer", "at": p(cc), "below": true }, "of": "re", "expect": { "is": "never" } }),
            ),
        ),
        common::signed_by(
            &p("Ori"),
            act(
                8,
                "Ori › this Mac",
                mac,
                "acceptance",
                json!({}),
                json!({ "strict": true, "says": "Taldo answers only failures", "when": { "is": "answer", "at": p(mac) }, "of": "re", "expect": { "is": "failure" } }),
            ),
        ),
    ];
    let t = fold(&log);
    assert!(t.in_force(cc).contains(&7) && t.in_force(mac).contains(&8));
    // a session answering its own question: refused, never written
    log.push(act(
        9,
        s,
        s,
        "answer",
        json!({ "re": 5 }),
        json!({ "says": "yes, deploy" }),
    ));
    // Taldo answering the question for you: refused; answering the failure: taken
    log.push(act(
        10,
        mac,
        mac,
        "answer",
        json!({ "re": 5 }),
        json!({ "says": "yes" }),
    ));
    log.push(act(
        11,
        mac,
        mac,
        "answer",
        json!({ "re": 6 }),
        json!({ "says": "Fix it" }),
    ));
    // you answer, at the session: taken — the rule is over the level below, never over you
    log.push(common::signed_by(
        &p("Ori"),
        act(
            12,
            "Ori",
            s,
            "answer",
            json!({ "re": 5 }),
            json!({ "says": "Yes, deploy" }),
        ),
    ));
    let t = fold(&log);
    let refused: Vec<i64> = t.rejected.iter().filter_map(|r| r["g"].as_i64()).collect();
    assert_eq!(refused, vec![9, 10], "{:?}", t.rejected);
    assert!(t
        .rejected
        .iter()
        .all(|r| r["why"].as_str().unwrap().contains("strict hold")));
    assert!(t.entries.contains_key(&11) && t.entries.contains_key(&12));
    // the level below cannot write or loosen the rule over itself: Taldo on this Mac writing a looser one is no rule
    log.push(act(13, mac, mac, "acceptance", json!({ "re": 8 }), json!({ "strict": false, "when": { "is": "answer", "at": p(mac) }, "of": "re", "expect": { "is": "question" } })));
    log.push(act(
        14,
        mac,
        mac,
        "answer",
        json!({ "re": 5 }),
        json!({ "says": "yes" }),
    ));
    let t = fold(&log);
    assert!(
        t.in_force(mac) == vec![8],
        "a level loosened the rule over itself"
    );
    assert!(
        t.rejected.iter().any(|r| r["g"] == 14),
        "its answer was taken after it wrote itself a looser rule"
    );
}
