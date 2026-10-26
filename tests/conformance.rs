//! THE FAKES PASS THE CONFORMANCE TESTS — so what holds on them holds on any adapter that passes them too.
use taldo_core::conformance;
use taldo_core::fakes::*;
use taldo_core::seams::{Job, Keys};

#[test]
fn fake_files_conform() {
    conformance::files(&FakeFiles::default(), "t/").unwrap();
}
#[test]
fn fake_store_conforms() {
    conformance::store(&FakeStore::default()).unwrap();
}
#[test]
fn fake_clock_conforms() {
    let c = FakeClock::default();
    conformance::clock(&c, &|ms| c.advance(ms)).unwrap();
}
#[test]
fn fake_run_conforms() {
    let job = |p: String| Job { who: "t".into(), program: p, args: vec![], env: Default::default() };
    conformance::sandbox(&FakeSandbox::default(), "Ori › Taldo › engine", &serde_json::json!({}), "echo", "echo again", "fails").unwrap();
    conformance::run(&FakeRun::default(), &|c| job(format!("exit:{c}")), &|s| job(format!("signal:{s}")), &|| job("forever".into()), &|_| {}).unwrap();
}
#[test]
fn fake_net_in_conforms() {
    let n = FakeNetIn::default();
    conformance::net_in(&n, &|_, r| n.call(r)).unwrap();
}
#[test]
fn fake_keys_conform() {
    conformance::keys(&FakeKeys::default(), "a test key").unwrap();
}
#[test]
fn fake_agent_conforms() {
    conformance::agent(&FakeAgent::default(), "/nowhere", "hello", &|_| {}).unwrap();
}
#[test]
fn fake_source_conforms() {
    let s = FakeSource::default();
    conformance::source(&s, &|p, b| s.put(p, b)).unwrap();
}
#[test]
fn fake_net_out_conforms() {
    let keys = FakeKeys::default();
    let record = FakeRecord::default();
    let reached = std::sync::atomic::AtomicUsize::new(0);
    let n = FakeNetOut { keys: &keys, record: &record, answer: Box::new(|_, r: &taldo_core::seams::Request| {
        reached.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        taldo_core::seams::Response {
            status: 200,
            headers: vec![],
            body: format!("{}{}", r.headers.iter().find(|(k, _)| k == "Authorization").map(|(_, v)| v.clone()).unwrap_or_default(), String::from_utf8_lossy(&r.body)).into_bytes(),
        }
    }) };
    conformance::net_out(&n, "http://fake", &|name, s| { taldo_core::seams::Keys::keep_secret(&keys, name, s).unwrap(); }, &|| reached.load(std::sync::atomic::Ordering::SeqCst), &|| record.happened.lock().unwrap().iter().map(|(m, _)| m.clone()).collect()).unwrap();
}
#[test]
fn fake_message_conforms() {
    conformance::message(&FakeMessage::default(), &serde_json::json!({ "endpoint": "fake://phone" })).unwrap();
}
#[test]
fn fake_render_conforms() {
    let r = FakeRender::default();
    // the fake runs no script: its page already says what one would
    r.pages.lock().unwrap().insert("fake://page".into(), "Hello from the page · said by the script".into());
    conformance::render(&r, "fake://page", "fake://nowhere").unwrap();
}
#[test]
fn fake_source_acts_for_someone() {
    // A MODEL IS A SOURCE: what it is asked, answered — a movement, asked of the record before and handed to it after
    let record = FakeRecord::default();
    let s = FakeSource::with_record(&record);
    let input = serde_json::json!({ "system": "Answer with exactly the one word asked for, nothing else.", "words": "Say the word: ready" });
    conformance::source_act(&s, "ask", &input, &|a| a["says"].as_str().is_some_and(|x| x.to_lowercase().contains("ready")), &|| record.happened.lock().unwrap().iter().map(|(m, _)| m.clone()).collect()).unwrap();
}
#[test]
fn fake_backup_conforms() {
    let b = FakeBackup::default();
    conformance::backup(&b, &|name| b.raw(name)).unwrap();
}
#[test]
fn fake_identity_conforms() {
    let keys = std::sync::Arc::new(FakeKeys::default());
    let i = FakeIdentity::over(keys.clone());
    conformance::identity(&i, &|name| keys.has_secret(name).unwrap().value).unwrap();
}
