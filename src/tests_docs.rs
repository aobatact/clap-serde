//! Checks that the key lists in `docs/*.md` match the keys the deserializers accept.
use crate::CommandWrap;
use std::collections::BTreeSet;

/// Keys in the first table of the markdown.
fn doc_keys(md: &str) -> BTreeSet<String> {
    md.lines()
        .skip_while(|l| !l.starts_with('|'))
        .take_while(|l| l.starts_with('|'))
        .skip(2) // header and separator
        .map(|l| l.split('|').nth(1).unwrap().trim().to_string())
        .filter(|k| !k.starts_with('('))
        .collect()
}

/// Keys listed in the `unknown field` error.
fn expected_keys(err: impl ToString) -> BTreeSet<String> {
    let msg = err.to_string();
    let (_, list) = msg
        .split_once("expected one of")
        .unwrap_or_else(|| panic!("unexpected error: {msg}"));
    // remove the position added by serde_json
    let list = list.rsplit_once(" at line").map_or(list, |(l, _)| l);
    list.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

fn check(name: &str, md: &str, wrap: impl Fn(&str) -> String) {
    let docs = doc_keys(md);
    assert!(!docs.is_empty(), "{name}: no keys found in docs");

    let err = serde_json::from_str::<CommandWrap>(&wrap("no_such_key_for_test"))
        .err()
        .unwrap();
    let accepted = expected_keys(err);
    let undocumented: Vec<_> = accepted.difference(&docs).collect();
    assert!(
        undocumented.is_empty(),
        "{name}: undocumented keys {undocumented:?}"
    );

    for key in &docs {
        if let Err(e) = serde_json::from_str::<CommandWrap>(&wrap(key)) {
            assert!(
                !e.to_string().contains("unknown field"),
                "{name}: documented key `{key}` is not accepted: {e}"
            );
        }
    }
}

#[test]
fn command_keys_are_documented() {
    check("Command", include_str!("../docs/app_keys.md"), |k| {
        format!(r#"{{"name": "app", "{k}": null}}"#)
    });
}

#[test]
fn arg_keys_are_documented() {
    check("Arg", include_str!("../docs/arg_keys.md"), |k| {
        format!(r#"{{"name": "app", "args": [{{"a": {{"{k}": null}}}}]}}"#)
    });
}

#[test]
fn group_keys_are_documented() {
    check("ArgGroup", include_str!("../docs/arg_groups.md"), |k| {
        format!(r#"{{"name": "app", "groups": {{"g": {{"{k}": null}}}}}}"#)
    });
}
