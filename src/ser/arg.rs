use super::SerializeConfig;
use crate::ArgWrap;
use clap::{Arg, Command};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

/// Wrapper of `&`[`Arg`] to serialize.
#[derive(Debug, Clone)]
pub struct ArgWrapRef<'a, 'b, C = ()> {
    arg: &'b Arg<'a>,
    pub(crate) config: C,
}

impl<'a, 'b> ArgWrapRef<'a, 'b> {
    pub fn new(arg: &'b Arg<'a>) -> Self {
        Self { arg, config: () }
    }
    pub fn with_config<C: SerializeConfig>(self, config: C) -> ArgWrapRef<'a, 'b, C> {
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

impl<'a, 'b> From<&'b ArgWrap<'a>> for ArgWrapRef<'a, 'b> {
    fn from(arg: &'b ArgWrap<'a>) -> Self {
        Self {
            arg: &arg.arg,
            config: (),
        }
    }
}

impl<'se> Serialize for ArgWrap<'se> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ArgWrapRef::from(self).serialize(serializer)
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
        let config = &self.wrap.config;
        let r = ser_value!(arg, serializer, config, [
            // (id, get_id), not seriazed here.
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
                // no hidden aliases because clap dosen't expose 
                (long,get_long),
                (value_names,get_value_names),
                [&] (number_of_values, get_num_vals),
                [&] (value_delimiter, get_value_delimiter),
                [&] (index,get_index),
                #[cfg(feature = "env")]
                (env, get_env),
                [&] (visible_aliases, get_visible_aliases),
                [&] (visible_short_aliases, get_visible_short_aliases),
            ]
            specialize  [
                (default_values, Arg::get_default_values, |x : &[&std::ffi::OsStr]| { x.len() > 0 } ),
                (value_hint, |s| { crate::de::arg::value_hint::ValueHint::from_clap_type(Arg::get_value_hint(s)) }),
                (arg_action, |s| { crate::de::arg::arg_action::ArgAction::from_clap_type(Arg::get_action(s).clone()) })
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
        let args = self.command.get_arguments();
        let setting = &self.config;
        serializer.collect_seq(args.map(|arg| ArgWrapRef {
            arg,
            config: setting.clone(),
        }))
    }
}
