#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc  = include_str!("../README.md")]

use clap::{Arg, ArgGroup, Command};
use serde::Deserializer;
use std::ops::Deref;

#[cfg(not(any(
    feature = "kebab-case-key",
    feature = "snake-case-key",
    feature = "pascal-case-key"
)))]
compile_error!("Case setting feature is missing. Either one should be set.");

#[cfg(any(
    all(feature = "kebab-case-key", feature = "snake-case-key"),
    all(feature = "kebab-case-key", feature = "pascal-case-key"),
    all(feature = "pascal-case-key", feature = "snake-case-key"),
))]
compile_error!("Case setting feature is conflicting. Only one should be set.");

#[macro_use]
mod de;
#[cfg(feature = "docsrs")]
pub mod documents;
#[cfg(feature = "yaml")]
mod yaml;

#[cfg(all(test, feature = "snake-case-key"))]
mod tests;

#[cfg(feature = "yaml")]
pub use yaml::{yaml_to_app, YamlWrap};

/**
Deserialize [`Command`] from [`Deserializer`].
```
const CLAP_TOML: &'static str = r#"
name = "app_clap_serde"
version = "1.0"
author = "tester"
about = "test-clap-serde"
"#;
let app = clap_serde::load(&mut toml::Deserializer::new(CLAP_TOML))
    .expect("parse failed");
assert_eq!(app.get_name(), "app_clap_serde");
assert_eq!(app.get_about(), Some("test-clap-serde"));
```
*/
pub fn load<'de, D>(de: D) -> Result<Command<'de>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::Deserialize;
    CommandWrap::deserialize(de).map(|a| a.into())
}

/**
Wrapper of [`Command`] to deserialize.
```
const CLAP_TOML: &'static str = r#"
name = "app_clap_serde"
version = "1.0"
author = "tester"
about = "test-clap-serde"
"#;
let app: clap::Command = toml::from_str::<clap_serde::CommandWrap>(CLAP_TOML)
    .expect("parse failed")
    .into();
assert_eq!(app.get_name(), "app_clap_serde");
assert_eq!(app.get_about(), Some("test-clap-serde"));
```
*/
#[derive(Debug, Clone)]
pub struct CommandWrap<'a> {
    app: Command<'a>,
}

#[deprecated]
pub type AppWrap<'a> = CommandWrap<'a>;

impl<'a> From<CommandWrap<'a>> for Command<'a> {
    fn from(a: CommandWrap<'a>) -> Self {
        a.app
    }
}

impl<'a> From<Command<'a>> for CommandWrap<'a> {
    fn from(app: Command<'a>) -> Self {
        CommandWrap { app }
    }
}

impl<'a> Deref for CommandWrap<'a> {
    type Target = Command<'a>;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

/// Wrapper of [`Arg`] to deserialize with [`DeserializeSeed`](`serde::de::DeserializeSeed`).
#[derive(Debug, Clone)]
pub struct ArgWrap<'a> {
    arg: Arg<'a>,
}

impl<'a> From<ArgWrap<'a>> for Arg<'a> {
    fn from(arg: ArgWrap<'a>) -> Self {
        arg.arg
    }
}

impl<'a> From<Arg<'a>> for ArgWrap<'a> {
    fn from(arg: Arg<'a>) -> Self {
        ArgWrap { arg }
    }
}

impl<'a> Deref for ArgWrap<'a> {
    type Target = Arg<'a>;

    fn deref(&self) -> &Self::Target {
        &self.arg
    }
}

pub(crate) struct ArgGroupWrap<'a> {
    group: ArgGroup<'a>,
}

impl<'a> From<ArgGroupWrap<'a>> for ArgGroup<'a> {
    fn from(group: ArgGroupWrap<'a>) -> Self {
        group.group
    }
}

impl<'a> From<ArgGroup<'a>> for ArgGroupWrap<'a> {
    fn from(group: ArgGroup<'a>) -> Self {
        ArgGroupWrap { group }
    }
}
