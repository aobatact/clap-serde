use clap::ColorChoice;
use serde::{Deserialize, Serialize};

enum_de!(ColorChoice,ColorChoice1,
    #[derive(Deserialize, Serialize, Clone, Copy)]
    #[cfg_attr(feature = "kebab-case-key" ,serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "snake-case-key" ,serde(rename_all = "snake_case"))]
    {
    Auto,
    Always,
    Never,
});
