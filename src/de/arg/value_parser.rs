use clap::builder::ValueParser;
use serde::Deserialize;

enum_de!(ValueParser, ValueParser1,
    #[derive(Deserialize, Clone, Copy)]
    #[cfg_attr(feature = "kebab-case-key" ,serde(rename_all = "kebab-case"))]
    #[cfg_attr(feature = "snake-case-key" ,serde(rename_all = "snake_case"))]
    { }
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
        RangedI64(s : i64 , e : i64) => {
            (s..e).into()
        },
        RangedInclusizeI64(s : i64 , e : i64) => {
            (s..=e).into()
        },
        RangedFromI64(s : i64) => {
            (s..).into()
        },
        RangedToI64(e : i64) => {
            (..e).into()
        },
        RangedToInclusizeI64(e : i64) => {
            (..=e).into()
        },
    }
);
