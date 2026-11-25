//! Tests which run with every case-key feature.
use crate::{CommandWrap, CommandWrapRef, NoSkip};
use clap::{Arg, ArgAction, Command, ValueHint};

fn command() -> Command {
    Command::new("x")
        .arg(
            Arg::new("path")
                .long("path")
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("os")
                .long("os")
                .value_parser(clap::value_parser!(std::ffi::OsString)),
        )
        .arg(
            Arg::new("num")
                .long("num")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(Arg::new("flag").long("flag").action(ArgAction::SetTrue))
}

#[test]
fn roundtrip_with_case_feature() {
    let c = command();
    for json in [
        serde_json::to_value(CommandWrapRef::new(&c)).unwrap(),
        serde_json::to_value(CommandWrapRef::new(&c).with_config(NoSkip)).unwrap(),
    ] {
        let c2: Command = serde_json::from_value::<CommandWrap>(json.clone())
            .unwrap()
            .into();
        assert_eq!(serde_json::to_value(CommandWrapRef::new(&c2)).unwrap(), {
            serde_json::to_value(CommandWrapRef::new(&c)).unwrap()
        });
        let m = c2
            .try_get_matches_from(["x", "--path", "a", "--os", "b", "--num", "3", "--flag"])
            .unwrap();
        assert_eq!(m.get_one::<u32>("num"), Some(&3));
        assert!(m.get_flag("flag"));
    }
}

#[cfg(feature = "kebab-case-key")]
const CASED_JSON: &str = r#"{
    "name": "app",
    "arg-required-else-help": true,
    "next-line-help": true,
    "args": [
        { "input": { "value-name": "FILE", "allow-hyphen-values": true } },
        { "verbose": { "short": "v", "action": "count" } }
    ],
    "subcommands": [
        { "sub": { "visible-alias": "s" } }
    ]
}"#;

#[cfg(feature = "pascal-case-key")]
const CASED_JSON: &str = r#"{
    "Name": "app",
    "ArgRequiredElseHelp": true,
    "NextLineHelp": true,
    "Args": [
        { "input": { "ValueName": "FILE", "AllowHyphenValues": true } },
        { "verbose": { "Short": "v", "Action": "Count" } }
    ],
    "Subcommands": [
        { "sub": { "VisibleAlias": "s" } }
    ]
}"#;

#[cfg(feature = "snake-case-key")]
const CASED_JSON: &str = r#"{
    "name": "app",
    "arg_required_else_help": true,
    "next_line_help": true,
    "args": [
        { "input": { "value_name": "FILE", "allow_hyphen_values": true } },
        { "verbose": { "short": "v", "action": "count" } }
    ],
    "subcommands": [
        { "sub": { "visible_alias": "s" } }
    ]
}"#;

#[test]
fn deserialize_cased_keys() {
    let c: Command = serde_json::from_str::<CommandWrap>(CASED_JSON)
        .unwrap()
        .into();
    assert!(c.is_arg_required_else_help_set());
    assert!(c.is_next_line_help_set());
    let input = c.get_arguments().find(|a| a.get_id() == "input").unwrap();
    assert_eq!(
        input
            .get_value_names()
            .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["FILE"])
    );
    assert!(input.is_allow_hyphen_values_set());
    let m = c
        .clone()
        .try_get_matches_from(["app", "-vv", "-x"])
        .unwrap();
    assert_eq!(m.get_count("verbose"), 2);
    assert_eq!(m.get_one::<String>("input").unwrap(), "-x");
    let sub = c.find_subcommand("sub").unwrap();
    assert_eq!(sub.get_visible_aliases().collect::<Vec<_>>(), ["s"]);
}

fn rich_command() -> Command {
    Command::new("app")
        .about("about")
        .next_help_heading("Heading")
        .arg_required_else_help(true)
        .arg(
            Arg::new("in_file")
                .long("in-file")
                .value_name("FILE")
                .value_hint(ValueHint::FilePath)
                .num_args(1..=2),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .action(ArgAction::Count)
                .visible_short_aliases(['V']),
        )
        .group(clap::ArgGroup::new("my_group").args(["in_file", "verbose"]))
        .subcommand(
            Command::new("sub_cmd")
                .long_flag("sub-cmd")
                .arg(Arg::new("x").long("x")),
        )
}

/// Check that the keys of the properties (not ids) follow the case-key feature.
fn check_keys(v: &serde_json::Value, kind: &str) {
    use crate::util::case_key;
    let obj = v.as_object().unwrap_or_else(|| panic!("{kind}: {v}"));
    for (k, child) in obj {
        let snake = k.replace('-', "_");
        let snake =
            if k.contains('_') || k.contains('-') || k.chars().all(|c| c.is_ascii_lowercase()) {
                snake
            } else {
                // PascalCase to snake_case
                let mut s = String::new();
                for (i, c) in k.chars().enumerate() {
                    if c.is_ascii_uppercase() && i > 0 {
                        s.push('_');
                    }
                    s.push(c.to_ascii_lowercase());
                }
                s
            };
        assert_eq!(k, &case_key(&snake), "{kind} key `{k}`");
        let entries = |c: &serde_json::Value| -> Vec<serde_json::Value> {
            match c {
                serde_json::Value::Array(a) => a
                    .iter()
                    .flat_map(|e| e.as_object().unwrap().values().cloned())
                    .collect(),
                serde_json::Value::Object(o) => o.values().cloned().collect(),
                _ => vec![],
            }
        };
        match (kind, snake.as_str()) {
            ("command", "args") => entries(child).iter().for_each(|a| check_keys(a, "arg")),
            ("command", "groups") => entries(child).iter().for_each(|g| check_keys(g, "group")),
            ("command", "subcommands") => {
                entries(child).iter().for_each(|c| check_keys(c, "command"))
            }
            _ => {}
        }
    }
}

#[test]
fn serialized_keys_follow_case_feature() {
    let c = rich_command();
    for json in [
        serde_json::to_value(CommandWrapRef::new(&c)).unwrap(),
        serde_json::to_value(CommandWrapRef::new(&c).with_config(NoSkip)).unwrap(),
    ] {
        check_keys(&json, "command");
        // ids are not converted
        let args = &json[crate::util::case_key("args").as_ref()];
        assert!(args[0].get("in_file").is_some(), "{args}");
        let c2: Command = serde_json::from_value::<CommandWrap>(json).unwrap().into();
        assert!(c2.find_subcommand("sub_cmd").is_some());
        assert!(c2.get_arguments().any(|a| a.get_id() == "in_file"));
    }
}

#[test]
#[cfg(feature = "schema")]
fn serialized_output_validates_against_schema() {
    let validator = jsonschema::validator_for(&crate::json_schema()).unwrap();
    let c = rich_command();
    for json in [
        serde_json::to_value(CommandWrapRef::new(&c)).unwrap(),
        serde_json::to_value(CommandWrapRef::new(&c).with_config(NoSkip)).unwrap(),
    ] {
        let errors: Vec<_> = validator
            .iter_errors(&json)
            .map(|e| e.to_string())
            .collect();
        assert!(errors.is_empty(), "{json}: {errors:?}");
    }
}
