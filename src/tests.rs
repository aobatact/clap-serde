use crate::CommandWrap;
use clap::{builder::ValueParser, Command};

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
        .any(|x| x.get_name() == "sub1" && x.get_about() == Some("subcommand_1")));
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub2" && x.get_about() == Some("subcommand_2")));
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
            type: non_empty_string
    - banana:
        short: b
        long: banana
        value_parser: non_empty_string
"#;

    let app: CommandWrap = serde_yaml::from_str(CLAP_YAML).expect("fail to make yaml");
    assert_eq!(app.get_name(), "app_clap_serde");

    let args = app.get_arguments().collect::<Vec<_>>();
    let vp: ValueParser = clap::builder::NonEmptyStringValueParser::default().into();
    assert!(args.iter().any(|x| x.get_id() == "apple"
        && x.get_short() == Some('a')
        && x.get_value_parser().type_id() == vp.type_id()));
    assert!(args.iter().any(|x| x.get_id() == "banana"
        && x.get_short() == Some('b')
        && x.get_long() == Some("banana")));
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
    assert_eq!(app.get_about(), Some("test-clap-serde"));
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
    assert_eq!(app.get_about(), Some("test-clap-serde"));
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
    assert_eq!(app.get_about(), Some("test-clap-serde"));
    let subs = app.get_subcommands().collect::<Vec<_>>();
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub1" && x.get_about() == Some("subcommand_1")));
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub2" && x.get_about() == Some("subcommand_2")));
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
    assert_eq!(app.get_about(), Some("test-clap-serde"));
    let subs = app.get_subcommands().collect::<Vec<_>>();
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub1" && x.get_about() == Some("subcommand_1")));
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub2" && x.get_about() == Some("subcommand_2")));
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
    assert_eq!(app.get_about(), Some("test-clap-serde"));
    let subs = app.get_subcommands().collect::<Vec<_>>();
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub1" && x.get_about() == Some("subcommand_1")));
    assert!(subs
        .iter()
        .any(|x| x.get_name() == "sub2" && x.get_about() == Some("subcommand_2")));
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
    assert_eq!(app.get_about(), Some("test-clap-serde"));
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
apple = { short =  }
"#;
    let app = Command::new("app").arg(Arg::new("apple").default_value("aaa"));
    let wrap = CommandWrap::from(app);
    let mut de = toml::Deserializer::new(CLAP_TOML);
    assert!(wrap.deserialize(&mut de).is_err());
}
