#![allow(deprecated)]
use clap::{AppFlags, AppSettings};
use serde::{de::DeserializeSeed, Deserialize};

enum_de!(AppSettings,AppSetting1,
    #[derive(Deserialize, Clone, Copy)]
    #[cfg_attr(feature = "kebab-case-key" ,serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "snake-case-key" ,serde(rename_all = "snake_case"))]
    {
    IgnoreErrors,
    WaitOnError,
    AllowHyphenValues,
    AllowNegativeNumbers,
    AllArgsOverrideSelf,
    AllowMissingPositional,
    TrailingVarArg,
    DontDelimitTrailingValues,
    InferLongArgs,
    InferSubcommands,
    SubcommandRequired,
    SubcommandRequiredElseHelp,
    AllowExternalSubcommands,
    #[cfg(feature = "unstable-multicall")]
    Multicall,
    AllowInvalidUtf8ForExternalSubcommands,
    UseLongFormatForHelpSubcommand,
    SubcommandsNegateReqs,
    ArgsNegateSubcommands,
    SubcommandPrecedenceOverArg,
    ArgRequiredElseHelp,
    DeriveDisplayOrder,
    DontCollapseArgsInUsage,
    NextLineHelp,
    DisableColoredHelp,
    DisableHelpFlag,
    DisableHelpSubcommand,
    DisableVersionFlag,
    PropagateVersion,
    Hidden,
    HidePossibleValues,
    HelpExpected,
    NoBinaryName,
    NoAutoHelp,
    NoAutoVersion,}
);

pub(crate) struct AppSettingSeed;
impl<'de> DeserializeSeed<'de> for AppSettingSeed {
    type Value = AppSettings;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        AppSetting1::deserialize(deserializer).map(|s| s.into())
    }
}

pub(crate) struct AppSettingsSeed;
impl<'de> DeserializeSeed<'de> for AppSettingsSeed {
    type Value = AppFlags;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Vec::<AppSetting1>::deserialize(deserializer).map(|s| {
            s.into_iter()
                .fold(AppFlags::default(), |a, b| a | AppSettings::from(b))
        })
    }
}
