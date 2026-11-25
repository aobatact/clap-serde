use clap::Command;
use serde::Deserializer;

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
