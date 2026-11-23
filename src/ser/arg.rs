use super::{DisplayStr, Entries, SerializeConfig};
use crate::de::arg::{arg_action::ArgAction, value_hint::ValueHint};
use crate::ArgWrap;
use clap::{Arg, Command};
use serde::{Serialize, Serializer};
use std::any::TypeId;
use std::ops::Deref;

/// Wrapper of `&`[`Arg`] to serialize.
///
/// This is serialized as a map with single entry `{ id: { ..properties } }`.
#[derive(Debug, Clone)]
pub struct ArgWrapRef<'a, C = ()> {
    arg: &'a Arg,
    config: C,
}

impl<'a> ArgWrapRef<'a> {
    /// Create a wrapper for [`Arg`].
    pub fn new(arg: &'a Arg) -> Self {
        Self { arg, config: () }
    }
}

impl<'a, C> ArgWrapRef<'a, C> {
    /// Set the config for serializing.
    /// See [`NoSkip`](`crate::NoSkip`) for details.
    pub fn with_config<C2: SerializeConfig>(self, config: C2) -> ArgWrapRef<'a, C2> {
        ArgWrapRef {
            arg: self.arg,
            config,
        }
    }
}

impl<C> Deref for ArgWrapRef<'_, C> {
    type Target = Arg;

    fn deref(&self) -> &Self::Target {
        self.arg
    }
}

impl<'a, C: Default> From<&'a Arg> for ArgWrapRef<'a, C> {
    fn from(arg: &'a Arg) -> Self {
        Self {
            arg,
            config: C::default(),
        }
    }
}

impl<'a, C: Default> From<&'a ArgWrap> for ArgWrapRef<'a, C> {
    fn from(arg: &'a ArgWrap) -> Self {
        Self {
            arg: &arg.arg,
            config: C::default(),
        }
    }
}

impl Serialize for ArgWrap {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ArgWrapRef::new(&self.arg).serialize(serializer)
    }
}

impl<C: SerializeConfig> Serialize for ArgWrapRef<'_, C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(Some((self.arg.get_id().as_str(), ArgProps(self))))
    }
}

/// Properties of the arg (the value of `{ id: { ..properties } }`).
struct ArgProps<'a, 'b, C>(&'b ArgWrapRef<'a, C>);

impl<C: SerializeConfig> Serialize for ArgProps<'_, '_, C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let a = self.0.arg;
        let all = self.0.config.serialize_all();
        let mut m = Entries::new(serializer, all)?;

        m.opt("help", a.get_help().map(DisplayStr))?;
        m.opt("long_help", a.get_long_help().map(DisplayStr))?;
        m.opt("help_heading", a.get_help_heading())?;
        m.opt("short", a.get_short())?;
        m.opt("long", a.get_long())?;

        let visible = a.get_visible_aliases().unwrap_or_default();
        m.seq("visible_aliases", visible.iter())?;
        m.seq("aliases", a.get_aliases().unwrap_or_default())?;
        let visible = a.get_visible_short_aliases().unwrap_or_default();
        m.seq("visible_short_aliases", visible.iter())?;
        m.seq(
            "short_aliases",
            a.get_all_short_aliases()
                .unwrap_or_default()
                .into_iter()
                .filter(|c| !visible.contains(c)),
        )?;

        m.opt("index", a.get_index())?;
        m.opt(
            "value_names",
            a.get_value_names()
                .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        )?;
        m.opt("num_args", a.get_num_args().map(NumArgsSer))?;
        m.opt("value_delimiter", a.get_value_delimiter())?;
        m.opt(
            "value_terminator",
            a.get_value_terminator().map(|s| s.as_str()),
        )?;
        m.seq(
            "default_values",
            a.get_default_values().iter().map(|v| v.to_string_lossy()),
        )?;
        #[cfg(feature = "env")]
        m.opt("env", a.get_env().map(|e| e.to_string_lossy()))?;

        let action = a.get_action();
        if all || !matches!(action, clap::ArgAction::Set) {
            m.opt("action", ArgAction::from_clap_type(action.clone()))?;
        }
        let hint = a.get_value_hint();
        if all || hint != clap::ValueHint::Unknown {
            m.opt("value_hint", ValueHint::from_clap_type(hint))?;
        }
        let value_type = value_parser_name(a.get_value_parser());
        let possible_values = if value_type == Some("string") {
            a.get_possible_values()
        } else {
            // e.g. `bool` has possible values, but it is represented by `value_parser`
            Vec::new()
        };
        if !possible_values.is_empty() {
            // `possible_values` sets `PossibleValuesParser`, so `value_parser` must not be written.
            m.value(
                "possible_values",
                &possible_values
                    .iter()
                    .map(|p| p.get_name())
                    .collect::<Vec<_>>(),
            )?;
        } else {
            m.opt("possible_values", None::<()>)?;
            if all || !is_default_value_parser(action, value_type) {
                m.opt("value_parser", value_type)?;
            }
        }

        m.flag("positional", a.is_positional())?;
        m.flag("required", a.is_required_set())?;
        m.flag("allow_hyphen_values", a.is_allow_hyphen_values_set())?;
        m.flag("allow_negative_numbers", a.is_allow_negative_numbers_set())?;
        m.flag("global", a.is_global_set())?;
        m.flag("next_line_help", a.is_next_line_help_set())?;
        m.flag("hide", a.is_hide_set())?;
        m.flag("hide_default_value", a.is_hide_default_value_set())?;
        m.flag("hide_possible_values", a.is_hide_possible_values_set())?;
        #[cfg(feature = "env")]
        {
            m.flag("hide_env", a.is_hide_env_set())?;
            m.flag("hide_env_values", a.is_hide_env_values_set())?;
        }
        m.flag("hide_short_help", a.is_hide_short_help_set())?;
        m.flag("hide_long_help", a.is_hide_long_help_set())?;
        m.flag("require_equals", a.is_require_equals_set())?;
        m.flag("exclusive", a.is_exclusive_set())?;
        m.flag("trailing_var_arg", a.is_trailing_var_arg_set())?;
        m.flag("last", a.is_last_set())?;
        m.flag("ignore_case", a.is_ignore_case_set())?;

        m.end()
    }
}

/// Name of the value parser which can be deserialized, if known.
fn value_parser_name(value_parser: &clap::builder::ValueParser) -> Option<&'static str> {
    use std::{ffi::OsString, path::PathBuf};
    let id = value_parser.type_id();
    macro_rules! find {
        ($($t:ty => $name:literal),* $(,)?) => {
            $(if id == TypeId::of::<$t>() { return Some($name); })*
        };
    }
    find!(
        String => "string",
        bool => "bool",
        OsString => "os_string",
        PathBuf => "path_buf",
        i64 => "i64",
        i32 => "i32",
        i16 => "i16",
        i8 => "i8",
        u64 => "u64",
        u32 => "u32",
        u16 => "u16",
        u8 => "u8",
    );
    None
}

/// Whether the value parser is what clap selects for the action by default.
fn is_default_value_parser(action: &clap::ArgAction, value_type: Option<&str>) -> bool {
    use clap::ArgAction::*;
    match value_type {
        // unknown value parsers cannot be serialized
        None => true,
        Some("string") => true,
        Some("bool") => matches!(action, SetTrue | SetFalse),
        Some("u8") => matches!(action, Count),
        Some(_) => false,
    }
}

/// Serialize [`ValueRange`](clap::builder::ValueRange) as number or `{ min, max }`.
struct NumArgsSer(clap::builder::ValueRange);

impl Serialize for NumArgsSer {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let (min, max) = (self.0.min_values(), self.0.max_values());
        if min == max {
            serializer.serialize_u64(min as u64)
        } else {
            let mut s = serializer.serialize_struct("NumArgs", 2)?;
            s.serialize_field("min", &min)?;
            if max == usize::MAX {
                s.skip_field("max")?;
            } else {
                s.serialize_field("max", &max)?;
            }
            s.end()
        }
    }
}

pub(crate) struct ArgsWrap<'a, C> {
    command: &'a Command,
    config: C,
}

impl<'a, C> ArgsWrap<'a, C> {
    pub(crate) fn new(command: &'a Command, config: C) -> Self {
        Self { command, config }
    }
}

impl<C: SerializeConfig> Serialize for ArgsWrap<'_, C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let c = self.command;
        serializer.collect_seq(
            c.get_arguments()
                .filter(|arg| !is_generated_arg(c, arg))
                .map(|arg| ArgWrapRef {
                    arg,
                    config: self.config.clone(),
                }),
        )
    }
}

/// Whether the arg is auto generated by clap in [`Command::build`].
///
/// They are skipped in serialization, because they will be generated again
/// and conflict with the deserialized ones.
pub(crate) fn is_generated_arg(command: &Command, arg: &Arg) -> bool {
    use clap::ArgAction::*;
    match arg.get_id().as_str() {
        "help" => {
            !command.is_disable_help_flag_set()
                && matches!(arg.get_action(), Help | HelpShort | HelpLong)
        }
        "version" => {
            !command.is_disable_version_flag_set()
                && matches!(arg.get_action(), Version)
                && (command.get_version().is_some() || command.get_long_version().is_some())
        }
        _ => false,
    }
}
