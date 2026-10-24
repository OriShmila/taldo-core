//! 1. IDENTITY — each part has its own identity, and every act is verified as its own.
mod common;
use taldo_core::identity::{b64url, signed_bytes};
use taldo_core::twin::fold;
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn p(s: &str) -> String {
    s.to_string()
}
fn act(g: i64, by: &str, at: &str, is: &str, value: Value) -> Value {
    json!({ "g": g, "ts": g, "by": p(by), "at": p(at), "is": is, "refs": {}, "value": value })
}
fn by_key(k: &SigningKey, mut e: Value) -> Value {
    let s: Signature = k.sign(signed_bytes(&e).as_bytes());
    let pubk: String = k
        .verifying_key()
        .to_sec1_point(true)
        .as_bytes()
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect();
    e["sig"] = json!({ "key": pubk, "sig": s.to_bytes().iter().map(|x| format!("{x:02x}")).collect::<String>() });
    e
}
/// a world: Ori's first key; the agent under Ori, given its key by Ori
fn world() -> Vec<Value> {
    vec![
        common::first_key("Ori"),
        common::under(1, "Ori › agent"),
    ]
}
fn takes(log: &[Value], e: Value) -> bool {
    let g = e["g"].as_i64().unwrap();
    let mut l = log.to_vec();
    l.push(e);
    fold(&l).entries.contains_key(&g)
}

#[test]
fn an_act_is_taken_only_signed_by_its_node_or_a_node_above_it() {
    let w = world();
    assert_eq!(
        fold(&w).entries.len(),
        2,
        "the first key or the agent's key was not taken"
    );
    assert!(
        takes(
            &w,
            common::signed(act(2, "Ori", "Ori", "say", json!({ "says": "mine" })))
        ),
        "Ori's own act was refused"
    );
    assert!(
        takes(
            &w,
            common::signed(act(2, "Ori › agent", "Ori › agent", "say", json!({})))
        ),
        "the agent's own act was refused"
    );
    assert!(
        takes(
            &w,
            by_key(
                &common::key_of(&p("Ori")),
                act(2, "Ori › agent", "Ori › agent", "say", json!({}))
            )
        ),
        "the level above could not act for the agent"
    );
    assert!(
        !takes(
            &w,
            by_key(
                &common::key_of(&p("Ori › agent")),
                act(2, "Ori", "Ori", "say", json!({}))
            )
        ),
        "the agent's key signed as Ori"
    );
    assert!(
        !takes(&w, act(2, "Ori", "Ori", "say", json!({}))),
        "an unsigned act was taken"
    );
    let stranger = SigningKey::from_slice(&Sha256::digest(b"stranger")).unwrap();
    assert!(
        !takes(
            &w,
            by_key(&stranger, act(2, "Ori", "Ori", "say", json!({})))
        ),
        "a stranger's key was taken"
    );
    let mut changed = common::signed(act(2, "Ori", "Ori", "say", json!({ "says": "original" })));
    changed["value"]["says"] = json!("changed after signing");
    assert!(
        !takes(&w, changed),
        "an act changed after it was signed was taken"
    );
}
#[test]
fn the_root_has_one_first_key_and_a_signed_act_is_taken_once() {
    let mut w = world();
    let second_first = json!({ "g": 2, "ts": 2, "by": "Ori", "at": "Ori", "is": "be", "refs": {}, "value": { "keys": [{ "key": "02".to_string() + &"11".repeat(32) }] } });
    assert!(
        !takes(&w, second_first),
        "a second unsigned first key was taken"
    );
    let said = common::signed(act(2, "Ori", "Ori", "say", json!({ "says": "once" })));
    w.push(said.clone());
    let mut again = said;
    again["g"] = json!(3);
    assert!(!takes(&w, again), "the same signed act was taken twice");
    assert!(
        takes(
            &w,
            common::signed(act(3, "Ori", "Ori", "say", json!({ "says": "once" })))
        ),
        "the same words at another moment were refused"
    );
}
#[test]
fn a_key_signs_from_a_platform_authenticator_too() {
    // what Touch ID returns: it signs its own data and the hash of the client's, whose challenge is the hash of the act
    let w = world();
    let k = common::key_of(&p("Ori"));
    let mut e = act(
        2,
        "Ori",
        "Ori › agent",
        "acceptance",
        json!({ "when": { "is": "say", "at": "Ori" }, "expect": { "from": "Ori › agent" } }),
    );
    let cd = json!({ "type": "webauthn.get", "challenge": b64url(&Sha256::digest(signed_bytes(&e).as_bytes())) }).to_string();
    let mut ad = Sha256::digest(b"localhost").to_vec();
    ad.extend([0x05, 0, 0, 0, 1]);
    let mut m = ad.clone();
    m.extend(Sha256::digest(cd.as_bytes()));
    let s: Signature = k.sign(&m);
    e["sig"] = json!({ "key": common::public(&p("Ori")), "sig": s.to_der().as_bytes().iter().map(|x| format!("{x:02x}")).collect::<String>(), "authenticatorData": b64url(&ad), "clientDataJSON": b64url(cd.as_bytes()) });
    assert!(
        takes(&w, e.clone()),
        "an act signed through the authenticator was refused"
    );
    let mut other = e;
    other["value"]["expect"]["from"] = json!(["Ori"]);
    assert!(
        !takes(&w, other),
        "the authenticator's signature was taken for another act"
    );
}
