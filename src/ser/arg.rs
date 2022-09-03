use crate::ArgWrap;
use serde::ser::SerializeMap;
use serde::Serialize;

use super::app::SerializeSetting;

impl<'de, Setting: SerializeSetting> Serialize for ArgWrap<'de, Setting> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let arg = &self.arg;
        let setting = self.ser_setting.serialize_all();
        let r = ser_value!(arg, serializer, setting, [
            (id, get_id),
            (get_default_values, get_default_values),
            is [
                (positional, is_positional),
                (required,is_required_set),
                (multiple_values,is_multiple_values_set),
                (multiple_occurrences,is_multiple_occurrences_set),
                (takes_value,is_takes_value_set),
                (allow_hyphen_values,is_allow_hyphen_values_set),
                (global,is_global_set),
                (next_line_help,is_next_line_help_set),
                (hide,is_hide_set),
                (hide_default_value,is_hide_default_value_set),
                (hide_possible_values,is_hide_possible_values_set),
                #[cfg(feature = "env")]
                (hide_env,is_hide_env_set),
                #[cfg(feature = "env")]
                (hide_env_values,is_hide_env_values_set),
                (hide_short_help,is_hide_short_help_set),
                (hide_long_help,is_hide_long_help_set),
                (use_value_delimiter,is_use_value_delimiter_set),
                (require_value_delimiter,is_require_value_delimiter_set),
                (require_equals,is_require_equals_set),
                (exclusive,is_exclusive_set),
                (last,is_last_set),
                (ignore_case,is_ignore_case_set),
            ],
            opt [
                (help, get_help),
                (long_help, get_long_help),
                (help_heading, get_help_heading),
                [&] (short,get_short),
                [&] (visible_short_aliases,get_visible_short_aliases),
                //aliases
                (long,get_long),
                // (get_possible_values,get_possible_values),
                (value_names,get_value_names),
                [&] (num_vals,get_num_vals),
                [&] (value_delimiter, get_value_delimiter),
                [&] (index,get_index),
                //value_hint
                #[cfg(feature = "env")]
                (get_env, get_env),
                //get_action
            ]
        ]);

        r
    }
}
