//! 1. IDENTITY keeps whether a person was there; 5. HOLD can read it. The core never requires it: an approval signed with a plain key
//! and one signed with a person present are both taken — and a hold on the one who acts on approvals tells them apart.
mod common;
use taldo_core::identity::{b64url, signed_bytes};
use taldo_core::twin::fold;
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn p(s: &str) -> String {
    s.to_string()
}
fn act(g: i64, by: &str, at: &str, is: &str, refs: Value, value: Value) -> Value {
    json!({ "g": g, "ts": g, "by": p(by), "at": p(at), "is": is, "refs": refs, "value": value })
}
fn finger() -> SigningKey {
    SigningKey::from_slice(&Sha256::digest(b"the finger")).unwrap()
}
fn finger_public() -> String {
    finger()
        .verifying_key()
        .to_sec1_point(true)
        .as_bytes()
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect()
}
/// signed through an authenticator, as Touch ID does; `flags` its word on the person (0x05: present and verified)
fn touched(mut e: Value, flags: u8) -> Value {
    let cd = json!({ "type": "webauthn.get", "challenge": b64url(&Sha256::digest(signed_bytes(&e).as_bytes())) }).to_string();
    let mut ad = Sha256::digest(b"localhost").to_vec();
    ad.extend([flags, 0, 0, 0, 1]);
    let mut m = ad.clone();
    m.extend(Sha256::digest(cd.as_bytes()));
    let s: Signature = finger().sign(&m);
    e["sig"] = json!({ "key": finger_public(), "sig": s.to_der().as_bytes().iter().map(|x| format!("{x:02x}")).collect::<String>(), "authenticatorData": b64url(&ad), "clientDataJSON": b64url(cd.as_bytes()) });
    e
}

#[test]
fn whether_a_person_was_there_is_kept_never_required_and_a_hold_reads_it() {
    let mac = "Ori › this Mac › Taldo";
    let mut log = vec![
        common::first_key("Ori"),
        common::signed(act(
            1,
            "Ori",
            "Ori",
            "be",
            json!({}),
            json!({ "keys": [{ "key": common::public(&p("Ori")) }, { "key": finger_public() }] }),
        )),
        common::under(901, "Ori › this Mac"),
        common::under(902, "Ori › Claude Code"),
        common::under(2, mac),
        common::signed(act(
            3,
            "Ori › Claude Code",
            "Ori › Claude Code",
            "propose",
            json!({}),
            json!({ "run": "ls" }),
        )),
    ];
    // kept, not required: a plain approval and one with a person there are both taken; one with the person only present, not verified,
    // reads as not there
    log.push(common::signed(act(
        4,
        "Ori",
        "Ori",
        "approve",
        json!({ "re": 3 }),
        json!({ "run": "ls" }),
    )));
    log.push(touched(
        act(
            5,
            "Ori",
            "Ori",
            "approve",
            json!({ "re": 3 }),
            json!({ "run": "ls" }),
        ),
        0x05,
    ));
    log.push(touched(
        act(
            6,
            "Ori",
            "Ori",
            "approve",
            json!({ "re": 3 }),
            json!({ "run": "ls" }),
        ),
        0x01,
    ));
    let t = fold(&log);
    assert!(
        t.entries.contains_key(&4) && t.entries.contains_key(&5) && t.entries.contains_key(&6),
        "the core required a person there"
    );
    assert_eq!(
        (t.present(4), t.present(5), t.present(6)),
        (false, true, false)
    );
    // a key its giver marked as signing only with a person there: what it signs counts as with a person there
    let mut marked = log.clone();
    marked.push(common::signed(act(
        10,
        "Ori",
        "Ori › this Mac",
        "be",
        json!({}),
        json!({ "keys": [{ "key": common::public(&p("Ori › this Mac")), "presence": true }] }),
    )));
    marked.push(common::signed(act(
        11,
        "Ori › this Mac",
        "Ori › this Mac",
        "say",
        json!({}),
        json!({}),
    )));
    assert!(
        fold(&marked).present(11),
        "a presence key's signature did not count as a person there"
    );
    // you hold Taldo on this Mac: what it runs answers an approval made with a person there — the hold is on the one who acts, never on you
    log.push(common::signed_by(&p("Ori"), act(7, "Ori › this Mac", mac, "acceptance", json!({}), json!({ "when": { "is": "ran", "at": p(mac) }, "of": "re", "expect": { "is": "approve", "present": true } }))));
    let t = fold(&log);
    assert_eq!(t.in_force(mac), vec![7]);
    let ran =
        |g: i64, re: i64| common::signed(act(g, mac, mac, "ran", json!({ "re": re }), json!({})));
    assert_eq!(
        t.would_break(&ran(8, 4)),
        vec![7],
        "running the plain approval would not break the hold"
    );
    assert!(
        t.would_break(&ran(8, 5)).is_empty(),
        "running the approval made with a person there would break the hold"
    );
    // and if it ran one anyway, the hold says so, and it is owed
    let mut good = log.clone();
    good.push(ran(8, 5));
    assert_eq!(fold(&good).holds(7), Some(true));
    let mut bad = log.clone();
    bad.push(ran(8, 4));
    let t = fold(&bad);
    assert_eq!(t.holds(7), Some(false));
    assert_eq!(t.owed(), vec![(mac.to_string(), 7)]);
}
