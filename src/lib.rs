#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc  = include_str!("../README.md")]

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

mod de;
mod ser;

#[cfg(feature = "docsrs")]
pub mod documents;

#[cfg(feature = "yaml")]
#[deprecated(since = "0.4", note = "use serde-yaml instead")]
mod yaml;
#[cfg(feature = "yaml")]
pub use yaml::{yaml_to_app, YamlWrap};

pub use de::app::CommandWrap;
pub use de::arg::ArgWrap;
pub use ser::app::CommandWrapRef;
pub use ser::arg::ArgWrapRef;
pub use ser::NoSkip;

#[deprecated(note = "use CommandWrap instead")]
pub type AppWrap<'a> = CommandWrap<'a>;

#[cfg(all(test, feature = "snake-case-key"))]
mod tests;

mod util;
pub use util::load;
