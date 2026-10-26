//! THE GRAPH (Taldo's schema): every node has its own id, a kind and a name; it comes to be by a `make` under its parent, and only
//! there. The parent holds what is under it; the owner is the person at the top of the chain; a node's level is its depth from that
//! person. Names need not be unique. An edge is only a mention: it never opens the node on the other side — your key opens a
//! client's org node and nothing below it, running something gives no reach into it, and a setup grant reaches below only while it
//! lasts. A person carries responsibilities: a stage held by a responsibility goes to whoever carries it now.
mod common;
use common::{act, make};
use taldo_core::twin::{fold, Twin};
use serde_json::{json, Value};

/// you, Harbor Tax's owner and two of its people; Taldo under you with its agent and servers; Harbor Tax under its owner, with Dana,
/// her case and its form, a responsibility, their Taldo (run on your servers) and an office laptop
fn world() -> Vec<Value> {
    vec![
        common::first_key("ori"),
        // each other person came in by a way in you gave: a link, a sign-up
        common::invite(901, "ori", "taxowner"),
        common::joins(902, "taxowner", 901),
        common::invite(903, "ori", "tax2"),
        common::joins(904, "tax2", 903),
        common::invite(905, "ori", "tax3"),
        common::joins(906, "tax3", 905),
        make(1, "ori", "taldoorg", "ori", "org"),
        make(2, "taldoorg", "agent", "taldoorg", "agent"),
        make(3, "taldoorg", "servers", "taldoorg", "machine"),
        make(4, "taxowner", "taxorg", "taxowner", "org"),
        make(5, "taxorg", "dana", "taxorg", "outside"),
        make(6, "taxorg", "case", "dana", "thing"),
        make(7, "taxorg", "form", "case", "document"),
        make(8, "taxorg", "resp1", "taxorg", "responsibility"),
        make(9, "taxorg", "taxtaldo", "taxorg", "sandbox"),
        // a device is let in with a finger: the owner's
        common::signed(json!({ "g": 13, "ts": 13, "arrived": 13, "by": "taxowner", "at": "taxowner", "is": "be", "refs": {}, "value": { "keys": [{ "key": common::public("taxowner") }, { "key": common::finger_public() }] } })),
        common::touched(json!({ "g": 14, "ts": 14, "arrived": 14, "by": "taxowner", "at": "laptop", "is": "make", "refs": { "in": "taxorg" }, "value": { "kind": "device", "keys": [{ "key": common::public("laptop") }] } })),
        act(11, "taxorg", "form", "be", json!({}), json!({ "name": "Form 106" })),
        act(12, "taxorg", "dana", "be", json!({}), json!({ "name": "Dana" })),
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
fn a_parent_holds_its_children_and_the_owner_is_the_person_at_the_top() {
    let t = with(&[]);
    assert!(t.rejected.is_empty(), "{:?}", t.rejected);
    assert_eq!(t.parent("case").as_deref(), Some("dana"));
    assert_eq!(t.owner("form").as_deref(), Some("taxowner"));
    assert_eq!(t.owner("laptop").as_deref(), Some("taxowner"), "a client's device sits under their org");
    assert_eq!(t.owner("agent").as_deref(), Some("ori"));
    assert_eq!(
        ["taxowner", "taxorg", "dana", "case", "form"].map(|n| t.level(n)),
        [Some(0), Some(1), Some(2), Some(3), Some(4)],
        "a level is the depth from the person"
    );
    assert_eq!(t.nodes["form"].kind, "document");
    assert_eq!(t.name("form").as_deref(), Some("Form 106"), "a name is a detail of the node");
    // the owner works anywhere under it, with no grant
    let t = with(&[act(20, "taxowner", "form", "be", json!({}), json!({ "state": "filled" }))]);
    assert!(taken(&t, 20), "{:?}", t.rejected);
}

#[test]
fn names_need_not_be_unique() {
    let t = with(&[make(20, "taxorg", "dana2", "taxorg", "outside"), act(21, "taxorg", "dana2", "be", json!({}), json!({ "name": "Dana" }))]);
    assert!(taken(&t, 20) && taken(&t, 21), "{:?}", t.rejected);
    assert_eq!(t.name("dana"), t.name("dana2"));
    assert_ne!(t.nodes["dana"].id, t.nodes["dana2"].id);
}

#[test]
fn a_node_comes_to_be_only_by_a_make_under_its_parent() {
    // naming what was never made: refused
    let t = with(&[act(20, "taxowner", "taxorg", "say", json!({ "about": "nobody" }), json!({}))]);
    assert!(!taken(&t, 20), "naming a node that was never made was taken");
    // acting at what was never made: refused
    let t = with(&[act(20, "taxowner", "ghost", "be", json!({}), json!({}))]);
    assert!(!taken(&t, 20), "acting at a node that was never made was taken");
    // an id already taken is never made again
    let t = with(&[make(20, "taxorg", "case", "taxorg", "thing")]);
    assert!(!taken(&t, 20), "a make took an id already in the graph");
    // a make under someone else's node, with no grant: refused
    let t = with(&[make(20, "taldoorg", "x", "taxorg", "thing")]);
    assert!(!taken(&t, 20), "Taldo made a node under Harbor Tax with no grant");
    // a make says only its kind: a name in it would outlive erasing what the node held
    let t = with(&[act(20, "taxorg", "bob", "make", json!({ "in": "taxorg" }), json!({ "kind": "outside", "name": "Bob" }))]);
    assert!(!taken(&t, 20), "a make carried a name");
}

#[test]
fn your_key_opens_their_org_node_and_nothing_below() {
    let grant = act(20, "taxowner", "taxorg", "grant", json!({}), json!({ "to": "taldoorg", "action": "read" }));
    let t = with(&[
        grant.clone(),
        act(21, "agent", "agent", "read", json!({ "on": "taxorg" }), json!({})),
        act(22, "agent", "agent", "read", json!({ "on": "case" }), json!({})),
    ]);
    assert!(taken(&t, 21), "the org node was not opened by the key on it: {}", why(&t, 21));
    assert!(!taken(&t, 22), "the key on their org opened what is below it");
    assert!(t.may("taldoorg", "taxorg", "read", 30).is_ok());
    assert!(t.may("taldoorg", "form", "read", 30).is_err(), "may let a key on the org reach below it");
    // and with no grant at all, not even the org node
    let t = with(&[act(21, "agent", "agent", "read", json!({ "on": "taxorg" }), json!({}))]);
    assert!(!taken(&t, 21), "Taldo read their org with no grant");
}

#[test]
fn a_setup_grant_reaches_below_their_org_only_while_it_lasts() {
    // made under a grant, a node is given no keys by its maker: who it is comes from above it
    let setup = act(20, "taxowner", "taxorg", "grant", json!({}), json!({ "to": "agent", "action": "make", "below": true }));
    let t = with(&[setup.clone(), act(21, "agent", "resp2", "make", json!({ "in": "taxorg" }), json!({ "kind": "responsibility" }))]);
    assert!(taken(&t, 21), "the setup grant did not let the agent set up: {}", why(&t, 21));
    assert_eq!(t.owner("resp2").as_deref(), Some("taxowner"), "what the agent made is theirs, not Taldo's");
    // a grant names its act: setting up is not reading
    let t = with(&[setup.clone(), act(21, "agent", "agent", "read", json!({ "on": "form" }), json!({}))]);
    assert!(!taken(&t, 21), "a setup grant let the agent read their clients");
    // setup ends: the grant is revoked, and nothing more is made
    let t = with(&[
        setup,
        act(21, "taxowner", "taxorg", "revoke", json!({ "re": 20 }), json!({})),
        act(22, "agent", "resp3", "make", json!({ "in": "taxorg" }), json!({ "kind": "responsibility" })),
    ]);
    assert!(!taken(&t, 22), "the agent kept setting up after setup ended");
}

#[test]
fn running_something_gives_no_reach_into_it() {
    let t = with(&[
        act(20, "servers", "servers", "run", json!({ "runs": "taxtaldo" }), json!({})),
        act(21, "servers", "servers", "read", json!({ "on": "taxtaldo" }), json!({})),
        act(22, "servers", "taxtaldo", "be", json!({}), json!({ "planted": true })),
    ]);
    assert!(taken(&t, 20), "a mention of what it runs was refused: {}", why(&t, 20));
    assert!(!taken(&t, 21), "running their Taldo let the servers read it");
    assert!(!taken(&t, 22), "running their Taldo let the servers write in it");
    // a mention back up the chain (the form mentions Dana) is only a mention: levels stay
    let t = with(&[act(23, "taxorg", "form", "be", json!({ "mentions": "dana" }), json!({}))]);
    assert!(taken(&t, 23), "{}", why(&t, 23));
    assert_eq!(t.level("form"), Some(4));
}

#[test]
fn a_stage_held_by_a_responsibility_goes_to_whoever_carries_it_now() {
    let finger = |e: Value| common::touched(e);
    let mut log = vec![
        // the owner's finger signs for the owner
        common::signed(json!({ "g": 19, "ts": 19, "arrived": 19, "by": "taxowner", "at": "taxowner", "is": "be", "refs": {}, "value": { "keys": [{ "key": common::public("taxowner") }, { "key": common::finger_public() }] } })),
        // intake under their org is the client manager's, and Person 2 carries it
        finger(json!({ "g": 20, "ts": 20, "arrived": 20, "by": "taxowner", "at": "taxorg", "is": "be", "refs": {}, "value": { "flow:intake": { "by": "resp1" } } })),
        act(21, "taxowner", "resp1", "be", json!({}), json!({ "carried_by": ["tax2"] })),
        act(22, "taxorg", "taxorg", "ask", json!({ "about": "case" }), json!({ "label": "intake", "says": "intake?", "moves": [{ "says": "Done", "act": { "is": "intake" } }] })),
    ];
    let t = with(&log);
    assert!(t.rejected.is_empty(), "{:?}", t.rejected);
    assert_eq!(t.holder_of(22).as_deref(), Some("resp1"));
    assert_eq!(t.asks_for("tax2"), vec![(22, false)], "the one who carries it does not see the stage");
    assert!(t.asks_for("tax3").is_empty());
    // Person 2 goes on vacation: Person 3 carries it now, and the stage goes with it
    log.push(act(23, "taxowner", "resp1", "be", json!({}), json!({ "carried_by": ["tax3"] })));
    let t = with(&log);
    assert!(t.asks_for("tax2").is_empty(), "the stage stayed with who no longer carries it");
    assert_eq!(t.asks_for("tax3"), vec![(22, false)]);
    // only who carries it now moves it
    let mut by_rp2 = log.clone();
    by_rp2.push(common::signed_by("tax2", json!({ "g": 24, "ts": 24, "arrived": 24, "by": "resp1", "at": "resp1", "is": "intake", "refs": { "re": 22 }, "value": {} })));
    let t = with(&by_rp2);
    assert!(t.ask_open(22), "who no longer carries it moved it");
    log.push(common::signed_by("tax3", json!({ "g": 24, "ts": 24, "arrived": 24, "by": "resp1", "at": "resp1", "is": "intake", "refs": { "re": 22 }, "value": {} })));
    let t = with(&log);
    assert_eq!(t.moved(22), Some(24), "who carries it could not move it: {:?}", t.rejected);
}
