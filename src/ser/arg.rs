use super::SerializeConfig;
use clap::{Arg, Command};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

/// Wrapper of `&[Arg]` to serialize.
#[derive(Debug, Clone)]
pub struct ArgWrapRef<'a, 'b, C = ()> {
    arg: &'b Arg<'a>,
    pub(crate) config: C,
}

impl<'a, 'b> ArgWrapRef<'a, 'b> {
    pub fn new(arg: &'b Arg<'a>) -> Self {
        Self { arg, config: () }
    }
    pub fn with_config<C>(self, config: C) -> ArgWrapRef<'a, 'b, C> {
        ArgWrapRef {
            arg: self.arg,
            config,
        }
    }
}

impl<'a, 'b> From<&'b Arg<'a>> for ArgWrapRef<'a, 'b> {
    fn from(arg: &'b Arg<'a>) -> Self {
        Self { arg, config: () }
    }
}

impl<'se, 'b, C: SerializeConfig> Serialize for ArgWrapRef<'se, 'b, C> {
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

impl<'se, 'wrap, C: SerializeConfig> Serialize for ArgWrapMaps<'se, 'wrap, C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let arg = &self.wrap.arg;
        let setting = self.wrap.config.serialize_all();
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

pub(crate) struct ArgsWrap<'a, 'b, C> {
    command: &'b Command<'a>,
    config: C,
}

impl<'a, 'b, C> ArgsWrap<'a, 'b, C> {
    pub fn new(command: &'b Command<'a>, config: C) -> Self {
        Self { command, config }
    }
}

impl<'a, 'b, C: SerializeConfig> Serialize for ArgsWrap<'a, 'b, C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser_args(serializer, self.command.get_arguments(), &self.config)
    }
}

pub(crate) fn ser_args<
    'a: 'b,
    'b,
    Ser: Serializer,
    I: Iterator<Item = &'b Arg<'a>>,
    C: SerializeConfig,
>(
    serializer: Ser,
    args: I,
    setting: C,
) -> Result<Ser::Ok, Ser::Error> {
    serializer.collect_seq(args.map(|arg| ArgWrapRef {
        arg,
        config: &setting,
    }))
}
