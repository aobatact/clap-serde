use clap::ColorChoice;
use serde::{de::DeserializeSeed, Deserialize, Serialize};

enum_de!(ColorChoice,ColorChoice1,
    #[derive(Deserialize, Serialize, Clone, Copy)]
    #[cfg_attr(feature = "kebab-case-key" ,serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "snake-case-key" ,serde(rename_all = "snake_case"))]
    {
    Auto,
    Always,
    Never,
});

pub(crate) fn to_ser(cc : ColorChoice) -> ColorChoice1 {
    match cc {
        ColorChoice::Auto => ColorChoice1::Auto,
        ColorChoice::Always => ColorChoice1::Always,
        ColorChoice::Never => ColorChoice1::Never,
    }
}

pub struct ColorChoiceSeed;
impl<'de> DeserializeSeed<'de> for ColorChoiceSeed {
    type Value = ColorChoice;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        ColorChoice1::deserialize(deserializer).map(|c| c.into())
    }
}
