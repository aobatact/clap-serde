use self::{arg_action::ArgAction, value_hint::ValueHint, value_parser::ValueParser};
use clap::{Arg, Command};
use serde::de::{DeserializeSeed, Error, Visitor};
use std::ops::Deref;

pub(crate) mod arg_action;
pub(crate) mod value_hint;
pub(crate) mod value_parser;

/// Wrapper of [`Arg`] to deserialize with [`DeserializeSeed`](`serde::de::DeserializeSeed`).
#[derive(Debug, Clone)]
pub struct ArgWrap {
    pub(crate) arg: Arg,
}

impl ArgWrap {
    pub fn new(arg: Arg) -> Self {
        Self { arg }
    }
}

impl From<ArgWrap> for Arg {
    fn from(arg: ArgWrap) -> Self {
        arg.arg
    }
}

impl From<Arg> for ArgWrap {
    fn from(arg: Arg) -> Self {
        ArgWrap { arg }
    }
}

impl Deref for ArgWrap {
    type Target = Arg;

    fn deref(&self) -> &Self::Target {
        &self.arg
    }
}

/// Modify the arg with `name` if it already exists in `app`, otherwise add a new arg.
#[cfg(feature = "override-arg")]
fn override_or_add_arg<E>(
    app: Command,
    name: String,
    f: impl FnOnce(Arg) -> Result<ArgWrap, E>,
) -> (Command, Result<(), E>) {
    if app.get_arguments().any(|a| a.get_id() == name.as_str()) {
        let mut result = Ok(());
        let app = app.mut_arg(name.as_str(), |a| match f(a.clone()) {
            Ok(a) => a.into(),
            Err(e) => {
                result = Err(e);
                a
            }
        });
        (app, result)
    } else {
        match f(Arg::new(name)) {
            Ok(a) => (app.arg(a), Ok(())),
            Err(e) => (app, Err(e)),
        }
    }
}

#[cfg(feature = "override-arg")]
struct ArgKVO(Option<Command>);

#[cfg(feature = "override-arg")]
impl<'de> Visitor<'de> for &mut ArgKVO {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("kv argument")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let name: String = map
            .next_key()?
            .ok_or_else(|| A::Error::missing_field("argument"))?;
        let app = self.0.take().expect("ArgKVO must hold a command");
        let (app, result) = override_or_add_arg(app, name, |a| map.next_value_seed(ArgVisitor(a)));
        self.0.replace(app);
        result
    }
}

#[cfg(feature = "override-arg")]
impl<'de> DeserializeSeed<'de> for &mut ArgKVO {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

#[cfg(not(feature = "override-arg"))]
#[derive(Clone, Copy)]
struct ArgKV;

#[cfg(not(feature = "override-arg"))]
impl<'de> Visitor<'de> for ArgKV {
    type Value = ArgWrap;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("kv argument")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let name: String = map
            .next_key()?
            .ok_or_else(|| A::Error::missing_field("argument"))?;
        map.next_value_seed(ArgVisitor::new_str(name))
    }
}

#[cfg(not(feature = "override-arg"))]
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
    #[cfg(not(feature = "override-arg"))]
    fn new_str(v: String) -> Self {
        Self(Arg::new(v))
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

        while let Some(key) = map.next_key::<String>()? {
            arg = parse_value!(key, arg, map, Arg, {
                    // action : specialized
                    (alias, String),
                    (aliases, Vec<String>),
                    (allow_hyphen_values, bool),
                    (allow_negative_numbers, bool),
                    (conflicts_with, String),
                    (conflicts_with_all, Vec<String>),
                    (default_missing_value, String),
                    (default_missing_values, Vec<String>),
                    (default_value, String),
                    // default_value_if : specialized
                    // default_value_ifs : specialized
                    (default_values, Vec<String>),
                    (display_order, usize),
                    // env : specialized
                    (exclusive, bool),
                    (global, bool),
                    (group, String),
                    (groups, Vec<String>),
                    (help, String),
                    // help_heading : specialized
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
                    (next_line_help, bool),
                    // num_args : specialized
                    (number_of_values, usize),
                    (overrides_with, String),
                    (overrides_with_all, Vec<String>),
                    // possible_values : specialized
                    (raw, bool),
                    (require_equals, bool),
                    (required, bool),
                    // required_if_eq: tuple2
                    (required_if_eq_all, Vec<(String, String)>),
                    (required_if_eq_any, Vec<(String, String)>),
                    (required_unless_present, String),
                    (required_unless_present_all, Vec<String>),
                    (required_unless_present_any, Vec<String>),
                    (requires, String),
                    (requires_all, Vec<String>),
                    // requires_if: tuple2
                    (requires_ifs, Vec<(String, String)>),
                    (short, char),
                    (short_alias, char),
                    (short_aliases, Vec<char>),
                    (trailing_var_arg, bool),
                    (use_value_delimiter, bool),
                    // value_hint : specialized
                    (value_delimiter, char),
                    (value_name, String),
                    (value_names, Vec<String>),
                    // value_parser : specialized
                    (value_terminator, String),
                    (visible_alias, String),
                    (visible_aliases, Vec<String>),
                    (visible_short_alias, char),
                    (visible_short_aliases, Vec<char>),
                },
                tuple2: {
                    (required_if_eq, (String, String)),
                    (requires_if, (String, String)),
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
                    // removed in clap 4
                    "allow_invalid_utf8",
                    "forbid_empty_values",
                    "max_occurrences",
                    "max_values",
                    "min_values",
                    "multiple_occurrences",
                    "multiple_values",
                    "possible_value",
                    "require_value_delimiter",
                    "takes_value",
                ]{
                    //3.1
                    "name" => "id",
                    "require_delimiter" => "use_value_delimiter",
                    "use_delimiter" => "use_value_delimiter",
                    "arg_action" => "action",
                },
                specialize:[
                    "action" => parse_opt!(arg, map, ArgAction, |v| arg.action(clap::ArgAction::from(v)))
                    "default_value_if" => parse_opt!(arg, map, DefaultValueIf, |v| v.apply(arg))
                    "default_value_ifs" => parse_opt!(arg, map, Vec<DefaultValueIf>, |v| {
                        v.into_iter().fold(arg, |arg, c| c.apply(arg))
                    })
                    "env" => {
                        #[cfg(feature = "env")] { parse_value_inner!(arg, map, Arg, String, env) }
                        #[cfg(not(feature = "env"))] { return Err(Error::custom("env feature disabled"))}}
                    "hide_env" => {
                        #[cfg(feature = "env")] { parse_value_inner!(arg, map, Arg, bool, hide_env) }
                        #[cfg(not(feature = "env"))] { return Err(Error::custom("env feature disabled"))}}
                    "hide_env_values" => {
                        #[cfg(feature = "env")] { parse_value_inner!(arg, map, Arg, bool, hide_env_values) }
                        #[cfg(not(feature = "env"))] { return Err(Error::custom("env feature disabled"))}}
                    // An empty string or `null` resets the heading
                    // (the arg will not have the heading of `Command::next_help_heading`).
                    "help_heading" => match map.next_value::<Option<String>>()? {
                        Some(h) if !h.is_empty() => arg.help_heading(h),
                        _ => arg.help_heading(clap::builder::Resettable::Reset),
                    }
                    "num_args" => parse_opt!(arg, map, NumArgs, |v| arg.num_args(v))
                    "possible_values" => parse_opt!(arg, map, Vec<String>, |v| {
                        if v.is_empty() {
                            arg
                        } else {
                            arg.value_parser(clap::builder::PossibleValuesParser::new(v))
                        }
                    })
                    "value_hint" => parse_opt!(arg, map, ValueHint, |v| arg.value_hint(clap::ValueHint::from(v)))
                    "value_parser" => parse_opt!(arg, map, ValueParser, |v| arg.value_parser(v))
                    // `positional` is determined by `long` and `short`. Only for the output of serialization.
                    "positional" => {map.next_value::<serde::de::IgnoredAny>()?; arg}
                ]
            );
        }
        Ok(ArgWrap::new(arg))
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
        Ok(argkvo.0.expect("ArgKVO must hold a command"))
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
        while let Some(name) = map.next_key::<String>()? {
            #[cfg(feature = "override-arg")]
            {
                let (next, result) =
                    override_or_add_arg(app, name, |a| map.next_value_seed(ArgVisitor(a)));
                app = next;
                result?;
            }
            #[cfg(not(feature = "override-arg"))]
            {
                app = app.arg(map.next_value_seed(ArgVisitor::new_str(name))?);
            }
        }
        Ok(app)
    }
}

/// Argument of [`Arg::default_value_if`] : `[arg, value, default]`.
///
/// If `value` is `null`, the default is applied when `arg` is present.
/// If `default` is `null`, the default value will be removed.
#[derive(serde::Deserialize)]
struct DefaultValueIf(String, Option<String>, Option<String>);

impl DefaultValueIf {
    fn apply(self, arg: Arg) -> Arg {
        use clap::builder::{ArgPredicate, OsStr, Resettable};
        let DefaultValueIf(id, value, default) = self;
        let predicate = match value {
            Some(v) => ArgPredicate::from(v),
            None => ArgPredicate::IsPresent,
        };
        let default: Resettable<OsStr> = match default {
            Some(d) => Resettable::Value(d.into()),
            None => Resettable::Reset,
        };
        arg.default_value_if(id, predicate, default)
    }
}

/// Argument of [`Arg::num_args`] : number or `{ min = .., max = .. }`.
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum NumArgs {
    Exact(usize),
    Range {
        #[serde(default)]
        min: usize,
        max: Option<usize>,
    },
}

impl From<NumArgs> for clap::builder::ValueRange {
    fn from(n: NumArgs) -> Self {
        match n {
            NumArgs::Exact(n) => n.into(),
            NumArgs::Range {
                min,
                max: Some(max),
            } => (min..=max).into(),
            NumArgs::Range { min, max: None } => (min..).into(),
        }
    }
}
