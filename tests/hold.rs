//! FIRST, EVERYTHING GETS DONE: whatever is open is owed to someone — a false acceptance to the node it holds, work held back to its
//! holder — and when every node does only what it is owed, everything settles: every acceptance in force holds, nothing waits,
//! nothing is owed. A HOLDER NEVER REFUSES WORK: work always lands and the worker can always do more. A holder decides only by
//! writing its acceptance — changed, to let the effect in, or as it is, to keep it out; nothing is kept as a verdict: what is accepted
//! is what the acceptances in force now let in. Keeping an effect out never ends anything: what the work was for is still owed, and
//! when it cannot be met under the holder's acceptance, it is owed to the holder of that lower acceptance, to change it. Then THE MECHANISM: each level holds the level below it. Any node works freely at itself and below, at every depth — its work always
//! lands; work is ACCEPTED only if every acceptance in force above the one who did it still holds with it, and otherwise stays work,
//! there but not accepted; whether an acceptance holds is read off what is accepted. An acceptance is in force only when the level
//! above its node wrote it, and only that level changes it. Work held back WAITS at the holder of each acceptance it breaks; the
//! holder decides by changing its own acceptance — then the work is accepted, or, held back still from higher up, it waits there:
//! it rises level by level to the top, whose own work nothing holds back. The same at every depth.
//!
//! Nothing here is a chosen case: trees, acceptances and changes are drawn at random (fixed seeds, so a run is always the same) and
//! the rules are checked on whatever comes out. An acceptance here is `{ expect: { at, has } }` — the node at `at` has the details in
//! `has`; a change is a `be` setting a node's detail `v`.
mod common;
use taldo_core::twin::fold;
use serde_json::{json, Value};

struct Rng(u64);
impl Rng {
    fn next(&mut self, n: usize) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 33) as usize) % n.max(1)
    }
}
fn k(p: &[String]) -> String {
    p.join(" › ")
}
fn below(p: &[String], q: &[String]) -> bool {
    q.starts_with(p)
}
/// the world's paths as the graph's ids: every path a condition names (`at`, `by`, `from`) is the id of the node made there
fn ids(v: Value) -> Value {
    match v {
        Value::Object(o) => Value::Object(
            o.into_iter()
                .map(|(key, x)| match (key.as_str(), &x) {
                    ("at" | "by" | "from", Value::Array(a)) => {
                        let id = a.iter().filter_map(Value::as_str).collect::<Vec<_>>().join(" › ");
                        (key, json!(id))
                    }
                    _ => (key, ids(x)),
                })
                .collect(),
        ),
        x => x,
    }
}
/// a node's id read back as the world's path
fn path(v: &Value) -> Vec<String> {
    v.as_str().unwrap().split(" › ").map(String::from).collect()
}
/// an entry, signed by its writer
fn entry(g: i64, by: &[String], at: &[String], is: &str, refs: Value, value: Value) -> Value {
    common::signed(
        json!({ "g": g, "ts": g, "arrived": g, "by": k(by), "at": k(at), "is": is, "refs": ids(refs), "value": ids(value) }),
    )
}
/// a node made by the one above it, given its key
fn made(g: i64, n: &[String]) -> Value {
    entry(g, &n[..n.len() - 1], n, "make", json!({ "in": k(&n[..n.len() - 1]) }), json!({ "kind": "", "keys": [{ "key": common::public(&k(n)) }] }))
}
/// the seeds drawn: 1..=40, or the range TALDO_SEEDS names ("1001..3000") — for a held-out run on seeds never looked at
fn seeds() -> std::ops::RangeInclusive<u64> {
    std::env::var("TALDO_SEEDS")
        .ok()
        .and_then(|s| {
            s.split_once("..")
                .and_then(|(a, b)| Some(a.parse().ok()?..=b.parse().ok()?))
        })
        .unwrap_or(1..=40)
}
fn ancestors_or_self(p: &[String]) -> Vec<Vec<String>> {
    (1..=p.len()).map(|i| p[..i].to_vec()).collect()
}

#[test]
fn everything_gets_done() {
    let (mut settled, mut rounds_max, mut opened, mut refusals, mut worked_on) = (0, 0, 0, 0, 0);
    for seed in seeds() {
        let mut r = Rng(seed * 7919);
        let mut nodes: Vec<Vec<String>> = vec![vec!["Ori".into()]];
        for i in 0..14 {
            let mut p = nodes[r.next(nodes.len())].clone();
            if p.len() < 6 {
                p.push(format!("n{i}"));
                nodes.push(p);
            }
        }
        // the root's first key; then each node made by the one above it, given its key
        let mut log: Vec<Value> = vec![common::first_key("Ori")];
        let mut g = 0;
        for n in nodes.iter().skip(1) {
            g += 1;
            log.push(made(g, n));
        }
        // what is asked: acceptances given from above at random nodes, each about a node in its subtree, most of them not true yet —
        // several may name the same node, from different levels
        for _ in 0..10 {
            let n = nodes[1 + r.next(nodes.len() - 1)].clone();
            let inside: Vec<&Vec<String>> = nodes.iter().filter(|x| below(&n, x)).collect();
            let x = inside[r.next(inside.len())].clone();
            g += 1;
            log.push(entry(
                g,
                &n[..n.len() - 1],
                &n,
                "acceptance",
                json!({}),
                json!({ "expect": { "at": x, "has": { "v": 1 + r.next(3) } } }),
            ));
        }
        // events asked for: at some nodes, whatever is said there must get an answer from that node or below — and things are said
        for _ in 0..4 {
            let n = nodes[1 + r.next(nodes.len() - 1)].clone();
            g += 1;
            let asker = n[..1 + r.next(n.len() - 1)].to_vec();
            log.push(entry(g, &n[..n.len() - 1], &n, "acceptance", json!({}), json!({ "says": "what is asked here gets an answer", "when": { "is": "say", "at": n, "by": asker }, "expect": { "from": n } })));
            for _ in 0..1 + r.next(3) {
                let who = asker.clone();
                g += 1;
                log.push(entry(
                    g,
                    &who,
                    &n,
                    "say",
                    json!({}),
                    json!({ "says": "a question" }),
                ));
            }
        }
        // and work done freely by anyone at itself and below, before anything is owed
        for _ in 0..10 {
            let w = nodes[r.next(nodes.len())].clone();
            let inside: Vec<&Vec<String>> = nodes.iter().filter(|x| below(&w, x)).collect();
            let at = inside[r.next(inside.len())].clone();
            g += 1;
            log.push(entry(
                g,
                &w,
                &at,
                "be",
                json!({}),
                json!({ "v": r.next(4) }),
            ));
        }
        let mut rounds = 0;
        loop {
            let t = fold(&log);
            let open: Vec<i64> = nodes
                .iter()
                .flat_map(|n| t.in_force(&k(n)))
                .filter(|a| t.holds(*a) != Some(true))
                .collect();
            let waits: Vec<i64> = nodes
                .iter()
                .flat_map(|h| t.waiting(&k(h)))
                .map(|(x, _)| x)
                .collect();
            let owed = t.owed();
            if rounds == 0 {
                opened += open.len();
            }
            // NOTHING OPEN IS OWED TO NO ONE: each false acceptance to the node it holds, each piece of held-back work to its holder
            for a in &open {
                let held: Vec<String> = path(&t.entries.get(a).unwrap()["at"]);
                let (at, holder) = (k(&held), k(&held[..held.len() - 1]));
                assert!(
                    owed.iter()
                        .any(|(who, what)| *what == *a && (*who == at || *who == holder)),
                    "seed {seed}: #{a} is not true and owed neither to {at} nor to its holder"
                );
            }
            for x in &waits {
                assert!(
                    owed.iter().any(|(_, what)| what == x),
                    "seed {seed}: work #{x} waits and is owed to no one"
                );
            }
            if owed.is_empty() {
                // SETTLED, AND CORRECT: every acceptance in force holds on what is accepted, and nothing waits
                assert!(open.is_empty() && waits.is_empty(), "seed {seed}: nothing is owed, yet {} acceptances are not true and {} pieces of work wait", open.len(), waits.len());
                settled += 1;
                break;
            }
            rounds += 1;
            assert!(
                rounds <= 200,
                "seed {seed}: still not done after 200 rounds — {} owed",
                owed.len()
            );
            // EVERY NODE DOES ONLY WHAT IT IS OWED:
            //   an acceptance at itself — work that makes what it expects true;
            //   an acceptance it holds — change it to fit what is accepted now (its node could not meet it under what is above);
            //   a change waiting at it — let the effect in (change its acceptances to fit the work), or keep it out (write them as they are)
            let mut kept_out: Vec<(Vec<String>, i64, Vec<String>)> = vec![];
            for (who, what) in owed {
                let who: Vec<String> = who.split(" › ").map(String::from).collect();
                let e = t.entries.get(&what).unwrap().clone();
                g += 1;
                let pairs: Vec<i64> = t
                    .waiting(&k(&who))
                    .into_iter()
                    .filter(|(x, _)| *x == what)
                    .map(|(_, a)| a)
                    .collect();
                if !pairs.is_empty() {
                    let let_in = e["is"] == "be" && r.next(2) == 0;
                    g -= 1;
                    for a in pairs {
                        let old = &t.entries.get(&a).unwrap();
                        let has = if let_in {
                            json!({ "v": e["value"]["v"] })
                        } else {
                            old["value"]["expect"]["has"].clone()
                        };
                        g += 1;
                        log.push(entry(
                            g,
                            &who,
                            &old["at"].as_str().unwrap().split(" › ").map(String::from).collect::<Vec<_>>(),
                            "acceptance",
                            json!({ "re": a }),
                            json!({ "expect": { "at": old["value"]["expect"]["at"], "has": has } }),
                        ));
                    }
                    if !let_in {
                        refusals += 1;
                        kept_out.push((
                            who.clone(),
                            what,
                            path(&e["by"]),
                        ));
                    }
                } else if e["is"] == "acceptance" && e["value"]["when"].is_object() {
                    // an event owed: answer, from itself, what was said at it and has no answer from it yet
                    let held: Vec<String> = path(&e["at"]);
                    assert_eq!(
                        who,
                        held,
                        "seed {seed}: the event #{what} at {} is owed to {}",
                        k(&held),
                        k(&who)
                    );
                    let asked: Vec<i64> = t.nodes[&k(&held)]
                        .at
                        .iter()
                        .copied()
                        .filter(|x| {
                            t.entries.get(x).unwrap()["is"] == "say"
                                && t.entries.get(x).unwrap()["by"] == e["value"]["when"]["by"]
                                && !log
                                    .iter()
                                    .any(|y| y["refs"]["re"] == json!(x) && y["by"] == json!(k(&held)))
                        })
                        .collect();
                    g -= 1;
                    for x in asked {
                        g += 1;
                        log.push(entry(
                            g,
                            &held,
                            &held,
                            "say",
                            json!({ "re": x }),
                            json!({ "says": "an answer" }),
                        ));
                        worked_on += 1;
                    }
                } else if e["is"] == "acceptance" {
                    let held: Vec<String> = path(&e["at"]);
                    let x: Vec<String> =
                        path(&e["value"]["expect"]["at"]);
                    if who == held {
                        log.push(entry(
                            g,
                            &who,
                            &x,
                            "be",
                            json!({}),
                            json!({ "v": e["value"]["expect"]["has"]["v"] }),
                        ));
                        worked_on += 1;
                    } else {
                        assert_eq!(
                            who,
                            held[..held.len() - 1].to_vec(),
                            "seed {seed}: #{what} at {} is owed to {}, neither it nor its holder",
                            k(&held),
                            k(&who)
                        );
                        let now = t
                            .nodes
                            .get(&k(&x))
                            .and_then(|n| n.accepted.get("v").cloned())
                            .unwrap_or(json!(null));
                        log.push(entry(
                            g,
                            &who,
                            &held,
                            "acceptance",
                            json!({ "re": what }),
                            json!({ "expect": { "at": x, "has": { "v": now } } }),
                        ));
                    }
                }
            }
            // KEEPING AN EFFECT OUT NEVER STOPS WORK: the work is still there and no longer waits at the holder who kept it out; its
            // maker's next work still lands
            let t2 = fold(&log);
            for (h, w, maker) in kept_out {
                assert!(
                    t2.entries.contains_key(&w),
                    "seed {seed}: work #{w} kept out by {} is gone",
                    k(&h)
                );
                assert!(
                    t2.waiting(&k(&h)).iter().all(|(y, _)| *y != w),
                    "seed {seed}: work #{w} still waits at {}, who kept it out",
                    k(&h)
                );
                g += 1;
                log.push(entry(
                    g,
                    &maker,
                    &maker,
                    "be",
                    json!({}),
                    json!({ "tried": "again" }),
                ));
                assert!(
                    fold(&log).entries.contains_key(&g),
                    "seed {seed}: after its work was kept out, {}'s next work did not land",
                    k(&maker)
                );
            }
        }
        rounds_max = rounds_max.max(rounds);
    }
    let n = seeds().count();
    assert!(settled == n && opened > 0 && refusals > 0 && worked_on > 0, "{settled} of {n} settled; {opened} open at the start, {worked_on} worked on, {refusals} kept out");
    println!(
        "all {n} settled, the slowest in {rounds_max} rounds, through {refusals} effects kept out"
    );
}

// THEN THE MECHANISM THAT KEEPS IT RIGHT
#[test]
fn each_level_holds_the_level_below() {
    let (mut held_back_deep, mut accepted, mut replaced, mut decided_in, mut rose) =
        (0, 0, 0, 0, 0);
    for seed in seeds() {
        let mut r = Rng(seed);
        // a tree of random shape: each new node under one already there
        let mut nodes: Vec<Vec<String>> = vec![vec!["Ori".into()]];
        for i in 0..14 {
            let mut p = nodes[r.next(nodes.len())].clone();
            if p.len() < 6 {
                p.push(format!("n{i}"));
                nodes.push(p);
            }
        }
        // the root's first key; then each node made by the one above it, given its key
        let mut log: Vec<Value> = vec![common::first_key("Ori")];
        let mut g = 0;
        for n in nodes.iter().skip(1) {
            g += 1;
            log.push(made(g, n));
        }
        // ONE TWIN, each new entry applied to it as it comes — the same as folding the whole log again, without reading it all again
        // two of them: `prev` as it was before this step's change, `cur` after it — each caught up with what came since it last read
        let (mut prev, mut cur) = (fold(&log), fold(&log));
        let (mut prev_applied, mut applied) = (log.len(), log.len());
        for _ in 0..300 {
            for x in &log[prev_applied..] {
                prev.apply(x);
            }
            prev_applied = log.len();
            let before = &prev;
            let w = nodes[r.next(nodes.len())].clone();
            g += 1;
            let waits: Vec<(Vec<String>, i64, i64)> = nodes
                .iter()
                .flat_map(|h| {
                    before
                        .waiting(&k(h))
                        .into_iter()
                        .map(move |(x, a)| (h.clone(), x, a))
                })
                .collect();
            let mut decision: Option<(Vec<String>, i64)> = None;
            if !waits.is_empty() && r.next(2) == 0 {
                // A HOLDER DECIDES on work waiting at it: it changes each of its own acceptances the work breaks, so the work fits them
                let (h, work, _) = waits[r.next(waits.len())].clone();
                let v = before.entries.get(&work).unwrap()["value"]["v"].clone();
                g -= 1;
                for (_, a) in before
                    .waiting(&k(&h))
                    .into_iter()
                    .filter(|(x, _)| *x == work)
                {
                    let old = &before.entries.get(&a).unwrap();
                    g += 1;
                    log.push(entry(g, &h, &old["at"].as_str().unwrap().split(" › ").map(String::from).collect::<Vec<_>>(), "acceptance", json!({ "re": a }), json!({ "expect": { "at": old["value"]["expect"]["at"], "has": { "v": v.clone() } } })));
                }
                decision = Some((h, work));
            } else if r.next(3) == 0 {
                // AN ACCEPTANCE, by anyone, at any node, about a node in that node's subtree as it is now — sometimes changing one there
                let n = nodes[r.next(nodes.len())].clone();
                let inside: Vec<&Vec<String>> = nodes.iter().filter(|x| below(&n, x)).collect();
                let x = inside[r.next(inside.len())].clone();
                let v = before
                    .nodes
                    .get(&k(&x))
                    .and_then(|t| t.value.get("v").cloned())
                    .unwrap_or(json!(0));
                let there = before.in_force(&k(&n));
                let refs = if !there.is_empty() && r.next(2) == 0 {
                    json!({ "re": there[r.next(there.len())] })
                } else {
                    json!({})
                };
                log.push(entry(
                    g,
                    &w,
                    &n,
                    "acceptance",
                    refs,
                    json!({ "expect": { "at": x, "has": { "v": v } } }),
                ));
            } else {
                // A CHANGE, by anyone, anywhere
                let at = nodes[r.next(nodes.len())].clone();
                log.push(entry(
                    g,
                    &w,
                    &at,
                    "be",
                    json!({}),
                    json!({ "v": r.next(3) }),
                ));
            }
            for x in &log[applied..] {
                cur.apply(x);
            }
            applied = log.len();
            let after = &cur;
            let e = &log[log.len() - 1];
            let at: Vec<String> = path(&e["at"]);
            let w: Vec<String> = path(&e["by"]);

            // 4. HELD-BACK WORK IS NEVER STUCK NOWHERE: every piece of work that landed is accepted, or waits at a holder, or was decided
            //    out by a holder writing its acceptance after it — never two of these; it waits at a holder only for an acceptance that
            //    holder holds; the top's own work is never held back
            let waiting: Vec<(Vec<String>, i64, i64)> = nodes
                .iter()
                .flat_map(|h| {
                    after
                        .waiting(&k(h))
                        .into_iter()
                        .map(move |(x, a)| (h.clone(), x, a))
                })
                .collect();
            // the newest acceptance at each node written by the level above it — read once, not again for every piece of work
            let mut newest_by_holder: std::collections::BTreeMap<String, i64> = Default::default();
            for y in log.iter().filter(|y| y["is"] == "acceptance") {
                let at: Vec<String> = path(&y["at"]);
                if at.len() > 1 && y["by"] == json!(k(&at[..at.len() - 1])) {
                    let g = y["g"].as_i64().unwrap();
                    let n = newest_by_holder.entry(k(&at)).or_insert(g);
                    *n = (*n).max(g);
                }
            }
            for x in log.iter().filter(|x| {
                x["is"] == "be" && after.entries.contains_key(&x["g"].as_i64().unwrap())
            }) {
                let xg = x["g"].as_i64().unwrap();
                let at_holders = waiting.iter().filter(|(_, y, _)| *y == xg).count();
                let maker: Vec<String> = path(&x["by"]);
                let decided = ancestors_or_self(&maker)
                    .iter()
                    .any(|n| n.len() > 1 && newest_by_holder.get(&k(n)).is_some_and(|y| *y > xg));
                assert!(
                    after.accepted(xg) || at_holders > 0 || decided,
                    "seed {seed}: work #{xg} is neither accepted, nor waiting, nor decided out"
                );
                assert!(
                    !(after.accepted(xg) && at_holders > 0),
                    "seed {seed}: work #{xg} is accepted and waiting"
                );
                if x["by"] == json!("Ori") {
                    assert!(
                        after.accepted(xg),
                        "seed {seed}: the top's own work #{xg} was held back"
                    );
                }
            }
            for (h, x, a) in &waiting {
                let held_at: Vec<String> =
                    path(&after.entries.get(a).unwrap()["at"]);
                assert!(
                    held_at.len() > 1
                        && held_at[..held_at.len() - 1] == h[..]
                        && after.in_force(&k(&held_at)).contains(a),
                    "seed {seed}: work #{x} waits at {} for #{a}, which it does not hold",
                    k(h)
                );
            }
            // 5. A DECISION SETTLES IT AT THAT LEVEL: the work is accepted, or it waits only higher up than the holder who decided
            if let Some((h, work)) = &decision {
                if after.accepted(*work) {
                    decided_in += 1;
                } else {
                    for (hh, _, _) in waiting.iter().filter(|(_, y, _)| y == work) {
                        assert!(hh.len() < h.len() && h.starts_with(hh), "seed {seed}: after {} decided, work #{work} still waits at {}, not above it", k(h), k(hh));
                    }
                    rose += 1;
                }
            }

            // 1. ANY NODE WORKS FREELY AT ITSELF AND BELOW, AT EVERY DEPTH: its work lands; never above or beside itself
            assert_eq!(
                after.entries.contains_key(&g),
                below(&w, &at),
                "seed {seed}: {}'s work at {} {}",
                k(&w),
                k(&at),
                if below(&w, &at) {
                    "did not land"
                } else {
                    "landed above or beside itself"
                }
            );
            // 2. WORK NEVER TURNS AN ACCEPTANCE HELD ABOVE ITS MAKER FROM TRUE TO FALSE — it stays work, not accepted — at every depth
            for n in ancestors_or_self(&w) {
                for a in before.in_force(&k(&n)) {
                    if before.holds(a) == Some(true) {
                        assert_eq!(
                            after.holds(a),
                            Some(true),
                            "seed {seed}: {}'s work broke #{a}, held over {}",
                            k(&w),
                            k(&n)
                        );
                    }
                }
            }
            if e["is"] == "be" && below(&w, &at) && !after.accepted(g) && w.len() >= 4 {
                held_back_deep += 1;
            }
            // the control: work at itself or below that touches no node any acceptance names is always accepted
            let named = nodes
                .iter()
                .flat_map(|n| before.in_force(&k(n)))
                .any(|a| before.entries.get(&a).unwrap()["value"]["expect"]["at"] == json!(k(&at)));
            if e["is"] == "be" && below(&w, &at) && !named {
                assert!(
                    after.accepted(g),
                    "seed {seed}: {}'s work at {} touched nothing held, and was not accepted",
                    k(&w),
                    k(&at)
                );
                accepted += 1;
            }
            // 3. AN ACCEPTANCE IS IN FORCE ONLY WHEN THE LEVEL ABOVE ITS NODE WROTE IT, AND ONLY THAT LEVEL CHANGES IT — its change being
            //    a change like any other: the old one stays in force until the change itself comes into force
            for n in &nodes {
                let force = after.in_force(&k(n));
                for a in &force {
                    let by: Vec<String> =
                        path(&after.entries.get(a).unwrap()["by"]);
                    assert!(
                        n.len() > 1 && by == n[..n.len() - 1],
                        "seed {seed}: #{a} at {} is in force, written by {}",
                        k(n),
                        k(&by)
                    );
                }
                let parent_wrote = |x: &Value| {
                    n.len() > 1
                        && x["by"] == json!(k(&n[..n.len() - 1]))
                        && x["at"] == json!(k(&n))
                        && x["is"] == "acceptance"
                };
                for x in log
                    .iter()
                    .filter(|x| parent_wrote(x) && after.accepted(x["g"].as_i64().unwrap()))
                {
                    let xg = x["g"].as_i64().unwrap();
                    let changed = log.iter().any(|y| {
                        parent_wrote(y)
                            && y["refs"]["re"] == json!(xg)
                            && after.accepted(y["g"].as_i64().unwrap())
                    });
                    assert_eq!(
                        force.contains(&xg),
                        !changed,
                        "seed {seed}: #{xg}, written by the level above {}, {} in force",
                        k(n),
                        if changed { "is still" } else { "is not" }
                    );
                    replaced += changed as usize;
                }
            }
        }
    }
    // the draw reached what it is for: work held back four levels down, work accepted, acceptances changed from above, decisions that
    // let work in, and work that rose past the holder who decided
    assert!(held_back_deep > 0 && accepted > 0 && replaced > 0 && decided_in > 0 && rose > 0, "the draw never reached the mechanism: {held_back_deep} held back deep, {accepted} accepted, {replaced} changed from above, {decided_in} let in by a decision, {rose} rose higher");
}
