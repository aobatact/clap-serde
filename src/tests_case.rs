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
