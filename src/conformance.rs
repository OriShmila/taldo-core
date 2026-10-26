//! CONFORMANCE: what every adapter of a seam must do, whatever environment it does it in. An adapter is used only when it passes
//! these, run against it where it runs; the fakes pass them too, so a test on fakes means what it means on the real thing. Each
//! check returns why it failed — never a panic, never an empty answer.
use crate::seams::*;

fn who() -> Who {
    "conformance".to_string()
}
fn ok<T>(a: Answer<T>, what: &str) -> Result<T, String> {
    a.map(|d| d.value).map_err(|f| format!("{what}: {f}"))
}
fn refused<T>(a: Answer<T>, what: &str) -> Result<(), String> {
    match a {
        Err(_) => Ok(()),
        Ok(_) => Err(format!("{what} was not refused")),
    }
}

/// FILES, on an adapter with nothing under `prefix` yet: versions are kept and read back exactly, kept items are written once,
/// watch misses nothing and repeats nothing, a missing item is a Fail, and delete does what its undo says
pub fn files(f: &dyn Files, prefix: &str) -> Result<(), String> {
    let w = who();
    let a = format!("{prefix}a");
    let v1 = ok(f.put(&w, &a, b"one", false), "put a first version")?;
    let v2 = ok(f.put(&w, &a, b"two", false), "put a second version")?;
    if (v1.n, v2.n) != (1, 2) {
        return Err(format!("versions are numbered {} and {}, not 1 and 2", v1.n, v2.n));
    }
    if ok(f.get(&w, &a, None), "get the newest")? != b"two" || ok(f.get(&w, &a, Some(1)), "get version 1")? != b"one" {
        return Err("a version did not read back exactly".into());
    }
    if ok(f.versions(&w, &a), "versions")? != vec![v1.clone(), v2.clone()] {
        return Err("versions did not list both, oldest first".into());
    }
    refused(f.get(&w, &format!("{prefix}missing"), None), "getting an item that is not there")?;
    refused(f.get(&w, &a, Some(9)), "getting a version that is not there")?;
    let k = format!("{prefix}kept");
    ok(f.put(&w, &k, b"program", true), "put a kept item")?;
    refused(f.put(&w, &k, b"other", true), "writing a kept item twice")?;
    let seen = ok(f.watch(&w, prefix, 0), "watch from the start")?;
    let names: Vec<(String, u64)> = seen.iter().map(|c| (c.item.clone(), c.version.n)).collect();
    if names != vec![(a.clone(), 1), (a.clone(), 2), (k.clone(), 1)] {
        return Err(format!("watch saw {names:?}, not every change in order"));
    }
    if !ok(f.watch(&w, prefix, 3), "watch after all")?.is_empty() {
        return Err("watch repeated changes already seen".into());
    }
    let listed = ok(f.list(&w, prefix), "list")?;
    if !(listed.contains(&a) && listed.contains(&k)) {
        return Err(format!("list gave {listed:?}"));
    }
    ok(f.delete(&w, &a), "delete")?;
    match f.delete_undo() {
        Undo::None => refused(f.get(&w, &a, None), "getting a deleted item whose delete has no undo")?,
        Undo::Yes(_) => {}
    }
    Ok(())
}

/// STORE, on an empty adapter: one writer — only the lease's holder appends or writes the state, a second holder is refused until
/// it is released — rows only ever above the last, and every entry read back exactly: by its number, in pages either way, and by
/// its signature
pub fn store(s: &dyn Store) -> Result<(), String> {
    // each entry signed for real, its signature written in upper case: a store finds it by the signature's standard spelling
    let sig = |g: i64| -> String {
        use p256::ecdsa::signature::Signer;
        let k = p256::ecdsa::SigningKey::from_slice(&<sha2::Sha256 as sha2::Digest>::digest(format!("store {g}").as_bytes())).expect("a key");
        let s: p256::ecdsa::Signature = k.sign(b"an entry");
        s.to_bytes().iter().map(|x| format!("{x:02X}")).collect()
    };
    let e = |g: i64| serde_json::json!({ "g": g, "is": "be", "sig": { "sig": sig(g) }, "value": { "n": g } });
    refused(s.append("a", &[e(1)]), "appending with no lease")?;
    ok(s.lease("a"), "take the lease")?;
    ok(s.lease("a"), "the same holder taking it again")?;
    refused(s.lease("b"), "a second writer taking the lease")?;
    refused(s.append("b", &[e(1)]), "appending by one who does not hold the lease")?;
    ok(s.append("a", &[e(1), e(2)]), "append")?;
    ok(s.append("a", &[e(5)]), "append again, with a gap")?;
    refused(s.append("a", &[e(4)]), "appending an entry below the last row")?;
    if s.entry(2) != Some(e(2)) || s.entry(3).is_some() {
        return Err("an entry did not read back exactly by its number".into());
    }
    let gs = |v: Vec<(i64, serde_json::Value)>| v.into_iter().map(|(g, _)| g).collect::<Vec<_>>();
    if gs(s.page(1, 5, false, 10)) != vec![1, 2, 5] || gs(s.page(1, 5, true, 2)) != vec![5, 2] {
        return Err("pages did not come back in order, or past their limit".into());
    }
    let standard = crate::identity::canonical_sig(&sig(2)).expect("a signature");
    if s.signed(&standard) != Some(2) || s.signed(&crate::identity::canonical_sig(&sig(3)).expect("a signature")).is_some() {
        return Err("an entry was not found by its signature's standard spelling (identity::canonical_sig)".into());
    }
    refused(s.put_state("b", b"s"), "writing the state without the lease")?;
    ok(s.put_state("a", b"state 1"), "write the state")?;
    if ok(s.state(), "read the state")?.as_deref() != Some(&b"state 1"[..]) {
        return Err("the state did not read back whole".into());
    }
    refused(s.release("b"), "releasing a lease one does not hold")?;
    ok(s.release("a"), "release")?;
    ok(s.lease("b"), "a new writer after release")?;
    Ok(())
}

/// CLOCK: a timer fires once it is due, once, and not before; `pass` lets time pass by `ms` (the fake moves it, a real clock waits)
pub fn clock(c: &dyn Clock, pass: &dyn Fn(u64)) -> Result<(), String> {
    let t0 = c.now();
    ok(c.at("soon", t0 + 50), "set a timer")?;
    ok(c.at("later", t0 + 10_000), "set a later timer")?;
    if !ok(c.due(), "due before")?.is_empty() {
        return Err("a timer fired before it was due".into());
    }
    pass(60);
    if c.now() < t0 + 50 {
        return Err("time did not pass".into());
    }
    if ok(c.due(), "due")? != vec!["soon".to_string()] {
        return Err("the due timer did not fire, or the later one did".into());
    }
    if !ok(c.due(), "due again")?.is_empty() {
        return Err("a timer fired twice".into());
    }
    Ok(())
}

/// RUN: a job that ends is seen ended with its code, and one a signal killed is seen killed by that signal; one that runs is alive
/// until stopped, and stopped is ended; the adapter lists only what still runs. `ends(code)`, `killed(signal)` and `runs()` are jobs
/// this environment can start
pub fn run(r: &dyn Run, ends: &dyn Fn(i32) -> Job, killed: &dyn Fn(i32) -> Job, runs: &dyn Fn() -> Job, pass: &dyn Fn(u64)) -> Result<(), String> {
    let a = ok(r.start(&ends(3)), "start a job that ends")?;
    let k = ok(r.start(&killed(9)), "start a job that is killed")?;
    let b = ok(r.start(&runs()), "start a job that runs")?;
    pass(200);
    match ok(r.ended(a), "the ended job")? {
        Some(e) if e.code == Some(3) && e.signal.is_none() => {}
        other => return Err(format!("a job that ended with 3 reads {other:?}")),
    }
    match ok(r.ended(k), "the killed job")? {
        Some(e) if e.signal == Some(9) && e.code.is_none() => {}
        other => return Err(format!("a job killed by signal 9 reads {other:?}")),
    }
    if !ok(r.alive(b), "the running job")? || ok(r.ended(b), "the running job's end")?.is_some() {
        return Err("a running job did not read as running".into());
    }
    if !ok(r.running(), "running")?.contains(&b) || ok(r.running(), "running")?.contains(&a) {
        return Err("running did not list exactly what runs".into());
    }
    ok(r.stop(b), "stop")?;
    if ok(r.alive(b), "a stopped job")? || ok(r.ended(b), "a stopped job's end")?.is_none() {
        return Err("a stopped job still runs".into());
    }
    refused(r.alive(999_999), "asking about a job that never was")?;
    Ok(())
}

/// SANDBOX: a level runs exactly the code it is given — `answers` a hash this environment has that answers what it is told, `other`
/// another, `fails` one that cannot go on, each run as `given` says (its seal); replaced by another hash, the other runs; stopped,
/// nothing runs and nothing is told; and a hash it does not have never runs
pub fn sandbox(s: &dyn Sandbox, level: &str, given: &serde_json::Value, answers: &str, other: &str, fails: &str) -> Result<(), String> {
    let changed = serde_json::json!({ "changed": {} });
    refused(s.run(level, "a hash that was never sealed", given), "running code it does not have")?;
    if ok(s.running(level), "running before any")?.is_some() {
        return Err("a level ran before it was given code".into());
    }
    ok(s.run(level, answers, given), "run")?;
    if ok(s.running(level), "running")?.as_deref() != Some(answers) {
        return Err(format!("{level} does not read as running {answers}"));
    }
    ok(s.tell(level, &changed), "tell a running level")?;
    ok(s.run(level, other, given), "run another hash")?;
    if ok(s.running(level), "running after a replace")?.as_deref() != Some(other) {
        return Err(format!("{level} still runs what it ran before {other}"));
    }
    ok(s.run(level, fails, given), "run code that fails")?;
    let failed = s.tell(level, &changed);
    if failed.as_ref().is_ok_and(|d| d.value["answer"].is_null()) {
        return Err("code that cannot go on was told, and said nothing of it".into());
    }
    ok(s.stop(level), "stop")?;
    if ok(s.running(level), "running after stop")?.is_some() {
        return Err("a stopped level still runs".into());
    }
    refused(s.tell(level, &changed), "telling a stopped level")?;
    Ok(())
}

/// NET IN: what is served answers every request whole — its method, path and body reach the handler as sent, the handler's status
/// and body reach the caller as answered — and nothing is answered once stopped. `call` sends a request to the address served, the
/// way a client of this environment would
pub fn net_in(n: &dyn NetIn, call: &dyn Fn(&str, Request) -> Result<Response, String>) -> Result<(), String> {
    let echo: Handler = std::sync::Arc::new(|r: Request| Response {
        status: if r.path == "/missing" { 404 } else { 200 },
        headers: vec![],
        body: format!("{} {} {}", r.method, r.path, String::from_utf8_lossy(&r.body)).into_bytes(),
    });
    let at = ok(n.serve(echo), "serve")?;
    let req = |m: &str, p: &str, b: &str| Request { who: None, method: m.into(), path: p.into(), headers: vec![], body: b.as_bytes().to_vec() };
    let got = call(&at, req("GET", "/alive", "")).map_err(|e| format!("a GET was not answered: {e}"))?;
    if got.status != 200 || got.body != b"GET /alive " {
        return Err(format!("a GET came back {} {:?}", got.status, String::from_utf8_lossy(&got.body)));
    }
    let got = call(&at, req("POST", "/write", "{\"a\":1}")).map_err(|e| format!("a POST was not answered: {e}"))?;
    if got.body != b"POST /write {\"a\":1}" {
        return Err(format!("a POST's body did not arrive whole: {:?}", String::from_utf8_lossy(&got.body)));
    }
    if call(&at, req("GET", "/missing", "")).map_err(|e| format!("a 404 was not answered: {e}"))?.status != 404 {
        return Err("the handler's status did not reach the caller".into());
    }
    ok(n.stop(), "stop")?;
    if call(&at, req("GET", "/alive", "")).is_ok() {
        return Err("a request was answered after stop".into());
    }
    Ok(())
}

/// KEYS: a key made for a name is the same key every time it is asked for; what it signs, the twin takes as signed by the node that
/// holds it — and by nobody else; a key forgotten signs nothing again. (A key that needs a person there is checked with one there:
/// `presence` keys are not signed here)
pub fn keys(k: &dyn Keys, name: &str) -> Result<(), String> {
    use crate::identity::signed_bytes;
    let public = ok(k.key(name, false), "make a key")?;
    if ok(k.key(name, false), "ask for it again")? != public {
        return Err("asking for a key again gave another key".into());
    }
    // a world where the root holds this key: an act signed with it is taken; one by a node it does not sign for is not
    let mut act = serde_json::json!({ "g": 1, "ts": 1, "by": "Root", "at": "Root", "is": "say", "refs": {}, "value": { "says": "signed" } });
    act["sig"] = ok(k.sign(name, signed_bytes(&act).as_bytes(), "a conformance test"), "sign")?;
    let first = serde_json::json!({ "g": 0, "ts": 0, "by": "Root", "at": "Root", "is": "be", "refs": {}, "value": { "keys": [{ "key": public }] } });
    let t = crate::twin::fold(&[first, act.clone()]);
    if !t.entries.contains_key(&1) {
        return Err(format!("what the key signed was not taken as signed by its node: {:?}", t.rejected));
    }
    let mut other = act.clone();
    other["value"] = serde_json::json!({ "says": "changed after signing" });
    if crate::identity::verify(&t, &other, i64::MAX).is_ok() {
        return Err("a signature held for words it did not sign".into());
    }
    ok(k.forget(name), "forget")?;
    refused(k.sign(name, b"x", "after forgetting"), "signing with a forgotten key")?;
    // A SECRET: kept, kept again in its place, gone when forgotten — and forgetting what is not kept is a Fail
    let secret = format!("{name} secret");
    if ok(k.has_secret(&secret), "ask for a secret not kept")? {
        return Err("a secret never kept was said to be kept".into());
    }
    ok(k.keep_secret(&secret, b"one"), "keep a secret")?;
    ok(k.keep_secret(&secret, b"two"), "keep it again")?;
    if !ok(k.has_secret(&secret), "ask for the secret kept")? {
        return Err("a secret kept was not said to be kept".into());
    }
    ok(k.forget_secret(&secret), "forget the secret")?;
    if ok(k.has_secret(&secret), "ask for the secret forgotten")? {
        return Err("a forgotten secret was still kept".into());
    }
    refused(k.forget_secret(&secret), "forgetting a secret not kept")?;
    Ok(())
}

/// AGENT: a session started with words says something back and ends its turn; more words are its next turn, heard the same way; a
/// stopped session takes no more words, and says it ended. `words` are words this runtime answers; `pass` lets time pass while it works
pub fn agent(a: &dyn Agent, dir: &str, words: &str, pass: &dyn Fn(u64)) -> Result<(), String> {
    let heard: std::sync::Arc<std::sync::Mutex<Vec<(String, Heard)>>> = Default::default();
    let h = heard.clone();
    let hear: Hear = std::sync::Arc::new(move |id: &str, x: Heard| h.lock().unwrap().push((id.to_string(), x)));
    let hear2 = hear.clone();
    let id = ok(a.start(dir, words, hear), "start a session")?;
    let turn = |what: &str, from: usize| -> Result<usize, String> {
        for _ in 0..1200 {
            let got = heard.lock().unwrap();
            let mine: Vec<&Heard> = got.iter().skip(from).filter(|(i, _)| *i == id).map(|(_, x)| x).collect();
            if mine.iter().any(|x| **x == Heard::TurnEnded) {
                if !mine.iter().any(|x| matches!(x, Heard::Said(s) if !s.is_empty())) {
                    return Err(format!("{what}: the turn ended with nothing said"));
                }
                return Ok(got.len());
            }
            drop(got);
            pass(100);
        }
        Err(format!("{what}: the turn did not end"))
    };
    let after = turn("the first turn", 0)?;
    ok(a.say(&id, words), "more words")?;
    turn("the next turn", after)?;
    ok(a.stop(&id), "stop")?;
    if !heard.lock().unwrap().iter().any(|(i, x)| *i == id && matches!(x, Heard::Ended(_))) {
        return Err("a stopped session did not say it ended".into());
    }
    refused(a.say(&id, words), "words to a stopped session")?;
    // TAKEN UP AGAIN, as after a restart: its next turn heard, and it takes words again
    let before = heard.lock().unwrap().len();
    ok(a.resume(&id, dir, words, hear2.clone()), "take a session up again")?;
    turn("the turn after taking it up again", before)?;
    ok(a.stop(&id), "stop it again")?;
    refused(a.say("no such session", words), "words to a session that never was")?;
    Ok(())
}

/// SOURCE, on one whose `put` places a file at a path: a folder lists what is in it (files with their size, folders as folders), a
/// file reads back exactly, and what is not there — or outside the source, or a folder read as a file — is a Fail
pub fn source(s: &dyn Source, put: &dyn Fn(&str, &[u8])) -> Result<(), String> {
    let w = who();
    put("a/b.txt", b"hi");
    put("c.txt", b"three");
    let top = ok(s.list(&w, ""), "list the top")?;
    let find = |l: &[Listed], n: &str| l.iter().find(|x| x.name == n).cloned();
    match (find(&top, "a"), find(&top, "c.txt")) {
        (Some(a), Some(c)) if a.folder && !c.folder && c.bytes == 5 && c.changed > 0 => {}
        _ => return Err(format!("the top listed {top:?}: not the folder a and the file c.txt of 5 bytes, with when it changed")),
    }
    let inner = ok(s.list(&w, "a"), "list a folder")?;
    if inner.len() != 1 || inner[0].name != "b.txt" || inner[0].bytes != 2 {
        return Err(format!("the folder a listed {inner:?}, not b.txt of 2 bytes"));
    }
    if ok(s.read(&w, "a/b.txt"), "read a file")? != b"hi" {
        return Err("a file did not read back exactly".into());
    }
    refused(s.read(&w, "a/missing"), "reading a file that is not there")?;
    refused(s.list(&w, "missing"), "listing a folder that is not there")?;
    refused(s.read(&w, "a"), "reading a folder as a file")?;
    refused(s.read(&w, "../c.txt"), "reading outside the source")?;
    refused(s.list(&w, ".."), "listing outside the source")?;
    Ok(())
}

/// A MOVEMENT FOR THE TESTS: for `who`, on `place`, a read
fn moving(place: &str) -> Move {
    Move { who: who(), on: place.into(), is: "read".into(), says: "a conformance call".into() }
}

/// NET OUT, against `at` — a server whose answer to every request is the request's own `Authorization` header and body, as its body,
/// and which counts what reached it (`reached`): a call reaches it whole with its method and body; with a secret, the secret Keys
/// keeps arrives put in as asked (a bearer token here), and a secret not kept is a Fail, never a call without it. A MOVEMENT is
/// asked of the record before (on a place the record refuses — "refused" — nothing reaches the server, and the twin's why is the
/// Fail) and handed to it after (`happened`: the movements the record took); a secret with no movement never goes out. `keep` keeps a
/// secret under a name
pub fn net_out(n: &dyn NetOut, at: &str, keep: &dyn Fn(&str, &[u8]), reached: &dyn Fn() -> usize, happened: &dyn Fn() -> Vec<Move>) -> Result<(), String> {
    let req = |m: &str, b: &str| Request { who: None, method: m.into(), path: "/".into(), headers: vec![], body: b.as_bytes().to_vec() };
    let got = ok(n.call(None, &format!("{at}/x?y=1"), req("POST", "hello"), None), "a call with no secret")?;
    if got.status != 200 || !String::from_utf8_lossy(&got.body).ends_with("hello") {
        return Err(format!("a call came back {} {:?}", got.status, String::from_utf8_lossy(&got.body)));
    }
    keep("conformance out", b"s3cret");
    let m = moving("an account");
    let got = ok(n.call(Some(&m), &format!("{at}/x"), req("GET", ""), Some(("conformance out", Auth::Bearer))), "a movement with a secret")?;
    if !String::from_utf8_lossy(&got.body).contains("Bearer s3cret") {
        return Err(format!("the secret did not arrive as a bearer token: {:?}", String::from_utf8_lossy(&got.body)));
    }
    if happened() != vec![m.clone()] {
        return Err(format!("the movement that happened was not handed to the record: {:?}", happened()));
    }
    refused(n.call(Some(&m), &format!("{at}/x"), req("GET", ""), Some(("never kept", Auth::Bearer))), "a call with a secret never kept")?;
    let before = reached();
    refused(n.call(Some(&moving("refused")), &format!("{at}/x"), req("GET", ""), Some(("conformance out", Auth::Bearer))), "a movement the record refuses")?;
    refused(n.call(None, &format!("{at}/x"), req("GET", ""), Some(("conformance out", Auth::Bearer))), "a secret with no movement")?;
    if reached() != before {
        return Err("a movement the record refused, or a secret with no movement, reached the far side".into());
    }
    if happened().len() != 1 {
        return Err(format!("a movement that did not happen was handed to the record: {:?}", happened()));
    }
    Ok(())
}

/// MESSAGE: it says what a device needs to let it reach it; sending to `to` (an address a device gave) is done, with words or with
/// none; an address that is none is a Fail, never a message lost in silence
pub fn message(m: &dyn Message, to: &serde_json::Value) -> Result<(), String> {
    let w = who();
    if ok(m.address(), "its address")?.is_empty() {
        return Err("it said no address".into());
    }
    ok(m.send(&w, to, None), "reach a device with nothing of the person's")?;
    ok(m.send(&w, to, Some("something waits")), "reach a device with words")?;
    refused(m.send(&w, &serde_json::json!({}), None), "sending to no address")?;
    Ok(())
}

/// RENDER, given `at` — the address of a page that shows "Hello from the page" and, with `script` setting `window.SAID`, shows what it
/// set too — and `nowhere`, an address nothing answers: the page is drawn as a picture with its words, the script given runs in it
/// first, and a page that is not there is a Fail
pub fn render(r: &dyn Render, at: &str, nowhere: &str) -> Result<(), String> {
    let w = who();
    let d = ok(r.draw(&w, at, 390, 844, None), "draw a page")?;
    if d.png.is_empty() || !d.text.contains("Hello from the page") {
        return Err(format!("the page was drawn without its picture or its words: {} bytes, {:?}", d.png.len(), d.text));
    }
    let d = ok(r.draw(&w, at, 390, 844, Some("window.SAID = 'said by the script'")), "draw a page with a script")?;
    if !d.text.contains("said by the script") {
        return Err(format!("the script given did not run in the page first: {:?}", d.text));
    }
    refused(r.draw(&w, nowhere, 390, 844, None), "drawing a page that is not there")?;
    Ok(())
}

/// MODEL: asked to say a word back, it answers with it, and the movement is handed to the record (`happened`); asked nothing, it is a
/// SOURCE, ACTING FOR SOMEONE: asked `what` with `input`, it answers as `answered` expects — a movement asked of the record before and
/// handed to it after; nothing asked, an act it does not do, or a movement the record refuses: a Fail, and nothing handed to the record
pub fn source_act(s: &dyn Source, what: &str, input: &serde_json::Value, answered: &dyn Fn(&serde_json::Value) -> bool, happened: &dyn Fn() -> Vec<Move>) -> Result<(), String> {
    let mv = Move { is: "write".into(), ..moving("a source") };
    let a = ok(s.act(&mv, what, input), what)?;
    if !answered(&a) {
        return Err(format!("asked {what}, it answered {a}"));
    }
    if happened() != vec![mv.clone()] {
        return Err(format!("the movement was not handed to the record: {:?}", happened()));
    }
    refused(s.act(&mv, what, &serde_json::json!({})), "asking nothing")?;
    refused(s.act(&mv, "nothing it does", input), "an act it does not do")?;
    refused(s.act(&Move { is: "write".into(), ..moving("refused") }, what, input), "a movement the record refuses")?;
    if happened().len() != 1 {
        return Err(format!("a movement that did not happen was handed to the record: {:?}", happened()));
    }
    Ok(())
}

/// BACKUP, on one whose `raw` reads what its place holds for a name as it is there: what is kept reads back exactly; the place never
/// holds it plain; kept again, the newest; listed under its prefix; a name never kept is a Fail
pub fn backup(b: &dyn Backup, raw: &dyn Fn(&str) -> Option<Vec<u8>>) -> Result<(), String> {
    let (name, plain) = ("record/000001", b"the person's record, entry by entry".to_vec());
    ok(b.keep(name, &plain), "keep")?;
    if ok(b.restore(name), "restore")? != plain {
        return Err("what was kept did not read back exactly".into());
    }
    let held = raw(name).ok_or("the place holds nothing for what was kept")?;
    if held.windows(8).any(|w| plain.windows(8).any(|p| p == w)) {
        return Err("the place holds what was kept plain".into());
    }
    ok(b.keep(name, b"newer"), "keep again")?;
    if ok(b.restore(name), "restore again")? != b"newer" {
        return Err("kept again, the newest did not read back".into());
    }
    if !ok(b.list("record/"), "list")?.iter().any(|n| n == name) {
        return Err("what was kept is not listed under its prefix".into());
    }
    refused(b.restore("record/never"), "restoring what was never kept")
}

/// IDENTITY: a fields kind is proven when every field is filled — its secrets kept at the edge (`kept` says it is), never in what is
/// handed back — and refused when one is missing; a way the adapter does not prove is refused, never guessed
pub fn identity(i: &dyn Identity, kept: &dyn Fn(&str) -> bool) -> Result<(), String> {
    let kind = Kind { name: "conformance".into(), way: serde_json::json!({ "way": "fields", "fields": [{ "name": "user" }, { "name": "token", "secret": true }] }) };
    if ok(i.begin(&kind), "begin a fields kind")?.is_null() {
        return Err("begin said nothing to do".into());
    }
    let p = ok(i.prove(&kind, "conformance account", &serde_json::json!({ "user": "u", "token": "T0KEN-conformance" })), "prove with every field")?;
    if format!("{p:?}").contains("T0KEN-conformance") {
        return Err("a secret was handed back".into());
    }
    if p.kept.is_empty() || !p.kept.iter().all(|k| kept(k)) {
        return Err(format!("the secret is not kept at the edge: {:?}", p.kept));
    }
    refused(i.prove(&kind, "conformance account", &serde_json::json!({ "user": "u" })), "proving with a field missing")?;
    let unknown = Kind { name: "none".into(), way: serde_json::json!({ "way": "nonsense" }) };
    refused(i.prove(&unknown, "x", &serde_json::json!({})), "proving a way it does not prove")
}
