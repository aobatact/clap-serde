use clap::builder::ValueParser;
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
                    #[serde(default)]
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

enum_de_value!(ValueParser, ValueParser1, 
    #[derive(Deserialize, Clone, Copy)]
    #[serde(tag = "type")]
    #[cfg_attr(feature = "kebab-case-key" ,serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "snake-case-key" ,serde(rename_all = "snake_case"))]
    {
        Bool => {
            ValueParser::bool()
        },
        String => {
            ValueParser::string()
        },
        OsString => {
            ValueParser::os_string()
        },
        PathBuf => {
            ValueParser::path_buf()
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

