//! A DECISION IS A THING MOVING. An ask is held — open, owed to its holder — until its thing moves; asking again about the same thing
//! is evidence, never a second decision; who holds it is read from the flows on the way up; a hand may make only the move; a label a
//! level keeps is never handed off; a hand that already moved a thing that came back is passed over; a move a person must make is
//! taken only with them there. None of it knows what any label means.
mod common;
use taldo_core::twin::fold;
use serde_json::{json, Value};

fn p(s: &str) -> String {
    s.to_string()
}
fn act(g: i64, by: &str, at: &str, is: &str, refs: Value, value: Value) -> Value {
    common::signed(
        json!({ "g": g, "ts": g, "arrived": g, "by": p(by), "at": p(at), "is": is, "refs": refs, "value": value }),
    )
}
fn ask(g: i64, by: &str, about: &str, label: &str, moves: Value) -> Value {
    act(
        g,
        by,
        by,
        "ask",
        json!({ "about": p(about) }),
        json!({ "label": label, "says": format!("{label}?"), "moves": moves }),
    )
}

#[test]
fn a_decision_is_held_until_its_thing_moves_by_whoever_the_flows_name() {
    let (cc, s, t, rule, mac) = (
        "Ori › Claude Code",
        "Ori › Claude Code › session a",
        "Ori › Claude Code › session b",
        "Ori › needs › nothing-left-behind",
        "Ori › this Mac › Taldo",
    );
    // fixing it is an act of whoever fixes — at its own place, never written on the requirement from beside it
    let fix = json!([{ "says": "Fix it", "act": { "is": "fixing" } }]);
    let mut log = vec![
        common::first_key("Ori"),
        common::signed(
            json!({ "g": 99999, "ts": 0, "arrived": 0, "by": "Ori", "at": "Ori", "is": "be", "refs": {}, "value": { "keys": [{ "key": common::public(&p("Ori")) }, { "key": common::finger_public() }] } }),
        ),
        common::under(901, "Ori › needs"),
        common::under(902, rule),
        common::under(903, "Ori › needs › other"),
        common::under(904, "Ori › clients"),
        common::under(905, "Ori › clients › The lawyer's office"),
        common::under(906, "Ori › catalog"),
        common::under(907, "Ori › catalog › Sign"),
        common::under(908, "Ori › needs › old-rule"),
        common::under(909, "Ori › needs › new-rule"),
        common::under(910, "Ori › needs › quiet-rule"),
        common::under(1, cc),
        common::under(2, s),
        common::under(3, t),
        common::under(4, "Ori › this Mac"),
        common::under(5, mac),
        // YOUR FLOW, set before anything broke: what breaks under Ori is moved by Claude Code
        common::touched(
            json!({ "g": 6, "ts": 6, "arrived": 6, "by": "Ori", "at": "Ori", "is": "be", "refs": {}, "value": json!({ "flow:broken": { "by": p(cc) } }) }),
        ),
        // the rule broke: Taldo on this Mac asks about it — your flow hands it to Claude Code
        ask(7, mac, rule, "broken", fix.clone()),
    ];
    let tw = fold(&log);
    assert!(tw.ask_open(7));
    assert_eq!(tw.holder_of(7).as_deref(), Some(cc));
    assert!(tw.asks_for("Ori").is_empty() && tw.asks_for(cc) == vec![(7, false)]);
    // asked again while open: evidence on the same decision, never a second one — 93 looks are one ask
    log.push(ask(8, mac, rule, "broken", fix.clone()));
    log.push(ask(9, s, rule, "broken", fix.clone()));
    let tw = fold(&log);
    assert_eq!(tw.joins(8), Some(7));
    assert_eq!(tw.joins(9), Some(7));
    assert_eq!(tw.evidence(7), vec![8, 9]);
    assert_eq!(
        tw.asks_for(cc),
        vec![(7, false)],
        "a repeat became a second decision"
    );
    // a session may not make the move: not its to make, and beside itself
    log.push(act(10, s, s, "fixing", json!({ "re": 7 }), json!({})));
    let tw = fold(&log);
    // a session saying it fixes it, at itself: taken as what it says — never the move, it does not hold it
    assert!(tw.ask_open(7), "a session moved a thing it does not hold");
    // the hand writing anything but the move, or the move somewhere else: refused
    log.push(act(
        12,
        cc,
        rule,
        "be",
        json!({ "re": 7 }),
        json!({ "says": "nothing to see" }),
    ));
    log.push(act(
        13,
        cc,
        "Ori › needs › other",
        "be",
        json!({ "re": 7 }),
        json!({ "fixing": "Ori › Claude Code" }),
    ));
    let tw = fold(&log);
    assert!(
        tw.rejected.iter().any(|r| r["g"] == 12) && tw.rejected.iter().any(|r| r["g"] == 13),
        "a hand did more than its move"
    );
    // the move: the thing moved — the decision is over, whatever the work it starts will take
    log.push(act(14, cc, cc, "fixing", json!({ "re": 7 }), json!({})));
    let tw = fold(&log);
    assert!(
        !tw.rejected.iter().any(|r| r["g"] == 14),
        "{:?}",
        tw.rejected
    );
    assert_eq!(tw.moved(7), Some(14));
    assert!(!tw.ask_open(7) && tw.asks_for(cc).is_empty());
    // IT CAME BACK: asked again after the move — a new decision, and the hand that already moved it is passed over: it is yours
    log.push(ask(15, mac, rule, "broken", fix.clone()));
    let tw = fold(&log);
    assert!(tw.ask_open(15) && tw.joins(15).is_none());
    assert_eq!(
        tw.holder_of(15).as_deref(),
        Some("Ori"),
        "the same hand got it again"
    );
    // you decline it: over
    log.push(common::signed_by(
        &p("Ori"),
        act(16, "Ori", "Ori", "decline", json!({ "re": 15 }), json!({})),
    ));
    let tw = fold(&log);
    assert!(!tw.ask_open(15));
    // A LABEL YOU KEEP: money is yours, with your finger, whatever a level below hands off
    log.push(common::touched(json!({ "g": 17, "ts": 17, "arrived": 17, "by": "Ori", "at": "Ori", "is": "be", "refs": {}, "value": json!({ "flow:money": { "mine": true, "present": true } }) })));
    log.push(act(
        18,
        cc,
        cc,
        "be",
        json!({}),
        json!({ "flow:money": { "by": p(s) } }),
    ));
    log.push(ask(
        19,
        t,
        "Ori › catalog › Sign",
        "money",
        json!([{ "says": "Pay ₪49 to Inforu", "to": { "sms": "paid" } }]),
    ));
    let tw = fold(&log);
    assert_eq!(
        tw.holder_of(19).as_deref(),
        Some("Ori"),
        "a level below handed off a label you keep"
    );
    assert_eq!(tw.asks_for("Ori"), vec![(19, true)]);
    // and the move needs you there: written without your finger, refused; the session's own try, refused
    log.push(common::signed_by(
        &p("Ori"),
        act(
            20,
            "Ori",
            "Ori › catalog › Sign",
            "be",
            json!({ "re": 19 }),
            json!({ "sms": "paid" }),
        ),
    ));
    log.push(act(
        21,
        s,
        "Ori › catalog › Sign",
        "be",
        json!({ "re": 19 }),
        json!({ "sms": "paid" }),
    ));
    let tw = fold(&log);
    assert!(
        tw.rejected
            .iter()
            .any(|r| r["g"] == 20 && r["why"].as_str().unwrap().contains("needs you there")),
        "{:?}",
        tw.rejected
    );
    assert!(tw.rejected.iter().any(|r| r["g"] == 21));
    assert!(tw.ask_open(19));
    // A SESSION CANNOT ANSWER ITSELF: an ask about its own node, its own details written — not the move; yours is
    log.push(ask(30, s, s, "question", json!([{ "says": "Yes", "to": { "question": "Yes" } }, { "says": "No", "to": { "question": "No" } }])));
    log.push(act(
        31,
        s,
        s,
        "be",
        json!({ "re": 30 }),
        json!({ "question": "Yes" }),
    ));
    let tw = fold(&log);
    assert!(tw.ask_open(30), "a session moved its own question");
    log.push(common::signed_by(
        &p("Ori"),
        act(
            32,
            "Ori",
            s,
            "be",
            json!({ "re": 30 }),
            json!({ "question": "No" }),
        ),
    ));
    let tw = fold(&log);
    assert_eq!(tw.moved(30), Some(32));
    // A FLOW SET LATER NEVER TAKES OVER WHAT ALREADY WAITS ON YOU: asked before any flow for its label, it stays yours
    log.push(ask(
        40,
        mac,
        "Ori › needs › old-rule",
        "stale",
        json!([{ "says": "Fix it", "to": { "fixing": "Ori › Claude Code" } }]),
    ));
    log.push(common::touched(json!({ "g": 41, "ts": 41, "arrived": 41, "by": "Ori", "at": "Ori", "is": "be", "refs": {}, "value": json!({ "flow:stale": { "by": p(cc) } }) })));
    log.push(ask(
        42,
        mac,
        "Ori › needs › new-rule",
        "stale",
        json!([{ "says": "Fix it", "to": { "fixing": "Ori › Claude Code" } }]),
    ));
    let tw = fold(&log);
    assert_eq!(
        tw.holder_of(40).as_deref(),
        Some("Ori"),
        "a flow set later took over a decision already waiting on you"
    );
    assert_eq!(tw.holder_of(42).as_deref(), Some(cc));
    // YOUR NO, over a hand: a decision a flow handed to Claude Code, declined by you, is over
    log.push(common::signed_by(
        &p("Ori"),
        act(43, "Ori", "Ori", "decline", json!({ "re": 42 }), json!({})),
    ));
    let tw = fold(&log);
    assert!(
        !tw.ask_open(42),
        "your no did not end a decision a flow handed off"
    );
    // AN ASK THAT SAYS A PERSON MUST MOVE IT: only with you there, whatever the flows say
    log.push(act(44, mac, mac, "ask", json!({ "about": p("Ori › Claude Code › session b") }), json!({ "label": "key", "says": "let it join?", "present": true, "moves": [{ "says": "Allow", "to": { "joined": true } }] })));
    log.push(common::signed_by(
        &p("Ori"),
        act(
            45,
            "Ori",
            t,
            "be",
            json!({ "re": 44 }),
            json!({ "joined": true }),
        ),
    ));
    let tw = fold(&log);
    assert!(
        tw.rejected.iter().any(|r| r["g"] == 45),
        "a move its ask said needs you was taken without you"
    );
    assert_eq!(
        tw.asks_for("Ori").iter().find(|x| x.0 == 44),
        Some(&(44, true))
    );
    // A MOVE THAT IS AN ACT OF ITS OWN KIND: approving a command is the approval itself, signed by its holder — a session approving its
    // own proposal does not move it; yours, with you there, does
    log.push(act(46, s, s, "ask", json!({ "about": p(mac) }), json!({ "label": "run", "says": "Run `npm test`?", "present": true, "moves": [{ "says": "Run it", "act": { "is": "approve", "value": {} } }] })));
    log.push(act(47, s, s, "approve", json!({ "re": 46 }), json!({})));
    let tw = fold(&log);
    assert!(tw.ask_open(46), "a session approved its own proposal");
    log.push(common::signed_by(
        &p("Ori"),
        act(48, "Ori", "Ori", "approve", json!({ "re": 46 }), json!({})),
    ));
    let tw = fold(&log);
    assert!(
        tw.rejected.iter().any(|r| r["g"] == 48),
        "an approval its ask said needs you was taken without you"
    );
    assert!(tw.ask_open(46));
    // A FLOW SET WITHOUT YOU THERE HANDS NOTHING OFF: handing off a decision of yours is yours to do, in person
    log.push(act(
        50,
        "Ori",
        "Ori",
        "be",
        json!({}),
        json!({ "flow:quiet": { "by": p(cc) } }),
    ));
    log.push(ask(
        51,
        mac,
        "Ori › needs › quiet-rule",
        "quiet",
        json!([{ "says": "Fix it", "to": { "fixing": "Ori › Claude Code" } }]),
    ));
    let tw = fold(&log);
    assert_eq!(
        tw.holder_of(51).as_deref(),
        Some("Ori"),
        "a flow set without you there handed a decision off"
    );
    // NAMING NEVER CREATES: an ask about a node nobody made is refused — and says where to attach; one about a real thing is taken
    let late = |g: i64, by: &str, about: &str| {
        common::signed(
            json!({ "g": g, "ts": g, "arrived": g, "by": p(by), "at": p(by), "is": "ask", "refs": { "about": p(about) }, "value": { "label": "version", "says": "take it?", "moves": [{ "says": "Take it", "act": { "is": "update" } }] } }),
        )
    };
    log.push(late(60, mac, "Ori › this Mac › Taldo › versions › abc1234"));
    log.push(late(61, mac, s));
    let tw = fold(&log);
    let no = tw
        .rejected
        .iter()
        .find(|r| r["g"] == 60)
        .expect("an ask about a place nobody made was taken");
    assert!(
        no["why"]
            .as_str()
            .unwrap()
            .contains("make it under what it belongs to"),
        "{no}"
    );
    assert!(
        !tw.nodes
            .contains_key("Ori › this Mac › Taldo › versions › abc1234"),
        "naming it made it"
    );
    assert!(
        tw.ask_open(61),
        "an ask about a real thing in the chain was refused"
    );
    // A LABEL NOBODY PLANNED: any label works the same — routed, held, moved
    log.push(ask(22, s, "Ori › clients › The lawyer's office", "cancel", json!([{ "says": "Cancel his plan", "to": { "plan": "cancelled" } }, { "says": "Keep it", "to": { "plan": "kept" } }])));
    log.push(common::signed_by(
        &p("Ori"),
        act(
            23,
            "Ori",
            "Ori › clients › The lawyer's office",
            "be",
            json!({}),
            json!({ "plan": "kept" }),
        ),
    ));
    let tw = fold(&log);
    assert_eq!(tw.moved(22), Some(23));
    assert!(!tw.ask_open(22));
}

#[test]
fn two_asks_about_one_thing_are_one_question_only_if_their_moves_land_alike() {
    let (cc, s, card) = ("Ori › Claude Code", "Ori › Claude Code › session a", "Ori › catalog › Sign");
    let step = |slug: &str| json!([{ "says": "Do it", "act": { "is": "be", "at": p(&format!("{card} › steps › {slug}")), "value": { "state": "doing" } } }]);
    let log = vec![
        common::first_key("Ori"),
        common::under(1, cc),
        common::under(2, s),
        common::under(901, "Ori › catalog"),
        common::under(902, card),
        act(3, "Ori", card, "be", json!({}), json!({ "says": "Sign" })),
        // two steps of Sign, asked by one session: both about Sign, both "work" — two questions, never one folded into the other
        ask(4, s, card, "work", step("signer-flow")),
        ask(5, s, card, "work", step("setter-flow")),
        // the first asked again: evidence on it
        ask(6, s, card, "work", step("signer-flow")),
    ];
    let tw = fold(&log);
    assert_eq!(tw.joins(5), None, "a different step was folded into another as evidence");
    assert_eq!(tw.joins(6), Some(4));
    assert!(tw.ask_open(4) && tw.ask_open(5));
}
