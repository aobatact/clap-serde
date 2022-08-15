use clap::builder::ValueParser as VP;
use serde::Deserialize;

macro_rules! enum_de_value {
    ($basety : ident, $newty :ident,
        $(#[$derive_meta:meta])*
        {
            $( $(
                #[ $cfg_meta_ex:meta ] )?
                $var_ex: ident $( { $( $(#[$cfg_v:meta])* $vx: ident : $vt: ty ),* } )?
                    => $to_ex: expr
            ,)*
        }
        {
            $(($pty: ty, $pty_upper : tt $(, $ty_as: ty)?)),*
        }
    ) => {
        enum_de!($basety, $newty,
            $(#[$derive_meta])* {}
            {
                $( $(
                    #[ $cfg_meta_ex ] )?
                    $var_ex $( { $( $(#[$cfg_v])* $vx : $vt ),* } )?
                        => $to_ex
                ,)*
                $(
                $pty_upper {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    min: Option<$pty>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    max: Option<$pty>,
                    #[serde(default = "get_true")]
                    max_inclusive: bool
                } => {
                    match (min, max, max_inclusive) {
                        (Some(s), Some(e), false) => clap::value_parser!($pty).range((s $(as $ty_as)*) ..(e $(as $ty_as)*)).into(),
                        (Some(s), Some(e), true) => clap::value_parser!($pty).range((s $(as $ty_as)*) ..=(e $(as $ty_as)*)).into(),
                        (Some(s), None, _) => clap::value_parser!($pty).range((s $(as $ty_as)*)..).into(),
                        (None, Some(e), false) => clap::value_parser!($pty).range(..(e $(as $ty_as)*)).into(),
                        (None, Some(e), true) => clap::value_parser!($pty).range(..=(e $(as $ty_as)*)).into(),
                        (None, None, _) => clap::value_parser!($pty).into(),
                    }
                },)*
            }
        );
    };
}

const fn get_true() -> bool {true}

enum_de_value!(VP, ValueParser1,
    #[derive(Deserialize, Clone, Copy)]
    #[serde(tag = "type")]
    #[cfg_attr(feature = "kebab-case-key" ,serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "snake-case-key" ,serde(rename_all = "snake_case"))]
    {
        Bool => {
            VP::bool()
        },
        String => {
            VP::string()
        },
        OsString => {
            VP::os_string()
        },
        PathBuf => {
            VP::path_buf()
        },
        Boolish => {
            clap::builder::BoolishValueParser::new().into()
        },
        Falsey => {
            clap::builder::FalseyValueParser::new().into()
        },
        NonEmptyString => {
            clap::builder::NonEmptyStringValueParser::new().into()
        },
    }
    {
        (i64, I64),
        (i32, I32, i64),
        (i16, I16, i64),
        (i8 , I8, i64),
        (u64, U64),
        (u32, U32, i64),
        (u16, U16, i64),
        (u8 , U8, i64)
    }
);

enum_de!(VP, ValueParser2,
    #[derive(Deserialize, Clone, Copy)]
    #[cfg_attr(feature = "kebab-case-key" ,serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "snake-case-key" ,serde(rename_all = "snake_case"))]
    {}
    {
        Bool => {
            VP::bool()
        },
        String => {
            VP::string()
        },
        OsString => {
            VP::os_string()
        },
        PathBuf => {
            VP::path_buf()
        },
        Boolish => {
            clap::builder::BoolishValueParser::new().into()
        },
        Falsey => {
            clap::builder::FalseyValueParser::new().into()
        },
        NonEmptyString => {
            clap::builder::NonEmptyStringValueParser::new().into()
        },
        I64 => {
            clap::value_parser!(i64).into()
        },
        I32 => {
            clap::value_parser!(i32).into()
        },
        I16 => {
            clap::value_parser!(i16).into()
        },
        I8 => {
            clap::value_parser!(i8).into()
        },
        U64 => {
            clap::value_parser!(u64).into()
        },
        U32 => {
            clap::value_parser!(u32).into()
        },
        U16 => {
            clap::value_parser!(u16).into()
        },
        U8 => {
            clap::value_parser!(u8).into()
        },
    }
);

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum ValueParser {
    Value(ValueParser2),
    Tagged(ValueParser1),
}

impl From<ValueParser> for VP {
    fn from(v: ValueParser) -> Self {
        match v {
            ValueParser::Value(v) => v.into(),
            ValueParser::Tagged(t) => t.into(),
        }
    }
}
