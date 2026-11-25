use clap::Command;
use serde::Deserializer;
use std::borrow::Cow;

use crate::CommandWrap;

/**
Deserialize [`Command`] from [`Deserializer`].
```
const CLAP_TOML: &'static str = r#"
name = "app_clap_serde"
version = "1.0"
author = "tester"
about = "test-clap-serde"
"#;
let app = clap_serde::load(toml::Deserializer::parse(CLAP_TOML).unwrap())
    .expect("parse failed");
assert_eq!(app.get_name(), "app_clap_serde");
assert_eq!(app.get_about().map(|s| s.to_string()).as_deref(), Some("test-clap-serde"));
```
*/
pub fn load<'de, D>(de: D) -> Result<Command, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::Deserialize;
    CommandWrap::deserialize(de).map(|a| a.into())
}

/// Convert the snake_case key to the case of the case-key feature.
#[cfg(any(
    feature = "snake-case-key",
    not(any(feature = "kebab-case-key", feature = "pascal-case-key"))
))]
pub(crate) fn case_key(s: &str) -> Cow<'_, str> {
    Cow::Borrowed(s)
}

/// Convert the snake_case key to the case of the case-key feature.
#[cfg(all(not(feature = "snake-case-key"), feature = "kebab-case-key"))]
pub(crate) fn case_key(s: &str) -> Cow<'_, str> {
    Cow::Owned(s.replace('_', "-"))
}

/// Convert the snake_case key to the case of the case-key feature.
#[cfg(all(
    not(feature = "snake-case-key"),
    not(feature = "kebab-case-key"),
    feature = "pascal-case-key"
))]
pub(crate) fn case_key(s: &str) -> Cow<'_, str> {
    Cow::Owned(
        s.split('_')
            .map(|w| {
                let mut c = w.chars();
                c.next()
                    .map(|f| f.to_ascii_uppercase().to_string() + c.as_str())
                    .unwrap_or_default()
            })
            .collect(),
    )
}
