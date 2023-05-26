use clap::ArgAction as AA;
use serde::Deserialize;

enum_de!(AA, ArgAction,
    #[derive(Deserialize, Clone, Copy)]
    #[cfg_attr(feature = "kebab-case-key" ,serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "snake-case-key" ,serde(rename_all = "snake_case"))]
    {
        Set,
        Append,
        SetTrue,
        SetFalse,
        Count,
        Help,
        Version,
    }
);
