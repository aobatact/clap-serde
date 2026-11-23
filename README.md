# clap-serde
Provides wrappers to deserialize and serialize [clap](https://crates.io/crates/clap) (v4) `Command` using [serde](https://crates.io/crates/serde).

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

let app: clap::Command = toml::from_str::<clap_serde::CommandWrap>(CLAP_TOML)
    .expect("parse failed")
    .into();
assert_eq!(app.get_name(), "app_clap_serde");
assert_eq!(app.get_about().unwrap().to_string(), "test-clap-serde");
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

let app: clap::Command = serde_json::from_str::<clap_serde::CommandWrap>(CLAP_JSON)
    .expect("parse failed")
    .into();
assert_eq!(app.get_name(), "app_clap_serde");
assert_eq!(app.get_about().unwrap().to_string(), "test-clap-serde");
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

## serialize

`Command` can be serialized with [`CommandWrapRef`](crate::CommandWrapRef),
and the output can be deserialized again with [`CommandWrap`](crate::CommandWrap).

```rust
use clap::{Arg, ArgAction, Command};
use clap_serde::{CommandWrap, CommandWrapRef};

let command = Command::new("app")
    .about("an app")
    .arg(Arg::new("verbose").short('v').action(ArgAction::Count))
    .subcommand(Command::new("sub").about("a subcommand"));

let json = serde_json::to_string(&CommandWrapRef::new(&command)).unwrap();
let command2: Command = serde_json::from_str::<CommandWrap>(&json).unwrap().into();
assert_eq!(command2.get_name(), "app");
assert_eq!(command2.get_subcommands().count(), 1);
```

By default, the flags with `false` and the values with `None` are skipped.
Use [`NoSkip`](crate::NoSkip) to serialize all of them.

Some properties are not serialized because clap does not provide getters for them
(e.g. `requires`, `conflicts_with`, `default_value_if`).

`value_parser` is identified only by the type of the parsed value, because clap does not expose more.
So `boolish` / `falsey` and custom parsers of unknown types are not serialized, and
`non_empty_string`, the ranges of integer parsers and custom parsers of the known types
are serialized as the basic parser of the type (e.g. `string`, `i64`).

# features
## env
Enables env feature in clap.
## help
Enables help feature in clap (needed for `help_template`). Enabled by default.
## yaml (deprecated, use serde-yaml instead)
Enables to use yaml.
## color
Enablse color feature in clap.

## (key case settings)
Settings names format for keys and enum values (e.g. `ArgAction`, `ValueHint`).
#### snake-case-key
snake_case. Enabled by default.
#### pascal-case-key
PascalCase. Same as variants name in enums.
#### kebab-case-key 
kebab-case.

## allow-deprecated
Allow deprecated keys, settings. Enabled by default.

`setting` / `settings` / `global_setting` / `global_settings` (clap 3 `AppSettings`) are
converted to the corresponding methods of `Command` (e.g. `settings: [arg_required_else_help]`
calls `Command::arg_required_else_help(true)`). `global_setting(s)` are also applied to all the subcommands.
See [the key list](crate::documents::app_keys) for the supported names.

## override-arg

Override a `Arg` with `DeserializeSeed`. If the `Arg` does not exist, a new one is added.

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
let de = toml::Deserializer::parse(CLAP_TOML).unwrap();
let wrap2 = wrap.deserialize(de).unwrap();
let apple = wrap2
    .get_arguments()
    .find(|a| a.get_id() == "apple")
    .unwrap();
assert!(apple.get_short() == Some('a'));
assert!(apple.get_default_values() == ["aaa"]);
# }
```
