use clap::ColorChoice;
use serde::{de::DeserializeSeed, Deserialize};

enum_de!(ColorChoice,ColorChoice1,
    #[derive(Deserialize, Clone, Copy)]
    #[cfg_attr(feature = "kebab-case-setting" ,serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "snake-case-setting" ,serde(rename_all = "snake_case"))]
    {
    Auto,
    Always,
    Never,
});

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
