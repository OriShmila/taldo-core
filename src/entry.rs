//! AN ENTRY (principle 2): one entry shape. A node is named by its id — its own, never derived from where it is; where it is, and who
//! owns it, is the graph's to say (`twin`), from the `make` that brought it under its parent.
//!
//! ENTRY  { g, ts, by: <id>, at: <id>, is: <kind>, refs?: { <role>: <id> | [<id>…] | <entry's number> }, value? }
//! A role is an edge from `at` to each node it names, readable both ways — only a mention: it never opens the node on the other side.
//! What a kind or a role means is not the kernel's.
use serde_json::Value;

/// an id: a non-empty name, the same however it is spaced
pub fn parse_id(v: &Value) -> Option<String> {
    let s = v.as_str()?.trim();
    (!s.is_empty()).then(|| s.to_string())
}
/// the nodes a role names: one id or a list of them — a number names an entry, not a node
pub fn ids_of(v: &Value) -> Vec<String> {
    match v {
        Value::Array(a) => a.iter().filter_map(parse_id).collect(),
        _ => parse_id(v).into_iter().collect(),
    }
}

/// AN ENTRY IS WELL FORMED when it has its g, `by` and `at` are ids, it says what it is (`is`), and every role names an id, a list
/// of ids, or the number of another entry — or why not. Where it may land is the twin's to say: it knows the graph
pub fn check(e: &Value) -> Result<(), String> {
    if e["g"].as_i64().is_none() {
        return Err("no g".into());
    }
    if parse_id(&e["by"]).is_none() {
        return Err(format!("by is not a node: {} — every entry is by a node", e["by"]));
    }
    if parse_id(&e["at"]).is_none() {
        return Err(format!("at is not a node: {}", e["at"]));
    }
    if e["is"].as_str().map_or(true, |s| s.trim().is_empty()) {
        return Err("it does not say what it is".into());
    }
    if let Some(refs) = e["refs"].as_object() {
        for (role, v) in refs {
            let ok = match v {
                Value::Number(n) => n.is_i64(),
                Value::Array(a) => a.iter().all(|x| parse_id(x).is_some()),
                _ => parse_id(v).is_some(),
            };
            if !ok {
                return Err(format!("refs.{role} is not a node, a list of nodes or an entry's number: {v}"));
            }
        }
    } else if !e["refs"].is_null() {
        return Err("refs is not an object".into());
    }
    Ok(())
}
