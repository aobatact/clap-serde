# Change Log

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
