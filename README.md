# clap-serde
Provides a wrapper to deserialize [clap](https://crates.io/crates/clap) app using [serde](https://crates.io/crates/serde).

[![Crates.io](https://img.shields.io/crates/v/clap-serde?style=flat-square)](https://crates.io/crates/clap-serde)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/aobatact/clap-serde/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/aobatact/clap-serde/blob/main/LICENSE-MIT)
[![API Reference](https://img.shields.io/docsrs/clap-serde?style=flat-square)](https://docs.rs/clap-serde)

## toml

```rust
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
```rust
const CLAP_JSON: &'static str = r#"{
"name" : "app_clap_serde", 
"version" : "1.0" , 
"author" : "json_tester", 
"about" : "test-clap-serde", 
"subcommands" : [
    { "sub1" : {"about" : "subcommand_1"}},
    { "sub2" : {"about" : "subcommand_2"}}
],
"args" : [
    { "apple" : {"short" : "a" } },
    { "banana" : {"short" : "b", "long" : "banana", "aliases" : [ "musa_spp" ]} }
],
"groups" : {
    "fruit" : { "args" : ["apple", "banana"] }
}
}"#;

let app: clap::App = serde_json::from_str::<clap_serde::CommandWrap>(CLAP_JSON)
    .expect("parse failed")
    .into();
assert_eq!(app.get_name(), "app_clap_serde");
assert_eq!(app.get_about(), Some("test-clap-serde"));
```

## yaml
```rust
const CLAP_YAML: &'static str = r#"
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

"#;
let app: clap_serde::CommandWrap = serde_yaml::from_str(CLAP_YAML).expect("fail to make yaml");
assert_eq!(app.get_name(), "app_clap_serde");
```

# features
## env
Enables env feature in clap.
## yaml (deprecated, use serde-yaml instead)
Enables to use yaml.
## color
Enablse color feature in clap.

## (key case settings)
Settings names format for keys and [`AppSettings`](`clap::AppSettings`).
#### snake-case-key
snake_case. Enabled by default.
#### pascal-case-key
PascalCase. Same as variants name in enum at `AppSettings`.
#### kebab-case-key 
kebab-case.

## allow-deprecated
Allow deprecated keys, settings. Enabled by default.

## override-args

Override a `Arg` with `DeserializeSeed`.

```rust
# #[cfg(feature = "override-arg")]
# {
# use clap::{Command, Arg};
use serde::de::DeserializeSeed;

const CLAP_TOML: &str = r#"
name = "app_clap_serde"
version = "1.0"
author = "aobat"
about = "test-clap-serde"
[args]
apple = { short = "a" }
"#;
let app = Command::new("app").arg(Arg::new("apple").default_value("aaa"));
let wrap = clap_serde::CommandWrap::from(app);
let mut de = toml::Deserializer::new(CLAP_TOML);
let wrap2 = wrap.deserialize(&mut de).unwrap();
let apple = wrap2
    .get_arguments()
    .find(|a| a.get_id() == "apple")
    .unwrap();
assert!(apple.get_short() == Some('a'));
assert!(apple.get_default_values() == ["aaa"]);
# }
```
