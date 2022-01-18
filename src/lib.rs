use clap::App;
use serde::{
    de::{Error, Visitor},
    Deserialize,
};

pub struct AppWrap<'a> {
    app: App<'a>,
}

impl<'a> From<AppWrap<'a>> for App<'a> {
    fn from(a: AppWrap<'a>) -> Self {
        a.app
    }
}

impl<'de> Deserialize<'de> for AppWrap<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AppVisitor;

        impl<'a> Visitor<'a> for AppVisitor {
            type Value = AppWrap<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("App Map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'a>,
            {
                const TMP_APP_NAME: &'static str = "__tmp__deserialize__name$$__";
                let mut app = App::new(TMP_APP_NAME);
                macro_rules! parse_value {
                    ( $app : ident, $map : ident, $value_type:ty, $register : ident) => {
                        App::$register($app, $map.next_value::<$value_type>()?)
                    };
                    ( $app : ident, $map : ident, ref $value_type:ty, $register : ident) => {
                        App::$register($app, &$map.next_value::<$value_type>()?)
                    };
                }

                while let Some( key) = map.next_key::<&str>()? {
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
                        "aliases" => parse_value!(app, map, ref Vec<&str>, aliases),//TODO: no alloc
                        "visible_alias" => parse_value!(app, map, &str, visible_alias),
                        "visible_aliases" => parse_value!(app, map, ref Vec<&str>, visible_aliases),//TODO: no alloc
                        "display_order" => parse_value!(app, map, usize, display_order),
                        // "help_message" => parse_value!(app, map, &str, help_message), .. are deprecated.
                        "help_message" | "version_message" => {
                            return Err(<A::Error>::custom("deprecated fields"))
                        }
                        "args" => todo!(),
                        "subcommands" => {
                            todo!()
                        },
                        "groups" => {
                            todo!()
                        }
                        "setting" | "settings" => {
                            todo!()
                        }
                        _ => app, //currently it ignores the invlid field
                    }
                }

                //check the name so as not to expose the tmp name.
                if app.get_name() == TMP_APP_NAME {
                    Err(<A::Error>::missing_field("name"))
                } else {
                    Ok(AppWrap { app })
                }
            }
        }

        deserializer.deserialize_map(AppVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::AppWrap;
    use clap::App;
    use serde::Deserialize;

    //currently fails... beacuse serde_yaml only supports `DeserializeOwned` and no zero copy deserialization
    // #[test]
    // fn name_yaml() {
    //     const NAME_YAML: &'static str = "name : app_clap_serde";
    //     let app: App = serde_yaml::from_str::<AppWrap>(NAME_YAML).expect("parse failed").into();
    //     assert_eq!(app.get_name(), "app_clap_serde");
    // }

    #[test]
    fn name_json() {
        const NAME_JSON: &'static str = "{ \"name\" : \"app_clap_serde\" }";
        let app : App = serde_json::from_str::<AppWrap>(NAME_JSON).expect("parse failed").into();
        assert_eq!(app.get_name(), "app_clap_serde");
    }

    #[test]
    fn name_toml() {
        const NAME_TOML: &'static str = "name = \"app_clap_serde\"";
        let app : App = toml::from_str::<AppWrap>(NAME_TOML).expect("parse failed").into();
        assert_eq!(app.get_name(), "app_clap_serde");
    }
}
