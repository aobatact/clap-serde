use crate::AppWrap;
use clap::App;
use serde::{
    de::{DeserializeSeed, Error, Visitor},
    Deserialize,
};

use super::appsettings::AppSetting1;

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
                (&str, name),
                (&str, version),
                (&str, long_version),
                (&str, author),
                (&str, bin_name),
                (&str, about),
                (&str, long_about),
                (&str, before_help),
                (&str, after_help),
                (&str, help_template),
                (&str, override_usage),
                (&str, override_help),
                (&str, alias),
                (&str, visible_alias),
                (ref Vec<&str>, visible_aliases),
                (usize, display_order),
            },
            deprecated: [
                "help_message",
                "version_message",
            ]
            [
                "args" => map.next_value_seed(super::arg::Args(app))?
                "subcommands" => map.next_value_seed(SubCommands(app))?
                "groups" => map.next_value_seed(super::group::Groups(app))?
                "setting" => app.setting(map.next_value_seed(super::appsettings::AppSettingSeed)?)
                "settings" => app.setting(map.next_value_seed(super::appsettings::AppSettingsSeed)?)
                "global_setting" => app.global_setting(map.next_value_seed(super::appsettings::AppSettingSeed)?)
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
