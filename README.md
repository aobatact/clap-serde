Deserializer for clap using serde.

## toml

```
const CLAP_TOML: &'static str = r#"
name = "app_clap_serde"
version = "1.0"
author = "toml_tester"
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

let app: clap::App = toml::from_str::<clap_serde::AppWrap>(CLAP_TOML)
    .expect("parse failed")
    .into();
assert_eq!(app.get_name(), "app_clap_serde");
assert_eq!(app.get_about(), Some("test-clap-serde"));
```

## json
```
const NAME_JSON: &'static str = r#"{
"name" : "app_clap_serde", 
"version" : "1.0" , 
"author" : "json_tester", 
"about" : "test-clap-serde", 
"subcommands" : {
    "sub1" : {"about" : "subcommand_1"},
    "sub2" : {"about" : "subcommand_2"}},
"args" : {
    "apple" : {"short" : "a" },
    "banana" : {"short" : "b", "long" : "banana", "aliases" : [ "musa_spp" ]}
},
"groups" : {
    "fruit" : { "args" : ["apple", "banana"] }
}
}"#;

let app: clap::App = serde_json::from_str::<clap_serde::AppWrap>(NAME_JSON)
    .expect("parse failed")
    .into();
assert_eq!(app.get_name(), "app_clap_serde");
assert_eq!(app.get_about(), Some("test-clap-serde"));
```

## yaml
Not working because [serde_yaml](https://crates.io/crates/serde_yaml) only accepts `DeserializeOwned`.
