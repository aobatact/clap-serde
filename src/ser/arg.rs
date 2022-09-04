use super::app::SerializeSetting;
use clap::{Arg, Command};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

/// Wrapper of [`Arg`] to deserialize with [`DeserializeSeed`](`serde::de::DeserializeSeed`).
#[derive(Debug, Clone)]
pub struct ArgWrapRef<'a, 'b, S = ()> {
    arg: &'b Arg<'a>,
    pub(crate) ser_setting: S,
}

impl<'se, 'b, Setting: SerializeSetting> Serialize for ArgWrapRef<'se, 'b, Setting> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_map(Some((self.arg.get_id(), ArgWrapMaps::new(self))))
    }
}

pub struct ArgWrapMaps<'se, 'wrap, S> {
    wrap: &'wrap ArgWrapRef<'se, 'wrap, S>,
}

impl<'se, 'wrap, S> ArgWrapMaps<'se, 'wrap, S> {
    pub fn new(wrap: &'wrap ArgWrapRef<'se, 'wrap, S>) -> Self {
        Self { wrap }
    }
}

impl<'se, 'wrap, Setting: SerializeSetting> Serialize for ArgWrapMaps<'se, 'wrap, Setting> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let arg = &self.wrap.arg;
        let setting = self.wrap.ser_setting.serialize_all();
        let r = ser_value!(arg, serializer, setting, [
            // (id, get_id),
            // (default_value, get_default_values),
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
                (env, get_env),
                //get_action
            ]
        ]);

        r
    }
}

pub(crate) struct ArgsWrap<'a, 'b, S> {
    command: &'b Command<'a>,
    setting: S,
}

impl<'a, 'b, S> ArgsWrap<'a, 'b, S> {
    pub(crate) fn new(command: &'b Command<'a>, setting: S) -> Self {
        Self { command, setting }
    }
}

impl<'a, 'b, Se: SerializeSetting> Serialize for ArgsWrap<'a, 'b, Se> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser_args(serializer, self.command.get_arguments(), &self.setting)
    }
}

pub(crate) fn ser_args<
    'a: 'b,
    'b,
    Ser: Serializer,
    I: Iterator<Item = &'b Arg<'a>>,
    S: SerializeSetting,
>(
    serializer: Ser,
    args: I,
    setting: S,
) -> Result<Ser::Ok, Ser::Error> {
    serializer.collect_seq(args.map(|arg| ArgWrapRef {
        arg,
        ser_setting: &setting,
    }))
}
