use crate::ArgWrap;
use clap::{App, Arg};
use serde::de::{DeserializeSeed, Error, Visitor};

pub struct ArgVisitor<'a>(Arg<'a>);

impl<'a> ArgVisitor<'a> {
    fn new_str(v: &'a str) -> Self {
        Self(Arg::new(v))
    }
}

impl<'a> Visitor<'a> for ArgVisitor<'a> {
    type Value = ArgWrap<'a>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Arg Map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'a>,
    {
        let mut arg = self.0;

        //TODO: handle_vec_or_str
        while let Some(key) = map.next_key::<&str>()? {
            arg = parse_value!(key, arg, map, Arg, {
                    (short, char),
                    (long, &str),
                    ref (aliases, Vec<&str>),
                    (help, &str),
                    (long_help, &str),
                    (required, bool),
                    (takes_value, bool),
                    (index, usize),
                    (global, bool),
                    (multiple_values, bool),
                    (hide, bool),
                    (next_line_help, bool),
                    (group, &str),
                    (number_of_values, usize),
                    (max_values, usize),
                    (min_values, usize),
                    (value_name, &str),
                    (use_delimiter, bool),
                    (allow_hyphen_values, bool),
                    (last, bool),
                    (require_delimiter, bool),
                    (value_delimiter, char),
                    (required_unless_present, &str),
                    (display_order, usize),
                    (default_value, &str),
                    ref (value_names, Vec<&str>),
                    ref (groups, Vec<&str>),
                    (requires, &str),
                    (conflicts_with, &str),
                    (overrides_with, &str),
                    (possible_values, Vec<&str>),
                    (ignore_case, bool),
                    ref (required_unless_present_any, Vec<&str>),
                    ref (required_unless_present_all, Vec<&str>),
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
                    "setting",
                    "settings",
                ]
                [
                    "env" => {
                        #[cfg(env)] { parse_value_inner!(arg, map, Arg, &str, env) }
                        #[cfg(not(env))] { return Err(Error::custom("env feature disabled"))}}
                ]
            );
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

impl<'de> DeserializeSeed<'de> for ArgWrap<'de> {
    type Value = ArgWrap<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ArgVisitor(self.arg))
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
            app = app.arg(map.next_value_seed(ArgVisitor::new_str(name))?);
        }
        Ok(app)
    }
}
