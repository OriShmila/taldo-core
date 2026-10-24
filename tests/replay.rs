//! NO ACT TAKEN TWICE, HOWEVER ITS SIGNATURE IS WRITTEN: a signature verifies in several spellings — upper-case hex, its mirror
//! (r, n−s), DER for the plain r‖s — and each is the same act already taken. Found by a review (24 Oct): an old act, its signature only
//! re-spelled, was taken again — a removed phone let back in, a rule weakened again.
mod common;
use common::*;
use p256::ecdsa::Signature;
use serde_json::{json, Value};
use taldo_core::identity::canonical_sig;
use taldo_core::twin::{fold, Twin};

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
fn unhex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}
/// the mirror of a plain r‖s signature: (r, n−s), which verifies too
fn mirrored(sig: &str) -> String {
    let s = Signature::from_slice(&unhex(sig)).unwrap();
    let (r, s) = s.split_scalars();
    hex(&Signature::from_scalars(r.to_bytes(), (-*s).to_bytes()).unwrap().to_bytes())
}
fn der(sig: &str) -> String {
    hex(Signature::from_slice(&unhex(sig)).unwrap().to_der().as_bytes())
}
fn world() -> (Vec<Value>, Value) {
    let mut es = vec![first_key("Ori")];
    let a = act(1, "Ori", "Ori", "be", json!({}), json!({ "name": "Ori" }));
    es.push(a.clone());
    (es, a)
}
fn taken_again(es: &[Value], again: &Value) -> bool {
    let t: Twin = fold(es);
    assert!(t.rejected.is_empty(), "{:?}", t.rejected);
    let mut e = again.clone();
    e["g"] = json!(9);
    t.admit(&e, 9).is_ok()
}
#[test]
fn an_act_is_taken_once_however_its_signature_is_spelled() {
    let (es, a) = world();
    let sig = a["sig"]["sig"].as_str().unwrap().to_string();
    for (how, spelled) in [("as written", sig.clone()), ("in upper case", sig.to_uppercase()), ("mirrored", mirrored(&sig)), ("as DER", der(&sig)), ("mirrored, as DER, upper case", der(&mirrored(&sig)).to_uppercase())] {
        let mut again = a.clone();
        again["sig"]["sig"] = json!(spelled);
        assert!(!taken_again(&es, &again), "the act was taken again with its signature {how}");
    }
}
#[test]
fn every_spelling_of_a_signature_is_one() {
    let (_, a) = world();
    let sig = a["sig"]["sig"].as_str().unwrap();
    let one = canonical_sig(sig).unwrap();
    for spelled in [sig.to_uppercase(), mirrored(sig), der(sig), der(&mirrored(sig)).to_uppercase()] {
        assert_eq!(canonical_sig(&spelled).as_deref(), Some(one.as_str()), "{spelled}");
    }
    assert_eq!(one, one.to_lowercase());
    assert_eq!(canonical_sig("not hex"), None);
    assert_eq!(canonical_sig("00"), None);
}
// A PASSKEY'S ACT, re-spelled from DER to its plain form (or mirrored), is the act already taken
#[test]
fn a_passkeys_act_is_taken_once_however_spelled() {
    let mut es = vec![first_key("Ori")];
    es.push(act(1, "Ori", "Ori", "be", json!({}), json!({ "keys": [{ "key": public("Ori") }, { "key": finger_public(), "presence": true }] })));
    let p = touched(json!({ "g": 2, "ts": 2, "by": "Ori", "at": "Ori", "is": "be", "refs": {}, "value": { "name": "Ori" } }));
    es.push(p.clone());
    let sig = p["sig"]["sig"].as_str().unwrap();
    let plain = hex(&Signature::from_der(&unhex(sig)).unwrap().to_bytes());
    for (how, spelled) in [("plain", plain.clone()), ("plain, mirrored", mirrored(&plain)), ("upper case", sig.to_uppercase())] {
        let mut again = p.clone();
        again["sig"]["sig"] = json!(spelled);
        assert!(!taken_again(&es, &again), "the passkey's act was taken again, its signature {how}");
    }
}
