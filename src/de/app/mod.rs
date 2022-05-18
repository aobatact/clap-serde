use crate::CommandWrap;
use appsettings::*;
use clap::Command;
use serde::{
    de::{DeserializeSeed, Error, Visitor},
    Deserialize,
};

mod appsettings;
#[cfg(feature = "color")]
mod color;

const TMP_APP_NAME: &str = "__tmp__deserialize__name__";
impl<'de> Deserialize<'de> for CommandWrap<'de> {
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

struct CommandVisitor<'a>(Command<'a>);

impl<'a> Visitor<'a> for CommandVisitor<'a> {
    type Value = CommandWrap<'a>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Command Map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'a>,
    {
        let mut app = self.0;
        //TODO: check the first key to get name from the input?
        //currently the name change in `Clap::Command::name` doesn't change the `Clap::Command::id` so might cause problems?
        while let Some(key) = map.next_key::<&str>()? {
            app = parse_value!(key, app, map, Command, {
                (about, &str),
                (after_help, &str),
                (after_long_help, &str),
                (alias, &str),
                ref (aliases, Vec<&str>),
                (allow_external_subcommands, bool),
                (allow_hyphen_values, bool),
                (allow_invalid_utf8_for_external_subcommands, bool),
                (allow_missing_positional, bool),
                (allow_negative_numbers, bool),
                //arg : not supported single arg(now)
                //args : specialized
                (arg_required_else_help, bool),
                (args_conflicts_with_subcommands, bool),
                (args_override_self, bool),
                (author, &str),
                (before_help, &str),
                (before_long_help, &str),
                (bin_name, &str),
                // color : specialized
                (disable_colored_help, bool),
                (disable_help_flag, bool),
                (disable_help_subcommand, bool),
                (disable_version_flag, bool),
                (display_order, usize),
                (dont_collapse_args_in_usage, bool),
                (dont_delimit_trailing_values, bool),
                // global_setting : specialized
                // global_settings : specialized (though the original method is deprecated)
                // group : not supported single group
                // groups : specialized
                // (help_heading, Option<&str>),
                (help_expected, bool),
                (help_template, &str),
                (hide, bool),
                (hide_possible_values, bool),
                (ignore_errors, bool),
                (infer_long_args, bool),
                (infer_subcommands, bool),
                (long_about, &str),
                (long_flag, &str),
                (long_flag_alias, &str),
                ref (long_flag_aliases, Vec<&str>),
                (long_version, &str),
                (max_term_width, usize),
                (name, &str),
                (next_display_order, Option<usize>),
                (next_help_heading, Option<&str>),
                (next_line_help, bool),
                (no_binary_name, bool),
                (override_help, &str),
                (override_usage, &str),
                (propagate_version, bool),
                // setting : specialized
                // settings : specialized (though the original method is deprecated)
                (short_flag, char),
                (short_flag_alias, char),
                ref (short_flag_aliases, Vec<char>),
                // subcommand : not supported single subcommand(now)
                // subcommands : specialized
                (term_width, usize),
                (trailing_var_arg, bool),
                (version, &str),
                (visible_alias, &str),
                ref (visible_aliases, Vec<&str>),
                (visible_long_flag_alias, &str),
                ref (visible_long_flag_aliases, Vec<&str>),
                (visible_short_flag_alias, char),
                ref (visible_short_flag_aliases, Vec<char>),
            },
            deprecated: [
                "help_message",
                "version_message",
            ]{
                "help_heading" => "next_help_heading",
            },
            specialize:
            [
                "args" => map.next_value_seed(super::arg::Args(app))?
                "color" => {
                    #[cfg(color)] {
                        app.color(map.next_value_seed(ColorChoiceSeed)?)
                    }
                    #[cfg(not(color))] { return Err(Error::custom("color feature disabled"))}}
                "subcommands" => map.next_value_seed(SubCommands(app))?
                "groups" => map.next_value_seed(super::group::Groups(app))?
                "setting" => app.setting(map.next_value_seed(AppSettingSeed)?)
                "settings" => app.setting(map.next_value_seed(AppSettingsSeed)?)
                "global_setting" => app.global_setting(map.next_value_seed(AppSettingSeed)?)
                "global_settings" => {
                    let sets = map.next_value::<Vec<AppSetting1>>()?.into_iter().map(|s|s.into());
                    for s in sets{
                        app = app.global_setting(s);
                    }
                    app
                }
            ]);
        }

        Ok(CommandWrap { app })
    }
}

pub struct NameSeed<'a>(&'a str);

impl<'de> DeserializeSeed<'de> for NameSeed<'de> {
    type Value = CommandWrap<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(CommandVisitor(Command::new(self.0)))
    }
}

impl<'de> DeserializeSeed<'de> for CommandWrap<'de> {
    type Value = CommandWrap<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(CommandVisitor(self.app))
    }
}

struct SubCommands<'a>(Command<'a>);
impl<'de> DeserializeSeed<'de> for SubCommands<'de> {
    type Value = Command<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de> Visitor<'de> for SubCommands<'de> {
    type Value = Command<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Subcommand")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut app = self.0;
        while let Some(name) = map.next_key::<&str>()? {
            let sub = map.next_value_seed(NameSeed(name))?;
            app = app.subcommand(sub);
        }
        Ok(app)
    }
}
