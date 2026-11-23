use crate::{ser::app::CommandWrapRef, CommandWrap, NoSkip};
use clap::{builder::ValueParser, Command};

fn about(c: &Command) -> Option<String> {
    c.get_about().map(|s| s.to_string())
}

#[test]
fn name_yaml() {
    const NAME_YAML: &str = "name: app_clap_serde\n";
    let app: CommandWrap = serde_yaml::from_str(NAME_YAML).expect("fail to make yaml");
    assert_eq!(app.get_name(), "app_clap_serde");
}

#[test]
fn test_yaml() {
    const CLAP_YAML: &str = r#"
name: app_clap_serde
version : "1.0"
about : yaml_support!
author : yaml_supporter

args:
    - apple : 
        short: a
    - banana:
        short: b
        long: banana
        aliases :
            - musa_spp

subcommands:
    - sub1: 
        about : subcommand_1
    - sub2: 
        about : subcommand_2

setting: trailing_var_arg

settings: 
    - help_expected
"#;

    let app: CommandWrap = serde_yaml::from_str(CLAP_YAML).expect("fail to make yaml");
    assert_eq!(app.get_name(), "app_clap_serde");
    let subs = app.get_subcommands().collect::<Vec<_>>();
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub1" && about(x).as_deref() == Some("subcommand_1")));
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub2" && about(x).as_deref() == Some("subcommand_2")));
    let args = app.get_arguments().collect::<Vec<_>>();
    assert!(args
        .iter()
        .any(|x| x.get_id() == "apple" && x.get_short() == Some('a')));
    assert!(args.iter().any(|x| x.get_id() == "banana"
        && x.get_short() == Some('b')
        && x.get_long() == Some("banana")));
}

#[test]
fn test_yaml_value_parser() {
    const CLAP_YAML: &str = r#"
name: app_clap_serde
version : "1.0"
about : yaml_support!
author : yaml_supporter

args:
    - apple : 
        short: a
        value_parser: 
            type: i64
            min: -100
            max: 100
            max_inclusive: true
    - banana:
        short: b
        long: banana
        value_parser: non_empty_string
"#;

    let app: CommandWrap = serde_yaml::from_str(CLAP_YAML).expect("fail to make yaml");
    assert_eq!(app.get_name(), "app_clap_serde");

    let args = app.get_arguments().collect::<Vec<_>>();
    let vp_i: ValueParser = clap::value_parser!(i64).into();
    let vp_s: ValueParser = clap::builder::NonEmptyStringValueParser::default().into();
    assert!(args.iter().any(|x| x.get_id() == "apple"
        && x.get_short() == Some('a')
        && x.get_value_parser().type_id() == vp_i.type_id()));
    assert!(args.iter().any(|x| x.get_id() == "banana"
        && x.get_short() == Some('b')
        && x.get_value_parser().type_id() == vp_s.type_id()));
}

#[test]
fn name_json() {
    const CLAP_JSON: &str = "{ \"name\" : \"app_clap_serde\" }";
    let app: Command = serde_json::from_str::<CommandWrap>(CLAP_JSON)
        .expect("parse failed")
        .into();
    assert_eq!(app.get_name(), "app_clap_serde");
}

#[test]
fn name_toml() {
    const CLAP_TOML: &str = "name = \"app_clap_serde\"";
    let app: Command = toml::from_str::<CommandWrap>(CLAP_TOML)
        .expect("parse failed")
        .into();
    assert_eq!(app.get_name(), "app_clap_serde");
}

#[test]
fn infos_json() {
    const NAME_JSON: &str = r#"{ "name" : "app_clap_serde", "version" : "1.0" , "author" : "aobat", "about" : "test-clap-serde" }"#;
    let app: Command = serde_json::from_str::<CommandWrap>(NAME_JSON)
        .expect("parse failed")
        .into();
    assert_eq!(app.get_name(), "app_clap_serde");
    assert_eq!(about(&app).as_deref(), Some("test-clap-serde"));
}

#[test]
fn infos_toml() {
    const CLAP_TOML: &str = r#"
name = "app_clap_serde"
version = "1.0"
author = "aobat"
about = "test-clap-serde"
"#;
    let app: Command = toml::from_str::<CommandWrap>(CLAP_TOML)
        .expect("parse failed")
        .into();
    assert_eq!(app.get_name(), "app_clap_serde");
    assert_eq!(about(&app).as_deref(), Some("test-clap-serde"));
}

#[test]
fn subcommands_json() {
    const CLAP_JSON: &str = r#"{
        "name" : "app_clap_serde", 
        "version" : "1.0" , 
        "author" : "aobat", 
        "about" : "test-clap-serde", 
        "subcommands" : [
            {"sub1" : {"about" : "subcommand_1"}},
            {"sub2" : {"about" : "subcommand_2"}}
        ]}"#;
    let app: Command = serde_json::from_str::<CommandWrap>(CLAP_JSON)
        .expect("parse failed")
        .into();
    assert_eq!(app.get_name(), "app_clap_serde");
    assert_eq!(about(&app).as_deref(), Some("test-clap-serde"));
    let subs = app.get_subcommands().collect::<Vec<_>>();
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub1" && about(x).as_deref() == Some("subcommand_1")));
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub2" && about(x).as_deref() == Some("subcommand_2")));
}

#[test]
fn subcommands_toml() {
    const CLAP_TOML: &str = r#"
name = "app_clap_serde"
version = "1.0"
author = "aobat"
about = "test-clap-serde"
[subcommands]
sub1 = { about = "subcommand_1" }
[subcommands.sub2]
about = "subcommand_2"
"#;
    let app: Command = toml::from_str::<CommandWrap>(CLAP_TOML)
        .expect("parse failed")
        .into();
    assert_eq!(app.get_name(), "app_clap_serde");
    assert_eq!(about(&app).as_deref(), Some("test-clap-serde"));
    let subs = app.get_subcommands().collect::<Vec<_>>();
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub1" && about(x).as_deref() == Some("subcommand_1")));
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub2" && about(x).as_deref() == Some("subcommand_2")));
}

#[test]
fn args_map_json() {
    const ARGS_JSON: &str = r#"{
        "name" : "app_clap_serde", 
        "args_map" : {
            "apple" : {"short" : "a" },
            "banana" : {"short" : "b", "long" : "banana", "aliases" : [ "musa_spp" ]}
        }
        }"#;
    let app: Command = serde_json::from_str::<CommandWrap>(ARGS_JSON)
        .expect("parse failed")
        .into();
    assert_eq!(app.get_name(), "app_clap_serde");
    let args = app.get_arguments().collect::<Vec<_>>();
    assert!(args
        .iter()
        .any(|x| x.get_id() == "apple" && x.get_short() == Some('a')));
    assert!(args.iter().any(|x| x.get_id() == "banana"
        && x.get_short() == Some('b')
        && x.get_long() == Some("banana")));
}

#[test]
fn args_json() {
    const ARGS_JSON: &str = r#"{
        "name" : "app_clap_serde", 
        "args" : [
            { "apple" : {"short" : "a" } },
            { "banana" : {"short" : "b", "long" : "banana", "aliases" : [ "musa_spp" ]}}
        ]
        }"#;
    let app: Command = serde_json::from_str::<CommandWrap>(ARGS_JSON)
        .expect("parse failed")
        .into();
    assert_eq!(app.get_name(), "app_clap_serde");
    let args = app.get_arguments().collect::<Vec<_>>();
    assert!(args
        .iter()
        .any(|x| x.get_id() == "apple" && x.get_short() == Some('a')));
    assert!(args.iter().any(|x| x.get_id() == "banana"
        && x.get_short() == Some('b')
        && x.get_long() == Some("banana")));
}

#[test]
fn args_toml() {
    const CLAP_TOML: &str = r#"
        name = "app_clap_serde"
        version = "1.0"
        author = "aobat"
        about = "test-clap-serde"
        [subcommands]
        sub1 = { about = "subcommand_1" }
        [subcommands.sub2]
        about = "subcommand_2"
        [args]
        apple = { short = "a" }
        banana = { short = "b", long = "banana", aliases = ["musa_spp"] }
    "#;
    let app: Command = toml::from_str::<CommandWrap>(CLAP_TOML)
        .expect("parse failed")
        .into();
    assert_eq!(app.get_name(), "app_clap_serde");
    assert_eq!(about(&app).as_deref(), Some("test-clap-serde"));
    let subs = app.get_subcommands().collect::<Vec<_>>();
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub1" && about(x).as_deref() == Some("subcommand_1")));
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub2" && about(x).as_deref() == Some("subcommand_2")));
    let args = app.get_arguments().collect::<Vec<_>>();
    assert!(args
        .iter()
        .any(|x| x.get_id() == "apple" && x.get_short() == Some('a')));
    assert!(args.iter().any(|x| x.get_id() == "banana"
        && x.get_short() == Some('b')
        && x.get_long() == Some("banana")));
}

#[test]
fn groups_toml() {
    const CLAP_TOML: &str = r#"
        name = "app_clap_serde"
        version = "1.0"
        author = "aobat"
        about = "test-clap-serde"
        [subcommands]
        sub1 = { about = "subcommand_1" }
        [subcommands.sub2]
        about = "subcommand_2"
        [args]
        apple = { short = "a" }
        banana = { short = "b", long = "banana", aliases = ["musa_spp"] }
        [groups]
        fruit = { args = ["apple", "banana"] }
    "#;
    let app: Command = toml::from_str::<CommandWrap>(CLAP_TOML)
        .expect("parse failed")
        .into();
    assert_eq!(app.get_name(), "app_clap_serde");
    assert_eq!(about(&app).as_deref(), Some("test-clap-serde"));
}

#[test]
fn arg_fail() {
    use clap::{Arg, Command};
    use serde::de::DeserializeSeed;

    const CLAP_TOML: &str = r#"
name = "app_clap_serde"
version = "1.0"
author = "aobat"
about = "test-clap-serde"
[args]
apple = { short = "not a char" }
"#;
    let app = Command::new("app").arg(Arg::new("apple").default_value("aaa"));
    let wrap = CommandWrap::from(app);
    let de = toml::Deserializer::parse(CLAP_TOML).unwrap();
    assert!(wrap.deserialize(de).is_err());
}

#[test]
fn serialize() {
    let c = Command::new("ser_test").about("testing _ser");
    let command: CommandWrapRef = (&c).into();
    let json = serde_json::to_string(&command).unwrap();
    let command2: CommandWrap = serde_json::from_str(&json).unwrap();
    assert_eq!(command.get_name(), command2.get_name());
}

#[test]
fn serialize_all() {
    let c = Command::new("ser_test").about("testing _ser");
    let command: CommandWrapRef<NoSkip> = (&c).into();
    let json = serde_json::to_string(&command).unwrap();
    let command2: CommandWrap = serde_json::from_str(&json).unwrap();
    assert_eq!(command.get_name(), command2.get_name());
}

fn rich_command() -> Command {
    use clap::{Arg, ArgAction, ArgGroup, ValueHint};
    Command::new("rich")
        .version("1.2.3")
        .author("someone")
        .about("about text")
        .long_about("long about text")
        .after_help("after")
        .visible_alias("r")
        .alias("hidden_alias")
        .arg_required_else_help(true)
        .propagate_version(true)
        .next_help_heading("HEADING")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("verbosity")
                .visible_alias("loud")
                .alias("noisy")
                .visible_short_alias('L')
                .short_alias('w')
                .action(ArgAction::Count)
                .global(true),
        )
        .arg(
            Arg::new("count")
                .long("count")
                .value_parser(clap::value_parser!(u32))
                .num_args(1..=3)
                .value_names(["N", "M"])
                .value_delimiter(',')
                .default_values(["1", "2"])
                .required(true),
        )
        .arg(
            Arg::new("mode")
                .long("mode")
                .value_parser(["fast", "slow"])
                .ignore_case(true),
        )
        .arg(
            Arg::new("file")
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .value_hint(ValueHint::FilePath)
                .index(1),
        )
        .arg(Arg::new("flag").long("flag").action(ArgAction::SetTrue))
        .group(ArgGroup::new("grp").args(["mode", "flag"]).multiple(true))
        .subcommand(
            Command::new("sub")
                .about("sub about")
                .short_flag('S')
                .long_flag("sub")
                .arg(Arg::new("x").long("x")),
        )
}

#[test]
fn serialize_roundtrip() {
    let c = rich_command();
    let json = serde_json::to_value(CommandWrapRef::new(&c)).unwrap();
    let c2: Command = serde_json::from_value::<CommandWrap>(json.clone())
        .unwrap()
        .into();
    let json2 = serde_json::to_value(CommandWrapRef::new(&c2)).unwrap();
    assert_eq!(json, json2);

    // check some values are actually serialized
    assert_eq!(json["about"], "about text");
    assert_eq!(json["visible_aliases"], serde_json::json!(["r"]));
    assert_eq!(json["aliases"], serde_json::json!(["hidden_alias"]));
    assert_eq!(json["arg_required_else_help"], true);
    assert_eq!(
        json["groups"]["grp"]["args"],
        serde_json::json!(["mode", "flag"])
    );
    let args = json["args"].as_array().unwrap();
    let verbose = &args[0]["verbose"];
    assert_eq!(verbose["action"], "count");
    assert_eq!(verbose["short_aliases"], serde_json::json!(["w"]));
    assert_eq!(verbose["visible_short_aliases"], serde_json::json!(["L"]));
    let count = &args[1]["count"];
    assert_eq!(count["num_args"], serde_json::json!({"min": 1, "max": 3}));
    assert_eq!(count["value_parser"], "u32");
    assert_eq!(count["default_values"], serde_json::json!(["1", "2"]));
    let mode = &args[2]["mode"];
    assert_eq!(mode["possible_values"], serde_json::json!(["fast", "slow"]));
    assert!(mode.get("value_parser").is_none());
    let file = &args[3]["file"];
    assert_eq!(file["value_parser"], "path_buf");
    assert_eq!(file["value_hint"], "file_path");
    let flag = &args[4]["flag"];
    assert_eq!(flag["action"], "set_true");
    assert_eq!(
        json["subcommands"][0]["sub"]["short_flag"],
        serde_json::json!("S")
    );

    // the deserialized command should work
    let m = c2
        .try_get_matches_from(["rich", "-vv", "--count", "1,2", "--mode", "FAST", "a.txt"])
        .unwrap();
    assert_eq!(m.get_count("verbose"), 2);
    assert_eq!(
        m.get_many::<u32>("count")
            .unwrap()
            .copied()
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    // `PossibleValuesParser` keeps the input as it is even with `ignore_case`
    assert_eq!(m.get_one::<String>("mode").unwrap(), "FAST");
    assert_eq!(
        m.get_one::<std::path::PathBuf>("file").unwrap(),
        std::path::Path::new("a.txt")
    );
}

#[test]
fn serialize_roundtrip_built() {
    // after `build`, auto generated `help` / `version` args are included
    let mut c = rich_command();
    c.build();
    let json = serde_json::to_value(CommandWrapRef::new(&c)).unwrap();
    let mut c2: Command = serde_json::from_value::<CommandWrap>(json.clone())
        .unwrap()
        .into();
    c2.build();
    let json2 = serde_json::to_value(CommandWrapRef::new(&c2)).unwrap();
    assert_eq!(json, json2);
    // generated args / subcommand are not serialized
    let ids = json["args"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|a| a.as_object().unwrap().keys())
        .collect::<Vec<_>>();
    assert!(!ids.iter().any(|id| *id == "help" || *id == "version"));
    assert_eq!(json["subcommands"].as_array().unwrap().len(), 1);
}

#[test]
fn serialize_roundtrip_no_skip() {
    let c = rich_command();
    let json = serde_json::to_value(CommandWrapRef::new(&c).with_config(NoSkip)).unwrap();
    assert_eq!(json["long_flag"], serde_json::Value::Null);
    assert_eq!(json["hide"], false);
    let c2: Command = serde_json::from_value::<CommandWrap>(json.clone())
        .unwrap()
        .into();
    let json2 = serde_json::to_value(CommandWrapRef::new(&c2).with_config(NoSkip)).unwrap();
    assert_eq!(json, json2);
}

#[test]
fn serialize_yaml_toml() {
    let c = rich_command();
    let wrap = CommandWrapRef::new(&c);
    let json = serde_json::to_value(&wrap).unwrap();

    let yaml = serde_yaml::to_string(&wrap).unwrap();
    let from_yaml: CommandWrap = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(serde_json::to_value(&from_yaml).unwrap(), json);

    let toml = toml::to_string(&wrap).unwrap();
    let from_toml: CommandWrap = toml::from_str(&toml).unwrap();
    assert_eq!(serde_json::to_value(&from_toml).unwrap(), json);
}

#[test]
fn clap4_arg_keys() {
    const CLAP_TOML: &str = r#"
name = "app"
[[args]]
[args.opt]
long = "opt"
num_args = { min = 0, max = 2 }
possible_values = ["a", "b"]
default_value_if = ["flag", "true", "a"]
[[args]]
[args.flag]
long = "flag"
arg_action = "set_true"
[[args]]
[args.rest]
num_args = { min = 1 }
trailing_var_arg = true
allow_hyphen_values = true
"#;
    let app: Command = toml::from_str::<CommandWrap>(CLAP_TOML).unwrap().into();
    let m = app
        .clone()
        .try_get_matches_from(["app", "--flag", "x", "-y"])
        .unwrap();
    assert_eq!(m.get_one::<String>("opt").unwrap(), "a");
    assert_eq!(
        m.get_many::<String>("rest").unwrap().collect::<Vec<_>>(),
        ["x", "-y"]
    );
    assert!(app
        .try_get_matches_from(["app", "--opt", "c", "x"])
        .is_err());
}

#[test]
fn null_is_ignored() {
    const CLAP_JSON: &str = r#"{
        "name": "app",
        "about": null,
        "args": [ { "a": { "long": null, "short": "a", "action": null } } ]
    }"#;
    let app: Command = serde_json::from_str::<CommandWrap>(CLAP_JSON)
        .unwrap()
        .into();
    assert!(app.get_about().is_none());
    let a = app.get_arguments().next().unwrap();
    assert_eq!(a.get_short(), Some('a'));
    assert_eq!(a.get_long(), None);
}

#[test]
#[cfg(feature = "allow-deprecated")]
fn settings_compat() {
    const CLAP_YAML: &str = r#"
name: app
setting: subcommand_required
settings:
    - propagate_version
    - disable_help_subcommand
"#;
    let app: Command = serde_yaml::from_str::<CommandWrap>(CLAP_YAML)
        .unwrap()
        .into();
    assert!(app.is_subcommand_required_set());
    assert!(app.is_propagate_version_set());
    assert!(app.is_disable_help_subcommand_set());

    assert!(serde_yaml::from_str::<CommandWrap>("name: app\nsetting: no_such_setting").is_err());
}

#[test]
fn removed_keys_are_error() {
    for key in ["takes_value", "multiple_values", "min_values"] {
        let json = format!(r#"{{"name": "app", "args": [{{"a": {{"{key}": true}}}}]}}"#);
        assert!(serde_json::from_str::<CommandWrap>(&json).is_err(), "{key}");
    }
}

#[test]
#[cfg(feature = "override-arg")]
fn override_arg_adds_missing() {
    use clap::Arg;
    use serde::de::DeserializeSeed;
    const CLAP_JSON: &str = r#"{ "args": [
        { "existing": { "short": "e" } },
        { "new_one": { "short": "n" } }
    ] }"#;
    let app = Command::new("app").arg(Arg::new("existing").default_value("x"));
    let mut de = serde_json::Deserializer::from_str(CLAP_JSON);
    let app: Command = CommandWrap::from(app).deserialize(&mut de).unwrap().into();
    let existing = app
        .get_arguments()
        .find(|a| a.get_id() == "existing")
        .unwrap();
    assert_eq!(existing.get_short(), Some('e'));
    assert_eq!(existing.get_default_values(), ["x"]);
    let new_one = app
        .get_arguments()
        .find(|a| a.get_id() == "new_one")
        .unwrap();
    assert_eq!(new_one.get_short(), Some('n'));
}

// ---- regression tests for the review of the clap 4 port ----

/// Round trip through a JSON string.
/// (`serde_json::Value` is not used because it sorts the keys and hides the order dependent bugs.)
fn roundtrip<C: crate::SerializeConfig + Default>(c: &Command) -> Command {
    let json = serde_json::to_string(&CommandWrapRef::<C>::from(c)).unwrap();
    serde_json::from_str::<CommandWrap>(&json).unwrap().into()
}

/// Round trip through a YAML string.
fn roundtrip_yaml<C: crate::SerializeConfig + Default>(c: &Command) -> Command {
    let yaml = serde_yaml::to_string(&CommandWrapRef::<C>::from(c)).unwrap();
    serde_yaml::from_str::<CommandWrap>(&yaml).unwrap().into()
}

#[test]
fn no_skip_flag_actions_keep_working() {
    use clap::{Arg, ArgAction};
    let c = Command::new("x")
        .arg(Arg::new("v").short('v').action(ArgAction::Count))
        .arg(Arg::new("t").short('t').action(ArgAction::SetTrue))
        .arg(Arg::new("f").short('f').action(ArgAction::SetFalse));
    let json = serde_json::to_value(CommandWrapRef::new(&c).with_config(NoSkip)).unwrap();
    assert_eq!(
        json["args"][0]["v"]["value_parser"],
        serde_json::Value::Null
    );
    let c2 = roundtrip::<NoSkip>(&c);
    let m = c2.try_get_matches_from(["x", "-vv", "-t", "-f"]).unwrap();
    assert_eq!(m.get_count("v"), 2);
    assert!(m.get_flag("t"));
    assert!(!m.get_flag("f"));
}

#[test]
fn next_help_heading_does_not_leak_into_args() {
    use clap::Arg;
    let c = Command::new("x")
        .arg(Arg::new("a").long("a"))
        .next_help_heading("H")
        .arg(Arg::new("b").long("b"))
        .next_help_heading("I");
    let heading = |c: &Command, id: &str| {
        c.get_arguments()
            .find(|a| a.get_id() == id)
            .unwrap()
            .get_help_heading()
            .map(str::to_owned)
    };
    let check = |c2: Command| {
        assert_eq!(heading(&c2, "a"), None);
        assert_eq!(heading(&c2, "b").as_deref(), Some("H"));
        assert_eq!(c2.get_next_help_heading(), Some("I"));
    };
    check(roundtrip::<()>(&c));
    check(roundtrip::<NoSkip>(&c));
    check(roundtrip_yaml::<()>(&c));
    // toml writes the values before the tables, so `next_help_heading` comes before `args`.
    let toml = toml::to_string(&CommandWrapRef::new(&c)).unwrap();
    assert!(toml.find("next_help_heading").unwrap() < toml.find("[[args]]").unwrap());
    check(toml::from_str::<CommandWrap>(&toml).unwrap().into());
}

#[test]
fn indistinguishable_value_parsers_are_not_serialized() {
    use clap::builder::{BoolishValueParser, FalseyValueParser};
    use clap::Arg;
    let c = Command::new("x")
        .arg(
            Arg::new("bool")
                .long("bool")
                .value_parser(clap::value_parser!(bool)),
        )
        .arg(
            Arg::new("boolish")
                .long("boolish")
                .value_parser(BoolishValueParser::new()),
        )
        .arg(
            Arg::new("falsey")
                .long("falsey")
                .value_parser(FalseyValueParser::new()),
        );
    let json = serde_json::to_value(CommandWrapRef::new(&c)).unwrap();
    assert_eq!(json["args"][0]["bool"]["value_parser"], "bool");
    assert!(json["args"][1]["boolish"].get("value_parser").is_none());
    assert!(json["args"][2]["falsey"].get("value_parser").is_none());
}

#[test]
fn display_order_roundtrip() {
    use clap::Arg;
    let c = Command::new("x")
        .arg(Arg::new("a").long("a"))
        .arg(Arg::new("z").long("z").display_order(100))
        .arg(Arg::new("pos"))
        .subcommand(Command::new("s1"))
        .subcommand(Command::new("s2").display_order(0))
        .arg(Arg::new("b").long("b"));
    let orders = |c: &Command| {
        (
            c.get_arguments()
                .map(|a| (a.get_id().to_string(), a.get_display_order()))
                .collect::<Vec<_>>(),
            c.get_subcommands()
                .map(|s| (s.get_name().to_owned(), s.get_display_order()))
                .collect::<Vec<_>>(),
        )
    };
    assert_eq!(orders(&roundtrip::<()>(&c)), orders(&c));
    assert_eq!(orders(&roundtrip::<NoSkip>(&c)), orders(&c));
    // the implicit orders are not written
    let json = serde_json::to_value(CommandWrapRef::new(&c)).unwrap();
    assert!(json["args"][0]["a"].get("display_order").is_none());
    assert_eq!(json["args"][1]["z"]["display_order"], 100);
}

#[test]
fn next_display_order_null_resets() {
    use clap::Arg;
    let app: Command = serde_json::from_str::<CommandWrap>(
        r#"{"name": "x", "next_display_order": null, "args": [{"a": {"long": "a"}}]}"#,
    )
    .unwrap()
    .into();
    assert_eq!(app.get_arguments().next().unwrap().get_display_order(), 999);
    let _ = Arg::new("unused");
}

#[test]
fn version_flag_is_not_disabled_implicitly() {
    let c = Command::new("x");
    let json = serde_json::to_value(CommandWrapRef::new(&c)).unwrap();
    assert!(json.get("disable_version_flag").is_none());
    assert!(json.get("dont_collapse_args_in_usage").is_none());
    // adding the version after deserialization should enable `--version`
    let c2 = roundtrip::<NoSkip>(&c).version("1.0");
    assert!(!c2.is_disable_version_flag_set());

    let c = Command::new("x").version("1.0").disable_version_flag(true);
    assert!(roundtrip::<()>(&c).is_disable_version_flag_set());
}

#[test]
#[cfg(feature = "allow-deprecated")]
fn clap3_settings_names() {
    const CLAP_YAML: &str = r#"
name: app
settings:
    - hidden
    - all_args_override_self
    - subcommands_negate_reqs
    - args_negate_subcommands
    - subcommand_required_else_help
    - derive_display_order
    - no_auto_help
    - no_auto_version
    - wait_on_error
    - use_long_format_for_help_subcommand
    - allow_invalid_utf8_for_external_subcommands
"#;
    let app: Command = serde_yaml::from_str::<CommandWrap>(CLAP_YAML)
        .unwrap()
        .into();
    assert!(app.is_hide_set());
    assert!(app.is_args_override_self());
    assert!(app.is_subcommand_negates_reqs_set());
    assert!(app.is_args_conflicts_with_subcommands_set());
    assert!(app.is_subcommand_required_set());
    assert!(app.is_arg_required_else_help_set());
}

#[test]
#[cfg(feature = "allow-deprecated")]
fn global_settings_propagate() {
    const CLAP_YAML: &str = r#"
name: app
global_settings: [disable_help_subcommand, hidden]
setting: subcommand_required
subcommands:
    - sub1:
        subcommands:
            - nested: {}
"#;
    let app: Command = serde_yaml::from_str::<CommandWrap>(CLAP_YAML)
        .unwrap()
        .into();
    let sub1 = app.find_subcommand("sub1").unwrap();
    let nested = sub1.find_subcommand("nested").unwrap();
    for c in [&app, sub1, nested] {
        assert!(c.is_disable_help_subcommand_set(), "{}", c.get_name());
        assert!(c.is_hide_set(), "{}", c.get_name());
    }
    // `setting` is not global
    assert!(app.is_subcommand_required_set());
    assert!(!sub1.is_subcommand_required_set());
}
