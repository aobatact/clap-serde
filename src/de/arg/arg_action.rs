use clap::ArgAction as AA;
use serde::{Deserialize, Serialize};

enum_de!(AA, ArgAction,
    #[derive(Serialize, Deserialize, Clone, Copy)]
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
