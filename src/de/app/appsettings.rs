#![allow(deprecated)]
use clap::{AppFlags, AppSettings};
use serde::{de::DeserializeSeed, Deserialize};

enum_de!(AppSettings,AppSetting1,
    #[derive(Deserialize, Clone, Copy)]
    #[cfg_attr(feature = "kebab-case-key" ,serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "snake-case-key" ,serde(rename_all = "snake_case"))]
    {
    #[cfg(feature="allow-deprecated")]
    IgnoreErrors,
    #[cfg(feature="allow-deprecated")]
    WaitOnError,
    #[cfg(feature="allow-deprecated")]
    AllowHyphenValues,
    #[cfg(feature="allow-deprecated")]
    AllowNegativeNumbers,
    #[cfg(feature="allow-deprecated")]
    AllArgsOverrideSelf,
    #[cfg(feature="allow-deprecated")]
    AllowMissingPositional,
    #[cfg(feature="allow-deprecated")]
    TrailingVarArg,
    #[cfg(feature="allow-deprecated")]
    DontDelimitTrailingValues,
    #[cfg(feature="allow-deprecated")]
    InferLongArgs,
    #[cfg(feature="allow-deprecated")]
    InferSubcommands,
    #[cfg(feature="allow-deprecated")]
    SubcommandRequired,
    #[cfg(feature="allow-deprecated")]
    SubcommandRequiredElseHelp,
    #[cfg(feature="allow-deprecated")]
    AllowExternalSubcommands,
    #[cfg(all(feature = "unstable-multicall", feature="allow-depreacted"))]
    Multicall,
    #[cfg(feature="allow-deprecated")]
    AllowInvalidUtf8ForExternalSubcommands,
    #[cfg(feature="allow-deprecated")]
    UseLongFormatForHelpSubcommand,
    #[cfg(feature="allow-deprecated")]
    SubcommandsNegateReqs,
    #[cfg(feature="allow-deprecated")]
    ArgsNegateSubcommands,
    #[cfg(feature="allow-deprecated")]
    SubcommandPrecedenceOverArg,
    #[cfg(feature="allow-deprecated")]
    ArgRequiredElseHelp,
    DeriveDisplayOrder,
    #[cfg(feature="allow-deprecated")]
    DontCollapseArgsInUsage,
    #[cfg(feature="allow-deprecated")]
    NextLineHelp,
    #[cfg(feature="allow-deprecated")]
    DisableColoredHelp,
    #[cfg(feature="allow-deprecated")]
    DisableHelpFlag,
    #[cfg(feature="allow-deprecated")]
    DisableHelpSubcommand,
    #[cfg(feature="allow-deprecated")]
    DisableVersionFlag,
    #[cfg(feature="allow-deprecated")]
    PropagateVersion,
    #[cfg(feature="allow-deprecated")]
    Hidden,
    #[cfg(feature="allow-deprecated")]
    HidePossibleValues,
    #[cfg(feature="allow-deprecated")]
    HelpExpected,
    #[cfg(feature="allow-deprecated")]
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
