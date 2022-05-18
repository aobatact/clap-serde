use crate::CommandWrap;
use clap::Command;

#[cfg(feature = "yaml")]
#[test]
fn name_yaml() {
    use yaml_rust::Yaml;

    const NAME_YAML: &str = "name: app_clap_serde\n";
    let yaml =
        Yaml::Array(yaml_rust::YamlLoader::load_from_str(NAME_YAML).expect("fail to make yaml"));
    let app = crate::load(crate::yaml::YamlWrap::new(&yaml)).expect("parse failed");
    assert_eq!(app.get_name(), "app_clap_serde");
}

#[cfg(feature = "yaml")]
#[test]
fn test_yaml() {
    use yaml_rust::Yaml;

    const NAME_YAML: &str = r#"
name: app_clap_serde
version : "1.0"
about : yaml_support!
author : yaml_supporter

args:
    - apple : 
        - short: a
    - banana:
        - short: b
        - long: banana
        - aliases :
            - musa_spp

subcommands:
    - sub1: 
        - about : subcommand_1
    - sub2: 
        - about : subcommand_2

setting: trailing_var_arg

settings: 
    - help_expected

global_setting: color_auto
global_setting: no_binary_name

global_settings: 
    - next_line_help
    - propagate_version
"#;
    let yaml =
        Yaml::Array(yaml_rust::YamlLoader::load_from_str(NAME_YAML).expect("fail to make yaml"));
    let app = crate::load(crate::yaml::YamlWrap::new(&yaml)).expect("parse failed");
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
        "subcommands" : {
            "sub1" : {"about" : "subcommand_1"},
            "sub2" : {"about" : "subcommand_2"}
        }}"#;
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
fn args_json() {
    const NAME_JSON: &str = r#"{
        "name" : "app_clap_serde", 
        "version" : "1.0" , 
        "author" : "aobat", 
        "about" : "test-clap-serde", 
        "subcommands" : {
            "sub1" : {"about" : "subcommand_1"},
            "sub2" : {"about" : "subcommand_2"}},
        "args" : {
            "apple" : {"short" : "a" },
            "banana" : {"short" : "b", "long" : "banana", "aliases" : [ "musa_spp" ]}
        }
        }"#;
    let app: Command = serde_json::from_str::<CommandWrap>(NAME_JSON)
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
