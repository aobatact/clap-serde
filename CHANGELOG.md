# Change Log

# 0.6.0

## Breaking Changes
- Bump clap to 4.6. `CommandWrap` and `ArgWrap` no longer have lifetime parameters.
- Removed keys which are removed in clap 4 (`takes_value`, `multiple_values`, `min_values`, `max_values`, etc.).
  Use `num_args`, `action`, `value_parser` and `possible_values` instead.
- `setting(s)` / `global_setting(s)` are mapped to the methods of `Command` (only with `allow-deprecated`),
  since `AppSettings` is removed in clap 4. `global_setting(s)` are applied to all the subcommands in the input.
  The settings removed in clap 4 (e.g. `derive_display_order`) are ignored.
- The key for `ArgAction` is `action` (`arg_action` is accepted with `allow-deprecated`).
- `null` values are ignored on deserialization
  (except `next_help_heading`, `next_display_order` and `help_heading` of `Arg`, where `null` resets the value).
- An empty `help_heading` of `Arg` means no heading.

## Features
- Serialize `Command` with `CommandWrapRef` (and `CommandWrap`, `ArgWrapRef`, `ArgWrap`).
  Use `NoSkip` config to serialize all the fields.
- New keys: `num_args`, `possible_values`, `flatten_help`, `trailing_var_arg` and `allow_negative_numbers` for `Arg`,
  `help_template` (with the new `help` feature).
- `override-arg` adds the arg if it does not exist.

## Fixes
- `color` key was always disabled.

# 0.5.1

## Fixes
- Relax requirement of deserizaler for `override-arg`.
- Fix env feature flag.

# 0.5.0

## Features
- Bump Clap version to 3.2 .

# 0.4.0

## Breaking Changes

- Change schema for Argumets(`args`) and SubCommands(`subcommands`) from Hash to Array of Hash. Old behavior can be used by `args_map` and, `subcommands_map`.
- Deprecated yaml feature. Use `serde-yaml >= 0.9` instead.

## Features

- Support Clap 3.1
- Can override arguments by using `DeserizalizeSeed for CommandWrap`.
