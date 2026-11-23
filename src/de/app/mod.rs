use clap::Command;
use serde::{
    de::{DeserializeSeed, Error, Visitor},
    Deserialize,
};
use std::ops::Deref;

#[cfg(feature = "allow-deprecated")]
mod appsettings;
#[cfg(feature = "color")]
pub(crate) mod color;

/// Wrapper of [`Command`] to deserialize (and serialize, see [`CommandWrapRef`](crate::CommandWrapRef)).
/// ```
/// const CLAP_TOML: &'static str = r#"
/// name = "app_clap_serde"
/// version = "1.0"
/// author = "tester"
/// about = "test-clap-serde"
/// "#;
/// let app: clap::Command = toml::from_str::<clap_serde::CommandWrap>(CLAP_TOML)
///     .expect("parse failed")
///     .into();
/// assert_eq!(app.get_name(), "app_clap_serde");
/// assert_eq!(app.get_about().map(|s| s.to_string()).as_deref(), Some("test-clap-serde"));
/// ```
#[derive(Debug, Clone)]
pub struct CommandWrap {
    pub(crate) app: Command,
}

impl CommandWrap {
    /// Create a wrapper for [`Command`].
    pub fn new(app: Command) -> Self {
        Self { app }
    }
}

impl From<CommandWrap> for Command {
    fn from(a: CommandWrap) -> Self {
        a.app
    }
}

impl From<Command> for CommandWrap {
    fn from(app: Command) -> Self {
        CommandWrap { app }
    }
}

impl Deref for CommandWrap {
    type Target = Command;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

const TMP_APP_NAME: &str = "__tmp__deserialize__name__";
impl<'de> Deserialize<'de> for CommandWrap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_map(CommandVisitor(Command::new(TMP_APP_NAME)))
            //check the name so as not to expose the tmp name.
            .and_then(|r| {
                if r.app.get_name() != TMP_APP_NAME {
                    Ok(r)
                } else {
                    Err(<D::Error>::missing_field("name"))
                }
            })
    }
}

struct CommandVisitor(Command);

impl<'de> Visitor<'de> for CommandVisitor {
    type Value = CommandWrap;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Command Map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut app = self.0;
        //TODO: check the first key to get name from the input?
        //currently the name change in `Clap::Command::name` doesn't change the `Clap::Command::id` so might cause problems?
        while let Some(key) = map.next_key::<String>()? {
            app = parse_value!(key, app, map, Command, {
                (about, String),
                (after_help, String),
                (after_long_help, String),
                (alias, String),
                (aliases, Vec<String>),
                (allow_external_subcommands, bool),
                (allow_hyphen_values, bool),
                (allow_negative_numbers, bool),
                (allow_missing_positional, bool),
                //arg : not supported single arg(now)
                //args : specialized
                (arg_required_else_help, bool),
                (args_conflicts_with_subcommands, bool),
                (args_override_self, bool),
                (author, String),
                (before_help, String),
                (before_long_help, String),
                (bin_name, String),
                // color : specialized
                (disable_colored_help, bool),
                (disable_help_flag, bool),
                (disable_help_subcommand, bool),
                (disable_version_flag, bool),
                (display_name, String),
                (display_order, usize),
                (dont_collapse_args_in_usage, bool),
                (dont_delimit_trailing_values, bool),
                (flatten_help, bool),
                // group : not supported single group
                // groups : specialized
                (help_expected, bool),
                // help_template : specialized
                (hide, bool),
                (hide_possible_values, bool),
                (ignore_errors, bool),
                (infer_long_args, bool),
                (infer_subcommands, bool),
                (long_about, String),
                (long_flag, String),
                (long_flag_alias, String),
                (long_flag_aliases, Vec<String>),
                (long_version, String),
                (max_term_width, usize),
                (multicall, bool),
                (name, String),
                (next_display_order, Option<usize>),
                // next_help_heading : specialized
                (next_line_help, bool),
                (no_binary_name, bool),
                (override_help, String),
                (override_usage, String),
                (propagate_version, bool),
                // setting(s) / global_setting(s) : specialized (clap 3 compatibility)
                (short_flag, char),
                (short_flag_alias, char),
                (short_flag_aliases, Vec<char>),
                // subcommand : not supported single subcommand(now)
                // subcommands : specialized
                (subcommand_help_heading, String),
                (subcommand_negates_reqs, bool),
                (subcommand_precedence_over_arg, bool),
                (subcommand_required, bool),
                (subcommand_value_name, String),
                (term_width, usize),
                (trailing_var_arg, bool),
                (version, String),
                (visible_alias, String),
                (visible_aliases, Vec<String>),
                (visible_long_flag_alias, String),
                (visible_long_flag_aliases, Vec<String>),
                (visible_short_flag_alias, char),
                (visible_short_flag_aliases, Vec<char>),
            },
            deprecated: [
                "help_message",
                "version_message",
                "allow_invalid_utf8_for_external_subcommands",
            ]{
                "help_heading" => "next_help_heading",
            },
            not_supported: {
                "arg" => "Use args instead",
                "group" => "Use groups instead",
            },
            specialize:
            [
                "args" => map.next_value_seed(super::arg::Args::<true>(app))?
                "args_map" => map.next_value_seed(super::arg::Args::<false>(app))?
                "color" => {
                    #[cfg(feature = "color")] { parse_opt!(app, map, color::ColorChoice1, |v| app.color(v.into())) }
                    #[cfg(not(feature = "color"))] { return Err(Error::custom("color feature disabled"))}}
                "next_help_heading" => match map.next_value::<Option<String>>()? {
                    Some(h) => app.next_help_heading(h),
                    None => app.next_help_heading(clap::builder::Resettable::Reset),
                }
                "subcommands" => map.next_value_seed(SubCommands::<true>(app))?
                "subcommands_map" => map.next_value_seed(SubCommands::<false>(app))?
                "groups" => map.next_value_seed(super::group::Groups(app))?
                "help_template" => {
                    #[cfg(feature = "help")] { parse_value_inner!(app, map, Command, String, help_template) }
                    #[cfg(not(feature = "help"))] { return Err(Error::custom("help feature disabled"))}}
                "setting" | "settings" | "global_setting" | "global_settings" => {
                    #[cfg(feature = "allow-deprecated")]
                    { appsettings::apply_settings(app, map.next_value()?)? }
                    #[cfg(not(feature = "allow-deprecated"))]
                    { return Err(Error::custom(format_args!("deprecated key: {}", key))) }
                }
            ]);
        }

        Ok(app.into())
    }
}

pub struct NameSeed(String);

impl<'de> DeserializeSeed<'de> for NameSeed {
    type Value = CommandWrap;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(CommandVisitor(Command::new(self.0)))
    }
}

impl<'de> DeserializeSeed<'de> for CommandWrap {
    type Value = CommandWrap;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(CommandVisitor(self.app))
    }
}

struct SubCommands<const KV_ARRAY: bool>(Command);
impl<'de, const KV_ARRAY: bool> DeserializeSeed<'de> for SubCommands<KV_ARRAY> {
    type Value = Command;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if KV_ARRAY {
            deserializer.deserialize_seq(self)
        } else {
            deserializer.deserialize_map(self)
        }
    }
}

impl<'de, const KV_ARRAY: bool> Visitor<'de> for SubCommands<KV_ARRAY> {
    type Value = Command;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Subcommand")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut app = self.0;
        while let Some(name) = map.next_key::<String>()? {
            let sub = map.next_value_seed(NameSeed(name))?;
            app = app.subcommand(sub);
        }
        Ok(app)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut app = self.0;
        while let Some(sub) = seq.next_element_seed(InnerSubCommand)? {
            app = app.subcommand(sub)
        }
        Ok(app)
    }
}

pub struct InnerSubCommand;
impl<'de> Visitor<'de> for InnerSubCommand {
    type Value = Command;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Subcommand Inner")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let k: String = map
            .next_key()?
            .ok_or_else(|| A::Error::invalid_length(0, &"missing command in subcommand"))?;
        let com = map.next_value_seed(NameSeed(k))?;
        Ok(com.into())
    }
}

impl<'de> DeserializeSeed<'de> for InnerSubCommand {
    type Value = Command;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}
