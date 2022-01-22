#![doc  = include_str!("../README.md")]

use std::ops::Deref;
use clap::{App, Arg, ArgGroup};
use serde::Deserializer;

#[cfg(all(feature = "kebab-case-setting", feature = "snake-case-setting"))]
compile_error!("Feature \"kebab-case-setting\" and \"snake-case-setting\" collides. At most one should be set.");

#[macro_use]
mod de;

#[cfg(test)]
mod tests;

/**
Deserialize [`App`] from [`Deserializer`].
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
pub fn load<'de, D>(de: D) -> Result<App<'de>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::Deserialize;
    AppWrap::deserialize(de).map(|a| a.into())
}

/**
Wrapper of [`App`] to deserialize.
```
const CLAP_TOML: &'static str = r#"
name = "app_clap_serde"
version = "1.0"
author = "tester"
about = "test-clap-serde"
"#;
let app: clap::App = toml::from_str::<clap_serde::AppWrap>(CLAP_TOML)
    .expect("parse failed")
    .into();
assert_eq!(app.get_name(), "app_clap_serde");
assert_eq!(app.get_about(), Some("test-clap-serde"));
```
*/
#[derive(Debug, Clone)]
pub struct AppWrap<'a> {
    app: App<'a>,
}

impl<'a> From<AppWrap<'a>> for App<'a> {
    fn from(a: AppWrap<'a>) -> Self {
        a.app
    }
}

impl<'a> From<App<'a>> for AppWrap<'a> {
    fn from(app: App<'a>) -> Self {
        AppWrap { app }
    }
}

impl<'a> Deref for AppWrap<'a> {
    type Target = App<'a>;

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
