use crate::AppWrap;
use clap::App;

//currently fails... beacuse serde_yaml only supports `DeserializeOwned` and no zero copy deserialization
// #[test]
// fn name_yaml() {
//     const NAME_YAML: &'static str = "name : app_clap_serde";
//     let app: App = serde_yaml::from_str::<AppWrap>(NAME_YAML).expect("parse failed").into();
//     assert_eq!(app.get_name(), "app_clap_serde");
// }

#[test]
fn name_json() {
    const NAME_JSON: &'static str = "{ \"name\" : \"app_clap_serde\" }";
    let app: App = serde_json::from_str::<AppWrap>(NAME_JSON)
        .expect("parse failed")
        .into();
    assert_eq!(app.get_name(), "app_clap_serde");
}

#[test]
fn name_toml() {
    const NAME_TOML: &'static str = "name = \"app_clap_serde\"";
    let app: App = toml::from_str::<AppWrap>(NAME_TOML)
        .expect("parse failed")
        .into();
    assert_eq!(app.get_name(), "app_clap_serde");
}

#[test]
fn infos_json() {
    const NAME_JSON: &'static str = r#"{ "name" : "app_clap_serde", "version" : "1.0" , "author" : "aobat", "about" : "test-clap-serde" }"#;
    let app: App = serde_json::from_str::<AppWrap>(NAME_JSON)
        .expect("parse failed")
        .into();
    assert_eq!(app.get_name(), "app_clap_serde");
    assert_eq!(app.get_about(), Some("test-clap-serde"));
}

#[test]
fn subcommands_json() {
    const NAME_JSON: &'static str = r#"{
        "name" : "app_clap_serde", 
        "version" : "1.0" , 
        "author" : "aobat", 
        "about" : "test-clap-serde", 
        "subcommands" : {
            "sub1" : {"about" : "subcommand_1"},
            "sub2" : {"about" : "subcommand_2"}
        }}"#;
    let app: App = serde_json::from_str::<AppWrap>(NAME_JSON)
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
    const NAME_JSON: &'static str = r#"{
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
    let app: App = serde_json::from_str::<AppWrap>(NAME_JSON)
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
    assert!(args.iter().any(|x| x.get_name() == "apple" && x.get_short() == Some('a')));
    assert!(args.iter().any(|x| x.get_name() == "banana" && x.get_short() == Some('b') && x.get_long() == Some("banana")));
}

