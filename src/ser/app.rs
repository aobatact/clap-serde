use clap::Command;
use serde::ser::SerializeMap;
use serde::Serialize;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct CommandWrapRef<'a, 'b, S = ()> {
    app: &'b Command<'a>,
    ser_setting: S,
}
impl<'a, 'b> CommandWrapRef<'a, 'b> {
    /// Create a wrapper for [`Command`].
    pub fn new(app: &'b Command<'a>) -> Self {
        Self {
            app,
            ser_setting: (),
        }
    }

    /// Add a setting for serializeing.
    /// See [`NoSkip`] for details.
    pub fn with_setting<S>(self, ser_setting: S) -> CommandWrapRef<'a, 'b, S> {
        CommandWrapRef {
            app: self.app,
            ser_setting,
        }
    }
}

impl<'a, 'b> Deref for CommandWrapRef<'a, 'b> {
    type Target = Command<'a>;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl<'a, 'b, S> From<CommandWrapRef<'a, 'b, S>> for &'b Command<'a> {
    fn from(a: CommandWrapRef<'a, 'b, S>) -> Self {
        a.app
    }
}

impl<'a, 'b> From<&'b Command<'a>> for CommandWrapRef<'a, 'b> {
    fn from(app: &'b Command<'a>) -> Self {
        CommandWrapRef {
            app,
            ser_setting: (),
        }
    }
}

impl<'de, 'de2, Setting: SerializeSetting> Serialize for CommandWrapRef<'de, 'de2, Setting> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let command = &self.app;
        let setting = self.ser_setting.serialize_all();
        let r = ser_value!(command, serializer, setting, [
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
            //args
            speciallize [
                (args, |s| {super::arg::ArgsWrap::new(s,&self.ser_setting)})
            ]
        ]);
        r
    }
}

pub trait SerializeSetting {
    fn serialize_all(&self) -> bool;
}

impl SerializeSetting for () {
    #[inline]
    fn serialize_all(&self) -> bool {
        false
    }
}

impl<S: SerializeSetting> SerializeSetting for &S {
    fn serialize_all(&self) -> bool {
        (*self).serialize_all()
    }
}

/// Serialize all the fields in Command.
/// If not set, the flags (getter begin with `is_`) with `false`
/// and values (getter begin with `get_`) with `None` will be skipped.
/// ```
/// # use clap::Command;
/// # use clap_serde::*;
/// # let command = Command::default();
/// let wrap = CommandWrap::new(command).with_setting(NoSkip);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct NoSkip;
impl SerializeSetting for NoSkip {
    #[inline]
    fn serialize_all(&self) -> bool {
        true
    }
}
