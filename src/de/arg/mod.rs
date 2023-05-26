use self::{arg_action::ArgAction, value_hint::ValueHint, value_parser::ValueParser};
use crate::ArgWrap;
use clap::{Arg, Command};
use serde::de::{DeserializeSeed, Error, Visitor};

mod arg_action;
mod value_hint;
mod value_parser;

#[cfg(feature = "override-arg")]
struct ArgKVO(Option<Command>);

#[cfg(feature = "override-arg")]
impl<'de> Visitor<'de> for &mut ArgKVO<'de> {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("kv argument")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let name: &str = map
            .next_key()?
            .ok_or_else(|| A::Error::missing_field("argument"))?;
        let mut status = Ok(());
        let app = unsafe { self.0.take().unwrap_unchecked() };
        let next = app.mut_arg(name, |a| match map.next_value_seed(ArgVisitor(a)) {
            Ok(a) => a.into(),
            Err(e) => {
                status = Err(e);
                Arg::new(name)
            }
        });
        self.0.replace(next);
        status
    }
}

#[cfg(feature = "override-arg")]
impl<'de> DeserializeSeed<'de> for &mut ArgKVO<'de> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

#[derive(Clone, Copy)]
struct ArgKV;

impl<'de> Visitor<'de> for ArgKV {
    type Value = ArgWrap;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("kv argument")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let name: &str = map
            .next_key()?
            .ok_or_else(|| A::Error::missing_field("argument"))?;
        map.next_value_seed(ArgVisitor::new_str(name))
    }
}

impl<'de> DeserializeSeed<'de> for ArgKV {
    type Value = ArgWrap;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

struct ArgVisitor(Arg);

impl ArgVisitor {
    fn new_str(v: &str) -> Self {
        Self(Arg::new(v.to_owned()))
    }
}

impl<'de> Visitor<'de> for ArgVisitor {
    type Value = ArgWrap;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Arg Map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut arg = self.0;

        while let Some(key) = map.next_key::<&str>()? {
            arg = parse_value!(key, arg, map, Arg, {
                    // action : specailized
                    (alias, String),
                    ref (aliases, Vec<String>),
                    (allow_hyphen_values, bool),
                    (conflicts_with, String),
                    ref (conflicts_with_all, Vec<String>),
                    (default_missing_value, String),
                    ref (default_missing_values, Vec<String>),
                    (default_value, String),
                    // default_value_if : tuple3
                    (default_value_ifs, Vec<(String, String, String)> ),
                    (display_order, usize),
                    // env : specialized
                    // env_os // not supported yet
                    (exclusive, bool),
                    (global, bool),
                    (group, String),
                    ref (groups, Vec<String>),
                    (help, String),
                    (help_heading, String),
                    (hide, bool),
                    (hide_default_value, bool),
                    // hide_env : specialized
                    // hide_env_values : specialized
                    (hide_long_help, bool),
                    (hide_possible_values, bool),
                    (hide_short_help, bool),
                    (id, String),
                    (ignore_case, bool),
                    (index, usize),
                    (last, bool),
                    (long, String),
                    (long_help, String),
                    (id, String),
                    (next_line_help, bool),
                    (number_of_values, usize),
                    (overrides_with, String),
                    ref (overrides_with_all, Vec<String>),
                    (require_equals, bool),
                    (raw, bool),
                    (required, bool),
                    // required_if_eq: tuple2
                    (required_if_eq_all, Vec<(String, String)>),
                    (required_if_eq_any, Vec<(String, String)>),
                    (required_unless_present, String),
                    ref (required_unless_present_any, Vec<String>),
                    ref (required_unless_present_all, Vec<String>),
                    (requires, String),
                    ref (requires_all, Vec<String>),
                    // requires_if: tuple2
                    (requires_ifs, Vec<(String, String)>),
                    (short, char),
                    (short_alias, char),
                    (short_aliases, Vec<char>),
                    (use_value_delimiter, bool),
                    // validator_regex : todo
                    // value_hint : specialized
                    (value_delimiter, char),
                    (value_name, String),
                    ref (value_names, Vec<String>),
                    // value_parser : specialized
                    (value_terminator, String),
                    (visible_alias, String),
                    ref (visible_aliases, Vec<String>),
                    (visible_short_alias, char),
                    (visible_short_aliases, Vec<char>),
                },
                tuple2: {
                    (required_if_eq, (String, String)),
                    (requires_if, (String, String)),
                },
                tuple3: {
                    (default_value_if, (String, String, String)),
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
                    "allow_invalid_utf8",
                    "forbid_empty_values",
                    "max_occurrences",
                    "max_values",
                    "min_values",
                    "multiple_occurrences",
                    "multiple_values",
                    "possible_value",
                    "possible_values",
                    "require_value_delimiter",
                    "takes_value",
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
                        arg.action(clap::ArgAction::from( map.next_value::<ArgAction>()?))
                    }
                    "env" => {
                        #[cfg(feature = "env")] { parse_value_inner!(arg, map, Arg, &str, env) }
                        #[cfg(not(feature = "env"))] { return Err(Error::custom("env feature disabled"))}}
                    "hide_env" => {
                        #[cfg(feature = "env")] { parse_value_inner!(arg, map, Arg, bool, hide_env) }
                        #[cfg(not(feature = "env"))] { return Err(Error::custom("env feature disabled"))}}
                    "hide_env_values" => {
                        #[cfg(feature = "env")] { parse_value_inner!(arg, map, Arg, bool, hide_env_values) }
                        #[cfg(not(feature = "env"))] { return Err(Error::custom("env feature disabled"))}}
                    "value_hint" => {
                        arg.value_hint(clap::ValueHint::from(map.next_value::<ValueHint>()?))
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

impl<'de> DeserializeSeed<'de> for ArgVisitor {
    type Value = ArgWrap;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de> DeserializeSeed<'de> for ArgWrap {
    type Value = ArgWrap;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ArgVisitor(self.arg))
    }
}

pub(crate) struct Args<const USE_ARRAY: bool>(pub(crate) Command);
impl<'de, const USE_ARRAY: bool> DeserializeSeed<'de> for Args<USE_ARRAY> {
    type Value = Command;

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

impl<'de, const USE_ARRAY: bool> Visitor<'de> for Args<USE_ARRAY> {
    type Value = Command;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("args")
    }

    #[cfg(feature = "override-arg")]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut argkvo = ArgKVO(Some(self.0));

        while (seq.next_element_seed(&mut argkvo)?).is_some() {}
        Ok(unsafe { argkvo.0.unwrap_unchecked() })
    }

    #[cfg(not(feature = "override-arg"))]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let x = ArgKV;
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
