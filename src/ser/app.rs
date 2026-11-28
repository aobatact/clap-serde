use super::arg::DEFAULT_DISPLAY_ORDER;
use super::{DisplayStr, Entries, SerializeConfig};
use crate::CommandWrap;
use clap::Command;
use serde::{Serialize, Serializer};
use std::ops::Deref;

/// Wrapper of `&`[`Command`] to serialize.
///
/// The output can be deserialized with [`CommandWrap`].
/// ```
/// # use clap::{Arg, Command};
/// # use clap_serde::{CommandWrap, CommandWrapRef};
/// let command = Command::new("app")
///     .about("an app")
///     .arg(Arg::new("verbose").short('v'));
/// let json = serde_json::to_string(&CommandWrapRef::new(&command)).unwrap();
/// let deserialized: CommandWrap = serde_json::from_str(&json).unwrap();
/// assert_eq!(deserialized.get_name(), "app");
/// ```
///
/// Some properties cannot be serialized because clap does not expose getters for them
/// (e.g. `requires`, `conflicts_with`, `default_value_if`).
/// `value_parser` is identified only by the type of the value
/// (see the README for details).
#[derive(Debug, Clone)]
pub struct CommandWrapRef<'a, C = ()> {
    command: &'a Command,
    config: C,
    /// The display order which [`Command::subcommand`] will assign implicitly.
    implicit_display_order: usize,
}

impl<'a> CommandWrapRef<'a> {
    /// Create a wrapper for [`Command`].
    pub fn new(command: &'a Command) -> Self {
        Self {
            command,
            config: (),
            implicit_display_order: DEFAULT_DISPLAY_ORDER,
        }
    }
}

impl<'a, C> CommandWrapRef<'a, C> {
    /// Set the config for serializing.
    /// See [`NoSkip`](`crate::NoSkip`) for details.
    pub fn with_config<C2: SerializeConfig>(self, config: C2) -> CommandWrapRef<'a, C2> {
        CommandWrapRef {
            command: self.command,
            config,
            implicit_display_order: self.implicit_display_order,
        }
    }

    /// Set the config for serializing.
    #[deprecated(since = "0.6.0", note = "use `with_config` instead")]
    pub fn with_setting<C2: SerializeConfig>(self, config: C2) -> CommandWrapRef<'a, C2> {
        self.with_config(config)
    }
}

impl<C> Deref for CommandWrapRef<'_, C> {
    type Target = Command;

    fn deref(&self) -> &Self::Target {
        self.command
    }
}

impl<'a, C> From<CommandWrapRef<'a, C>> for &'a Command {
    fn from(a: CommandWrapRef<'a, C>) -> Self {
        a.command
    }
}

impl<'a, C: Default> From<&'a Command> for CommandWrapRef<'a, C> {
    fn from(command: &'a Command) -> Self {
        CommandWrapRef {
            command,
            config: C::default(),
            implicit_display_order: DEFAULT_DISPLAY_ORDER,
        }
    }
}

impl<'a, C: Default> From<&'a CommandWrap> for CommandWrapRef<'a, C> {
    fn from(app: &'a CommandWrap) -> Self {
        CommandWrapRef {
            command: &app.app,
            config: C::default(),
            implicit_display_order: DEFAULT_DISPLAY_ORDER,
        }
    }
}

impl Serialize for CommandWrap {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        CommandWrapRef::new(&self.app).serialize(serializer)
    }
}

impl<C: SerializeConfig> Serialize for CommandWrapRef<'_, C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let c = self.command;
        let mut m = Entries::new(serializer, self.config.serialize_all())?;

        m.value("name", c.get_name())?;
        m.opt("display_name", c.get_display_name())?;
        m.opt("bin_name", c.get_bin_name())?;
        m.opt("version", c.get_version())?;
        m.opt("long_version", c.get_long_version())?;
        m.opt("author", c.get_author())?;
        m.opt("about", c.get_about().map(DisplayStr))?;
        m.opt("long_about", c.get_long_about().map(DisplayStr))?;
        m.opt("before_help", c.get_before_help().map(DisplayStr))?;
        m.opt("before_long_help", c.get_before_long_help().map(DisplayStr))?;
        m.opt("after_help", c.get_after_help().map(DisplayStr))?;
        m.opt("after_long_help", c.get_after_long_help().map(DisplayStr))?;
        m.opt("override_usage", c.get_overridden_usage().map(DisplayStr))?;
        m.opt("subcommand_help_heading", c.get_subcommand_help_heading())?;
        m.opt("subcommand_value_name", c.get_subcommand_value_name())?;
        m.opt("short_flag", c.get_short_flag())?;
        m.opt("long_flag", c.get_long_flag())?;
        let display_order = c.get_display_order();
        if display_order != self.implicit_display_order {
            m.value("display_order", &display_order)?;
        } else {
            m.opt("display_order", None::<()>)?;
        }

        m.seq("visible_aliases", c.get_visible_aliases())?;
        m.seq("aliases", c.get_aliases())?;
        let visible: Vec<_> = c.get_visible_short_flag_aliases().collect();
        m.seq("visible_short_flag_aliases", visible.iter())?;
        m.seq(
            "short_flag_aliases",
            c.get_all_short_flag_aliases()
                .filter(|a| !visible.contains(a)),
        )?;
        let visible: Vec<_> = c.get_visible_long_flag_aliases().collect();
        m.seq("visible_long_flag_aliases", visible.iter())?;
        m.seq(
            "long_flag_aliases",
            c.get_all_long_flag_aliases()
                .filter(|a| !visible.contains(a)),
        )?;

        #[cfg(feature = "color")]
        {
            let color = c.get_color();
            if self.config.serialize_all() || color != clap::ColorChoice::Auto {
                m.opt(
                    "color",
                    crate::de::app::color::ColorChoice1::from_clap_type(color),
                )?;
            }
        }

        m.flag("no_binary_name", c.is_no_binary_name_set())?;
        m.flag("args_override_self", c.is_args_override_self())?;
        m.flag(
            "dont_delimit_trailing_values",
            c.is_dont_delimit_trailing_values_set(),
        )?;
        // `is_disable_version_flag_set` is also true when the version is not set.
        if c.get_version().is_some() || c.get_long_version().is_some() {
            m.flag("disable_version_flag", c.is_disable_version_flag_set())?;
        } else {
            m.opt("disable_version_flag", None::<()>)?;
        }
        m.flag("propagate_version", c.is_propagate_version_set())?;
        m.flag("next_line_help", c.is_next_line_help_set())?;
        m.flag("disable_help_flag", c.is_disable_help_flag_set())?;
        m.flag(
            "disable_help_subcommand",
            c.is_disable_help_subcommand_set(),
        )?;
        m.flag("disable_colored_help", c.is_disable_colored_help_set())?;
        // `dont_collapse_args_in_usage` is always true in clap 4.
        m.flag("arg_required_else_help", c.is_arg_required_else_help_set())?;
        m.flag("allow_negative_numbers", c.is_allow_negative_numbers_set())?;
        m.flag("trailing_var_arg", c.is_trailing_var_arg_set())?;
        m.flag(
            "allow_missing_positional",
            c.is_allow_missing_positional_set(),
        )?;
        m.flag("hide", c.is_hide_set())?;
        m.flag("subcommand_required", c.is_subcommand_required_set())?;
        m.flag(
            "allow_external_subcommands",
            c.is_allow_external_subcommands_set(),
        )?;
        m.flag(
            "args_conflicts_with_subcommands",
            c.is_args_conflicts_with_subcommands_set(),
        )?;
        m.flag(
            "subcommand_precedence_over_arg",
            c.is_subcommand_precedence_over_arg_set(),
        )?;
        m.flag(
            "subcommand_negates_reqs",
            c.is_subcommand_negates_reqs_set(),
        )?;
        m.flag("multicall", c.is_multicall_set())?;
        m.flag("flatten_help", c.is_flatten_help_set())?;

        if self.config.serialize_all()
            || c.get_arguments()
                .any(|a| !super::arg::is_generated_arg(c, a))
        {
            m.value("args", &super::arg::ArgsWrap::new(c, self.config.clone()))?;
        }
        if self.config.serialize_all() || c.get_groups().next().is_some() {
            m.value(
                "groups",
                &super::group::GroupsWrap::new(c, self.config.clone()),
            )?;
        }
        if self.config.serialize_all()
            || c.get_subcommands().any(|s| !is_generated_subcommand(c, s))
        {
            m.value("subcommands", &SubCommandsWrap(c, self.config.clone()))?;
        }
        // `Command::arg` assigns `next_help_heading` to the args added after it,
        // so this must be written after `args`.
        // (The args without heading also have `help_heading: null` for the formats which reorder keys.)
        m.opt("next_help_heading", c.get_next_help_heading())?;
        m.end()
    }
}

struct SubCommandsWrap<'a, C>(&'a Command, C);

impl<C: SerializeConfig> Serialize for SubCommandsWrap<'_, C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let c = self.0;
        // `Command::arg` and `Command::subcommand` share the counter of the display order.
        // The args are deserialized before the subcommands.
        let mut order = c
            .get_arguments()
            .filter(|a| !super::arg::is_generated_arg(c, a) && !a.is_positional())
            .count();
        serializer.collect_seq(
            c.get_subcommands()
                .filter(|s| !is_generated_subcommand(c, s))
                .map(|s| {
                    order += 1;
                    SubcommandWrap(s, self.1.clone(), order - 1)
                }),
        )
    }
}

struct SubcommandWrap<'a, C>(&'a Command, C, usize);

impl<C: SerializeConfig> Serialize for SubcommandWrap<'_, C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(Some((
            self.0.get_name(),
            CommandWrapRef {
                command: self.0,
                config: self.1.clone(),
                implicit_display_order: self.2,
            },
        )))
    }
}

/// Whether the subcommand is the `help` subcommand auto generated by clap in [`Command::build`].
fn is_generated_subcommand(command: &Command, sub: &Command) -> bool {
    sub.get_name() == "help"
        && !command.is_disable_help_subcommand_set()
        && sub.is_disable_help_flag_set()
        && sub.get_about().is_some_and(|a| {
            a.to_string() == "Print this message or the help of the given subcommand(s)"
        })
}
