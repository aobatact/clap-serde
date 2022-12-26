#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc  = include_str!("../README.md")]

use clap::{Arg, ArgGroup, Command};
use serde::Deserializer;
use std::ops::Deref;
mod x;

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
#[deprecated(since = "0.4", note = "use serde-yaml instead")]
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
pub fn load<'de, D>(de: D) -> Result<Command, D::Error>
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
pub struct CommandWrap {
    app: Command,
}

#[deprecated]
pub type AppWrap = CommandWrap;

impl From<CommandWrap> for Command {
    fn from(a: CommandWrap) -> Self {
        a.app
    }
}

impl From<Command> for CommandWrap {
    fn from(app: Command) -> Self {
        CommandWrap { app }
    }
}

impl Deref for CommandWrap {
    type Target = Command;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

/// Wrapper of [`Arg`] to deserialize with [`DeserializeSeed`](`serde::de::DeserializeSeed`).
#[derive(Debug, Clone)]
pub struct ArgWrap {
    arg: Arg,
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

pub(crate) struct ArgGroupWrap {
    group: ArgGroup,
}

impl From<ArgGroupWrap> for ArgGroup {
    fn from(group: ArgGroupWrap) -> Self {
        group.group
    }
}

impl From<ArgGroup> for ArgGroupWrap {
    fn from(group: ArgGroup) -> Self {
        ArgGroupWrap { group }
    }
}
