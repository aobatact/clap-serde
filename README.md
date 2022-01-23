Provides a wrapper to deserialize clap app using serde.

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
```
const YAML_STR: &'static str = r#"
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
let yaml = yaml_rust::Yaml::Array(yaml_rust::YamlLoader::load_from_str(YAML_STR).expect("not a yaml"));
let app = clap_serde::yaml_to_app(&yaml).expect("parse failed from yaml");
assert_eq!(app.get_name(), "app_clap_serde");
```

# features
## env
Enables env feature in clap.
## yaml 
Enables to use yaml.
## color
Enablse color feature in clap.

## (settings name letter)
Settings names format for [`AppSettings`](`clap::AppSettings`) and [`ArgSettings`](`clap::ArgSettings`).
- PascalCase (no-feature, same as variants name in enum)
- kebab-case (kebab-case-setting)
- snake_case (snake-case-setting)
