# Command Key List
- clap : 4.6
- clap-serde : 0.6.0

(Keys can be changed by the case-key features. `null` values are ignored.)

| key | type | feature |
| - | - | - |
| about | `String` | |
| after_help | `String` | |
| after_long_help | `String` | |
| alias | `String` | |
| aliases | `Vec<String>` | |
| allow_external_subcommands | `bool` | |
| allow_hyphen_values | `bool` | |
| allow_missing_positional | `bool` | |
| allow_negative_numbers | `bool` | |
| (arg) | not supported single arg (now) | |
| arg_required_else_help | `bool` | |
| args | `Vec<Map<String, `[Arg](`crate::documents::arg_keys`)`>>` | |
| args_map | `Map<String, `[Arg](`crate::documents::arg_keys`)`>` | |
| args_conflicts_with_subcommands | `bool` | |
| args_override_self | `bool` | |
| author | `String` | |
| before_help | `String` | |
| before_long_help | `String` | |
| bin_name | `String` | |
| color | [`ColorChoice`](#colorchoice) | color |
| disable_colored_help | `bool` | |
| disable_help_flag | `bool` | |
| disable_help_subcommand | `bool` | |
| disable_version_flag | `bool` | |
| display_name | `String` | |
| display_order | `usize` | |
| dont_collapse_args_in_usage | `bool` | |
| dont_delimit_trailing_values | `bool` | |
| flatten_help | `bool` | |
| global_setting | [`Settings`](#settings) | allow-deprecated |
| global_settings | `Vec<`[`Settings`](#settings)`>` | allow-deprecated |
| (group) | not supported single group (now) | |
| groups | `Map<String, `[ArgGroup](`crate::documents::arg_groups_keys`)`>` | |
| help_expected | `bool` | |
| help_template | `String` | help |
| hide | `bool` | |
| hide_possible_values | `bool` | |
| ignore_errors | `bool` | |
| infer_long_args | `bool` | |
| infer_subcommands | `bool` | |
| long_about | `String` | |
| long_flag | `String` | |
| long_flag_alias | `String` | |
| long_flag_aliases | `Vec<String>` | |
| long_version | `String` | |
| max_term_width | `usize` | |
| multicall | `bool` | |
| name | `String` | |
| next_display_order | `usize` | |
| next_help_heading | `String` | |
| next_line_help | `bool` | |
| no_binary_name | `bool` | |
| override_help | `String` | |
| override_usage | `String` | |
| propagate_version | `bool` | |
| setting | [`Settings`](#settings) | allow-deprecated |
| settings | `Vec<`[`Settings`](#settings)`>` | allow-deprecated |
| short_flag | `char` | |
| short_flag_alias | `char` | |
| short_flag_aliases | `Vec<char>` | |
| (subcommand) | not supported single subcommand (now) | |
| subcommands | `Vec<Map<String, Command>>` | |
| subcommands_map | `Map<String, Command>` | |
| subcommand_help_heading | `String` | |
| subcommand_negates_reqs | `bool` | |
| subcommand_precedence_over_arg | `bool` | |
| subcommand_required | `bool` | |
| subcommand_value_name | `String` | |
| term_width | `usize` | |
| trailing_var_arg | `bool` | |
| version | `String` | |
| visible_alias | `String` | |
| visible_aliases | `Vec<String>` | |
| visible_long_flag_alias | `String` | |
| visible_long_flag_aliases | `Vec<String>` | |
| visible_short_flag_alias | `char` | |
| visible_short_flag_aliases | `Vec<char>` | |

## Settings
For `setting`, `settings`, `global_setting`, `global_settings` (compatibility with clap 3 `AppSettings`).
Each name calls the method of the same name with `true`
(`global_*` are applied to the command itself, the same as `setting`).

- ignore_errors
- allow_hyphen_values
- allow_negative_numbers
- args_override_self
- allow_missing_positional
- trailing_var_arg
- dont_delimit_trailing_values
- infer_long_args
- infer_subcommands
- subcommand_required
- allow_external_subcommands
- multicall
- subcommand_negates_reqs
- args_conflicts_with_subcommands
- subcommand_precedence_over_arg
- arg_required_else_help
- dont_collapse_args_in_usage
- next_line_help
- disable_colored_help
- disable_help_flag
- disable_help_subcommand
- disable_version_flag
- propagate_version
- hide
- hide_possible_values
- help_expected
- no_binary_name

## ColorChoice
- auto
- always
- never
