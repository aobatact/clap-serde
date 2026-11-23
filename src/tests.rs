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
