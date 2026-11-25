//! Tests of the JSON Schema.
use crate::{json_schema, schema::key, CommandWrap};
use serde_json::{json, Value};

fn props<'a>(schema: &'a Value, def: &str) -> &'a serde_json::Map<String, Value> {
    schema["$defs"][def]["properties"].as_object().unwrap()
}

fn is_nullable(prop: &Value) -> bool {
    prop["anyOf"]
        .as_array()
        .is_some_and(|v| v.contains(&json!({ "type": "null" })))
}

/// Input with `{ k: v }` in the command, arg or arg group.
fn input(def: &str, k: &str, v: Value) -> Value {
    let mut m = serde_json::Map::new();
    m.insert(k.to_string(), v);
    let inner = Value::Object(m);
    let mut root = serde_json::Map::new();
    root.insert(key("name"), "app".into());
    match def {
        "command" => {
            if k != key("name") {
                root.extend(inner.as_object().unwrap().clone());
            }
        }
        "arg" => {
            root.insert(key("args"), json!([{ "a": inner }]));
        }
        "arg_group" => {
            root.insert(key("groups"), json!({ "g": inner }));
        }
        _ => unreachable!(),
    }
    Value::Object(root)
}

fn deserialize(v: &Value) -> Result<CommandWrap, serde_json::Error> {
    serde_json::from_value::<CommandWrap>(v.clone())
}

#[test]
fn schema_keys_are_accepted() {
    let schema = json_schema();
    for def in ["command", "arg", "arg_group"] {
        for (k, prop) in props(&schema, def) {
            if def == "command" && k == &key("name") {
                continue;
            }
            let result = deserialize(&input(def, k, Value::Null));
            if is_nullable(prop) {
                assert!(
                    result.is_ok(),
                    "{def}.{k}: null should be accepted: {result:?}"
                );
            } else {
                let err = result
                    .err()
                    .unwrap_or_else(|| panic!("{def}.{k}: null should be rejected"));
                assert!(
                    !err.to_string().contains("unknown field"),
                    "{def}.{k}: {err}"
                );
                assert!(!err.to_string().contains("disabled"), "{def}.{k}: {err}");
            }
        }
    }
}

fn enum_values(schema: &Value, def: &str) -> Vec<String> {
    let v = &schema["$defs"][def];
    let e = if v["enum"].is_array() {
        &v["enum"]
    } else {
        &v["anyOf"][0]["enum"]
    };
    e.as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect()
}

#[test]
fn schema_enum_values_are_accepted() {
    let schema = json_schema();
    let check = |def: &str, k: &str, v: Value| {
        let i = input(def, &key(k), v.clone());
        assert!(
            deserialize(&i).is_ok(),
            "{k} = {v}: {:?}",
            deserialize(&i).err()
        );
    };
    for v in enum_values(&schema, "arg_action") {
        check("arg", "action", v.into());
    }
    for v in enum_values(&schema, "value_hint") {
        check("arg", "value_hint", v.into());
    }
    for v in enum_values(&schema, "value_parser") {
        check("arg", "value_parser", v.clone().into());
        check("arg", "value_parser", json!({ "type": v }));
    }
    check(
        "arg",
        "value_parser",
        json!({ "type": key("i64"), "min": -1, "max": 3, "max_inclusive": false }),
    );
    check("arg", "num_args", json!({ "min": 1 }));
    check("arg", "default_value_if", json!(["x", null, null]));
    check("arg", "required_if_eq", json!(["x", "y"]));
    #[cfg(feature = "color")]
    for v in enum_values(&schema, "color_choice") {
        check("command", "color", v.into());
    }
    #[cfg(feature = "allow-deprecated")]
    for v in enum_values(&schema, "settings") {
        check("command", "settings", json!([v]));
        check("command", "global_setting", v.into());
    }
}

/// The keys in the schema are the same as the ones the deserializers accept
/// (except the ones disabled by the features).
#[test]
#[cfg(feature = "snake-case-key")]
fn schema_has_all_keys() {
    use std::collections::BTreeSet;
    let schema = json_schema();
    for def in ["command", "arg", "arg_group"] {
        let err = deserialize(&input(def, "no_such_key_for_test", Value::Null))
            .err()
            .unwrap()
            .to_string();
        let list = err.split_once("expected one of").unwrap().1;
        let accepted: BTreeSet<_> = list
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .filter(|s| !s.is_empty())
            .collect();
        for k in accepted {
            let enabled = match deserialize(&input(def, k, Value::Null)) {
                Err(e) => {
                    let e = e.to_string();
                    !(e.contains("disabled") || e.contains("deprecated key"))
                }
                Ok(_) => true,
            };
            assert_eq!(
                props(&schema, def).contains_key(k),
                enabled,
                "{def}.{k}: in schema / enabled"
            );
        }
    }
}

#[cfg(feature = "snake-case-key")]
mod validate {
    use super::*;
    use clap::{Arg, ArgAction, ArgGroup, Command, ValueHint};

    fn validator() -> jsonschema::Validator {
        let schema = json_schema();
        jsonschema::meta::validate(&schema).unwrap();
        jsonschema::validator_for(&schema).unwrap()
    }

    fn assert_valid(v: &jsonschema::Validator, i: &Value) {
        let errors: Vec<_> = v.iter_errors(i).map(|e| e.to_string()).collect();
        assert!(errors.is_empty(), "{i}: {errors:?}");
    }

    #[test]
    fn serialized_output_is_valid() {
        let v = validator();
        let c = Command::new("app")
            .version("1.0")
            .about("about")
            .alias("ap")
            .short_flag('a')
            .next_help_heading("H")
            .arg(
                Arg::new("input")
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .value_hint(ValueHint::FilePath)
                    .num_args(1..=3)
                    .help_heading(None::<&str>),
            )
            .arg(
                Arg::new("v")
                    .short('v')
                    .action(ArgAction::Count)
                    .visible_short_aliases(['V']),
            )
            .arg(
                Arg::new("mode")
                    .long("mode")
                    .value_parser(["a", "b"])
                    .default_value("a"),
            )
            .group(ArgGroup::new("g").args(["v", "mode"]).multiple(true))
            .subcommand(
                Command::new("sub").arg(
                    Arg::new("x")
                        .long("x")
                        .value_parser(clap::value_parser!(u8)),
                ),
            );
        let wrap = crate::CommandWrapRef::new(&c);
        assert_valid(&v, &serde_json::to_value(&wrap).unwrap());
        assert_valid(
            &v,
            &serde_json::to_value(wrap.with_config(crate::NoSkip)).unwrap(),
        );
    }

    #[test]
    fn examples_are_valid() {
        let v = validator();
        let yaml: Value = serde_yaml::from_str(
            r#"
name: app
args:
    - input: { value_name: FILE }
    - verbose: { short: v, action: count }
next_help_heading: Options
args_map:
    level: { long: level, value_parser: { type: u8, max: 3 }, num_args: { min: 0, max: 1 } }
groups:
    g: { args: [input, level] }
subcommands:
    sub: { about: sub, default_value_if: null }
"#,
        )
        .unwrap();
        // `default_value_if` is not a key of Command.
        assert!(!v.is_valid(&yaml));
        let mut yaml = yaml;
        yaml["subcommands"]["sub"]
            .as_object_mut()
            .unwrap()
            .remove("default_value_if");
        assert_valid(&v, &yaml);
        assert!(deserialize(&yaml).is_ok());
    }

    #[test]
    fn invalid_inputs_are_rejected() {
        let v = validator();
        for i in [
            json!({ "about": "no name" }),
            json!({ "name": "app", "abuot": "typo" }),
            json!({ "name": "app", "version": 1 }),
            json!({ "name": "app", "short_flag": "ab" }),
            json!({ "name": "app", "args": [{ "a": { "action": "unknown" } }] }),
            json!({ "name": "app", "args": [{ "a": {}, "b": {} }] }),
            json!({ "name": "app", "args": { "a": { "num_args": -1 } } }),
            json!({ "name": "app", "groups": [{ "g": {} }] }),
            json!({ "name": "app", "subcommands": { "s": { "args": { "a": { "lng": "x" } } } } }),
        ] {
            assert!(!v.is_valid(&i), "should be invalid: {i}");
            assert!(deserialize(&i).is_err(), "deserializer should reject: {i}");
        }
    }

    /// `schema/clap-serde.schema.json` is generated by this test with `UPDATE_SCHEMA=1`.
    #[test]
    #[cfg(all(
        feature = "env",
        feature = "color",
        feature = "help",
        feature = "allow-deprecated"
    ))]
    fn schema_file_is_up_to_date() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/schema/clap-serde.schema.json");
        let expected = serde_json::to_string_pretty(&json_schema()).unwrap() + "\n";
        if std::env::var_os("UPDATE_SCHEMA").is_some() {
            std::fs::write(path, &expected).unwrap();
        }
        let actual = std::fs::read_to_string(path).unwrap_or_default();
        assert!(
            actual == expected,
            "{path} is outdated. Run `UPDATE_SCHEMA=1 cargo test --features docsrs schema_file`"
        );
    }
}
