//! THE RECORD AS ROWS: a twin whose entries are kept as rows (read back only when asked for) is the same twin as the one folded from
//! the list — every node, every ask, every hold, every rejection — and so is one started again from its state (`snapshot`) with only
//! the entries after it applied. Every check is made before anything changes: an act refused leaves no trace, its signature included.
//! Naming never creates, whatever the writer's clock says.
mod common;
use taldo_core::rows::Source;
use taldo_core::twin::{fold, Twin};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

fn p(s: &str) -> String {
    s.to_string()
}
fn act(g: i64, by: &str, at: &str, is: &str, refs: Value, value: Value) -> Value {
    common::signed(json!({ "g": g, "ts": g, "by": p(by), "at": p(at), "is": is, "refs": refs, "value": value }))
}

/// rows kept in memory, as a store keeps them: each by number, read back as text
#[derive(Default)]
struct Rows(Mutex<BTreeMap<i64, String>>);
impl Source for Rows {
    fn entry(&self, g: i64) -> Option<Value> {
        self.0.lock().unwrap().get(&g).and_then(|l| serde_json::from_str(l).ok())
    }
    fn page(&self, from: i64, to: i64, newest_first: bool, limit: usize) -> Vec<(i64, Value)> {
        let m = self.0.lock().unwrap();
        let it = m.range(from..=to).map(|(g, l)| (*g, serde_json::from_str(l).unwrap()));
        if newest_first {
            it.rev().take(limit).collect()
        } else {
            it.take(limit).collect()
        }
    }
    fn signed(&self, sig: &str) -> Option<i64> {
        self.0.lock().unwrap().iter().find(|(_, l)| serde_json::from_str::<Value>(l).unwrap()["sig"]["sig"] == sig).map(|(g, _)| *g)
    }
}
impl Rows {
    fn put(&self, e: &Value) {
        self.0.lock().unwrap().insert(e["g"].as_i64().unwrap(), e.to_string());
    }
}

/// AN ENTRY IN, as a program keeps rows: checked, written as its row only when taken, then landed
fn take(t: &mut Twin, rows: &Rows, e: &Value, arrived: i64) {
    match t.admit(e, arrived) {
        Ok(present) => {
            rows.put(e);
            t.land(e, present, arrived);
        }
        Err(why) => t.rejected.push(why),
    }
}

/// what a twin says, read whole: each node, each entry, each ask and hold, what waits and what is owed, what was rejected
fn said(t: &Twin) -> Value {
    let nodes: Value = t.nodes.iter().map(|(k, n)| (k.clone(), json!({ "value": n.value, "accepted": n.accepted, "at": n.at, "by": n.by, "named": n.named, "kids": t.children(k), "force": t.in_force(k), "waiting": t.waiting(k), "asks": t.asks_for(k), "flows": t.flows(k) }))).collect::<serde_json::Map<_, _>>().into();
    let entries: Vec<Value> = t.entries.iter().map(|(g, e)| {
        let ask = (e["is"] == "ask").then(|| json!({ "open": t.ask_open(g), "holder": t.holder_of(g), "moved": t.moved(g), "joins": t.joins(g), "evidence": t.evidence(g) }));
        let hold = (e["is"] == "acceptance").then(|| json!({ "holds": t.holds(g), "unanswered": t.unanswered(g) }));
        json!({ "g": g, "e": *e, "accepted": t.accepted(g), "present": t.present(g), "kind": t.entries.kind_of(g), "ask": ask, "hold": hold })
    }).collect();
    let rejected: Vec<Value> = t.rejected.iter().map(|r| json!([r["g"], r["why"]])).collect();
    json!({ "nodes": nodes, "entries": entries, "rejected": rejected, "owed": t.owed(), "len": t.entries.len(), "top": t.entries.top() })
}

/// A RECORD WITH EVERYTHING THE CORE READS: keys at every level, a flow set with you there, asks raised, moved, declined and raised
/// again, holds of state and of events (one strict), work let in and kept out, and acts refused for each reason
fn record() -> Vec<Value> {
    let (cc, s, t, mac, card) = ("Ori › Claude Code", "Ori › Claude Code › session a", "Ori › Claude Code › session b", "Ori › this Mac › Taldo", "Ori › catalog › Sign");
    let fix = json!([{ "says": "Fix it", "act": { "is": "fixing" } }]);
    let go = json!([{ "says": "Go", "to": { "state": "doing" } }]);
    let mut log = vec![
        common::first_key("Ori"),
        common::signed(json!({ "g": 1, "ts": 1, "by": "Ori", "at": "Ori", "is": "be", "refs": {}, "value": { "keys": [{ "key": common::public(&p("Ori")) }, { "key": common::finger_public() }] } })),
        common::under(2, cc),
        common::under(130, "Ori › catalog"),
        common::under(131, card),
        common::under(132, "Ori › needs"),
        common::under(133, "Ori › needs › x"),
        common::under(3, s),
        common::under(4, t),
        common::under(5, "Ori › this Mac"),
        common::under(6, mac),
        act(7, "Ori", card, "be", json!({}), json!({ "says": "Sign", "state": "later" })),
        common::touched(json!({ "g": 8, "ts": 8, "by": "Ori", "at": "Ori", "is": "be", "refs": {}, "value": { "flow:broken": { "by": p(cc) } } })),
        act(9, mac, mac, "ask", json!({ "about": p("Ori › needs › x") }), json!({ "label": "broken", "says": "broken?", "moves": fix })),
        act(10, mac, mac, "ask", json!({ "about": p("Ori › needs › x") }), json!({ "label": "broken", "says": "broken again?", "moves": fix })),
        act(11, s, s, "ask", json!({ "about": p(card) }), json!({ "label": "work", "says": "may I?", "moves": go })),
        act(12, "Ori", card, "be", json!({ "re": 11 }), json!({ "state": "doing" })),
        act(13, t, t, "ask", json!({ "about": p(card) }), json!({ "label": "design", "says": "this one?", "moves": go })),
        act(14, "Ori", "Ori", "decline", json!({ "re": 13 }), json!({})),
        act(15, cc, s, "fixing", json!({ "re": 9 }), json!({})),
        // a hold of state over the card: its state must stay doing — work that breaks it is kept out
        common::signed_by(&p("Ori"), act(16, "Ori › catalog", card, "acceptance", json!({}), json!({ "says": "it stays doing", "expect": { "at": p(card), "has": { "state": "doing" } } }))),
        act(17, "Ori", card, "be", json!({}), json!({ "says": "Sign, the form" })),
        common::signed_by(&p("Ori"), act(18, card, card, "be", json!({}), json!({ "state": "done" }))),
        // an event hold: every say at the session is heard by Taldo
        common::signed_by(&p("Ori"), act(19, cc, s, "acceptance", json!({}), json!({ "says": "every say is heard", "when": { "is": "say", "at": p(s) }, "expect": { "is": "heard", "from": p(mac) } }))),
        act(20, s, s, "say", json!({}), json!({ "says": "hello" })),
        act(21, s, s, "say", json!({}), json!({ "says": "again" })),
        act(22, mac, mac, "heard", json!({ "re": 20 }), json!({})),
        // a strict hold: no session answers a decision
        common::signed_by(&p("Ori"), act(23, "Ori", cc, "acceptance", json!({}), json!({ "strict": true, "says": "no session answers a decision", "when": { "is": "answer", "at": p(cc), "below": true }, "of": "re", "expect": { "is": "never" } }))),
        act(24, s, s, "answer", json!({ "re": 11 }), json!({})),
        // refused: not its writer's place; not signed by its writer; a signed act twice
        act(25, s, t, "be", json!({}), json!({})),
        common::signed_by(&p(t), act(26, s, s, "be", json!({}), json!({ "x": 1 }))),
        // the hold of state changed by the level above: done is let in now
        common::signed_by(&p("Ori"), act(27, "Ori › catalog", card, "acceptance", json!({ "re": 16 }), json!({ "says": "it is doing or done", "expect": { "at": p(card), "has": {} } }))),
        act(28, mac, mac, "ask", json!({ "about": p(card) }), json!({ "label": "work", "says": "and now?", "moves": go })),
    ];
    let mut twice = log[25].clone();
    twice["g"] = json!(29);
    log.push(twice);
    for g in 30..120 {
        let at = if g % 2 == 0 { s } else { t };
        log.push(act(g, at, at, ["step", "heard", "said", "be"][g as usize % 4], json!({}), json!({ "says": format!("step {g}") })));
    }
    log
}

#[test]
fn a_twin_kept_as_rows_is_the_twin_folded_from_the_list() {
    let log = record();
    let folded = fold(&log);
    assert!(folded.rejected.len() >= 4, "the record refuses something for each reason: {:?}", folded.rejected);
    let rows = Arc::new(Rows::default());
    let mut t = Twin::stored(rows.clone());
    t.entries.index(&["says", "keys"]);
    for e in &log {
        take(&mut t, &rows, e, e["ts"].as_i64().unwrap());
    }
    // a detail kept at hand finds what walking the entries finds
    for f in ["says", "keys", "moves"] {
        assert_eq!(t.entries.having(f), folded.entries.having(f), "{f}");
    }
    assert!(!t.entries.having("says").is_empty());
    // work kept out by a hold of state, then let in when the level above changed the hold
    let early = fold(&log[..23]);
    assert!(!early.accepted(18) && early.entries.contains_key(&18) && folded.accepted(18));
    assert_eq!(said(&t), said(&folded));
    // the entries are read back from their rows, a page at a time, from either end
    let back: Vec<i64> = t.entries.iter().rev().map(|(g, _)| g).collect();
    let mut fwd: Vec<i64> = folded.entries.iter().map(|(g, _)| g).collect();
    fwd.reverse();
    assert_eq!(back, fwd);
    assert_eq!(t.entries.range(40..=50).count(), folded.entries.range(40..=50).count());
    assert_eq!(t.entries.of_kind("ask"), folded.entries.of_kind("ask"));
}

#[test]
fn a_twin_started_again_from_its_state_is_the_same_twin() {
    let log = record();
    let folded = fold(&log);
    for cut in [1, 9, 15, 19, 24, 28, 60, log.len()] {
        let rows = Arc::new(Rows::default());
        let mut t = Twin::stored(rows.clone());
        for e in &log[..cut] {
            take(&mut t, &rows, e, e["ts"].as_i64().unwrap());
        }
        if cut > 9 {
            t.entries.index(&["says"]);
        }
        let state = t.snapshot();
        let mut again = Twin::restore(&state, rows.clone()).unwrap();
        assert_eq!(said(&again), said(&t), "started again at {cut}");
        assert_eq!(again.entries.having("says"), t.entries.having("says"), "started again at {cut}");
        for e in &log[cut..] {
            take(&mut again, &rows, e, e["ts"].as_i64().unwrap());
        }
        assert_eq!(said(&again), said(&folded), "started again at {cut}, then the rest");
        assert_eq!(again.entries.having("says"), folded.entries.having("says"), "started again at {cut}, then the rest");
    }
    // a state cut short, or not a state, is never read as one
    let rows = Arc::new(Rows::default());
    let state = fold(&log[..5]).snapshot();
    assert!(Twin::restore(&state[..state.len() - 1], rows.clone()).is_err());
    assert!(Twin::restore(b"not a state", rows).is_err());
}

#[test]
fn an_act_refused_leaves_no_trace() {
    let log = vec![common::first_key("Ori")];
    let mut t = fold(&log);
    let at = 10;
    // naming a node never made: refused, and nothing of it stays — its signature neither
    let mut e = common::signed(json!({ "g": 1, "ts": at, "by": "Ori", "at": "Ori", "is": "say", "refs": { "about": "Ori › b" }, "value": {} }));
    let before = format!("{:?}", said(&t));
    assert!(t.admit(&e, at).is_err());
    assert_eq!(format!("{:?}", said(&t)), before, "a refused act changed the twin");
    t.apply_arrived(&e, at);
    assert_eq!(t.rejected.len(), 1);
    // the node made by its owner: the same signed act, given again, is taken — it was never taken before
    t.apply_arrived(&common::under(2, "Ori › b"), at);
    e["g"] = json!(3);
    t.apply_arrived(&e, at);
    assert!(t.entries.contains_key(&3), "{:?}", t.rejected);
    // a writer's clock skips nothing: an act naming a node never made is refused, whenever it says it was written
    let old = common::signed(json!({ "g": 4, "ts": 5, "by": "Ori", "at": "Ori", "is": "say", "refs": { "about": "Ori › c" }, "value": {} }));
    assert!(t.admit(&old, at).is_err());
    assert!(t.admit(&old, 0).is_err());
}
