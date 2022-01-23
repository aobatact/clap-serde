use crate::AppWrap;
use appsettings::*;
use clap::App;
use serde::{
    de::{DeserializeSeed, Error, Visitor},
    Deserialize,
};

mod appsettings;
#[cfg(feature = "color")]
mod color;

const TMP_APP_NAME: &'static str = "__tmp__deserialize__name__";
impl<'de> Deserialize<'de> for AppWrap<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_map(AppVisitor(App::new(TMP_APP_NAME)))
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

struct AppVisitor<'a>(App<'a>);

impl<'a> Visitor<'a> for AppVisitor<'a> {
    type Value = AppWrap<'a>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("App Map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'a>,
    {
        let mut app = self.0;
        //TODO: check the first key to get name from the input?
        //currently the name change in `Clap::App::name` doesn't change the `Clap::App::id` so might cause problems?
        while let Some(key) = map.next_key::<&str>()? {
            app = parse_value!(key, app, map, App, {
                (about, &str),
                (after_help, &str),
                (after_long_help, &str),
                (alias, &str),
                ref (aliases, Vec<&str>),
                //arg : not supported single arg(now)
                //args : specialized
                (author, &str),
                (before_help, &str),
                (before_long_help, &str),
                (bin_name, &str),
                // color : specialized
                (display_order, usize),
                // global_setting : specialized
                // global_settings : specialized (though the original method is deprecated)
                // group : not supported single group
                // groups : specialized
                (help_heading, Option<&str>),
                (help_template, &str),
                (long_about, &str),
                (long_flag, &str),
                (long_flag_alias, &str),
                ref (long_flag_aliases, Vec<&str>),
                (long_version, &str),
                (max_term_width, usize),
                (name, &str),
                (override_help, &str),
                (override_usage, &str),
                // setting : specialized
                // settings : specialized (though the original method is deprecated)
                (short_flag, char),
                (short_flag_alias, char),
                ref (short_flag_aliases, Vec<char>),
                // subcommand : not supported single subcommand(now)
                // subcommands : specialized
                (term_width, usize),
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
            ]
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

        Ok(AppWrap { app })
    }
}

pub struct NameSeed<'a>(&'a str);

impl<'de> DeserializeSeed<'de> for NameSeed<'de> {
    type Value = AppWrap<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(AppVisitor(App::new(self.0)))
    }
}

impl<'de> DeserializeSeed<'de> for AppWrap<'de> {
    type Value = AppWrap<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(AppVisitor(self.app))
    }
}

struct SubCommands<'a>(App<'a>);
impl<'de> DeserializeSeed<'de> for SubCommands<'de> {
    type Value = App<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de> Visitor<'de> for SubCommands<'de> {
    type Value = App<'de>;

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
