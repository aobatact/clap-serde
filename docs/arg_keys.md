# Arg Key List
- clap : 4.6
- clap-serde : 0.6.0

(Keys can be changed by the case-key features. `null` values are ignored.)

|key | type|feature|
|-|-|-|
|action | [`ArgAction`](#argaction)||
|alias|`String`||
|aliases|`Vec<String>`||
|allow_hyphen_values|`bool`||
|allow_negative_numbers|`bool`||
|conflicts_with|`String`||
|conflicts_with_all|`Vec<String>`||
|default_missing_value|`String`||
|default_missing_values|`Vec<String>`||
|default_value|`String`||
|default_value_if | `[arg, value or null, default or null]` (`null` value means "if present") ||
|default_value_ifs|`Vec<[arg, value or null, default or null]>` ||
|default_values|`Vec<String>`||
|display_order|`usize`||
|env | `String`|env|
|exclusive|`bool`||
|global|`bool`||
|group|`String`||
|groups|`Vec<String>`||
|help|`String`||
|help_heading|`String` (empty string or `null` means no heading, overriding `next_help_heading` of the command)||
|hide|`bool`||
|hide_default_value|`bool`||
|hide_env | `bool`|env|
|hide_env_values | `bool`|env|
|hide_long_help|`bool`||
|hide_possible_values|`bool`||
|hide_short_help|`bool`||
|id|`String`||
|ignore_case|`bool`||
|index|`usize`||
|last|`bool`||
|long|`String`||
|long_help|`String`||
|next_line_help|`bool`||
|num_args|`usize` or `{ min = usize, max = usize }` (`max` can be omitted for unbounded)||
|number_of_values|`usize`||
|overrides_with|`String`||
|overrides_with_all|`Vec<String>`||
|positional|`bool` (ignored, only output by serialization)||
|possible_values|`Vec<String>` (sets `PossibleValuesParser`)||
|raw|`bool`||
|require_equals|`bool`||
|required|`bool`||
|required_if_eq| `(String, String)`||
|required_if_eq_all|`Vec<(String, String)>`||
|required_if_eq_any|`Vec<(String, String)>`||
|required_unless_present|`String`||
|required_unless_present_all|`Vec<String>`||
|required_unless_present_any|`Vec<String>`||
|requires|`String`||
|requires_all|`Vec<String>`||
|requires_if | `(String, String)` ||
|requires_ifs|`Vec<(String, String)>`||
|short|`char`||
|short_alias|`char`||
|short_aliases|`Vec<char>`||
|trailing_var_arg|`bool`||
|use_value_delimiter|`bool`||
|value_delimiter|`char`||
|value_hint | [`ValueHint`](#valuehint)||
|value_name|`String`||
|value_names|`Vec<String>`||
|value_parser|[`ValueParser`](#valueparser)||
|value_terminator|`String`||
|visible_alias|`String`||
|visible_aliases|`Vec<String>`||
|visible_short_alias|`char`||
|visible_short_aliases|`Vec<char>`||

Keys removed in clap 4 (`takes_value`, `multiple_values`, `multiple_occurrences`, `min_values`,
`max_values`, `max_occurrences`, `allow_invalid_utf8`, `forbid_empty_values`, `possible_value`,
`require_value_delimiter`) are errors. Use `num_args`, `action`, `value_parser` and `possible_values` instead.

With `allow-deprecated`, `name` → `id`, `arg_action` → `action`,
`use_delimiter` / `require_delimiter` → `use_value_delimiter` are accepted.

## ArgAction
- set
- append
- set_true
- set_false
- count
- help
- help_short
- help_long
- version

## ValueHint
- unknown
- other
- any_path
- file_path
- dir_path
- executable_path
- command_name
- command_string
- command_with_arguments
- username
- hostname
- url
- email_address

## ValueParser
One of the names below, or a map with `type` for the integer types with range
(`{ type = "i64", min = -10, max = 10, max_inclusive = true }`).

- bool
- string
- os_string
- path_buf
- boolish
- falsey
- non_empty_string
- i64, i32, i16, i8, u64, u32, u16, u8
