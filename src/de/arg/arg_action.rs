use clap::ArgAction;
use serde::Deserialize;

enum_de!(ArgAction, ArgAction1,
    #[derive(Deserialize, Clone, Copy)]
    #[cfg_attr(feature = "kebab-case-key" ,serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "snake-case-key" ,serde(rename_all = "snake_case"))]
    {
        Set,
        Append,
        StoreValue,
        IncOccurrence,
        SetTrue,
        SetFalse,
        Count,
        Help,
        Version,
    }
);
