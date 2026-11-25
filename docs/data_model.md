# Data Model

- clap : 4.6
- clap-serde : 0.6.0

This page describes the overall structure of the input of [`CommandWrap`](crate::CommandWrap)
and the output of [`CommandWrapRef`](crate::CommandWrapRef).
For the list of keys, see [Command](crate::documents::app_keys), [Arg](crate::documents::arg_keys)
and [ArgGroup](crate::documents::arg_groups_keys).
The JSON Schema of the input is available with the `schema` feature (see the README).

## Structure

```text
Command := Map {
    name: String,                       // required at the top level
    <command key>: <value>, ...         // see app_keys
    args:        Seq<Map{ id: Arg }>    // or Map{ id: Arg, ... }
    args_map:    Map{ id: Arg, ... }
    groups:      Map{ id: ArgGroup, ... }
    subcommands:     Seq<Map{ name: Command }>  // or Map{ name: Command, ... }
    subcommands_map: Map{ name: Command, ... }
}
Arg      := Map { <arg key>: <value>, ... }      // see arg_keys
ArgGroup := Map { <group key>: <value>, ... }    // see arg_groups
```

### Command
- A map from keys to values. Each key calls the builder method of the same name of
  [`clap::Command`], in the order of the input.
- `name` is required for the top level command. For the subcommands, the name is taken from the key
  (see below).

### args
`args` accepts both of the forms below.

- A sequence of single-entry maps `[{ id: Arg }, ...]`.
- A map `{ id: Arg, ... }`.

The key of each entry is the id of the `Arg` (`Arg::new(id)`).
The args are added in the order of the input, which is also the order in the help message.
`args_map` accepts only the map form. Use it with the formats which are not self-describing.

With the `override-arg` feature, an arg which already exists in the command
is modified instead of being replaced. See the README.

### subcommands
`subcommands` accepts the same forms as `args`, and `subcommands_map` accepts only the map form.
The key of each entry is the name of the subcommand, and the value is a `Command`
(it can have `args`, `subcommands`, etc.).

### groups
`groups` accepts only the map form `{ id: ArgGroup, ... }`.
The key of each entry is the id of the `ArgGroup`.

## Values
- Keys and enum values (e.g. [`ArgAction`](crate::documents::arg_keys#argaction)) are
  `snake_case` by default. They can be changed with the `kebab-case-key` and `pascal-case-key` features.
- `null` values are ignored, except `next_help_heading`, `next_display_order`
  and `help_heading` of `Arg`, where `null` resets the value.
- Unknown keys are errors. The deprecated keys are errors unless the `allow-deprecated` feature is enabled.

## Order of keys
The keys are applied in the order of the input, so the order matters for the keys which affect
the following ones. For example, `next_help_heading` applies to the args after it.

```rust
# #[cfg(feature = "snake-case-key")]
# {
const CLAP_YAML: &str = r#"
name: app
args:
    - input: {}
next_help_heading: Options
args_map:
    verbose: { short: v }
"#;
let app: clap::Command = serde_yaml::from_str::<clap_serde::CommandWrap>(CLAP_YAML)
    .unwrap()
    .into();
let heading = |id: &str| {
    app.get_arguments()
        .find(|a| a.get_id() == id)
        .unwrap()
        .get_help_heading()
};
assert_eq!(heading("input"), None);
assert_eq!(heading("verbose"), Some("Options"));
# }
```

Each `Arg` can also set its own heading with `help_heading`.

## Serialization
[`CommandWrapRef`](crate::CommandWrapRef) outputs the following form,
which can be deserialized again with [`CommandWrap`](crate::CommandWrap).

- `name` and the other keys of the command.
  The keys and enum values follow the case-key feature, as in the input.
- `args` as a sequence of single-entry maps.
- `groups` as a map.
- `subcommands` as a sequence of single-entry maps.
  Each subcommand also has its `name` key.

The flags with `false` and the values with `None` are skipped by default.
Use [`NoSkip`](crate::NoSkip) to output all of them.
Some properties can not be serialized because clap has no getters for them. See the README for the details.

```rust
# #[cfg(feature = "snake-case-key")]
# {
use clap::{Arg, ArgGroup, Command};
let command = Command::new("app")
    .arg(Arg::new("a").short('a'))
    .group(ArgGroup::new("g").arg("a"))
    .subcommand(Command::new("sub"));
let json = serde_json::to_value(clap_serde::CommandWrapRef::new(&command)).unwrap();
assert_eq!(
    json,
    serde_json::json!({
        "name": "app",
        "args": [{ "a": { "short": "a" } }],
        "groups": { "g": { "args": ["a"] } },
        "subcommands": [{ "sub": { "name": "sub" } }]
    })
);
# }
```
