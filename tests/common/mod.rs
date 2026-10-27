//! Keys for test worlds: each node's key is drawn from its id, so any test can sign as any node it builds — and give it its key.
#![allow(dead_code)]
use taldo_core::identity::signed_bytes;
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub fn key_of(id: &str) -> SigningKey {
    SigningKey::from_slice(&Sha256::digest(id.as_bytes())).unwrap()
}
pub fn public(id: &str) -> String {
    key_of(id)
        .verifying_key()
        .to_sec1_point(true)
        .as_bytes()
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect()
}
/// the entry, signed by its writer's own key
pub fn signed(mut e: Value) -> Value {
    arrives(&mut e);
    let by = e["by"].as_str().unwrap().to_string();
    let s: Signature = key_of(&by).sign(signed_bytes(&e).as_bytes());
    e["sig"] = json!({ "key": public(&by), "sig": s.to_bytes().iter().map(|x| format!("{x:02x}")).collect::<String>() });
    e
}
/// the root's first key, unsigned: the one entry nobody vouches for
pub fn first_key(root: &str) -> Value {
    json!({ "g": 0, "ts": 0, "arrived": 0, "by": root, "at": root, "is": "be", "refs": {}, "value": { "keys": [{ "key": public(root) }] } })
}
/// A ROW AS KEPT: arrived when its writer wrote it — the test worlds' own choice; a receiver keeps its own time
pub fn arrives(e: &mut Value) {
    if e.get("arrived").is_none() {
        e["arrived"] = e["ts"].clone();
    }
}
/// an act, signed by its writer's own key
pub fn act(g: i64, by: &str, at: &str, is: &str, refs: Value, value: Value) -> Value {
    signed(json!({ "g": g, "ts": g, "by": by, "at": at, "is": is, "refs": refs, "value": value }))
}
/// a node made under its parent, with its own key — a make says only its kind (and keys): its name is a detail after it, so erasing
/// what it holds never leaves who it was
pub fn make(g: i64, by: &str, id: &str, parent: &str, kind: &str) -> Value {
    act(g, by, id, "make", json!({ "in": parent }), json!({ "kind": kind, "keys": [{ "key": public(id) }] }))
}
/// the entry, signed by a node above its writer — the level above acting for the level below
pub fn signed_by(signer: &str, mut e: Value) -> Value {
    arrives(&mut e);
    let s: Signature = key_of(signer).sign(signed_bytes(&e).as_bytes());
    e["sig"] = json!({ "key": public(signer), "sig": s.to_bytes().iter().map(|x| format!("{x:02x}")).collect::<String>() });
    e
}

/// the finger: a key signed through an authenticator, as Touch ID does — a person there
pub fn finger() -> SigningKey {
    SigningKey::from_slice(&Sha256::digest(b"the finger")).unwrap()
}
pub fn finger_public() -> String {
    finger()
        .verifying_key()
        .to_sec1_point(true)
        .as_bytes()
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect()
}
/// the entry, signed with the finger, present and verified
pub fn touched(mut e: Value) -> Value {
    arrives(&mut e);
    use taldo_core::identity::b64url;
    let cd = json!({ "type": "webauthn.get", "challenge": b64url(&Sha256::digest(signed_bytes(&e).as_bytes())) }).to_string();
    let mut ad = Sha256::digest(b"localhost").to_vec();
    ad.extend([0x05, 0, 0, 0, 1]);
    let mut m = ad.clone();
    m.extend(Sha256::digest(cd.as_bytes()));
    let s: p256::ecdsa::DerSignature = finger().sign(&m);
    e["sig"] = json!({ "key": finger_public(), "sig": s.as_bytes().iter().map(|x| format!("{x:02x}")).collect::<String>(), "authenticatorData": b64url(&ad), "clientDataJSON": b64url(cd.as_bytes()) });
    e
}
/// a node made under the node its id names one step up ("Ori › Taldo" under "Ori"), by that node, with its own key — test worlds keep
/// readable ids; the graph never reads them
pub fn under(g: i64, id: &str) -> Value {
    let (parent, _) = id.rsplit_once(" › ").expect("an id with a parent");
    make(g, parent, id, parent, "thing")
}
/// nodes made, each under the one its id names one step up, numbered from `g` — the places a test world acts at
pub fn tree(g: i64, ids: &[&str]) -> Vec<Value> {
    ids.iter().enumerate().map(|(i, id)| under(g + i as i64, id)).collect()
}

/// A WAY IN FOR ONE MORE PERSON, given by a node that exists (a link, a sign-up): a `join` grant at itself
pub fn invite(g: i64, by: &str, comes: &str) -> Value {
    act(g, by, by, "grant", json!({}), json!({ "action": "join", "as": comes }))
}
/// a person come in by a way in: their own first key, answering it
pub fn joins(g: i64, person: &str, invite: i64) -> Value {
    signed(json!({ "g": g, "ts": g, "by": person, "at": person, "is": "be", "refs": { "re": invite }, "value": { "keys": [{ "key": public(person) }] } }))
}

/// THE TEST WORLDS' TALDO: a node of kind taldo under a person, whose key marks every line its keeper keeps
pub const TALDO: &str = "Taldo";
/// Taldo made under `person`, its key its own
pub fn taldo(g: i64, person: &str) -> Value {
    act(g, person, TALDO, "make", json!({ "in": person }), json!({ "kind": "taldo", "keys": [{ "key": public(TALDO) }] }))
}
/// what Taldo's key signs, as Keys signs it
pub fn mark_sign(bytes: &[u8]) -> Result<Value, String> {
    let s: Signature = key_of(TALDO).sign(bytes);
    Ok(json!({ "key": public(TALDO), "sig": s.to_bytes().iter().map(|x| format!("{x:02x}")).collect::<String>() }))
}
/// AS THE KEEPER: a line taken — judged, marked, landed — and kept as one with what a purge erases and the state after
pub fn keep(store: &taldo_core::fakes::FakeStore, t: &mut taldo_core::twin::Twin, mut e: Value) -> Result<(), Value> {
    use taldo_core::seams::Store;
    arrives(&mut e);
    let a = e["arrived"].as_i64().unwrap();
    let mut next = t.clone();
    let taken = next.take(&e, a, TALDO, t.entries.top(), &mark_sign)?;
    store.keep("k", &[taken.row], &taken.erased, &next.snapshot()).unwrap();
    next.rows_kept();
    *t = next;
    Ok(())
}
/// TALDO'S WORD OF WHERE THE RECORD IS, signed — as its keeper keeps it outside the store
pub fn anchor_of(t: &taldo_core::twin::Twin, state: &[u8]) -> Value {
    let mut a = t.anchor(state, false);
    a["sig"] = mark_sign(taldo_core::rows::canonical_of(&a).as_bytes()).unwrap();
    a
}
/// a store, its lease taken by the keeper
pub fn store() -> std::sync::Arc<taldo_core::fakes::FakeStore> {
    let s = std::sync::Arc::new(taldo_core::fakes::FakeStore::default());
    taldo_core::seams::Store::lease(&*s, "k").unwrap();
    s
}
