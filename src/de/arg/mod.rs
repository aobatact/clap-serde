use self::{arg_action::ArgAction, value_hint::ValueHint, value_parser::ValueParser};
use crate::ArgWrap;
use clap::{Arg, Command};
use serde::de::{DeserializeSeed, Error, Visitor};
use std::marker::PhantomData;

mod arg_action;
mod value_hint;
mod value_parser;

#[cfg(feature = "override-arg")]
struct ArgKVO<'a>(Command<'a>);

#[cfg(feature = "override-arg")]
impl<'de> Visitor<'de> for ArgKVO<'de> {
    type Value = ArgKVO<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("kv argument")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let name: &str = map.next_key()?.ok_or(A::Error::missing_field("argument"))?;
        let mut error = None;
        let x = self
            .0
            .mut_arg(name, |a| match map.next_value_seed(ArgVisitor(a)) {
                Ok(a) => a.into(),
                Err(e) => {
                    error = Some(e);
                    Arg::new(name)
                }
            });
        Ok(ArgKVO(x))
    }
}

#[cfg(feature = "override-arg")]
impl<'de> DeserializeSeed<'de> for ArgKVO<'de> {
    type Value = ArgKVO<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

#[derive(Clone, Copy)]
struct ArgKV<'de>(PhantomData<&'de ()>);

impl<'de> Visitor<'de> for ArgKV<'de> {
    type Value = ArgWrap<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("kv argument")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let name: &str = map.next_key()?.ok_or(A::Error::missing_field("argument"))?;
        map.next_value_seed(ArgVisitor::new_str(name))
    }
}

impl<'de> DeserializeSeed<'de> for ArgKV<'de> {
    type Value = ArgWrap<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

struct ArgVisitor<'a>(Arg<'a>);

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
                    // action : specailized
                    (alias, &str),
                    ref (aliases, Vec<&str>),
                    (allow_hyphen_values, bool),
                    (allow_invalid_utf8, bool),
                    (conflicts_with, &str),
                    ref (conflicts_with_all, Vec<&str>),
                    (default_missing_value, &str),
                    ref (default_missing_values, Vec<&str>),
                    // (default_missing_value_os, &OsStr), // need Deseriaze to OsStr
                    // ref (default_missing_values_os, Vec<&OsStr>),
                    (default_value, &str),
                    // default_value_if : tuple3
                    ref (default_value_ifs, Vec<(&str, Option<&str>, Option<&str>)> ),
                    (display_order, usize),
                    // env : specialized
                    // env_os // not supported yet
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
                    (id, &str),
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
                    (id, &str),
                    (next_line_help, bool),
                    (number_of_values, usize),
                    (overrides_with, &str),
                    ref (overrides_with_all, Vec<&str>),
                    (possible_value, &str),
                    (possible_values, Vec<&str>),
                    (raw, bool),
                    (require_value_delimiter, bool),
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
                    (use_value_delimiter, bool),
                    // validator_regex : todo
                    // value_hint : specialized
                    (value_delimiter, char),
                    (value_name, &str),
                    ref (value_names, Vec<&str>),
                    // value_parser : specialized
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
                    "validator_regex",
                    "with_name",
                ]{
                    //3.1
                    "name" => "id",
                    "require_delimiter" => "require_value_delimiter",
                    "use_delimiter" => "use_value_delimiter",
                },
                // not_supported: {
                // },
                specialize:[
                    "arg_action" => {
                        arg.action(map.next_value::<ArgAction>()?.into())
                    }
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
                        arg.value_hint(map.next_value::<ValueHint>()?.into())
                    }
                    "value_parser" => {
                        arg.value_parser(map.next_value::<ValueParser>()?)
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

pub(crate) struct Args<'a, const USE_ARRAY: bool>(pub(crate) Command<'a>);
impl<'de, const USE_ARRAY: bool> DeserializeSeed<'de> for Args<'de, USE_ARRAY> {
    type Value = Command<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if USE_ARRAY {
            deserializer.deserialize_seq(self)
        } else {
            deserializer.deserialize_map(self)
        }
    }
}

impl<'de, const USE_ARRAY: bool> Visitor<'de> for Args<'de, USE_ARRAY> {
    type Value = Command<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("args")
    }

    #[cfg(feature = "override-arg")]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use serde::de::IgnoredAny;

        if let Some(len) = seq.size_hint() {
            let mut x = ArgKVO(self.0);
            for _ in 0..len {
                x = seq.next_element_seed(x)?.ok_or_else(|| {
                    A::Error::invalid_length(0, &"Actual size is shorter then size_hint")
                })?;
            }
            match seq.next_element()? {
                Some(IgnoredAny) => Err(A::Error::invalid_length(
                    0,
                    &"Actual size is longer then size_hint",
                )),
                None => Ok(x.0),
            }
        } else {
            let mut com = self.0;
            while let Some(a) = seq.next_element_seed(ArgKV(PhantomData))? {
                if com
                    .get_arguments()
                    .any(|prv_arg| prv_arg.get_id() == a.get_id())
                {
                    com = com.mut_arg(a.get_id(), |mut prv_arg| {
                        prv_arg.clone_from(&a);
                        prv_arg
                    })
                } else {
                    com = com.arg(a);
                }
            }
            Ok(com)
        }
    }

    #[cfg(not(feature = "override-arg"))]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let x = ArgKV(PhantomData);
        let mut com = self.0;
        while let Some(a) = seq.next_element_seed(x)? {
            com = com.arg(a);
        }
        Ok(com)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut app = self.0;
        while let Some(name) = map.next_key::<&str>()? {
            #[cfg(feature = "override-arg")]
            {
                let mut error = None;
                app = app.mut_arg(name, |a| match map.next_value_seed(ArgVisitor(a)) {
                    Ok(a) => a.into(),
                    Err(e) => {
                        error = Some(e);
                        Arg::new(name)
                    }
                });
                if let Some(error) = error {
                    return Err(error);
                }
            }
            #[cfg(not(feature = "override-arg"))]
            {
                app = app.arg(map.next_value_seed(ArgVisitor::new_str(name))?);
            }
        }
        Ok(app)
    }
}
