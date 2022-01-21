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

        //TODO: handle_vec_or_str
        while let Some(key) = map.next_key::<&str>()? {
            arg = parse_value!(key, arg, map, Arg, {
                (char, short),
                (&str, long),
                (ref Vec<&str>, aliases),
                (&str, help),
                (&str, long_help),
                (bool, required),
                (bool, takes_value),
                (usize, index),
                (bool, global),
                (bool, multiple_values),
                (bool, hide),
                (bool, next_line_help),
                (&str, group),
                (usize, number_of_values),
                (usize, max_values),
                (usize, min_values),
                (&str, value_name),
                (bool, use_delimiter),
                (bool, allow_hyphen_values),
                (bool, last),
                (bool, require_delimiter),
                (char, value_delimiter),
                (&str, required_unless_present),
                (usize, display_order),
                (&str, default_value),
                (ref Vec<&str>, value_names),
                (ref Vec<&str>, groups),
                (&str, requires),
                (&str, conflicts_with),
                (&str, overrides_with),
                (Vec<&str>, possible_values),
                (bool, ignore_case),
                (ref Vec<&str>, required_unless_present_any),
                (ref Vec<&str>, required_unless_present_all),
                //     "default_value_if" => todo!(),
                //     "default_value_ifs" => todo!(),
                //     #[cfg(env)]
                //     "env" => parse_value!(arg, map, &str, env),
                //     "requires_if" => todo!(), //parse_value!(arg, map, &str, requires_if),
                //     "requires_ifs" => todo!(),
            },
            deprecated:
            [
                "required_if",
                "multiple",
                "required_unless",
                "required_unless_one",
                "required_unless_all",
            ]);
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
