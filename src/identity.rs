//! IDENTITY — each part has its own identity, and every act is verified as its own.
//!
//! An entry is from its node (`by`) only if it is signed by a key that node, or a node above it, holds — or, for a responsibility,
//! a key of whoever carries it now (`carried_by`). A node's keys are its detail
//! `keys` — `[{ "key": <public key, SEC1 hex> }]` — written at it by itself or a node above it. What is signed is the entry's own act:
//! `by`, `at`, `is`, `refs`, `value` and `ts` — when the writer acted, by its own clock — every object's keys in order. The same act
//! done twice is two acts; the same signed act twice is one. A signature is either plain (r‖s over those bytes) or from a
//! platform authenticator, a passkey (its data and the hash of a client's, whose challenge is the hash of those bytes).
//! The one entry taken unsigned is the root's first key: nobody is above the root to vouch for it.
//! WHETHER A PERSON WAS THERE is kept as a fact of each entry — an authenticator saying its user was present and verified, or a key its
//! giver marked `presence` (one that signs only with a person there: a passkey) — for a hold to
//! read. The core never requires it: what needs a person there is decided by holds, not here.
//! GETTING BACK IN is the twin's take-over (twin.rs): a way back the person granted signs a `takeover` that brings their new key.
use crate::entry::{ids_of, parse_id};
use crate::twin::Twin;
use p256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::Mutex;

/// THE ACT, as signed: by, at, is, refs, value, ts — every object's keys in order
pub fn signed_bytes(e: &Value) -> String {
    fn sorted(v: &Value) -> Value {
        match v {
            Value::Object(o) => {
                let mut keys: Vec<&String> = o.keys().collect();
                keys.sort();
                Value::Object(
                    keys.into_iter()
                        .map(|k| (k.clone(), sorted(&o[k])))
                        .collect(),
                )
            }
            Value::Array(a) => Value::Array(a.iter().map(sorted).collect()),
            x => x.clone(),
        }
    }
    let mut act = Map::new();
    for f in ["by", "at", "is", "refs", "value", "ts"] {
        act.insert(f.into(), e[f].clone());
    }
    sorted(&Value::Object(act)).to_string()
}
fn unhex(s: &str) -> Option<Vec<u8>> {
    (s.len() % 2 == 0)
        .then(|| {
            (0..s.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok())
                .collect::<Option<Vec<u8>>>()
        })
        .flatten()
}
fn unb64(s: &str) -> Option<Vec<u8>> {
    let (mut out, mut acc, mut bits) = (vec![], 0u32, 0);
    for c in s.trim_end_matches('=').chars() {
        let v = match c {
            'A'..='Z' => c as u32 - 65,
            'a'..='z' => c as u32 - 71,
            '0'..='9' => c as u32 + 4,
            '-' | '+' => 62,
            '_' | '/' => 63,
            _ => return None,
        };
        acc = acc << 6 | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
        }
    }
    Some(out)
}
pub fn b64url(b: &[u8]) -> String {
    const A: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut out = String::new();
    for c in b.chunks(3) {
        let n = (c[0] as u32) << 16
            | (*c.get(1).unwrap_or(&0) as u32) << 8
            | *c.get(2).unwrap_or(&0) as u32;
        for i in 0..=c.len() {
            out.push(A[(n >> (18 - 6 * i) & 63) as usize] as char);
        }
    }
    out
}

/// A SIGNATURE IN ITS ONE STANDARD SPELLING: the same signature can be written several ways that all verify — upper or lower case
/// hex, DER or the plain r‖s, and its mirror (r, n−s) — so what was already taken is recognised by this spelling, never by the text
/// as written: decoded whatever its form, its s made the low one, written as plain r‖s in lower-case hex. None for what is no
/// signature at all
pub fn canonical_sig(sig: &str) -> Option<String> {
    let bytes = unhex(sig)?;
    // read as the signer reads it: DER first, then the plain r‖s — so a DER signature that happens to be 64 bytes is read the same here
    let s = Signature::from_der(&bytes).or_else(|_| Signature::from_slice(&bytes)).ok()?;
    Some(b64hex(&s.normalize_s().to_bytes()))
}
/// THE KEY THAT SIGNED the entry's act, and whether a person was there — or why none did
fn signer(e: &Value) -> Result<(String, bool), String> {
    let p = &e["sig"];
    let key = p["key"].as_str().ok_or("it is not signed")?.to_string();
    let k = VerifyingKey::from_sec1_bytes(&unhex(&key).ok_or("its key is not hex")?)
        .map_err(|_| "its key is not a key")?;
    let sig = unhex(p["sig"].as_str().unwrap_or("")).ok_or("its signature is not hex")?;
    let act = signed_bytes(e);
    match (
        p["authenticatorData"].as_str(),
        p["clientDataJSON"].as_str(),
    ) {
        (Some(ad), Some(cd)) => {
            let (ad, cd) = (
                unb64(ad).ok_or("authenticatorData is not base64")?,
                unb64(cd).ok_or("clientDataJSON is not base64")?,
            );
            let client: Value =
                serde_json::from_slice(&cd).map_err(|_| "clientDataJSON is not JSON")?;
            if client["challenge"].as_str() != Some(&b64url(&Sha256::digest(act.as_bytes()))) {
                return Err("the authenticator signed something else".into());
            }
            let s = Signature::from_der(&sig)
                .or_else(|_| Signature::from_slice(&sig))
                .map_err(|_| "its signature is not a signature")?;
            // the authenticator's own word: its user was present (UP) and verified (UV)
            let present = ad.get(32).is_some_and(|f| f & 0x05 == 0x05);
            let mut signed = ad;
            signed.extend(Sha256::digest(&cd));
            k.verify(&signed, &s)
                .map_err(|_| "the signature does not match")?;
            Ok((key, present))
        }
        _ => {
            let s = Signature::from_slice(&sig).map_err(|_| "its signature is not a signature")?;
            k.verify(act.as_bytes(), &s)
                .map_err(|_| "the signature does not match")?;
            Ok((key, false))
        }
    }
}
/// signatures already verified, with what they signed and whether a person was there: each checked once however often a log is folded
static VERIFIED: Mutex<Option<BTreeMap<(String, String), (String, bool)>>> = Mutex::new(None);
const VERIFIED_AT_MOST: usize = 4_096;

/// WHO SIGNED AN ENTRY, as admission found it: the node whose key signed (`signer`) — itself, a node above it, or one carrying it —
/// and whether a person was there. Every rule of who acted reads this, never the `by` an entry says, nor the keys a node has today
#[derive(Clone, Debug, PartialEq)]
pub struct Signed {
    pub signer: String,
    pub present: bool,
}
/// THE NODES WHOSE KEYS SIGN FOR A NODE — the one answer to it, for entries and requests alike: itself and every node above it, and
/// whoever carries any of them now, with every node above them — nearest first; never one blocked, nor one deleted — but a person who
/// deleted themselves, answering that very delete (`own_undo`: `Twin::own_undo`), signs for themselves
pub fn signing_nodes(t: &Twin, by: &str, own_undo: bool) -> Vec<String> {
    let mut nodes = t.chain(by);
    for x in nodes.clone() {
        for c in t.nodes.get(&x).and_then(|n| n.who.get("carried_by")).map(ids_of).unwrap_or_default() {
            nodes.extend(t.chain(&c).into_iter().filter(|y| !nodes.contains(y)).collect::<Vec<_>>());
        }
    }
    nodes.retain(|x| !t.blocked(x) && (!t.deleted(x) || (own_undo && x == by)));
    nodes
}
/// A NODE'S OWN KEYS, when an act arrived: as decided from above, and a take-over's once it waited — each with its node, and whether
/// its giver marked it a presence key
fn own_keys(t: &Twin, x: &str, arrived: i64) -> Vec<(String, bool, String)> {
    t.nodes.get(x).and_then(|n| n.who.get("keys")).and_then(Value::as_array).cloned().unwrap_or_default().into_iter().chain(t.taken_over(x, arrived))
        .filter_map(|k| Some((k["key"].as_str()?.to_string(), k["presence"] == true, x.to_string())))
        .collect()
}
/// THE KEYS THAT SIGN FOR A NODE, each with whether its giver marked it a presence key: every signing node's own (`signing_nodes`)
pub fn keys_for(t: &Twin, by: &str, arrived: i64) -> Vec<(String, bool)> {
    signers(t, by, arrived, false).into_iter().map(|(k, p, _)| (k, p)).collect()
}
fn signers(t: &Twin, by: &str, arrived: i64, undo: bool) -> Vec<(String, bool, String)> {
    signing_nodes(t, by, undo).iter().flat_map(|x| own_keys(t, x, arrived)).collect()
}
/// IDENTITY: the entry is from its node — or why not; and whether a person was there when it was signed
pub fn verify(t: &Twin, e: &Value, arrived: i64) -> Result<Signed, String> {
    let by = parse_id(&e["by"]).ok_or("by is not a node")?;
    // A TAKE-OVER is signed by the way back it names — with that way's own key only, never a node above it or carrying it — when its
    // person granted that very way
    let keys = match parse_id(&e["refs"]["proof"]).filter(|_| e["is"] == "takeover") {
        Some(p) if t.takeover_granted(&by, &p, arrived) => own_keys(t, &p, arrived),
        Some(p) => return Err(format!("{p} is not a way back {by} granted")),
        None => signers(t, &by, arrived, t.own_undo(e)),
    };
    // A PERSON'S FIRST KEY: nobody is above them and nothing knows them yet — signed with a key it names (whether they may come to be
    // is the twin's, `admit`); unsigned only at the record's very beginning, where nothing can be asked of it yet
    let first = !t.nodes.contains_key(&by)
        && parse_id(&e["at"]).as_deref() == Some(by.as_str())
        && e["is"] == "be"
        && e["value"]["keys"].is_array()
        && keys.is_empty();
    if first && e["sig"].is_null() && t.nodes.is_empty() {
        return Ok(Signed { signer: by, present: false });
    }
    let seen = (
        e["sig"]["sig"].as_str().unwrap_or("").to_string(),
        signed_bytes(e),
    );
    let known = VERIFIED
        .lock()
        .unwrap()
        .as_ref()
        .and_then(|v| v.get(&seen).cloned());
    let (k, present) = match known {
        Some(x) => x,
        None => signer(e)?,
    };
    let Some((_, marked, signer)) = keys
        .iter()
        .find(|(x, _, _)| *x == k)
        .cloned()
        .or((first && e["value"]["keys"].as_array().is_some_and(|ks| ks.iter().any(|x| x["key"] == k.as_str()))).then(|| (k.clone(), false, by.clone())))
    else {
        // A KEY OF A NODE BLOCKED OR DELETED, said as that
        let held = t.chain(&by).into_iter().find(|x| t.nodes.get(x).and_then(|n| n.who.get("keys")).and_then(Value::as_array).is_some_and(|ks| ks.iter().any(|y| y["key"] == k.as_str())));
        return Err(match held {
            Some(x) if t.blocked(&x) => format!("{x} is blocked: its key signs nothing from then on"),
            Some(x) if t.deleted(&x) => format!("{x} is deleted: its key signs nothing"),
            _ => format!("that key does not sign for {by}"),
        });
    };
    let mut v = VERIFIED.lock().unwrap();
    let v = v.get_or_insert_with(BTreeMap::new);
    // KEPT FOR A WHILE, NEVER FOR EVER: a log folded again finds its signatures checked; a twin that lives on (and keeps its entries
    // as rows) never holds every signature it ever checked
    if v.len() >= VERIFIED_AT_MOST {
        v.clear();
    }
    v.insert(seen, (k, present));
    Ok(Signed { signer, present: present || marked })
}

/// A REQUEST SIGNED AS AN ACT: a screen or an agent proves who makes a request the way every entry proves who wrote it — the act
/// `{ by, at: by, is: "request", value: { method, path, body (its sha256, hex) }, ts }` signed with a key that signs for `by`. Nothing
/// is recorded for it; the same rule holds it. Taken only when the act names this very request, was made within `window` ms of
/// `now` (either way), and its signature, in its standard spelling (`canonical_sig`), is not one of `seen` — so a request can be
/// neither changed nor replayed, however its signature is written. Who made it — or why not
pub fn requested_by(
    t: &Twin,
    act: &Value,
    method: &str,
    path: &str,
    body: &[u8],
    now: i64,
    window: i64,
    seen: &dyn Fn(&str) -> bool,
) -> Result<String, String> {
    if act["is"] != "request" || act["at"] != act["by"] {
        return Err("it is not a request signed as one".into());
    }
    let v = &act["value"];
    if v["method"] != method || v["path"] != path || v["body"] != b64hex(&Sha256::digest(body)) {
        return Err("the signed request is not this request".into());
    }
    let ts = act["ts"].as_i64().ok_or("the signed request says no time")?;
    if (now - ts).abs() > window {
        return Err(format!("the signed request is from {ts}, not within {window} ms of now ({now})"));
    }
    let sig = act["sig"]["sig"].as_str().ok_or("the request is not signed")?;
    // SEEN BY ITS STANDARD SPELLING: the same request signed once, written another way, is the same request
    let sig = canonical_sig(sig).ok_or("its signature is not a signature")?;
    if seen(&sig) {
        return Err("this signed request was made already".into());
    }
    verify(t, act, now)?;
    parse_id(&act["by"]).ok_or_else(|| "by is not a node".into())
}
fn b64hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
