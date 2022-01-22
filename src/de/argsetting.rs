use clap::{ArgFlags, ArgSettings};
use serde::{de::DeserializeSeed, Deserialize};

enum_de!(ArgSettings, ArgSetting1,
   #[derive(Deserialize, Clone, Copy)]
   #[cfg_attr(feature = "kebab-case-setting", serde(rename_all = "kebab-case"))]
   #[cfg_attr(feature = "snake-case-setting", serde(rename_all = "snake_case"))]
   {
       Required,
       MultipleValues,
       MultipleOccurrences,
       ForbidEmptyValues,
       Global,
       Hidden,
       TakesValue,
       UseValueDelimiter,
       NextLineHelp,
       RequireDelimiter,
       HidePossibleValues,
       AllowHyphenValues,
       RequireEquals,
       Last,
       HideDefaultValue,
       IgnoreCase,
       #[cfg(feature = "env")]
       HideEnv,
       #[cfg(feature = "env")]
       HideEnvValues,
       HiddenShortHelp,
       HiddenLongHelp,
       AllowInvalidUtf8,
       Exclusive,
   }
);

pub(crate) struct ArgSettingSeed;
impl<'de> DeserializeSeed<'de> for ArgSettingSeed {
    type Value = ArgSettings;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        ArgSetting1::deserialize(deserializer).map(|s| s.into())
    }
}

pub(crate) struct ArgSettingsSeed;
impl<'de> DeserializeSeed<'de> for ArgSettingsSeed {
    type Value = ArgFlags;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Vec::<ArgSetting1>::deserialize(deserializer).map(|s| {
            s.into_iter()
                .fold(ArgFlags::default(), |a, b| a | ArgSettings::from(b))
        })
    }
}
