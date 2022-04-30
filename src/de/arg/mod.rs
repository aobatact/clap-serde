use self::value_hint::ValueHintSeed;
use crate::ArgWrap;
use clap::{Command, Arg};
use serde::de::{DeserializeSeed, Error, Visitor};

mod value_hint;

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

        while let Some(key) = map.next_key::<&str>()? {
            arg = parse_value!(key, arg, map, Arg, {
                    (alias, &str),
                    ref (aliases, Vec<&str>),
                    (allow_hyphen_values, bool),
                    (allow_invalid_utf8, bool),
                    (conflicts_with, &str),
                    ref (conflicts_with_all, Vec<&str>),
                    (default_missing_value, &str),
                    ref (default_missing_values, Vec<&str>),
                    (default_value, &str),
                    // default_value_if : tuple3
                    ref (default_value_ifs, Vec<(&str, Option<&str>, Option<&str>)> ),
                    (display_order, usize),
                    // env : specialized
                    (exclusive, bool),
                    (forbid_empty_values, bool),
                    (global, bool),
                    (group, &str),
                    ref (groups, Vec<&str>),
                    (help, &str),
                    (help_heading, &str),
                    (hide, bool),
                    (hide_default_value, bool),
                    // hide_env : specialized
                    // hide_env_values : specialized
                    (hide_long_help, bool),
                    (hide_possible_values, bool),
                    (hide_short_help, bool),
                    (ignore_case, bool),
                    (index, usize),
                    (last, bool),
                    (long, &str),
                    (long_help, &str),
                    (max_occurrences, usize),
                    (max_values, usize),
                    (min_values, usize),
                    (multiple_occurrences, bool),
                    (multiple_values, bool),
                    (name, &str),
                    (next_line_help, bool),
                    (number_of_values, usize),
                    (overrides_with, &str),
                    ref (overrides_with_all, Vec<&str>),
                    (possible_value, &str),
                    (possible_values, Vec<&str>),
                    (raw, bool),
                    (require_delimiter, bool),
                    (require_equals, bool),
                    (required, bool),
                    // required_if_eq: tuple2
                    ref (required_if_eq_all, Vec<(&str, &str)>),
                    ref (required_if_eq_any, Vec<(&str, &str)>),
                    (required_unless_present, &str),
                    ref (required_unless_present_any, Vec<&str>),
                    ref (required_unless_present_all, Vec<&str>),
                    (requires, &str),
                    ref (requires_all, Vec<&str>),
                    // requires_if: tuple2
                    ref (requires_ifs, Vec<(&str, &str)>),
                    (short, char),
                    (short_alias, char),
                    ref (short_aliases, Vec<char>),
                    (takes_value, bool),
                    (use_delimiter, bool),
                    // validator_regex
                    // value_hint : specialized
                    (value_delimiter, char),
                    (value_name, &str),
                    ref (value_names, Vec<&str>),
                    (value_terminator, &str),
                    (visible_alias, &str),
                    ref (visible_aliases, Vec<&str>),
                    (visible_short_alias, char),
                    ref (visible_short_aliases, Vec<char>),
                },
                tuple2: {
                    (required_if_eq, (&str, &str)),
                    (requires_if, (&str, &str)),
                },
                tuple3: {
                    (default_value_if, (&str, Option<&str>, Option<&str>)),
                },
                deprecated:
                [
                    "case_insensitive",
                    "empty_values",
                    "from_usage",
                    "hidden",
                    "hidden_long_help",
                    "hidden_short_help",
                    "multiple",
                    "required_if",
                    "required_ifs",
                    "required_unless",
                    "required_unless_all",
                    "required_unless_one",
                    "set",
                    "setting",
                    "settings",
                    "with_name",
                ]
                specialize:[
                    "env" => {
                        #[cfg(env)] { parse_value_inner!(arg, map, Arg, &str, env) }
                        #[cfg(not(env))] { return Err(Error::custom("env feature disabled"))}}
                    "hide_env" => {
                        #[cfg(env)] { parse_value_inner!(arg, map, Arg, bool, hide_env) }
                        #[cfg(not(env))] { return Err(Error::custom("env feature disabled"))}}
                    "hide_env_values" => {
                        #[cfg(env)] { parse_value_inner!(arg, map, Arg, bool, hide_env_values) }
                        #[cfg(not(env))] { return Err(Error::custom("env feature disabled"))}}
                    "value_hint" => {
                        arg.value_hint(map.next_value_seed(ValueHintSeed)?)
                    }
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

pub(crate) struct Args<'a>(pub(crate) Command<'a>);
impl<'de> DeserializeSeed<'de> for Args<'de> {
    type Value = Command<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de> Visitor<'de> for Args<'de> {
    type Value = Command<'de>;

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
