use clap::ValueHint as VH;
use serde::{Deserialize, Serialize};

enum_de!(VH,ValueHint,
    #[derive(Serialize, Deserialize, Clone, Copy)]
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

impl ValueHint{
    pub fn from_vh(vh : VH) -> Self {
        match vh {
            VH::Unknown => ValueHint::Unknown,
            VH::Other => ValueHint::Other,
            VH::AnyPath => ValueHint::AnyPath,
            VH::FilePath => ValueHint::FilePath,
            VH::DirPath => ValueHint::DirPath,
            VH::ExecutablePath => ValueHint::ExecutablePath,
            VH::CommandName => ValueHint::CommandName,
            VH::CommandString => ValueHint::CommandString,
            VH::CommandWithArguments => ValueHint::CommandWithArguments,
            VH::Username => ValueHint::Username,
            VH::Hostname => ValueHint::Hostname,
            VH::Url => ValueHint::Url,
            VH::EmailAddress => ValueHint::EmailAddress,
            _ => unimplemented!()
        }
    }
}