# App Key List
- clap : 3.0.10
- clap-serde : 0.3.0

(Keys can be changed by the case-key features)

| key | type | feature |
| - | - | - |
| about| `&str`|
| after_help| `&str`|
| after_long_help| `&str`|
| alias| `&str`|
| aliases| `Vec<&str>`|
| (arg) | not supported single arg (now)|
| args | [Args](`crate::documents::arg_keys`) |
| author| `&str`|
| before_help| `&str`|
| before_long_help| `&str`|
| bin_name| `&str`|
| color | [`ColorChoice`](#colorchoice)| color |
| display_order| `usize`|
| global_setting | [`AppSettings`](#appsettings) |
| global_settings | `Vec<`[`AppSettings`](#appsettings)`>` |
| (group) | not supported single group (now)|
| groups | [ArgGroup](`crate::documents::arg_groups_keys`) |
| help_heading| `Option<&str>`|
| help_template| `&str`|
| long_about| `&str`|
| long_flag| `&str`|
| long_flag_alias| `&str`|
| long_flag_aliases| `Vec<&str>`|
| long_version| `&str`|
| max_term_width| `usize`|
| name| `&str`|
| override_help| `&str`|
| override_usage| `&str`|
| setting | [`AppSettings`](#appsettings) |
| settings | `Vec<`[`AppSettings`](#appsettings)`>` |
| short_flag|`char`|
| short_flag_alias|`char`|
| short_flag_aliases| `Vec<char>`|
| (subcommand) | not supported single subcommand (now)|
| subcommands | `Map<&str, App>`|
| term_width| `usize`|
| version| `&str`|
| visible_alias| `&str`|
| visible_aliases| `Vec<&str>`|
| visible_long_flag_alias| `&str`|
| visible_long_flag_aliases| `Vec<&str>`|
| visible_short_flag_alias|`char`|
| visible_short_flag_aliases| `Vec<char>`|


## AppSettings
For setting, settings, global_setting, global_settings,

- ignore_errors
- wait_on_error
- allow_hyphen_values
- allow_negative_numbers
- all_args_override_self
- allow_missing_positional
- trailing_var_arg
- dont_delimit_trailing_values
- infer_long_args
- infer_subcommands
- subcommand_required
- subcommand_required_else_help
- allow_external_subcommands
- multicall
- allow_invalid_utf_8_for_external_subcommands
- use_long_format_for_help_subcommand
- subcommands_negate_reqs
- args_negate_subcommands
- subcommand_precedence_over_arg
- arg_required_else_help
- derive_display_order
- dont_collapse_args_in_usage
- next_line_help
- disable_colored_help
- disable_help_flag
- disable_help_subcommand
- disable_version_flag
- propagate_version
- hidden
- hide_possible_values
- help_expected
- no_binary_name
- no_auto_help
- no_auto_version

# ColorChoice
- auto
- always
- never
