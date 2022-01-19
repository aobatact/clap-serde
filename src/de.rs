use crate::AppWrap;
use clap::App;
use serde::{
    de::{DeserializeSeed, Error, Visitor},
    Deserialize,
};

const TMP_APP_NAME: &'static str = "__tmp__deserialize__name__";
impl<'de> Deserialize<'de> for AppWrap<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_map(AppVisitor(TMP_APP_NAME))
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

struct AppVisitor<'a>(&'a str);

impl<'a> Visitor<'a> for AppVisitor<'a> {
    type Value = AppWrap<'a>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("App Map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'a>,
    {
        let mut app = App::new(self.0);
        macro_rules! parse_value {
            ( $app : ident, $map : ident, $value_type:ty, $register : ident) => {
                App::$register($app, $map.next_value::<$value_type>()?)
            };
            ( $app : ident, $map : ident, ref $value_type:ty, $register : ident) => {
                App::$register($app, &$map.next_value::<$value_type>()?)
            };
        }

        //TODO: check the first key to get name from the input?
        //currently the name change in `Clap::App::name` doesn't change the `Clap::App::id` so might cause problems? 
        while let Some(key) = map.next_key::<&str>()? {
            app = match key {
                "name" => parse_value!(app, map, &str, name),
                "version" => parse_value!(app, map, &str, version),
                "long_version" => parse_value!(app, map, &str, long_version),
                "author" => parse_value!(app, map, &str, author),
                "bin_name" => parse_value!(app, map, &str, bin_name),
                "about" => parse_value!(app, map, &str, about),
                "long_about" => parse_value!(app, map, &str, long_about),
                "before_help" => parse_value!(app, map, &str, before_help),
                "after_help" => parse_value!(app, map, &str, after_help),
                "template" => parse_value!(app, map, &str, help_template),
                "usage" => parse_value!(app, map, &str, override_usage),
                "help" => parse_value!(app, map, &str, override_help),
                "alias" => parse_value!(app, map, &str, alias),
                "aliases" => parse_value!(app, map, ref Vec<&str>, aliases), //TODO: no alloc
                "visible_alias" => parse_value!(app, map, &str, visible_alias),
                "visible_aliases" => parse_value!(app, map, ref Vec<&str>, visible_aliases), //TODO: no alloc
                "display_order" => parse_value!(app, map, usize, display_order),
                // "help_message" => parse_value!(app, map, &str, help_message), .. are deprecated.
                "help_message" | "version_message" => {
                    return Err(<A::Error>::custom("deprecated fields"))
                }
                "args" => todo!(),
                "subcommands" => map.next_value_seed(SubCommands(app))?,
                "groups" => {
                    todo!()
                }
                "setting" | "settings" => {
                    todo!()
                }
                _ => app, //currently it ignores the invlid field
            }
        }

        Ok(AppWrap { app })
    }
}

struct NameSeed<'a>(&'a str);
impl<'de> DeserializeSeed<'de> for NameSeed<'de> {
    type Value = AppWrap<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(AppVisitor(self.0))
    }
}

struct SubCommands<'a>(App<'a>);
impl<'de> DeserializeSeed<'de> for SubCommands<'de> {
    type Value = App<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SubCommandVisitor<'a>(App<'a>);
        impl<'de> Visitor<'de> for SubCommandVisitor<'de> {
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

        deserializer.deserialize_map(SubCommandVisitor(self.0))
    }
}
