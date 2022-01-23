use clap::ValueHint;
use serde::{de::DeserializeSeed, Deserialize};

enum_de!(ValueHint,ValueHint1,
    #[derive(Deserialize, Clone, Copy)]
    #[cfg_attr(feature = "kebab-case-key" ,serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "snake-case-key" ,serde(rename_all = "snake_case"))]
    {
    Unknown,
    Other,
    AnyPath,
    FilePath,
    DirPath,
    ExecutablePath,
    CommandName,
    CommandString,
    CommandWithArguments,
    Username,
    Hostname,
    Url,
    EmailAddress,
});

pub struct ValueHintSeed;

impl<'de> DeserializeSeed<'de> for ValueHintSeed {
    type Value = ValueHint;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        ValueHint1::deserialize(deserializer).map(|v| v.into())
    }
}
