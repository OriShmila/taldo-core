//! A REQUEST SIGNED AS AN ACT: who makes a request is proven as who wrote an entry is — and a request can be neither changed nor
//! replayed nor made by a key that does not sign for its node.
mod common;
use taldo_core::identity::requested_by;
use taldo_core::twin::fold;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn p(s: &str) -> String {
    s.to_string()
}
fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
fn request(by: &str, method: &str, path: &str, body: &[u8], ts: i64) -> Value {
    common::signed(json!({ "g": 0, "ts": ts, "arrived": ts, "by": p(by), "at": p(by), "is": "request", "refs": {},
        "value": { "method": method, "path": path, "body": hex(&Sha256::digest(body)) } }))
}

#[test]
fn a_signed_request_proves_who_made_it_and_nothing_else() {
    // Ori, and a device of Ori's given its key
    let t = fold(&[
        common::first_key("Ori"),
        common::under(1, "Ori › devices"),
        common::under(2, "Ori › devices › laptop"),
    ]);
    let none = |_: &str| false;
    let r = request("Ori › devices › laptop", "POST", "/write", b"{\"a\":1}", 1000);
    assert_eq!(requested_by(&t, &r, "POST", "/write", b"{\"a\":1}", 1500, 60_000, &none), Ok(p("Ori › devices › laptop")));
    // another request, another body, too old, replayed: refused
    assert!(requested_by(&t, &r, "POST", "/other", b"{\"a\":1}", 1500, 60_000, &none).is_err(), "it held for another path");
    assert!(requested_by(&t, &r, "POST", "/write", b"{\"a\":2}", 1500, 60_000, &none).is_err(), "it held for another body");
    assert!(requested_by(&t, &r, "POST", "/write", b"{\"a\":1}", 100_000, 60_000, &none).is_err(), "it held long after");
    let sig = r["sig"]["sig"].as_str().unwrap().to_string();
    assert!(requested_by(&t, &r, "POST", "/write", b"{\"a\":1}", 1500, 60_000, &|s| s == sig).is_err(), "it held replayed");
    // a node with no key given: refused
    let stranger = request("Ori › devices › stolen", "GET", "/", b"", 1000);
    assert!(requested_by(&t, &stranger, "GET", "/", b"", 1500, 60_000, &none).is_err(), "a node with no key was taken");
}
