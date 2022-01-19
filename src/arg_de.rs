use crate::ArgWrap;
use clap::{App, Arg};
use serde::de::{DeserializeSeed, Error, Visitor};

struct ArgVisitor<'a>(&'a str);

impl<'a> Visitor<'a> for ArgVisitor<'a> {
    type Value = ArgWrap<'a>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Arg Map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'a>,
    {
        let mut arg = Arg::new(self.0);
        macro_rules! parse_value {
            ( $app : ident, $map : ident, $value_type:ty, $register : ident) => {
                Arg::$register($app, $map.next_value::<$value_type>()?)
            };
            ( $app : ident, $map : ident, ref $value_type:ty, $register : ident) => {
                Arg::$register($app, &$map.next_value::<$value_type>()?)
            };
        }

        //TODO: handle_vec_or_str
        while let Some(key) = map.next_key::<&str>()? {
            arg = match key {
                "short" => parse_value!(arg, map, char, short),
                "long" => parse_value!(arg, map, &str, long),
                "aliases" => parse_value!(arg, map, ref Vec<&str>, aliases), //TODO: no alloc
                "help" => parse_value!(arg, map, &str, help),
                "long_help" => parse_value!(arg, map, &str, long_help),
                "required" => parse_value!(arg, map, bool, required),
                "takes_value" => parse_value!(arg, map, bool, takes_value),
                "index" => parse_value!(arg, map, usize, index),
                "global" => parse_value!(arg, map, bool, global),
                "multiple_occurences" => parse_value!(arg, map, bool, multiple_occurrences),
                "multiple_values" => parse_value!(arg, map, bool, multiple_values),
                "hide" => parse_value!(arg, map, bool, hide),
                "next_line_help" => parse_value!(arg, map, bool, next_line_help),
                "group" => parse_value!(arg, map, &str, group),
                "number_of_values" => parse_value!(arg, map, usize, number_of_values),
                "max_values" => parse_value!(arg, map, usize, max_values),
                "min_values" => parse_value!(arg, map, usize, min_values),
                "value_name" => parse_value!(arg, map, &str, value_name),
                "use_delimiter" => parse_value!(arg, map, bool, use_delimiter),
                "allow_hyphen_values" => parse_value!(arg, map, bool, allow_hyphen_values),
                "last" => parse_value!(arg, map, bool, last),
                "require_delimiter" => parse_value!(arg, map, bool, require_delimiter),
                "value_delimiter" => parse_value!(arg, map, char, value_delimiter),
                "required_unless_present" => parse_value!(arg, map, &str, required_unless_present),
                "display_order" => parse_value!(arg, map, usize, display_order),
                "default_value" => parse_value!(arg, map, &str, default_value),
                "default_value_if" => todo!(),
                "default_value_ifs" => todo!(),
                #[cfg(env)]
                "env" => parse_value!(arg, map, &str, env),
                "value_names" => parse_value!(arg, map, ref Vec<&str>, value_names),
                "groups" => parse_value!(arg, map, ref Vec<&str>, groups),
                "requires" => parse_value!(arg, map, &str, requires),
                "requires_if" => todo!(), //parse_value!(arg, map, &str, requires_if),
                "requires_ifs" => todo!(),
                "conflicts_with" => parse_value!(arg, map, &str, conflicts_with),
                "overrides_with" => parse_value!(arg, map, &str, overrides_with),
                "possible_values" => parse_value!(arg, map, Vec<&str>, possible_values),
                "case_insensitive" => parse_value!(arg, map, bool, ignore_case),
                "required_unless_present_any" => {
                    parse_value!(arg, map, ref Vec<&str>, required_unless_present_any)
                }
                "requierd_unless_present_all" => {
                    parse_value!(arg, map, ref Vec<&str>, required_unless_present_all)
                }
                unkonw => return Err(Error::unknown_field(unkonw, &[""])),
            }
        }
        Ok(ArgWrap { arg })
    }
}

impl<'de> DeserializeSeed<'de> for ArgVisitor<'de> {
    type Value = ArgWrap<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

pub(crate) struct Args<'a>(pub(crate) App<'a>);
impl<'de> DeserializeSeed<'de> for Args<'de> {
    type Value = App<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de> Visitor<'de> for Args<'de> {
    type Value = App<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("args")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut app = self.0;
        while let Some(name) = map.next_key::<&str>()? {
            app = app.arg(map.next_value_seed(ArgVisitor(name))?);
        }
        Ok(app)
    }
}
