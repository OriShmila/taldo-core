//! FAKE ADAPTERS, in memory: what tests run the app against — no process, no disk, no port, no wall clock. Each passes the same
//! conformance tests as a real adapter (`conformance`), so a test that holds on them holds on the real ones. Time is simulated:
//! `FakeClock::advance` moves it, and a timer fires at once.
use crate::seams::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::Mutex;

fn done<T>(value: T) -> Answer<T> {
    Ok(Done { value, cost: Cost::default() })
}
fn fail<T>(seam: &'static str, call: &'static str, why: String) -> Answer<T> {
    Err(Fail { seam, call, why, retry: None })
}
fn sha(b: &[u8]) -> String {
    Sha256::digest(b).iter().map(|x| format!("{x:02x}")).collect()
}

// ─── Files ───

#[derive(Default)]
pub struct FakeFiles {
    items: Mutex<BTreeMap<String, (bool, Vec<Vec<u8>>)>>,
    changes: Mutex<Vec<Change>>,
}
impl Files for FakeFiles {
    fn put(&self, _: &Who, item: &str, bytes: &[u8], kept: bool) -> Answer<Revision> {
        let mut items = self.items.lock().unwrap();
        let e = items.entry(item.to_string()).or_insert((kept, vec![]));
        if e.0 && !e.1.is_empty() {
            return fail("files", "put", format!("{item} is kept: it is written once"));
        }
        e.1.push(bytes.to_vec());
        let v = Revision { n: e.1.len() as u64, sha256: sha(bytes) };
        self.changes.lock().unwrap().push(Change { item: item.to_string(), version: v.clone() });
        done(v)
    }
    fn get(&self, _: &Who, item: &str, n: Option<u64>) -> Answer<Vec<u8>> {
        let items = self.items.lock().unwrap();
        let Some((_, vs)) = items.get(item) else { return fail("files", "get", format!("{item} is not here")) };
        let i = n.unwrap_or(vs.len() as u64);
        match vs.get((i as usize).wrapping_sub(1)) {
            Some(b) => done(b.clone()),
            None => fail("files", "get", format!("{item} has no version {i}")),
        }
    }
    fn versions(&self, _: &Who, item: &str) -> Answer<Vec<Revision>> {
        let items = self.items.lock().unwrap();
        let Some((_, vs)) = items.get(item) else { return fail("files", "versions", format!("{item} is not here")) };
        done(vs.iter().enumerate().map(|(i, b)| Revision { n: i as u64 + 1, sha256: sha(b) }).collect())
    }
    fn list(&self, _: &Who, prefix: &str) -> Answer<Vec<String>> {
        done(self.items.lock().unwrap().keys().filter(|k| k.starts_with(prefix)).cloned().collect())
    }
    fn watch(&self, _: &Who, prefix: &str, after: u64) -> Answer<Vec<Change>> {
        let under: Vec<Change> = self.changes.lock().unwrap().iter().filter(|c| c.item.starts_with(prefix)).cloned().collect();
        done(under.into_iter().skip(after as usize).collect())
    }
    fn delete_undo(&self) -> Undo {
        Undo::None
    }
    fn delete(&self, _: &Who, item: &str) -> Answer<()> {
        match self.items.lock().unwrap().remove(item) {
            Some(_) => done(()),
            None => fail("files", "delete", format!("{item} is not here")),
        }
    }
}

// ─── Store: the twin's rows, by number and by signature ───

#[derive(Default)]
pub struct FakeStore {
    holder: Mutex<Option<String>>,
    rows: Mutex<BTreeMap<i64, serde_json::Value>>,
    state: Mutex<Option<Vec<u8>>>,
}
impl FakeStore {
    fn holds(&self, holder: &str, call: &'static str) -> Result<(), Fail> {
        match self.holder.lock().unwrap().as_deref() {
            Some(h) if h == holder => Ok(()),
            Some(h) => Err(Fail { seam: "store", call, why: format!("{h} holds the writer's lease, not {holder}"), retry: None }),
            None => Err(Fail { seam: "store", call, why: format!("nobody holds the writer's lease; {holder} must take it first"), retry: None }),
        }
    }
}
impl crate::rows::Source for FakeStore {
    fn entry(&self, g: i64) -> Option<serde_json::Value> {
        self.rows.lock().unwrap().get(&g).cloned()
    }
    fn page(&self, from: i64, to: i64, newest_first: bool, limit: usize) -> Vec<(i64, serde_json::Value)> {
        let rows = self.rows.lock().unwrap();
        let it = rows.range(from..=to).map(|(g, v)| (*g, v.clone()));
        if newest_first { it.rev().take(limit).collect() } else { it.take(limit).collect() }
    }
    fn signed(&self, sig: &str) -> Option<i64> {
        // found by the standard spelling of each kept entry's signature, as every source finds it
        self.rows.lock().unwrap().iter().find(|(_, v)| v["sig"]["sig"].as_str().and_then(crate::identity::canonical_sig).as_deref() == Some(sig)).map(|(g, _)| *g)
    }
}
impl Store for FakeStore {
    fn lease(&self, holder: &str) -> Answer<()> {
        let mut h = self.holder.lock().unwrap();
        match h.as_deref() {
            Some(x) if x != holder => fail("store", "lease", format!("{x} holds the writer's lease")),
            _ => {
                *h = Some(holder.to_string());
                done(())
            }
        }
    }
    fn release(&self, holder: &str) -> Answer<()> {
        self.holds(holder, "release")?;
        *self.holder.lock().unwrap() = None;
        done(())
    }
    fn append(&self, holder: &str, entries: &[serde_json::Value]) -> Answer<i64> {
        self.holds(holder, "append")?;
        let mut rows = self.rows.lock().unwrap();
        let mut top = rows.keys().last().copied().unwrap_or(i64::MIN);
        for e in entries {
            let Some(g) = e["g"].as_i64() else { return fail("store", "append", format!("an entry with no number: {e}")) };
            if g <= top {
                return fail("store", "append", format!("entry {g} is not above every row kept ({top})"));
            }
            top = g;
        }
        for e in entries {
            rows.insert(e["g"].as_i64().unwrap(), e.clone());
        }
        done(top)
    }
    fn state(&self) -> Answer<Option<Vec<u8>>> {
        done(self.state.lock().unwrap().clone())
    }
    fn put_state(&self, holder: &str, state: &[u8]) -> Answer<()> {
        self.holds(holder, "put_state")?;
        *self.state.lock().unwrap() = Some(state.to_vec());
        done(())
    }
}

// ─── Clock: simulated time ───

#[derive(Default)]
pub struct FakeClock {
    now: Mutex<u64>,
    timers: Mutex<BTreeMap<String, u64>>,
}
impl FakeClock {
    /// move simulated time on by `ms`
    pub fn advance(&self, ms: u64) {
        *self.now.lock().unwrap() += ms;
    }
}
impl Clock for FakeClock {
    fn now(&self) -> u64 {
        *self.now.lock().unwrap()
    }
    fn at(&self, name: &str, at: u64) -> Answer<()> {
        self.timers.lock().unwrap().insert(name.to_string(), at);
        done(())
    }
    fn due(&self) -> Answer<Vec<String>> {
        let now = self.now();
        let mut t = self.timers.lock().unwrap();
        let due: Vec<String> = t.iter().filter(|(_, at)| **at <= now).map(|(n, _)| n.clone()).collect();
        for n in &due {
            t.remove(n);
        }
        done(due)
    }
}

// ─── Sandbox: levels whose code answers by its hash — "echo" (and "echo again") say back what they were told, "fails" cannot go on ───

#[derive(Default)]
pub struct FakeSandbox {
    levels: Mutex<BTreeMap<String, String>>,
}
impl Sandbox for FakeSandbox {
    fn run(&self, level: &str, code: &str, _given: &serde_json::Value) -> Answer<()> {
        if !matches!(code, "echo" | "echo again" | "fails") {
            return fail("sandbox", "run", format!("no code {code} is sealed here"));
        }
        self.levels.lock().unwrap().insert(level.into(), code.into());
        done(())
    }
    fn tell(&self, level: &str, _given: &serde_json::Value, event: &serde_json::Value) -> Answer<serde_json::Value> {
        match self.levels.lock().unwrap().get(level).map(String::as_str) {
            Some("echo" | "echo again") => done(serde_json::json!({ "told": event })),
            Some(_) => fail("sandbox", "tell", format!("{level} cannot go on")),
            None => fail("sandbox", "tell", format!("nothing runs as {level}")),
        }
    }
    fn stop(&self, level: &str, _given: &serde_json::Value) -> Answer<()> {
        match self.levels.lock().unwrap().remove(level) {
            Some(_) => done(()),
            None => fail("sandbox", "stop", format!("nothing runs as {level}")),
        }
    }
    fn running(&self, level: &str, _given: &serde_json::Value) -> Answer<Option<String>> {
        done(self.levels.lock().unwrap().get(level).cloned())
    }
}

// ─── Run: programs that end at once with a code ("exit:<code>"), or run until stopped ("forever") ───

#[derive(Default)]
pub struct FakeRun {
    jobs: Mutex<BTreeMap<u64, Option<Ended>>>,
}
impl Run for FakeRun {
    fn start(&self, job: &Job) -> Answer<u64> {
        let end = if let Some(c) = job.program.strip_prefix("exit:") {
            match c.parse::<i32>() {
                Ok(code) => Some(Ended { code: Some(code), signal: None, out_tail: String::new() }),
                Err(_) => return fail("run", "start", format!("{} is no program here", job.program)),
            }
        } else if let Some(s) = job.program.strip_prefix("signal:") {
            match s.parse::<i32>() {
                Ok(signal) => Some(Ended { code: None, signal: Some(signal), out_tail: String::new() }),
                Err(_) => return fail("run", "start", format!("{} is no program here", job.program)),
            }
        } else if job.program == "forever" {
            None
        } else {
            return fail("run", "start", format!("{} is no program here", job.program));
        };
        let mut j = self.jobs.lock().unwrap();
        let h = j.len() as u64 + 1;
        j.insert(h, end);
        done(h)
    }
    fn alive(&self, handle: u64) -> Answer<bool> {
        match self.jobs.lock().unwrap().get(&handle) {
            Some(e) => done(e.is_none()),
            None => fail("run", "alive", format!("no job {handle}")),
        }
    }
    fn stop(&self, handle: u64) -> Answer<Ended> {
        let mut j = self.jobs.lock().unwrap();
        match j.get_mut(&handle) {
            Some(e) => {
                let end = e.clone().unwrap_or(Ended { code: None, signal: Some(9), out_tail: String::new() });
                *e = Some(end.clone());
                done(end)
            }
            None => fail("run", "stop", format!("no job {handle}")),
        }
    }
    fn ended(&self, handle: u64) -> Answer<Option<Ended>> {
        match self.jobs.lock().unwrap().get(&handle) {
            Some(e) => done(e.clone()),
            None => fail("run", "ended", format!("no job {handle}")),
        }
    }
    fn running(&self) -> Answer<Vec<u64>> {
        done(self.jobs.lock().unwrap().iter().filter(|(_, e)| e.is_none()).map(|(h, _)| *h).collect())
    }
}

// ─── Net in: reached by a call in memory ───

#[derive(Default)]
pub struct FakeNetIn {
    handler: Mutex<Option<Handler>>,
}
impl FakeNetIn {
    /// a request, as if it came over the network: answered by the handler served, or refused when nothing is served
    pub fn call(&self, req: Request) -> Result<Response, String> {
        let h = self.handler.lock().unwrap().clone();
        h.map(|h| h(req)).ok_or_else(|| "nothing is served here".to_string())
    }
}
impl NetIn for FakeNetIn {
    fn serve(&self, handler: Handler) -> Answer<String> {
        *self.handler.lock().unwrap() = Some(handler);
        done("memory".to_string())
    }
    fn stop(&self) -> Answer<()> {
        *self.handler.lock().unwrap() = None;
        done(())
    }
}

// ─── Keys: in memory, a person always there ───

#[derive(Default)]
pub struct FakeKeys {
    keys: Mutex<BTreeMap<String, p256::ecdsa::SigningKey>>,
    pub(crate) secrets: Mutex<BTreeMap<String, Vec<u8>>>,
}
fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
impl Keys for FakeKeys {
    fn key(&self, name: &str, _presence: bool) -> Answer<String> {
        let mut k = self.keys.lock().unwrap();
        let key = k.entry(name.to_string()).or_insert_with(|| {
            // drawn from the name, so a test world is the same at every run
            p256::ecdsa::SigningKey::from_slice(&Sha256::digest(name.as_bytes())).expect("a key from a hash")
        });
        done(hex(key.verifying_key().to_sec1_point(true).as_bytes()))
    }
    fn sign(&self, name: &str, bytes: &[u8], _reason: &str) -> Answer<serde_json::Value> {
        use p256::ecdsa::signature::Signer;
        let k = self.keys.lock().unwrap();
        let Some(key) = k.get(name) else { return fail("keys", "sign", format!("{name} has no key")) };
        let s: p256::ecdsa::Signature = key.sign(bytes);
        done(serde_json::json!({ "key": hex(key.verifying_key().to_sec1_point(true).as_bytes()), "sig": hex(&s.to_bytes()) }))
    }
    fn forget(&self, name: &str) -> Answer<()> {
        match self.keys.lock().unwrap().remove(name) {
            Some(_) => done(()),
            None => fail("keys", "forget", format!("{name} has no key")),
        }
    }
    fn keep_secret(&self, name: &str, secret: &[u8]) -> Answer<()> {
        self.secrets.lock().unwrap().insert(name.to_string(), secret.to_vec());
        done(())
    }
    fn has_secret(&self, name: &str) -> Answer<bool> {
        done(self.secrets.lock().unwrap().contains_key(name))
    }
    fn forget_secret(&self, name: &str) -> Answer<()> {
        match self.secrets.lock().unwrap().remove(name) {
            Some(_) => done(()),
            None => fail("keys", "forget_secret", format!("no secret is kept as {name}")),
        }
    }
}

// ─── Agent: a session that hears its words and says them back, at once ───

#[derive(Default)]
pub struct FakeAgent {
    sessions: Mutex<BTreeMap<String, (Hear, bool)>>,
}
impl Agent for FakeAgent {
    fn start(&self, _dir: &str, words: &str, hear: Hear) -> Answer<String> {
        let id = {
            let mut s = self.sessions.lock().unwrap();
            let id = alloc_id(s.len());
            s.insert(id.clone(), (hear.clone(), true));
            id
        };
        hear(&id, Heard::Said(format!("heard: {words}")));
        hear(&id, Heard::TurnEnded);
        done(id)
    }
    fn say(&self, session: &str, words: &str) -> Answer<()> {
        let hear = match self.sessions.lock().unwrap().get(session) {
            Some((_, false)) => return fail("agent", "say", format!("{session} was stopped")),
            Some((h, true)) => h.clone(),
            None => return fail("agent", "say", format!("no session {session}")),
        };
        hear(session, Heard::Said(format!("heard: {words}")));
        hear(session, Heard::TurnEnded);
        done(())
    }
    fn resume(&self, session: &str, _dir: &str, words: &str, hear: Hear) -> Answer<()> {
        self.sessions.lock().unwrap().insert(session.to_string(), (hear.clone(), true));
        hear(session, Heard::Said(format!("heard: {words}")));
        hear(session, Heard::TurnEnded);
        done(())
    }
    fn working(&self, session: &str) -> Answer<bool> {
        match self.sessions.lock().unwrap().get(session) {
            Some(_) => done(false),
            None => fail("agent", "working", format!("no session {session}")),
        }
    }
    fn stop(&self, session: &str) -> Answer<()> {
        let hear = match self.sessions.lock().unwrap().get_mut(session) {
            Some(s) => {
                s.1 = false;
                s.0.clone()
            }
            None => return fail("agent", "stop", format!("no session {session}")),
        };
        hear(session, Heard::Ended("stopped".into()));
        done(())
    }
}
fn alloc_id(n: usize) -> String {
    format!("s{}", n + 1)
}

/// A SOURCE IN MEMORY: files by their path, put in by the test; a folder is any path some file is under. It acts as a model does
/// (`ask`: its answer is what it was told, then what it was asked), each act a movement through its record when it has one
#[derive(Default)]
pub struct FakeSource<'a> {
    files: Mutex<BTreeMap<String, (Vec<u8>, u64)>>,
    record: Option<&'a FakeRecord>,
}
impl<'a> FakeSource<'a> {
    pub fn with_record(record: &'a FakeRecord) -> FakeSource<'a> {
        FakeSource { files: Default::default(), record: Some(record) }
    }
    pub fn put(&self, path: &str, bytes: &[u8]) {
        let mut f = self.files.lock().unwrap();
        let n = f.len() as u64 + 1;
        f.insert(path.to_string(), (bytes.to_vec(), n));
    }
}
impl Source for FakeSource<'_> {
    fn list(&self, _: &Who, path: &str) -> Answer<Vec<Listed>> {
        let f = self.files.lock().unwrap();
        let under = if path.is_empty() { String::new() } else { format!("{path}/") };
        let mut out: BTreeMap<String, Listed> = BTreeMap::new();
        for (p, (b, at)) in f.iter().filter(|(p, _)| p.starts_with(&under)) {
            let rest = &p[under.len()..];
            let (name, folder) = match rest.split_once('/') {
                Some((n, _)) => (n, true),
                None => (rest, false),
            };
            let l = out.entry(name.to_string()).or_insert(Listed { name: name.to_string(), folder, bytes: 0, changed: 0 });
            if !folder {
                l.bytes = b.len() as u64;
            }
            l.changed = l.changed.max(*at);
        }
        if out.is_empty() && !path.is_empty() {
            return fail("source", "list", format!("{path} is no folder here"));
        }
        done(out.into_values().collect())
    }
    fn read(&self, _: &Who, path: &str) -> Answer<Vec<u8>> {
        match self.files.lock().unwrap().get(path) {
            Some((b, _)) => done(b.clone()),
            None => fail("source", "read", format!("{path} is no file here")),
        }
    }
    fn act(&self, m: &Move, what: &str, input: &serde_json::Value) -> Answer<serde_json::Value> {
        if what != "ask" {
            return fail("source", "act", format!("this source does not {what}"));
        }
        let words = input["words"].as_str().filter(|w| !w.trim().is_empty()).ok_or(Fail { seam: "source", call: "act", why: "nothing was asked".into(), retry: None })?;
        let record = self.record.ok_or(Fail { seam: "source", call: "act", why: "no record to ask".into(), retry: None })?;
        record.may(m).map_err(|why| Fail { seam: "source", call: "act", why, retry: None })?;
        record.happened(m, &Cost::default()).map_err(|why| Fail { seam: "source", call: "act", why, retry: None })?;
        done(serde_json::json!({ "says": format!("{} / {words}", input["system"].as_str().unwrap_or("")) }))
    }
}

/// THE RECORD IN MEMORY: a movement on a place named "refused" may not happen (the twin's no, with why); every other may, and what
/// happened is kept, with its cost
#[derive(Default)]
pub struct FakeRecord {
    pub happened: Mutex<Vec<(Move, Cost)>>,
}
impl Record for FakeRecord {
    fn may(&self, m: &Move) -> Result<(), String> {
        if m.on == "refused" {
            return Err(format!("no grant lets {} {} {}", m.who, m.is, m.on));
        }
        Ok(())
    }
    fn happened(&self, m: &Move, cost: &Cost) -> Result<(), String> {
        self.may(m)?;
        self.happened.lock().unwrap().push((m.clone(), cost.clone()));
        Ok(())
    }
}

/// CALLING OUT IN MEMORY: each call answered by `answer` (given what it was sent, the secret put in as the header the edge would add);
/// a movement asked of `record` before and handed to it after
pub struct FakeNetOut<'a> {
    pub keys: &'a FakeKeys,
    pub record: &'a FakeRecord,
    pub answer: Box<dyn Fn(&str, &Request) -> Response + 'a>,
}
impl NetOut for FakeNetOut<'_> {
    fn call(&self, m: Option<&Move>, url: &str, mut r: Request, secret: Option<(&str, Auth)>) -> Answer<Response> {
        if secret.is_some() && m.is_none() {
            return fail("net_out", "call", "a secret goes out only in a movement for someone".into());
        }
        if let Some(m) = m {
            self.record.may(m).map_err(|why| Fail { seam: "net_out", call: "call", why, retry: None })?;
        }
        if let Some((name, auth)) = secret {
            let Some(s) = self.keys.secrets.lock().unwrap().get(name).cloned() else { return fail("net_out", "call", format!("no secret is kept as {name}")) };
            let s = String::from_utf8_lossy(&s).to_string();
            // a header of its own goes as that header, as the edge sends it; every other way, as the Authorization
            match auth {
                Auth::Header(h) => r.headers.push((h, s)),
                other => r.headers.push(("Authorization".into(), match other {
                    Auth::Bearer => format!("Bearer {s}"),
                    Auth::Basic(u) => format!("Basic {u}:{s}"),
                    Auth::Header(_) => unreachable!("sent as its own header above"),
                })),
            }
        }
        let answer = (self.answer)(url, &r);
        if let Some(m) = m {
            self.record.happened(m, &Cost::default()).map_err(|why| Fail { seam: "net_out", call: "call", why: format!("it went out, but the record refused it: {why}"), retry: None })?;
        }
        done(answer)
    }
}

/// REACHING A PERSON IN MEMORY: what was sent, to whom; an address is any object with an `endpoint`
#[derive(Default)]
pub struct FakeMessage {
    pub sent: Mutex<Vec<(serde_json::Value, Option<String>)>>,
}
impl Message for FakeMessage {
    fn address(&self) -> Answer<String> {
        done("fake".into())
    }
    fn send(&self, _: &Who, to: &serde_json::Value, says: Option<&str>) -> Answer<()> {
        if to["endpoint"].as_str().is_none() {
            return fail("message", "send", format!("{to} is no address"));
        }
        self.sent.lock().unwrap().push((to.clone(), says.map(String::from)));
        done(())
    }
}

/// DRAWING IN MEMORY: a page is its words, from `pages` by its address; its picture a few bytes standing for one
#[derive(Default)]
pub struct FakeRender {
    pub pages: Mutex<BTreeMap<String, String>>,
}
impl Render for FakeRender {
    fn draw(&self, _: &Who, url: &str, width: u32, height: u32, _script: Option<&str>) -> Answer<Drawn> {
        match self.pages.lock().unwrap().get(url) {
            Some(t) => done(Drawn { png: format!("png {width}x{height}").into_bytes(), text: t.clone(), errors: vec![] }),
            None => fail("render", "draw", format!("{url} did not answer")),
        }
    }
}

// ─── Identity: fields, proven at once; secrets kept by its Keys ───

/// AN IDENTITY IN MEMORY: proves a fields kind when every field is offered, keeping each secret field in its Keys under
/// `<name>/<field>`; any other way is not its to prove
pub struct FakeIdentity {
    keys: std::sync::Arc<FakeKeys>,
}
impl FakeIdentity {
    pub fn over(keys: std::sync::Arc<FakeKeys>) -> FakeIdentity {
        FakeIdentity { keys }
    }
}
impl Identity for FakeIdentity {
    fn begin(&self, kind: &Kind) -> Answer<serde_json::Value> {
        match kind.way["way"].as_str() {
            Some("fields") => done(serde_json::json!({ "fill": kind.way["fields"] })),
            _ => fail("identity", "begin", format!("{} is not a way this adapter proves", kind.way["way"])),
        }
    }
    fn prove(&self, kind: &Kind, name: &str, offered: &serde_json::Value) -> Answer<Proven> {
        if kind.way["way"] != "fields" {
            return fail("identity", "prove", format!("{} is not a way this adapter proves", kind.way["way"]));
        }
        let fields = kind.way["fields"].as_array().cloned().unwrap_or_default();
        if let Some(f) = fields.iter().find(|f| f["name"].as_str().is_some_and(|n| offered[n].as_str().is_none())) {
            return fail("identity", "prove", format!("{} was not filled", f["name"]));
        }
        let mut kept = vec![];
        for f in fields.iter().filter(|f| f["secret"] == true) {
            let n = f["name"].as_str().unwrap_or("");
            let at = format!("{name}/{n}");
            self.keys.keep_secret(&at, offered[n].as_str().unwrap_or("").as_bytes())?;
            kept.push(at);
        }
        done(Proven { subject: name.to_string(), key: None, kept })
    }
}

// ─── Backup: kept in memory, never plain ───

/// A BACKUP IN MEMORY: each kept thing held scrambled (a test's stand-in for sealing: the place never holds it plain), read back exactly
#[derive(Default)]
pub struct FakeBackup {
    held: Mutex<BTreeMap<String, Vec<u8>>>,
}
impl FakeBackup {
    /// what the place holds for `name`, as it is there
    pub fn raw(&self, name: &str) -> Option<Vec<u8>> {
        self.held.lock().unwrap().get(name).cloned()
    }
}
fn scramble(b: &[u8]) -> Vec<u8> {
    b.iter().map(|x| x ^ 0xA5).collect()
}
impl Backup for FakeBackup {
    fn keep(&self, name: &str, bytes: &[u8]) -> Answer<()> {
        self.held.lock().unwrap().insert(name.into(), scramble(bytes));
        done(())
    }
    fn restore(&self, name: &str) -> Answer<Vec<u8>> {
        match self.held.lock().unwrap().get(name) {
            Some(b) => done(scramble(b)),
            None => fail("backup", "restore", format!("{name} was never kept")),
        }
    }
    fn list(&self, prefix: &str) -> Answer<Vec<String>> {
        done(self.held.lock().unwrap().keys().filter(|k| k.starts_with(prefix)).cloned().collect())
    }
}
