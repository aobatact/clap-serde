use crate::CommandWrap;
use clap::Command;
use serde::ser::SerializeMap;
use serde::Serialize;
use std::ops::Deref;

use super::SerializeConfig;

/// Wrapper of `&`[`Command`] to serialize.
#[derive(Debug, Clone)]
pub struct CommandWrapRef<'command, 'wrap, C = ()> {
    command: &'wrap Command<'command>,
    config: C,
}

impl<'a, 'b> CommandWrapRef<'a, 'b> {
    /// Create a wrapper for [`Command`].
    pub fn new(app: &'b Command<'a>) -> Self {
        Self {
            command: app,
            config: (),
        }
    }

    /// Add a setting for serializeing.
    /// See [`NoSkip`](`crate::NoSkip`) for details.
    pub fn with_setting<C: SerializeConfig>(self, config: C) -> CommandWrapRef<'a, 'b, C> {
        CommandWrapRef {
            command: self.command,
            config,
        }
    }
}

impl<'a, 'b> Deref for CommandWrapRef<'a, 'b> {
    type Target = Command<'a>;

    fn deref(&self) -> &Self::Target {
        &self.command
    }
}

impl<'a, 'b, S> From<CommandWrapRef<'a, 'b, S>> for &'b Command<'a> {
    fn from(a: CommandWrapRef<'a, 'b, S>) -> Self {
        a.command
    }
}

impl<'a, 'b> From<&'b Command<'a>> for CommandWrapRef<'a, 'b> {
    fn from(command: &'b Command<'a>) -> Self {
        CommandWrapRef {
            command,
            config: (),
        }
    }
}

impl<'a, 'b> From<&'b CommandWrap<'a>> for CommandWrapRef<'a, 'b> {
    fn from(app: &'b CommandWrap<'a>) -> Self {
        CommandWrapRef {
            command: &app.app,
            config: (),
        }
    }
}

impl<'a> Serialize for CommandWrap<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let wrap_ref = CommandWrapRef::from(self);
        wrap_ref.serialize(serializer)
    }
}

impl<'a, 'b, C: SerializeConfig> Serialize for CommandWrapRef<'a, 'b, C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let command = self.command;
        let config = self.config.serialize_all();
        let r = ser_value!(command, serializer, config, [
            (name, get_name),
            #[cfg(feature = "color")]
            [&] {crate::de::app::color::to_ser} (color, get_color),
            is [
                (no_binary_name, is_no_binary_name_set),
                (dont_delimit_trailing_values,is_dont_delimit_trailing_values_set),
                (disable_version_flag,is_disable_version_flag_set),
                (propagate_version,is_propagate_version_set),
                (next_line_help,is_next_line_help_set),
                (disable_help_flag,is_disable_help_flag_set),
                (disable_help_subcommand,is_disable_help_subcommand_set),
                (disable_colored_help,is_disable_colored_help_set),
                (dont_collapse_args_in_usage,is_dont_collapse_args_in_usage_set),
                (arg_required_else_help,is_arg_required_else_help_set),
                (allow_negative_numbers,is_allow_negative_numbers_set),
                (trailing_var_arg,is_trailing_var_arg_set),
                (allow_missing_positional,is_allow_missing_positional_set),
                (hide,is_hide_set),
                (subcommand_required,is_subcommand_required_set),
                (allow_external_subcommands,is_allow_external_subcommands_set),
                (allow_invalid_utf8_for_external_subcommands,is_allow_invalid_utf8_for_external_subcommands_set),
                (args_conflicts_with_subcommands,is_args_conflicts_with_subcommands_set),
                (subcommand_precedence_over_arg,is_subcommand_precedence_over_arg_set),
                (subcommand_negates_reqs,is_subcommand_negates_reqs_set),
                (multicall,is_multicall_set)
            ],
            opt [
                (display_name, get_display_name),
                (bin_name, get_bin_name),
                (version, get_version),
                (long_version, get_long_version),
                (author, get_author),
                (long_flag, get_long_flag),
                [&] (short_flag, get_short_flag),
                (about, get_about),
                (long_about, get_long_about),
                (next_help_heading, get_next_help_heading),
                // visible alias
                // subcommands
                (subcommand_help_heading, get_subcommand_help_heading),
                (subcommand_value_name, get_subcommand_value_name),
                (before_help, get_before_help),
                (before_long_help, get_before_long_help),
                (after_help, get_after_help),
                (after_long_help, get_after_long_help),
            ]
            //groups
            speciallize [
                (args, |s| {super::arg::ArgsWrap::new(s, &self.config)}),
                (subcommands, |s| {SubCommandsWrap(s, self.config.clone())})
            ]
        ]);
        r
    }
}

struct SubCommandsWrap<'a, 'b, C>(&'b Command<'a>, C);

impl<'a, 'b, C: SerializeConfig> Serialize for SubCommandsWrap<'a, 'b, C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.0.get_subcommands().map(|s| SubcommandWrap(s, self.1.clone())))
    }
}

struct SubcommandWrap<'a, 'b, C>(&'b Command<'a>, C);

impl<'a, 'b, C: SerializeConfig> Serialize for SubcommandWrap<'a, 'b, C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_map(Some((
            self.0.get_name(),
            CommandWrapRef {
                command: self.0,
                config: self.1.clone(),
            },
        )))
    }
}
