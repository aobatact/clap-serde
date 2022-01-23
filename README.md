# clap-serde
Provides a wrapper to deserialize [clap](https://crates.io/crates/clap) app using [serde](https://crates.io/crates/serde).

[![Crates.io](https://img.shields.io/crates/v/clap-serde?style=flat-square)](https://crates.io/crates/clap-serde)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/aobatact/clap-serde/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/aobatact/clap-serde/blob/main/LICENSE-MIT)
[API Reference](https://docs.rs/clap-serde)

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
const CLAP_JSON: &'static str = r#"{
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

let app: clap::App = serde_json::from_str::<clap_serde::AppWrap>(CLAP_JSON)
    .expect("parse failed")
    .into();
assert_eq!(app.get_name(), "app_clap_serde");
assert_eq!(app.get_about(), Some("test-clap-serde"));
```

## yaml
`clap-serde` provides a Deserializer for yaml. This requires `yaml` feature.
```
const CLAP_YAML: &'static str = r#"
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

"#;
let yaml = yaml_rust::Yaml::Array(yaml_rust::YamlLoader::load_from_str(CLAP_YAML).expect("not a yaml"));
let app = clap_serde::yaml_to_app(&yaml).expect("parse failed from yaml");
assert_eq!(app.get_name(), "app_clap_serde");
```

# features
## env
Enables env feature in clap.
## yaml
Enables to use yaml. Enabled by default.
## color
Enablse color feature in clap.

## (settings name letter)
Settings names format for [`AppSettings`](`clap::AppSettings`).
#### pascal-case-setting 
PascalCase. Same as variants name in enum.
#### kebab-case-setting 
kebab-case. Enabled by default.
#### snake-case-setting 
snake_case.
