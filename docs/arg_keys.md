# Arg Key List
- clap | 3.0.10
- clap-serde | 0.3.0

(Keys can be changed by the case-key features)

|key | type|feature|
|-|-|-|
|alias|`&str`||
|aliases|`Vec<&str>`||
|allow_hyphen_values|`bool`||
|allow_invalid_utf8|`bool`||
|conflicts_with|`&str`||
|conflicts_with_all|`Vec<&str>`||
|default_missing_value|`&str`||
|default_missing_values|`Vec<&str>`||
|default_value|`&str`||
|default_value_if | `(&str,Option<&str>,Option<&str>)` ||
|default_value_ifs|`Vec<(&str,Option<&str>,Option<&str>)>` ||
|display_order|`usize`||
|env | `&str`|env|
|exclusive|`bool`||
|forbid_empty_values|`bool`||
|global|`bool`||
|group|`&str`||
|groups|`Vec<&str>`||
|help|`&str`||
|help_heading|`&str`||
|hide|`bool`||
|hide_default_value|`bool`||
|hide_env | `bool`|env|
|hide_env_values | `bool`|env|
|hide_long_help|`bool`||
|hide_possible_values|`bool`||
|hide_short_help|`bool`||
|ignore_case|`bool`||
|index|`usize`||
|last|`bool`||
|long|`&str`||
|long_help|`&str`||
|max_occurrences|`usize`||
|max_values|`usize`||
|min_values|`usize`||
|multiple_occurrences|`bool`||
|multiple_values|`bool`||
|name|`&str`||
|next_line_help|`bool`||
|number_of_values|`usize`||
|overrides_with|`&str`||
|overrides_with_all|`Vec<&str>`||
|possible_value|`&str`||
|possible_values|`Vec<&str>`||
|raw|`bool`||
|require_delimiter|`bool`||
|require_equals|`bool`||
|required|`bool`||
|required_if_eq| `(&str,&str)`||
|required_if_eq_all|`Vec<(&str,&str)>`||
|required_if_eq_any|`Vec<(&str,&str)>`||
|required_unless_present|`&str`||
|required_unless_present_any|`Vec<&str>`||
|required_unless_present_all|`Vec<&str>`||
|requires|`&str`||
|requires_all|`Vec<&str>`||
|requires_if | `(&str,&str)` ||
|requires_ifs|`Vec<(&str,&str)>`||
|short|`char`||
|short_alias|`char`||
|short_aliases|`Vec<char>`||
|takes_value|`bool`||
|use_delimiter|`bool`||
|(validator_regex) | not supported yet||
|value_hint | [`ValueHint`](#valuehint)||
|value_delimiter|`char`||
|value_name|`&str`||
|value_names|`Vec<&str>`||
|value_terminator|`&str`||
|visible_alias|`&str`||
|visible_aliases|`Vec<&str>`||
|visible_short_alias|`char`||
|visible_short_aliases|`Vec<char>`||

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
