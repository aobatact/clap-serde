//!# clap-serde
//!Provides a wrapper to deserialize [clap](https://crates.io/crates/clap) app using [serde](https://crates.io/crates/serde).
//!
//![![Crates.io](https://img.shields.io/crates/v/clap-serde?style=flat-square)](https://crates.io/crates/clap-serde)
//![![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/aobatact/clap-serde/blob/main/LICENSE-APACHE)
//![![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/aobatact/clap-serde/blob/main/LICENSE-MIT)
//![![API Reference](https://img.shields.io/docsrs/clap-serde?style=flat-square)](https://docs.rs/clap-serde)
//!
//!## toml
//!
//!```rust
//!const CLAP_TOML: &'static str = r#"
//!name = "app_clap_serde"
//!version = "1.0"
//!author = "toml_tester"
//!about = "test-clap-serde"
//![subcommands]
//!sub1 = { about = "subcommand_1" }
//![subcommands.sub2]
//!about = "subcommand_2"
//![args]
//!apple = { short = "a" }
//!banana = { short = "b", long = "banana", aliases = ["musa_spp"] }
//![groups]
//!fruit = { args = ["apple", "banana"] }
//!"#;
//!
//!let app: clap::App = toml::from_str::<clap_serde::AppWrap>(CLAP_TOML)
//!    .expect("parse failed")
//!    .into();
//!assert_eq!(app.get_name(), "app_clap_serde");
//!assert_eq!(app.get_about(), Some("test-clap-serde"));
//!```
//!
//!## json
//!```rust
//!const CLAP_JSON: &'static str = r#"{
//!"name" : "app_clap_serde",
//!"version" : "1.0" ,
//!"author" : "json_tester",
//!"about" : "test-clap-serde",
//!"subcommands" : [
//!    { "sub1" : {"about" : "subcommand_1"}},
//!    { "sub2" : {"about" : "subcommand_2"}}
//!],
//!"args" : [
//!    { "apple" : {"short" : "a" } },
//!    { "banana" : {"short" : "b", "long" : "banana", "aliases" : [ "musa_spp" ]} }
//!],
//!"groups" : {
//!    "fruit" : { "args" : ["apple", "banana"] }
//!}
//!}"#;
//!
//!let app: clap::App = serde_json::from_str::<clap_serde::CommandWrap>(CLAP_JSON)
//!    .expect("parse failed")
//!    .into();
//!assert_eq!(app.get_name(), "app_clap_serde");
//!assert_eq!(app.get_about(), Some("test-clap-serde"));
//!```
//!
//!## yaml
//!```rust
//!const CLAP_YAML: &'static str = r#"
//!name: app_clap_serde
//!version : "1.0"
//!about : yaml_support!
//!author : yaml_supporter
//!
//!args:
//!    - apple :
//!        short: a
//!    - banana:
//!        short: b
//!        long: banana
//!        aliases :
//!            - musa_spp
//!
//!subcommands:
//!    - sub1:
//!        about : subcommand_1
//!    - sub2:
//!        about : subcommand_2
//!
//!"#;
//!let app: clap_serde::CommandWrap = serde_yaml::from_str(CLAP_YAML).expect("fail to make yaml");
//!assert_eq!(app.get_name(), "app_clap_serde");
//!```
//!
//!# features
//!## env
//!Enables env feature in clap.
//!## yaml (deprecated, use serde-yaml instead)
//!Enables to use yaml.
//!## color
//!Enablse color feature in clap.
//!
//!## (key case settings)
//!Settings names format for keys and [`AppSettings`](`clap::AppSettings`).
//!#### snake-case-key
//!snake_case. Enabled by default.
//!#### pascal-case-key
//!PascalCase. Same as variants name in enum at `AppSettings`.
//!#### kebab-case-key
//!kebab-case.
//!
//!## allow-deprecated
//!Allow deprecated keys, settings. Enabled by default.
//!
//!## override-args
//!
//!Override a `Arg` with `DeserializeSeed`.
//!
//!```rust
//!# #[cfg(feature = "override-arg")]
//!# {
//!# use clap::{Command, Arg};
//!use serde::de::DeserializeSeed;
//!
//!const CLAP_TOML: &str = r#"
//!name = "app_clap_serde"
//!version = "1.0"
//!author = "aobat"
//!about = "test-clap-serde"
//![args]
//!apple = { short = "a" }
//!"#;
//!let app = Command::new("app").arg(Arg::new("apple").default_value("aaa"));
//!let wrap = clap_serde::CommandWrap::from(app);
//!let mut de = toml::Deserializer::new(CLAP_TOML);
//!let wrap2 = wrap.deserialize(&mut de).unwrap();
//!let apple = wrap2
//!    .get_arguments()
//!    .find(|a| a.get_id() == "apple")
//!    .unwrap();
//!assert!(apple.get_short() == Some('a'));
//!assert!(apple.get_default_values() == ["aaa"]);
//!# }
//!```

use clap::{Arg, ArgGroup, Command};
use serde::Deserializer;
use std::ops::Deref;
#[macro_use]
mod de {
    #[macro_use]
    mod macros {}
    mod app {
        use crate::CommandWrap;
        use clap::Command;
        use serde::{
            de::{DeserializeSeed, Error, Visitor},
            Deserialize,
        };
        const TMP_APP_NAME: &str = "__tmp__deserialize__name__";
        impl<'de> Deserialize<'de> for CommandWrap {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer
                    .deserialize_map(CommandVisitor(Command::new(TMP_APP_NAME)))
                    .and_then(|r| {
                        if r.app.get_name() != TMP_APP_NAME {
                            Ok(r)
                        } else {
                            Err(<D::Error>::missing_field("name"))
                        }
                    })
            }
        }
        struct CommandVisitor(Command);
        impl<'de> Visitor<'de> for CommandVisitor {
            type Value = CommandWrap;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Command Map")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut app = self.0;
                while let Some(key) = map.next_key::<&str>()? {
                    app = {
                        #[allow(unused_mut)]
                        let mut key;
                        {
                            key = key;
                        };
                        #[allow(unused_labels)]
                        'parse_value_jmp_loop: loop {
                            break 'parse_value_jmp_loop match key {
                                "about" => <Command>::about(app, map.next_value::<&str>()?),
                                "after_help" => {
                                    <Command>::after_help(app, map.next_value::<&str>()?)
                                }
                                "after_long_help" => {
                                    <Command>::after_long_help(app, map.next_value::<&str>()?)
                                }
                                "alias" => <Command>::alias(app, map.next_value::<&str>()?),
                                "aliases" => {
                                    <Command>::aliases(app, &map.next_value::<Vec<&str>>()?)
                                }
                                "allow_external_subcommands" => {
                                    <Command>::allow_external_subcommands(
                                        app,
                                        map.next_value::<bool>()?,
                                    )
                                }
                                "allow_hyphen_values" => {
                                    <Command>::allow_hyphen_values(app, map.next_value::<bool>()?)
                                }
                                "allow_missing_positional" => <Command>::allow_missing_positional(
                                    app,
                                    map.next_value::<bool>()?,
                                ),
                                "allow_negative_numbers" => <Command>::allow_negative_numbers(
                                    app,
                                    map.next_value::<bool>()?,
                                ),
                                "arg_required_else_help" => <Command>::arg_required_else_help(
                                    app,
                                    map.next_value::<bool>()?,
                                ),
                                "args_conflicts_with_subcommands" => {
                                    <Command>::args_conflicts_with_subcommands(
                                        app,
                                        map.next_value::<bool>()?,
                                    )
                                }
                                "args_override_self" => {
                                    <Command>::args_override_self(app, map.next_value::<bool>()?)
                                }
                                "author" => <Command>::author(app, map.next_value::<&str>()?),
                                "before_help" => {
                                    <Command>::before_help(app, map.next_value::<&str>()?)
                                }
                                "before_long_help" => {
                                    <Command>::before_long_help(app, map.next_value::<&str>()?)
                                }
                                "bin_name" => <Command>::bin_name(app, map.next_value::<&str>()?),
                                "disable_colored_help" => {
                                    <Command>::disable_colored_help(app, map.next_value::<bool>()?)
                                }
                                "disable_help_flag" => {
                                    <Command>::disable_help_flag(app, map.next_value::<bool>()?)
                                }
                                "disable_help_subcommand" => <Command>::disable_help_subcommand(
                                    app,
                                    map.next_value::<bool>()?,
                                ),
                                "disable_version_flag" => {
                                    <Command>::disable_version_flag(app, map.next_value::<bool>()?)
                                }
                                "display_name" => {
                                    <Command>::display_name(app, map.next_value::<&str>()?)
                                }
                                "display_order" => {
                                    <Command>::display_order(app, map.next_value::<usize>()?)
                                }
                                "dont_collapse_args_in_usage" => {
                                    <Command>::dont_collapse_args_in_usage(
                                        app,
                                        map.next_value::<bool>()?,
                                    )
                                }
                                "dont_delimit_trailing_values" => {
                                    <Command>::dont_delimit_trailing_values(
                                        app,
                                        map.next_value::<bool>()?,
                                    )
                                }
                                "help_expected" => {
                                    <Command>::help_expected(app, map.next_value::<bool>()?)
                                }
                                "hide" => <Command>::hide(app, map.next_value::<bool>()?),
                                "hide_possible_values" => {
                                    <Command>::hide_possible_values(app, map.next_value::<bool>()?)
                                }
                                "ignore_errors" => {
                                    <Command>::ignore_errors(app, map.next_value::<bool>()?)
                                }
                                "infer_long_args" => {
                                    <Command>::infer_long_args(app, map.next_value::<bool>()?)
                                }
                                "infer_subcommands" => {
                                    <Command>::infer_subcommands(app, map.next_value::<bool>()?)
                                }
                                "long_about" => {
                                    <Command>::long_about(app, map.next_value::<&str>()?)
                                }
                                "long_flag" => <Command>::long_flag(app, map.next_value::<&str>()?),
                                "long_flag_alias" => {
                                    <Command>::long_flag_alias(app, map.next_value::<&str>()?)
                                }
                                "long_flag_aliases" => <Command>::long_flag_aliases(
                                    app,
                                    &map.next_value::<Vec<&str>>()?,
                                ),
                                "long_version" => {
                                    <Command>::long_version(app, map.next_value::<&str>()?)
                                }
                                "max_term_width" => {
                                    <Command>::max_term_width(app, map.next_value::<usize>()?)
                                }
                                "multicall" => <Command>::multicall(app, map.next_value::<bool>()?),
                                "name" => <Command>::name(app, map.next_value::<&str>()?),
                                "next_display_order" => <Command>::next_display_order(
                                    app,
                                    map.next_value::<Option<usize>>()?,
                                ),
                                "next_help_heading" => <Command>::next_help_heading(
                                    app,
                                    map.next_value::<Option<&str>>()?,
                                ),
                                "next_line_help" => {
                                    <Command>::next_line_help(app, map.next_value::<bool>()?)
                                }
                                "no_binary_name" => {
                                    <Command>::no_binary_name(app, map.next_value::<bool>()?)
                                }
                                "override_help" => {
                                    <Command>::override_help(app, map.next_value::<&str>()?)
                                }
                                "override_usage" => {
                                    <Command>::override_usage(app, map.next_value::<&str>()?)
                                }
                                "propagate_version" => {
                                    <Command>::propagate_version(app, map.next_value::<bool>()?)
                                }
                                "short_flag" => {
                                    <Command>::short_flag(app, map.next_value::<char>()?)
                                }
                                "short_flag_alias" => {
                                    <Command>::short_flag_alias(app, map.next_value::<char>()?)
                                }
                                "short_flag_aliases" => <Command>::short_flag_aliases(
                                    app,
                                    map.next_value::<Vec<char>>()?,
                                ),
                                "subcommand_help_heading" => <Command>::subcommand_help_heading(
                                    app,
                                    map.next_value::<&str>()?,
                                ),
                                "subcommand_negates_reqs" => <Command>::subcommand_negates_reqs(
                                    app,
                                    map.next_value::<bool>()?,
                                ),
                                "subcommand_required" => {
                                    <Command>::subcommand_required(app, map.next_value::<bool>()?)
                                }
                                "subcommand_value_name" => {
                                    <Command>::subcommand_value_name(app, map.next_value::<&str>()?)
                                }
                                "propagate_version" => {
                                    <Command>::propagate_version(app, map.next_value::<bool>()?)
                                }
                                "term_width" => {
                                    <Command>::term_width(app, map.next_value::<usize>()?)
                                }
                                "trailing_var_arg" => {
                                    <Command>::trailing_var_arg(app, map.next_value::<bool>()?)
                                }
                                "version" => <Command>::version(app, map.next_value::<&str>()?),
                                "visible_alias" => {
                                    <Command>::visible_alias(app, map.next_value::<&str>()?)
                                }
                                "visible_aliases" => {
                                    <Command>::visible_aliases(app, &map.next_value::<Vec<&str>>()?)
                                }
                                "visible_long_flag_alias" => <Command>::visible_long_flag_alias(
                                    app,
                                    map.next_value::<&str>()?,
                                ),
                                "visible_long_flag_aliases" => {
                                    <Command>::visible_long_flag_aliases(
                                        app,
                                        &map.next_value::<Vec<&str>>()?,
                                    )
                                }
                                "visible_short_flag_alias" => <Command>::visible_short_flag_alias(
                                    app,
                                    map.next_value::<char>()?,
                                ),
                                "visible_short_flag_aliases" => {
                                    <Command>::visible_short_flag_aliases(
                                        app,
                                        map.next_value::<Vec<char>>()?,
                                    )
                                }
                                "args" => map.next_value_seed(super::arg::Args::<true>(app))?,
                                "args_map" => {
                                    map.next_value_seed(super::arg::Args::<false>(app))?
                                }
                                "color" => {
                                    #[cfg(not(color))]
                                    {
                                        return Err(Error::custom("color feature disabled"));
                                    }
                                }
                                "subcommands" => map.next_value_seed(SubCommands::<true>(app))?,
                                "subcommands_map" => {
                                    map.next_value_seed(SubCommands::<false>(app))?
                                }
                                "groups" => map.next_value_seed(super::group::Groups(app))?,
                                depr @ ("help_message"
                                | "version_message"
                                | "setting"
                                | "settings"
                                | "global_setting"
                                | "global_settings"
                                | "help_template"
                                | "allow_invalid_utf8_for_external_subcommands") => {
                                    return Err(Error::custom(::core::fmt::Arguments::new_v1(
                                        &["deprecated key: "],
                                        &[::core::fmt::ArgumentV1::new_display(&depr)],
                                    )))
                                }
                                #[cfg(feature = "allow-deprecated")]
                                "help_heading" => {
                                    const N_KEY: &str = "\"next_help_heading\"";
                                    {
                                        key = N_KEY;
                                    };
                                    continue 'parse_value_jmp_loop;
                                }
                                "arg" => {
                                    return Err(Error::custom(::core::fmt::Arguments::new_v1(
                                        &["not supported key : ", ", ", " "],
                                        &match (&"\"arg\"", &"\"Use args instead\";") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    )))
                                }
                                "group" => {
                                    return Err(Error::custom(::core::fmt::Arguments::new_v1(
                                        &["not supported key : ", ", ", " "],
                                        &match (&"\"group\"", &"\"Use groups instead\";") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    )))
                                }
                                unknown => {
                                    return Err(Error::unknown_field(
                                        unknown,
                                        &[
                                            "about",
                                            "after_help",
                                            "after_long_help",
                                            "alias",
                                            "aliases",
                                            "allow_external_subcommands",
                                            "allow_hyphen_values",
                                            "allow_missing_positional",
                                            "allow_negative_numbers",
                                            "arg_required_else_help",
                                            "args_conflicts_with_subcommands",
                                            "args_override_self",
                                            "author",
                                            "before_help",
                                            "before_long_help",
                                            "bin_name",
                                            "disable_colored_help",
                                            "disable_help_flag",
                                            "disable_help_subcommand",
                                            "disable_version_flag",
                                            "display_name",
                                            "display_order",
                                            "dont_collapse_args_in_usage",
                                            "dont_delimit_trailing_values",
                                            "help_expected",
                                            "hide",
                                            "hide_possible_values",
                                            "ignore_errors",
                                            "infer_long_args",
                                            "infer_subcommands",
                                            "long_about",
                                            "long_flag",
                                            "long_flag_alias",
                                            "long_flag_aliases",
                                            "long_version",
                                            "max_term_width",
                                            "multicall",
                                            "name",
                                            "next_display_order",
                                            "next_help_heading",
                                            "next_line_help",
                                            "no_binary_name",
                                            "override_help",
                                            "override_usage",
                                            "propagate_version",
                                            "short_flag",
                                            "short_flag_alias",
                                            "short_flag_aliases",
                                            "subcommand_help_heading",
                                            "subcommand_negates_reqs",
                                            "subcommand_required",
                                            "subcommand_value_name",
                                            "propagate_version",
                                            "term_width",
                                            "trailing_var_arg",
                                            "version",
                                            "visible_alias",
                                            "visible_aliases",
                                            "visible_long_flag_alias",
                                            "visible_long_flag_aliases",
                                            "visible_short_flag_alias",
                                            "visible_short_flag_aliases",
                                            "\"args\"",
                                            "\"args_map\"",
                                            "\"color\"",
                                            "\"subcommands\"",
                                            "\"subcommands_map\"",
                                            "\"groups\"",
                                        ],
                                    ))
                                }
                            };
                        }
                    };
                }
                Ok(CommandWrap { app })
            }
        }
        pub struct NameSeed<'de>(&'de str);
        impl<'de> DeserializeSeed<'de> for NameSeed<'de> {
            type Value = CommandWrap;
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_map(CommandVisitor(Command::new(self.0.to_owned())))
            }
        }
        impl<'de> DeserializeSeed<'de> for CommandWrap {
            type Value = CommandWrap;
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_map(CommandVisitor(self.app))
            }
        }
        struct SubCommands<const KV_ARRAY: bool>(Command);
        impl<'de, const KV_ARRAY: bool> DeserializeSeed<'de> for SubCommands<KV_ARRAY> {
            type Value = Command;
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                if KV_ARRAY {
                    deserializer.deserialize_seq(self)
                } else {
                    deserializer.deserialize_map(self)
                }
            }
        }
        impl<'de, const KV_ARRAY: bool> Visitor<'de> for SubCommands<KV_ARRAY> {
            type Value = Command;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Subcommand")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut app = self.0;
                while let Some(name) = map.next_key::<&str>()? {
                    let sub = map.next_value_seed(NameSeed(name))?;
                    app = app.subcommand(sub);
                }
                Ok(app)
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut app = self.0;
                while let Some(sub) = seq.next_element_seed(InnerSubCommand)? {
                    app = app.subcommand(sub)
                }
                Ok(app)
            }
        }
        pub struct InnerSubCommand;
        impl<'de> Visitor<'de> for InnerSubCommand {
            type Value = Command;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Subcommand Inner")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let k = map
                    .next_key()?
                    .ok_or_else(|| A::Error::invalid_length(0, &"missing command in subcommand"))?;
                let com = map.next_value_seed(NameSeed(k))?;
                Ok(com.into())
            }
        }
        impl<'de> DeserializeSeed<'de> for InnerSubCommand {
            type Value = Command;
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_map(self)
            }
        }
    }
    mod arg {
        use self::{arg_action::ArgAction, value_hint::ValueHint, value_parser::ValueParser};
        use crate::ArgWrap;
        use clap::{Arg, Command};
        use serde::de::{DeserializeSeed, Error, Visitor};
        mod arg_action {
            use clap::ArgAction as AA;
            use serde::Deserialize;
            #[serde(rename_all = "snake_case")]
            pub(crate) enum ArgAction {
                Set,
                Append,
                SetTrue,
                SetFalse,
                Count,
                Help,
                Version,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for ArgAction {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "variant identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    4u64 => _serde::__private::Ok(__Field::__field4),
                                    5u64 => _serde::__private::Ok(__Field::__field5),
                                    6u64 => _serde::__private::Ok(__Field::__field6),
                                    _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 7",
                                    )),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "set" => _serde::__private::Ok(__Field::__field0),
                                    "append" => _serde::__private::Ok(__Field::__field1),
                                    "set_true" => _serde::__private::Ok(__Field::__field2),
                                    "set_false" => _serde::__private::Ok(__Field::__field3),
                                    "count" => _serde::__private::Ok(__Field::__field4),
                                    "help" => _serde::__private::Ok(__Field::__field5),
                                    "version" => _serde::__private::Ok(__Field::__field6),
                                    _ => _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    ),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"set" => _serde::__private::Ok(__Field::__field0),
                                    b"append" => _serde::__private::Ok(__Field::__field1),
                                    b"set_true" => _serde::__private::Ok(__Field::__field2),
                                    b"set_false" => _serde::__private::Ok(__Field::__field3),
                                    b"count" => _serde::__private::Ok(__Field::__field4),
                                    b"help" => _serde::__private::Ok(__Field::__field5),
                                    b"version" => _serde::__private::Ok(__Field::__field6),
                                    _ => {
                                        let __value = &_serde::__private::from_utf8_lossy(__value);
                                        _serde::__private::Err(_serde::de::Error::unknown_variant(
                                            __value, VARIANTS,
                                        ))
                                    }
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<ArgAction>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = ArgAction;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "enum ArgAction",
                                )
                            }
                            fn visit_enum<__A>(
                                self,
                                __data: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::EnumAccess<'de>,
                            {
                                match match _serde::de::EnumAccess::variant(__data) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    (__Field::__field0, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ArgAction::Set)
                                    }
                                    (__Field::__field1, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ArgAction::Append)
                                    }
                                    (__Field::__field2, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ArgAction::SetTrue)
                                    }
                                    (__Field::__field3, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ArgAction::SetFalse)
                                    }
                                    (__Field::__field4, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ArgAction::Count)
                                    }
                                    (__Field::__field5, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ArgAction::Help)
                                    }
                                    (__Field::__field6, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ArgAction::Version)
                                    }
                                }
                            }
                        }
                        const VARIANTS: &'static [&'static str] = &[
                            "set",
                            "append",
                            "set_true",
                            "set_false",
                            "count",
                            "help",
                            "version",
                        ];
                        _serde::Deserializer::deserialize_enum(
                            __deserializer,
                            "ArgAction",
                            VARIANTS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<ArgAction>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::clone::Clone for ArgAction {
                #[inline]
                fn clone(&self) -> ArgAction {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ArgAction {}
            impl From<ArgAction> for AA {
                fn from(s: ArgAction) -> AA {
                    match s {
                        ArgAction::Set => AA::Set,
                        ArgAction::Append => AA::Append,
                        ArgAction::SetTrue => AA::SetTrue,
                        ArgAction::SetFalse => AA::SetFalse,
                        ArgAction::Count => AA::Count,
                        ArgAction::Help => AA::Help,
                        ArgAction::Version => AA::Version,
                    }
                }
            }
        }
        mod value_hint {
            use clap::ValueHint as VH;
            use serde::Deserialize;
            #[serde(rename_all = "snake_case")]
            pub(crate) enum ValueHint {
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
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for ValueHint {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                            __field7,
                            __field8,
                            __field9,
                            __field10,
                            __field11,
                            __field12,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "variant identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    4u64 => _serde::__private::Ok(__Field::__field4),
                                    5u64 => _serde::__private::Ok(__Field::__field5),
                                    6u64 => _serde::__private::Ok(__Field::__field6),
                                    7u64 => _serde::__private::Ok(__Field::__field7),
                                    8u64 => _serde::__private::Ok(__Field::__field8),
                                    9u64 => _serde::__private::Ok(__Field::__field9),
                                    10u64 => _serde::__private::Ok(__Field::__field10),
                                    11u64 => _serde::__private::Ok(__Field::__field11),
                                    12u64 => _serde::__private::Ok(__Field::__field12),
                                    _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 13",
                                    )),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "unknown" => _serde::__private::Ok(__Field::__field0),
                                    "other" => _serde::__private::Ok(__Field::__field1),
                                    "any_path" => _serde::__private::Ok(__Field::__field2),
                                    "file_path" => _serde::__private::Ok(__Field::__field3),
                                    "dir_path" => _serde::__private::Ok(__Field::__field4),
                                    "executable_path" => _serde::__private::Ok(__Field::__field5),
                                    "command_name" => _serde::__private::Ok(__Field::__field6),
                                    "command_string" => _serde::__private::Ok(__Field::__field7),
                                    "command_with_arguments" => {
                                        _serde::__private::Ok(__Field::__field8)
                                    }
                                    "username" => _serde::__private::Ok(__Field::__field9),
                                    "hostname" => _serde::__private::Ok(__Field::__field10),
                                    "url" => _serde::__private::Ok(__Field::__field11),
                                    "email_address" => _serde::__private::Ok(__Field::__field12),
                                    _ => _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    ),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"unknown" => _serde::__private::Ok(__Field::__field0),
                                    b"other" => _serde::__private::Ok(__Field::__field1),
                                    b"any_path" => _serde::__private::Ok(__Field::__field2),
                                    b"file_path" => _serde::__private::Ok(__Field::__field3),
                                    b"dir_path" => _serde::__private::Ok(__Field::__field4),
                                    b"executable_path" => _serde::__private::Ok(__Field::__field5),
                                    b"command_name" => _serde::__private::Ok(__Field::__field6),
                                    b"command_string" => _serde::__private::Ok(__Field::__field7),
                                    b"command_with_arguments" => {
                                        _serde::__private::Ok(__Field::__field8)
                                    }
                                    b"username" => _serde::__private::Ok(__Field::__field9),
                                    b"hostname" => _serde::__private::Ok(__Field::__field10),
                                    b"url" => _serde::__private::Ok(__Field::__field11),
                                    b"email_address" => _serde::__private::Ok(__Field::__field12),
                                    _ => {
                                        let __value = &_serde::__private::from_utf8_lossy(__value);
                                        _serde::__private::Err(_serde::de::Error::unknown_variant(
                                            __value, VARIANTS,
                                        ))
                                    }
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<ValueHint>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = ValueHint;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "enum ValueHint",
                                )
                            }
                            fn visit_enum<__A>(
                                self,
                                __data: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::EnumAccess<'de>,
                            {
                                match match _serde::de::EnumAccess::variant(__data) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    (__Field::__field0, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueHint::Unknown)
                                    }
                                    (__Field::__field1, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueHint::Other)
                                    }
                                    (__Field::__field2, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueHint::AnyPath)
                                    }
                                    (__Field::__field3, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueHint::FilePath)
                                    }
                                    (__Field::__field4, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueHint::DirPath)
                                    }
                                    (__Field::__field5, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueHint::ExecutablePath)
                                    }
                                    (__Field::__field6, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueHint::CommandName)
                                    }
                                    (__Field::__field7, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueHint::CommandString)
                                    }
                                    (__Field::__field8, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueHint::CommandWithArguments)
                                    }
                                    (__Field::__field9, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueHint::Username)
                                    }
                                    (__Field::__field10, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueHint::Hostname)
                                    }
                                    (__Field::__field11, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueHint::Url)
                                    }
                                    (__Field::__field12, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueHint::EmailAddress)
                                    }
                                }
                            }
                        }
                        const VARIANTS: &'static [&'static str] = &[
                            "unknown",
                            "other",
                            "any_path",
                            "file_path",
                            "dir_path",
                            "executable_path",
                            "command_name",
                            "command_string",
                            "command_with_arguments",
                            "username",
                            "hostname",
                            "url",
                            "email_address",
                        ];
                        _serde::Deserializer::deserialize_enum(
                            __deserializer,
                            "ValueHint",
                            VARIANTS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<ValueHint>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::clone::Clone for ValueHint {
                #[inline]
                fn clone(&self) -> ValueHint {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ValueHint {}
            impl From<ValueHint> for VH {
                fn from(s: ValueHint) -> VH {
                    match s {
                        ValueHint::Unknown => VH::Unknown,
                        ValueHint::Other => VH::Other,
                        ValueHint::AnyPath => VH::AnyPath,
                        ValueHint::FilePath => VH::FilePath,
                        ValueHint::DirPath => VH::DirPath,
                        ValueHint::ExecutablePath => VH::ExecutablePath,
                        ValueHint::CommandName => VH::CommandName,
                        ValueHint::CommandString => VH::CommandString,
                        ValueHint::CommandWithArguments => VH::CommandWithArguments,
                        ValueHint::Username => VH::Username,
                        ValueHint::Hostname => VH::Hostname,
                        ValueHint::Url => VH::Url,
                        ValueHint::EmailAddress => VH::EmailAddress,
                    }
                }
            }
        }
        mod value_parser {
            use clap::builder::ValueParser as VP;
            use serde::Deserialize;
            const fn get_true() -> bool {
                true
            }
            #[serde(tag = "type")]
            #[serde(rename_all = "snake_case")]
            pub(crate) enum ValueParser1 {
                Bool,
                String,
                OsString,
                PathBuf,
                Boolish,
                Falsey,
                NonEmptyString,
                I64 {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    min: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    max: Option<i64>,
                    #[serde(default = "get_true")]
                    max_inclusive: bool,
                },
                I32 {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    min: Option<i32>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    max: Option<i32>,
                    #[serde(default = "get_true")]
                    max_inclusive: bool,
                },
                I16 {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    min: Option<i16>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    max: Option<i16>,
                    #[serde(default = "get_true")]
                    max_inclusive: bool,
                },
                I8 {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    min: Option<i8>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    max: Option<i8>,
                    #[serde(default = "get_true")]
                    max_inclusive: bool,
                },
                U64 {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    min: Option<u64>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    max: Option<u64>,
                    #[serde(default = "get_true")]
                    max_inclusive: bool,
                },
                U32 {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    min: Option<u32>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    max: Option<u32>,
                    #[serde(default = "get_true")]
                    max_inclusive: bool,
                },
                U16 {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    min: Option<u16>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    max: Option<u16>,
                    #[serde(default = "get_true")]
                    max_inclusive: bool,
                },
                U8 {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    min: Option<u8>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    max: Option<u8>,
                    #[serde(default = "get_true")]
                    max_inclusive: bool,
                },
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for ValueParser1 {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                            __field7,
                            __field8,
                            __field9,
                            __field10,
                            __field11,
                            __field12,
                            __field13,
                            __field14,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "variant identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    4u64 => _serde::__private::Ok(__Field::__field4),
                                    5u64 => _serde::__private::Ok(__Field::__field5),
                                    6u64 => _serde::__private::Ok(__Field::__field6),
                                    7u64 => _serde::__private::Ok(__Field::__field7),
                                    8u64 => _serde::__private::Ok(__Field::__field8),
                                    9u64 => _serde::__private::Ok(__Field::__field9),
                                    10u64 => _serde::__private::Ok(__Field::__field10),
                                    11u64 => _serde::__private::Ok(__Field::__field11),
                                    12u64 => _serde::__private::Ok(__Field::__field12),
                                    13u64 => _serde::__private::Ok(__Field::__field13),
                                    14u64 => _serde::__private::Ok(__Field::__field14),
                                    _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 15",
                                    )),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "bool" => _serde::__private::Ok(__Field::__field0),
                                    "string" => _serde::__private::Ok(__Field::__field1),
                                    "os_string" => _serde::__private::Ok(__Field::__field2),
                                    "path_buf" => _serde::__private::Ok(__Field::__field3),
                                    "boolish" => _serde::__private::Ok(__Field::__field4),
                                    "falsey" => _serde::__private::Ok(__Field::__field5),
                                    "non_empty_string" => _serde::__private::Ok(__Field::__field6),
                                    "i64" => _serde::__private::Ok(__Field::__field7),
                                    "i32" => _serde::__private::Ok(__Field::__field8),
                                    "i16" => _serde::__private::Ok(__Field::__field9),
                                    "i8" => _serde::__private::Ok(__Field::__field10),
                                    "u64" => _serde::__private::Ok(__Field::__field11),
                                    "u32" => _serde::__private::Ok(__Field::__field12),
                                    "u16" => _serde::__private::Ok(__Field::__field13),
                                    "u8" => _serde::__private::Ok(__Field::__field14),
                                    _ => _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    ),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"bool" => _serde::__private::Ok(__Field::__field0),
                                    b"string" => _serde::__private::Ok(__Field::__field1),
                                    b"os_string" => _serde::__private::Ok(__Field::__field2),
                                    b"path_buf" => _serde::__private::Ok(__Field::__field3),
                                    b"boolish" => _serde::__private::Ok(__Field::__field4),
                                    b"falsey" => _serde::__private::Ok(__Field::__field5),
                                    b"non_empty_string" => _serde::__private::Ok(__Field::__field6),
                                    b"i64" => _serde::__private::Ok(__Field::__field7),
                                    b"i32" => _serde::__private::Ok(__Field::__field8),
                                    b"i16" => _serde::__private::Ok(__Field::__field9),
                                    b"i8" => _serde::__private::Ok(__Field::__field10),
                                    b"u64" => _serde::__private::Ok(__Field::__field11),
                                    b"u32" => _serde::__private::Ok(__Field::__field12),
                                    b"u16" => _serde::__private::Ok(__Field::__field13),
                                    b"u8" => _serde::__private::Ok(__Field::__field14),
                                    _ => {
                                        let __value = &_serde::__private::from_utf8_lossy(__value);
                                        _serde::__private::Err(_serde::de::Error::unknown_variant(
                                            __value, VARIANTS,
                                        ))
                                    }
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        const VARIANTS: &'static [&'static str] = &[
                            "bool",
                            "string",
                            "os_string",
                            "path_buf",
                            "boolish",
                            "falsey",
                            "non_empty_string",
                            "i64",
                            "i32",
                            "i16",
                            "i8",
                            "u64",
                            "u32",
                            "u16",
                            "u8",
                        ];
                        let __tagged = match _serde::Deserializer::deserialize_any(
                            __deserializer,
                            _serde::__private::de::TaggedContentVisitor::<__Field>::new(
                                "type",
                                "internally tagged enum ValueParser1",
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match __tagged.tag {
                            __Field::__field0 => {
                                match _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                        "ValueParser1",
                                        "Bool",
                                    ),
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(ValueParser1::Bool)
                            }
                            __Field::__field1 => {
                                match _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                        "ValueParser1",
                                        "String",
                                    ),
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(ValueParser1::String)
                            }
                            __Field::__field2 => {
                                match _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                        "ValueParser1",
                                        "OsString",
                                    ),
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(ValueParser1::OsString)
                            }
                            __Field::__field3 => {
                                match _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                        "ValueParser1",
                                        "PathBuf",
                                    ),
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(ValueParser1::PathBuf)
                            }
                            __Field::__field4 => {
                                match _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                        "ValueParser1",
                                        "Boolish",
                                    ),
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(ValueParser1::Boolish)
                            }
                            __Field::__field5 => {
                                match _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                        "ValueParser1",
                                        "Falsey",
                                    ),
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(ValueParser1::Falsey)
                            }
                            __Field::__field6 => {
                                match _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    _serde::__private::de::InternallyTaggedUnitVisitor::new(
                                        "ValueParser1",
                                        "NonEmptyString",
                                    ),
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(ValueParser1::NonEmptyString)
                            }
                            __Field::__field7 => {
                                #[allow(non_camel_case_types)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __ignore,
                                }
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            2u64 => _serde::__private::Ok(__Field::__field2),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "min" => _serde::__private::Ok(__Field::__field0),
                                            "max" => _serde::__private::Ok(__Field::__field1),
                                            "max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"min" => _serde::__private::Ok(__Field::__field0),
                                            b"max" => _serde::__private::Ok(__Field::__field1),
                                            b"max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<ValueParser1>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ValueParser1;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant ValueParser1::I64",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<i64>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (0usize , & "struct variant ValueParser1::I64 with 3 elements")) ;
                                                }
                                            };
                                        let __field1 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<i64>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (1usize , & "struct variant ValueParser1::I64 with 3 elements")) ;
                                                }
                                            };
                                        let __field2 =
                                            match match _serde::de::SeqAccess::next_element::<bool>(
                                                &mut __seq,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => get_true(),
                                            };
                                        _serde::__private::Ok(ValueParser1::I64 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Option<i64>> =
                                            _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<Option<i64>> =
                                            _serde::__private::None;
                                        let mut __field2: _serde::__private::Option<bool> =
                                            _serde::__private::None;
                                        while let _serde::__private::Some(__key) =
                                            match _serde::de::MapAccess::next_key::<__Field>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            }
                                        {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("min")) ;
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<i64>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max")) ;
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<i64>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private::Option::is_some(&__field2)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max_inclusive")) ;
                                                    }
                                                    __field2 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            bool,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("min") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("max") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private::Some(__field2) => __field2,
                                            _serde::__private::None => get_true(),
                                        };
                                        _serde::__private::Ok(ValueParser1::I64 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                }
                                const FIELDS: &'static [&'static str] =
                                    &["min", "max", "max_inclusive"];
                                _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<ValueParser1>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            __Field::__field8 => {
                                #[allow(non_camel_case_types)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __ignore,
                                }
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            2u64 => _serde::__private::Ok(__Field::__field2),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "min" => _serde::__private::Ok(__Field::__field0),
                                            "max" => _serde::__private::Ok(__Field::__field1),
                                            "max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"min" => _serde::__private::Ok(__Field::__field0),
                                            b"max" => _serde::__private::Ok(__Field::__field1),
                                            b"max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<ValueParser1>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ValueParser1;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant ValueParser1::I32",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<i32>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (0usize , & "struct variant ValueParser1::I32 with 3 elements")) ;
                                                }
                                            };
                                        let __field1 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<i32>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (1usize , & "struct variant ValueParser1::I32 with 3 elements")) ;
                                                }
                                            };
                                        let __field2 =
                                            match match _serde::de::SeqAccess::next_element::<bool>(
                                                &mut __seq,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => get_true(),
                                            };
                                        _serde::__private::Ok(ValueParser1::I32 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Option<i32>> =
                                            _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<Option<i32>> =
                                            _serde::__private::None;
                                        let mut __field2: _serde::__private::Option<bool> =
                                            _serde::__private::None;
                                        while let _serde::__private::Some(__key) =
                                            match _serde::de::MapAccess::next_key::<__Field>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            }
                                        {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("min")) ;
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<i32>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max")) ;
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<i32>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private::Option::is_some(&__field2)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max_inclusive")) ;
                                                    }
                                                    __field2 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            bool,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("min") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("max") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private::Some(__field2) => __field2,
                                            _serde::__private::None => get_true(),
                                        };
                                        _serde::__private::Ok(ValueParser1::I32 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                }
                                const FIELDS: &'static [&'static str] =
                                    &["min", "max", "max_inclusive"];
                                _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<ValueParser1>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            __Field::__field9 => {
                                #[allow(non_camel_case_types)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __ignore,
                                }
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            2u64 => _serde::__private::Ok(__Field::__field2),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "min" => _serde::__private::Ok(__Field::__field0),
                                            "max" => _serde::__private::Ok(__Field::__field1),
                                            "max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"min" => _serde::__private::Ok(__Field::__field0),
                                            b"max" => _serde::__private::Ok(__Field::__field1),
                                            b"max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<ValueParser1>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ValueParser1;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant ValueParser1::I16",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<i16>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (0usize , & "struct variant ValueParser1::I16 with 3 elements")) ;
                                                }
                                            };
                                        let __field1 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<i16>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (1usize , & "struct variant ValueParser1::I16 with 3 elements")) ;
                                                }
                                            };
                                        let __field2 =
                                            match match _serde::de::SeqAccess::next_element::<bool>(
                                                &mut __seq,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => get_true(),
                                            };
                                        _serde::__private::Ok(ValueParser1::I16 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Option<i16>> =
                                            _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<Option<i16>> =
                                            _serde::__private::None;
                                        let mut __field2: _serde::__private::Option<bool> =
                                            _serde::__private::None;
                                        while let _serde::__private::Some(__key) =
                                            match _serde::de::MapAccess::next_key::<__Field>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            }
                                        {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("min")) ;
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<i16>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max")) ;
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<i16>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private::Option::is_some(&__field2)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max_inclusive")) ;
                                                    }
                                                    __field2 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            bool,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("min") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("max") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private::Some(__field2) => __field2,
                                            _serde::__private::None => get_true(),
                                        };
                                        _serde::__private::Ok(ValueParser1::I16 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                }
                                const FIELDS: &'static [&'static str] =
                                    &["min", "max", "max_inclusive"];
                                _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<ValueParser1>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            __Field::__field10 => {
                                #[allow(non_camel_case_types)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __ignore,
                                }
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            2u64 => _serde::__private::Ok(__Field::__field2),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "min" => _serde::__private::Ok(__Field::__field0),
                                            "max" => _serde::__private::Ok(__Field::__field1),
                                            "max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"min" => _serde::__private::Ok(__Field::__field0),
                                            b"max" => _serde::__private::Ok(__Field::__field1),
                                            b"max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<ValueParser1>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ValueParser1;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant ValueParser1::I8",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<i8>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (0usize , & "struct variant ValueParser1::I8 with 3 elements")) ;
                                                }
                                            };
                                        let __field1 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<i8>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (1usize , & "struct variant ValueParser1::I8 with 3 elements")) ;
                                                }
                                            };
                                        let __field2 =
                                            match match _serde::de::SeqAccess::next_element::<bool>(
                                                &mut __seq,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => get_true(),
                                            };
                                        _serde::__private::Ok(ValueParser1::I8 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Option<i8>> =
                                            _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<Option<i8>> =
                                            _serde::__private::None;
                                        let mut __field2: _serde::__private::Option<bool> =
                                            _serde::__private::None;
                                        while let _serde::__private::Some(__key) =
                                            match _serde::de::MapAccess::next_key::<__Field>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            }
                                        {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("min")) ;
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<i8>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max")) ;
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<i8>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private::Option::is_some(&__field2)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max_inclusive")) ;
                                                    }
                                                    __field2 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            bool,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("min") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("max") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private::Some(__field2) => __field2,
                                            _serde::__private::None => get_true(),
                                        };
                                        _serde::__private::Ok(ValueParser1::I8 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                }
                                const FIELDS: &'static [&'static str] =
                                    &["min", "max", "max_inclusive"];
                                _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<ValueParser1>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            __Field::__field11 => {
                                #[allow(non_camel_case_types)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __ignore,
                                }
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            2u64 => _serde::__private::Ok(__Field::__field2),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "min" => _serde::__private::Ok(__Field::__field0),
                                            "max" => _serde::__private::Ok(__Field::__field1),
                                            "max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"min" => _serde::__private::Ok(__Field::__field0),
                                            b"max" => _serde::__private::Ok(__Field::__field1),
                                            b"max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<ValueParser1>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ValueParser1;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant ValueParser1::U64",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<u64>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (0usize , & "struct variant ValueParser1::U64 with 3 elements")) ;
                                                }
                                            };
                                        let __field1 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<u64>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (1usize , & "struct variant ValueParser1::U64 with 3 elements")) ;
                                                }
                                            };
                                        let __field2 =
                                            match match _serde::de::SeqAccess::next_element::<bool>(
                                                &mut __seq,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => get_true(),
                                            };
                                        _serde::__private::Ok(ValueParser1::U64 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Option<u64>> =
                                            _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<Option<u64>> =
                                            _serde::__private::None;
                                        let mut __field2: _serde::__private::Option<bool> =
                                            _serde::__private::None;
                                        while let _serde::__private::Some(__key) =
                                            match _serde::de::MapAccess::next_key::<__Field>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            }
                                        {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("min")) ;
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<u64>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max")) ;
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<u64>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private::Option::is_some(&__field2)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max_inclusive")) ;
                                                    }
                                                    __field2 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            bool,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("min") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("max") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private::Some(__field2) => __field2,
                                            _serde::__private::None => get_true(),
                                        };
                                        _serde::__private::Ok(ValueParser1::U64 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                }
                                const FIELDS: &'static [&'static str] =
                                    &["min", "max", "max_inclusive"];
                                _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<ValueParser1>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            __Field::__field12 => {
                                #[allow(non_camel_case_types)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __ignore,
                                }
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            2u64 => _serde::__private::Ok(__Field::__field2),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "min" => _serde::__private::Ok(__Field::__field0),
                                            "max" => _serde::__private::Ok(__Field::__field1),
                                            "max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"min" => _serde::__private::Ok(__Field::__field0),
                                            b"max" => _serde::__private::Ok(__Field::__field1),
                                            b"max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<ValueParser1>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ValueParser1;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant ValueParser1::U32",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<u32>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (0usize , & "struct variant ValueParser1::U32 with 3 elements")) ;
                                                }
                                            };
                                        let __field1 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<u32>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (1usize , & "struct variant ValueParser1::U32 with 3 elements")) ;
                                                }
                                            };
                                        let __field2 =
                                            match match _serde::de::SeqAccess::next_element::<bool>(
                                                &mut __seq,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => get_true(),
                                            };
                                        _serde::__private::Ok(ValueParser1::U32 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Option<u32>> =
                                            _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<Option<u32>> =
                                            _serde::__private::None;
                                        let mut __field2: _serde::__private::Option<bool> =
                                            _serde::__private::None;
                                        while let _serde::__private::Some(__key) =
                                            match _serde::de::MapAccess::next_key::<__Field>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            }
                                        {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("min")) ;
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<u32>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max")) ;
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<u32>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private::Option::is_some(&__field2)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max_inclusive")) ;
                                                    }
                                                    __field2 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            bool,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("min") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("max") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private::Some(__field2) => __field2,
                                            _serde::__private::None => get_true(),
                                        };
                                        _serde::__private::Ok(ValueParser1::U32 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                }
                                const FIELDS: &'static [&'static str] =
                                    &["min", "max", "max_inclusive"];
                                _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<ValueParser1>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            __Field::__field13 => {
                                #[allow(non_camel_case_types)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __ignore,
                                }
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            2u64 => _serde::__private::Ok(__Field::__field2),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "min" => _serde::__private::Ok(__Field::__field0),
                                            "max" => _serde::__private::Ok(__Field::__field1),
                                            "max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"min" => _serde::__private::Ok(__Field::__field0),
                                            b"max" => _serde::__private::Ok(__Field::__field1),
                                            b"max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<ValueParser1>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ValueParser1;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant ValueParser1::U16",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<u16>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (0usize , & "struct variant ValueParser1::U16 with 3 elements")) ;
                                                }
                                            };
                                        let __field1 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<u16>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (1usize , & "struct variant ValueParser1::U16 with 3 elements")) ;
                                                }
                                            };
                                        let __field2 =
                                            match match _serde::de::SeqAccess::next_element::<bool>(
                                                &mut __seq,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => get_true(),
                                            };
                                        _serde::__private::Ok(ValueParser1::U16 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Option<u16>> =
                                            _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<Option<u16>> =
                                            _serde::__private::None;
                                        let mut __field2: _serde::__private::Option<bool> =
                                            _serde::__private::None;
                                        while let _serde::__private::Some(__key) =
                                            match _serde::de::MapAccess::next_key::<__Field>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            }
                                        {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("min")) ;
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<u16>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max")) ;
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<u16>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private::Option::is_some(&__field2)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max_inclusive")) ;
                                                    }
                                                    __field2 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            bool,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("min") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("max") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private::Some(__field2) => __field2,
                                            _serde::__private::None => get_true(),
                                        };
                                        _serde::__private::Ok(ValueParser1::U16 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                }
                                const FIELDS: &'static [&'static str] =
                                    &["min", "max", "max_inclusive"];
                                _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<ValueParser1>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            __Field::__field14 => {
                                #[allow(non_camel_case_types)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __ignore,
                                }
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            2u64 => _serde::__private::Ok(__Field::__field2),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "min" => _serde::__private::Ok(__Field::__field0),
                                            "max" => _serde::__private::Ok(__Field::__field1),
                                            "max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"min" => _serde::__private::Ok(__Field::__field0),
                                            b"max" => _serde::__private::Ok(__Field::__field1),
                                            b"max_inclusive" => {
                                                _serde::__private::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<ValueParser1>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ValueParser1;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant ValueParser1::U8",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<u8>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (0usize , & "struct variant ValueParser1::U8 with 3 elements")) ;
                                                }
                                            };
                                        let __field1 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                Option<u8>,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (1usize , & "struct variant ValueParser1::U8 with 3 elements")) ;
                                                }
                                            };
                                        let __field2 =
                                            match match _serde::de::SeqAccess::next_element::<bool>(
                                                &mut __seq,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => get_true(),
                                            };
                                        _serde::__private::Ok(ValueParser1::U8 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<Option<u8>> =
                                            _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<Option<u8>> =
                                            _serde::__private::None;
                                        let mut __field2: _serde::__private::Option<bool> =
                                            _serde::__private::None;
                                        while let _serde::__private::Some(__key) =
                                            match _serde::de::MapAccess::next_key::<__Field>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            }
                                        {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("min")) ;
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<u8>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max")) ;
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            Option<u8>,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private::Option::is_some(&__field2)
                                                    {
                                                        return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("max_inclusive")) ;
                                                    }
                                                    __field2 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            bool,
                                                        >(
                                                            &mut __map
                                                        ) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(
                                                                    __err,
                                                                );
                                                            }
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("min") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("max") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private::Some(__field2) => __field2,
                                            _serde::__private::None => get_true(),
                                        };
                                        _serde::__private::Ok(ValueParser1::U8 {
                                            min: __field0,
                                            max: __field1,
                                            max_inclusive: __field2,
                                        })
                                    }
                                }
                                const FIELDS: &'static [&'static str] =
                                    &["min", "max", "max_inclusive"];
                                _serde::Deserializer::deserialize_any(
                                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                        __tagged.content,
                                    ),
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<ValueParser1>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
            };
            #[automatically_derived]
            impl ::core::clone::Clone for ValueParser1 {
                #[inline]
                fn clone(&self) -> ValueParser1 {
                    let _: ::core::clone::AssertParamIsClone<Option<i64>>;
                    let _: ::core::clone::AssertParamIsClone<Option<i64>>;
                    let _: ::core::clone::AssertParamIsClone<bool>;
                    let _: ::core::clone::AssertParamIsClone<Option<i32>>;
                    let _: ::core::clone::AssertParamIsClone<Option<i32>>;
                    let _: ::core::clone::AssertParamIsClone<Option<i16>>;
                    let _: ::core::clone::AssertParamIsClone<Option<i16>>;
                    let _: ::core::clone::AssertParamIsClone<Option<i8>>;
                    let _: ::core::clone::AssertParamIsClone<Option<i8>>;
                    let _: ::core::clone::AssertParamIsClone<Option<u64>>;
                    let _: ::core::clone::AssertParamIsClone<Option<u64>>;
                    let _: ::core::clone::AssertParamIsClone<Option<u32>>;
                    let _: ::core::clone::AssertParamIsClone<Option<u32>>;
                    let _: ::core::clone::AssertParamIsClone<Option<u16>>;
                    let _: ::core::clone::AssertParamIsClone<Option<u16>>;
                    let _: ::core::clone::AssertParamIsClone<Option<u8>>;
                    let _: ::core::clone::AssertParamIsClone<Option<u8>>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ValueParser1 {}
            impl From<ValueParser1> for VP {
                fn from(s: ValueParser1) -> VP {
                    match s {
                        ValueParser1::Bool => VP::bool(),
                        ValueParser1::String => VP::string(),
                        ValueParser1::OsString => VP::os_string(),
                        ValueParser1::PathBuf => VP::path_buf(),
                        ValueParser1::Boolish => clap::builder::BoolishValueParser::new().into(),
                        ValueParser1::Falsey => clap::builder::FalseyValueParser::new().into(),
                        ValueParser1::NonEmptyString => {
                            clap::builder::NonEmptyStringValueParser::new().into()
                        }
                        ValueParser1::I64 {
                            min,
                            max,
                            max_inclusive,
                        } => match (min, max, max_inclusive) {
                            (Some(s), Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i64>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s)..(e))
                            .into(),
                            (Some(s), Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i64>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s)..=(e))
                            .into(),
                            (Some(s), None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i64>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s)..)
                            .into(),
                            (None, Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i64>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..(e))
                            .into(),
                            (None, Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i64>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..=(e))
                            .into(),
                            (None, None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i64>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .into(),
                        },
                        ValueParser1::I32 {
                            min,
                            max,
                            max_inclusive,
                        } => match (min, max, max_inclusive) {
                            (Some(s), Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i32>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..(e as i64))
                            .into(),
                            (Some(s), Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i32>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..=(e as i64))
                            .into(),
                            (Some(s), None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i32>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..)
                            .into(),
                            (None, Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i32>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..(e as i64))
                            .into(),
                            (None, Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i32>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..=(e as i64))
                            .into(),
                            (None, None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i32>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .into(),
                        },
                        ValueParser1::I16 {
                            min,
                            max,
                            max_inclusive,
                        } => match (min, max, max_inclusive) {
                            (Some(s), Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i16>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..(e as i64))
                            .into(),
                            (Some(s), Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i16>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..=(e as i64))
                            .into(),
                            (Some(s), None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i16>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..)
                            .into(),
                            (None, Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i16>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..(e as i64))
                            .into(),
                            (None, Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i16>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..=(e as i64))
                            .into(),
                            (None, None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i16>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .into(),
                        },
                        ValueParser1::I8 {
                            min,
                            max,
                            max_inclusive,
                        } => match (min, max, max_inclusive) {
                            (Some(s), Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i8>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..(e as i64))
                            .into(),
                            (Some(s), Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i8>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..=(e as i64))
                            .into(),
                            (Some(s), None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i8>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..)
                            .into(),
                            (None, Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i8>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..(e as i64))
                            .into(),
                            (None, Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i8>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..=(e as i64))
                            .into(),
                            (None, None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<i8>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .into(),
                        },
                        ValueParser1::U64 {
                            min,
                            max,
                            max_inclusive,
                        } => match (min, max, max_inclusive) {
                            (Some(s), Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u64>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s)..(e))
                            .into(),
                            (Some(s), Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u64>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s)..=(e))
                            .into(),
                            (Some(s), None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u64>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s)..)
                            .into(),
                            (None, Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u64>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..(e))
                            .into(),
                            (None, Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u64>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..=(e))
                            .into(),
                            (None, None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u64>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .into(),
                        },
                        ValueParser1::U32 {
                            min,
                            max,
                            max_inclusive,
                        } => match (min, max, max_inclusive) {
                            (Some(s), Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u32>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..(e as i64))
                            .into(),
                            (Some(s), Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u32>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..=(e as i64))
                            .into(),
                            (Some(s), None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u32>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..)
                            .into(),
                            (None, Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u32>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..(e as i64))
                            .into(),
                            (None, Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u32>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..=(e as i64))
                            .into(),
                            (None, None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u32>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .into(),
                        },
                        ValueParser1::U16 {
                            min,
                            max,
                            max_inclusive,
                        } => match (min, max, max_inclusive) {
                            (Some(s), Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u16>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..(e as i64))
                            .into(),
                            (Some(s), Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u16>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..=(e as i64))
                            .into(),
                            (Some(s), None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u16>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..)
                            .into(),
                            (None, Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u16>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..(e as i64))
                            .into(),
                            (None, Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u16>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..=(e as i64))
                            .into(),
                            (None, None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u16>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .into(),
                        },
                        ValueParser1::U8 {
                            min,
                            max,
                            max_inclusive,
                        } => match (min, max, max_inclusive) {
                            (Some(s), Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u8>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..(e as i64))
                            .into(),
                            (Some(s), Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u8>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..=(e as i64))
                            .into(),
                            (Some(s), None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u8>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range((s as i64)..)
                            .into(),
                            (None, Some(e), false) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u8>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..(e as i64))
                            .into(),
                            (None, Some(e), true) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u8>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .range(..=(e as i64))
                            .into(),
                            (None, None, _) => {
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u8>::new();
                                (&&&&&&auto).value_parser()
                            }
                            .into(),
                        },
                    }
                }
            }
            #[serde(rename_all = "snake_case")]
            pub(crate) enum ValueParser2 {
                Bool,
                String,
                OsString,
                PathBuf,
                Boolish,
                Falsey,
                NonEmptyString,
                I64,
                I32,
                I16,
                I8,
                U64,
                U32,
                U16,
                U8,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for ValueParser2 {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                            __field7,
                            __field8,
                            __field9,
                            __field10,
                            __field11,
                            __field12,
                            __field13,
                            __field14,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "variant identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    4u64 => _serde::__private::Ok(__Field::__field4),
                                    5u64 => _serde::__private::Ok(__Field::__field5),
                                    6u64 => _serde::__private::Ok(__Field::__field6),
                                    7u64 => _serde::__private::Ok(__Field::__field7),
                                    8u64 => _serde::__private::Ok(__Field::__field8),
                                    9u64 => _serde::__private::Ok(__Field::__field9),
                                    10u64 => _serde::__private::Ok(__Field::__field10),
                                    11u64 => _serde::__private::Ok(__Field::__field11),
                                    12u64 => _serde::__private::Ok(__Field::__field12),
                                    13u64 => _serde::__private::Ok(__Field::__field13),
                                    14u64 => _serde::__private::Ok(__Field::__field14),
                                    _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 15",
                                    )),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "bool" => _serde::__private::Ok(__Field::__field0),
                                    "string" => _serde::__private::Ok(__Field::__field1),
                                    "os_string" => _serde::__private::Ok(__Field::__field2),
                                    "path_buf" => _serde::__private::Ok(__Field::__field3),
                                    "boolish" => _serde::__private::Ok(__Field::__field4),
                                    "falsey" => _serde::__private::Ok(__Field::__field5),
                                    "non_empty_string" => _serde::__private::Ok(__Field::__field6),
                                    "i64" => _serde::__private::Ok(__Field::__field7),
                                    "i32" => _serde::__private::Ok(__Field::__field8),
                                    "i16" => _serde::__private::Ok(__Field::__field9),
                                    "i8" => _serde::__private::Ok(__Field::__field10),
                                    "u64" => _serde::__private::Ok(__Field::__field11),
                                    "u32" => _serde::__private::Ok(__Field::__field12),
                                    "u16" => _serde::__private::Ok(__Field::__field13),
                                    "u8" => _serde::__private::Ok(__Field::__field14),
                                    _ => _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    ),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"bool" => _serde::__private::Ok(__Field::__field0),
                                    b"string" => _serde::__private::Ok(__Field::__field1),
                                    b"os_string" => _serde::__private::Ok(__Field::__field2),
                                    b"path_buf" => _serde::__private::Ok(__Field::__field3),
                                    b"boolish" => _serde::__private::Ok(__Field::__field4),
                                    b"falsey" => _serde::__private::Ok(__Field::__field5),
                                    b"non_empty_string" => _serde::__private::Ok(__Field::__field6),
                                    b"i64" => _serde::__private::Ok(__Field::__field7),
                                    b"i32" => _serde::__private::Ok(__Field::__field8),
                                    b"i16" => _serde::__private::Ok(__Field::__field9),
                                    b"i8" => _serde::__private::Ok(__Field::__field10),
                                    b"u64" => _serde::__private::Ok(__Field::__field11),
                                    b"u32" => _serde::__private::Ok(__Field::__field12),
                                    b"u16" => _serde::__private::Ok(__Field::__field13),
                                    b"u8" => _serde::__private::Ok(__Field::__field14),
                                    _ => {
                                        let __value = &_serde::__private::from_utf8_lossy(__value);
                                        _serde::__private::Err(_serde::de::Error::unknown_variant(
                                            __value, VARIANTS,
                                        ))
                                    }
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<ValueParser2>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = ValueParser2;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "enum ValueParser2",
                                )
                            }
                            fn visit_enum<__A>(
                                self,
                                __data: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::EnumAccess<'de>,
                            {
                                match match _serde::de::EnumAccess::variant(__data) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    (__Field::__field0, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::Bool)
                                    }
                                    (__Field::__field1, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::String)
                                    }
                                    (__Field::__field2, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::OsString)
                                    }
                                    (__Field::__field3, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::PathBuf)
                                    }
                                    (__Field::__field4, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::Boolish)
                                    }
                                    (__Field::__field5, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::Falsey)
                                    }
                                    (__Field::__field6, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::NonEmptyString)
                                    }
                                    (__Field::__field7, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::I64)
                                    }
                                    (__Field::__field8, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::I32)
                                    }
                                    (__Field::__field9, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::I16)
                                    }
                                    (__Field::__field10, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::I8)
                                    }
                                    (__Field::__field11, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::U64)
                                    }
                                    (__Field::__field12, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::U32)
                                    }
                                    (__Field::__field13, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::U16)
                                    }
                                    (__Field::__field14, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                        _serde::__private::Ok(ValueParser2::U8)
                                    }
                                }
                            }
                        }
                        const VARIANTS: &'static [&'static str] = &[
                            "bool",
                            "string",
                            "os_string",
                            "path_buf",
                            "boolish",
                            "falsey",
                            "non_empty_string",
                            "i64",
                            "i32",
                            "i16",
                            "i8",
                            "u64",
                            "u32",
                            "u16",
                            "u8",
                        ];
                        _serde::Deserializer::deserialize_enum(
                            __deserializer,
                            "ValueParser2",
                            VARIANTS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<ValueParser2>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::clone::Clone for ValueParser2 {
                #[inline]
                fn clone(&self) -> ValueParser2 {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ValueParser2 {}
            impl From<ValueParser2> for VP {
                fn from(s: ValueParser2) -> VP {
                    match s {
                        ValueParser2::Bool => VP::bool(),
                        ValueParser2::String => VP::string(),
                        ValueParser2::OsString => VP::os_string(),
                        ValueParser2::PathBuf => VP::path_buf(),
                        ValueParser2::Boolish => clap::builder::BoolishValueParser::new().into(),
                        ValueParser2::Falsey => clap::builder::FalseyValueParser::new().into(),
                        ValueParser2::NonEmptyString => {
                            clap::builder::NonEmptyStringValueParser::new().into()
                        }
                        ValueParser2::I64 => {
                            use ::clap::builder::via_prelude::*;
                            let auto = ::clap::builder::_AutoValueParser::<i64>::new();
                            (&&&&&&auto).value_parser()
                        }
                        .into(),
                        ValueParser2::I32 => {
                            use ::clap::builder::via_prelude::*;
                            let auto = ::clap::builder::_AutoValueParser::<i32>::new();
                            (&&&&&&auto).value_parser()
                        }
                        .into(),
                        ValueParser2::I16 => {
                            use ::clap::builder::via_prelude::*;
                            let auto = ::clap::builder::_AutoValueParser::<i16>::new();
                            (&&&&&&auto).value_parser()
                        }
                        .into(),
                        ValueParser2::I8 => {
                            use ::clap::builder::via_prelude::*;
                            let auto = ::clap::builder::_AutoValueParser::<i8>::new();
                            (&&&&&&auto).value_parser()
                        }
                        .into(),
                        ValueParser2::U64 => {
                            use ::clap::builder::via_prelude::*;
                            let auto = ::clap::builder::_AutoValueParser::<u64>::new();
                            (&&&&&&auto).value_parser()
                        }
                        .into(),
                        ValueParser2::U32 => {
                            use ::clap::builder::via_prelude::*;
                            let auto = ::clap::builder::_AutoValueParser::<u32>::new();
                            (&&&&&&auto).value_parser()
                        }
                        .into(),
                        ValueParser2::U16 => {
                            use ::clap::builder::via_prelude::*;
                            let auto = ::clap::builder::_AutoValueParser::<u16>::new();
                            (&&&&&&auto).value_parser()
                        }
                        .into(),
                        ValueParser2::U8 => {
                            use ::clap::builder::via_prelude::*;
                            let auto = ::clap::builder::_AutoValueParser::<u8>::new();
                            (&&&&&&auto).value_parser()
                        }
                        .into(),
                    }
                }
            }
            #[serde(untagged)]
            pub(crate) enum ValueParser {
                Value(ValueParser2),
                Tagged(ValueParser1),
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for ValueParser {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        let __content = match < _serde :: __private :: de :: Content as _serde :: Deserialize > :: deserialize (__deserializer) { _serde :: __private :: Ok (__val) => __val , _serde :: __private :: Err (__err) => { return _serde :: __private :: Err (__err) ; } } ;
                        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                            <ValueParser2 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                            ),
                            ValueParser::Value,
                        ) {
                            return _serde::__private::Ok(__ok);
                        }
                        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                            <ValueParser1 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                            ),
                            ValueParser::Tagged,
                        ) {
                            return _serde::__private::Ok(__ok);
                        }
                        _serde::__private::Err(_serde::de::Error::custom(
                            "data did not match any variant of untagged enum ValueParser",
                        ))
                    }
                }
            };
            impl From<ValueParser> for VP {
                fn from(v: ValueParser) -> Self {
                    match v {
                        ValueParser::Value(v) => v.into(),
                        ValueParser::Tagged(t) => t.into(),
                    }
                }
            }
        }
        struct ArgKV;
        #[automatically_derived]
        impl ::core::clone::Clone for ArgKV {
            #[inline]
            fn clone(&self) -> ArgKV {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ArgKV {}
        impl<'de> Visitor<'de> for ArgKV {
            type Value = ArgWrap;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("kv argument")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let name: &str = map
                    .next_key()?
                    .ok_or_else(|| A::Error::missing_field("argument"))?;
                map.next_value_seed(ArgVisitor::new_str(name))
            }
        }
        impl<'de> DeserializeSeed<'de> for ArgKV {
            type Value = ArgWrap;
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_map(self)
            }
        }
        struct ArgVisitor(Arg);
        impl ArgVisitor {
            fn new_str(v: &str) -> Self {
                Self(Arg::new(v.to_owned()))
            }
        }
        impl<'de> Visitor<'de> for ArgVisitor {
            type Value = ArgWrap;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Arg Map")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut arg = self.0;
                while let Some(key) = map.next_key::<&str>()? {
                    arg = {
                        #[allow(unused_mut)]
                        let mut key;
                        {
                            key = key;
                        };
                        #[allow(unused_labels)]
                        'parse_value_jmp_loop: loop {
                            break 'parse_value_jmp_loop match key {
                                "alias" => <Arg>::alias(arg, map.next_value::<String>()?),
                                "aliases" => <Arg>::aliases(arg, &map.next_value::<Vec<String>>()?),
                                "allow_hyphen_values" => {
                                    <Arg>::allow_hyphen_values(arg, map.next_value::<bool>()?)
                                }
                                "conflicts_with" => {
                                    <Arg>::conflicts_with(arg, map.next_value::<String>()?)
                                }
                                "conflicts_with_all" => <Arg>::conflicts_with_all(
                                    arg,
                                    &map.next_value::<Vec<String>>()?,
                                ),
                                "default_missing_value" => {
                                    <Arg>::default_missing_value(arg, map.next_value::<String>()?)
                                }
                                "default_missing_values" => <Arg>::default_missing_values(
                                    arg,
                                    &map.next_value::<Vec<String>>()?,
                                ),
                                "default_value" => {
                                    <Arg>::default_value(arg, map.next_value::<String>()?)
                                }
                                "default_value_ifs" => <Arg>::default_value_ifs(
                                    arg,
                                    map.next_value::<Vec<(String, String, String)>>()?,
                                ),
                                "display_order" => {
                                    <Arg>::display_order(arg, map.next_value::<usize>()?)
                                }
                                "exclusive" => <Arg>::exclusive(arg, map.next_value::<bool>()?),
                                "global" => <Arg>::global(arg, map.next_value::<bool>()?),
                                "group" => <Arg>::group(arg, map.next_value::<String>()?),
                                "groups" => <Arg>::groups(arg, &map.next_value::<Vec<String>>()?),
                                "help" => <Arg>::help(arg, map.next_value::<String>()?),
                                "help_heading" => {
                                    <Arg>::help_heading(arg, map.next_value::<String>()?)
                                }
                                "hide" => <Arg>::hide(arg, map.next_value::<bool>()?),
                                "hide_default_value" => {
                                    <Arg>::hide_default_value(arg, map.next_value::<bool>()?)
                                }
                                "hide_long_help" => {
                                    <Arg>::hide_long_help(arg, map.next_value::<bool>()?)
                                }
                                "hide_possible_values" => {
                                    <Arg>::hide_possible_values(arg, map.next_value::<bool>()?)
                                }
                                "hide_short_help" => {
                                    <Arg>::hide_short_help(arg, map.next_value::<bool>()?)
                                }
                                "id" => <Arg>::id(arg, map.next_value::<String>()?),
                                "ignore_case" => <Arg>::ignore_case(arg, map.next_value::<bool>()?),
                                "index" => <Arg>::index(arg, map.next_value::<usize>()?),
                                "last" => <Arg>::last(arg, map.next_value::<bool>()?),
                                "long" => <Arg>::long(arg, map.next_value::<String>()?),
                                "long_help" => <Arg>::long_help(arg, map.next_value::<String>()?),
                                "id" => <Arg>::id(arg, map.next_value::<String>()?),
                                "next_line_help" => {
                                    <Arg>::next_line_help(arg, map.next_value::<bool>()?)
                                }
                                "number_of_values" => {
                                    <Arg>::number_of_values(arg, map.next_value::<usize>()?)
                                }
                                "overrides_with" => {
                                    <Arg>::overrides_with(arg, map.next_value::<String>()?)
                                }
                                "overrides_with_all" => <Arg>::overrides_with_all(
                                    arg,
                                    &map.next_value::<Vec<String>>()?,
                                ),
                                "require_equals" => {
                                    <Arg>::require_equals(arg, map.next_value::<bool>()?)
                                }
                                "raw" => <Arg>::raw(arg, map.next_value::<bool>()?),
                                "required" => <Arg>::required(arg, map.next_value::<bool>()?),
                                "required_if_eq_all" => <Arg>::required_if_eq_all(
                                    arg,
                                    map.next_value::<Vec<(String, String)>>()?,
                                ),
                                "required_if_eq_any" => <Arg>::required_if_eq_any(
                                    arg,
                                    map.next_value::<Vec<(String, String)>>()?,
                                ),
                                "required_unless_present" => {
                                    <Arg>::required_unless_present(arg, map.next_value::<String>()?)
                                }
                                "required_unless_present_any" => {
                                    <Arg>::required_unless_present_any(
                                        arg,
                                        &map.next_value::<Vec<String>>()?,
                                    )
                                }
                                "required_unless_present_all" => {
                                    <Arg>::required_unless_present_all(
                                        arg,
                                        &map.next_value::<Vec<String>>()?,
                                    )
                                }
                                "requires" => <Arg>::requires(arg, map.next_value::<String>()?),
                                "requires_all" => {
                                    <Arg>::requires_all(arg, &map.next_value::<Vec<String>>()?)
                                }
                                "requires_ifs" => <Arg>::requires_ifs(
                                    arg,
                                    map.next_value::<Vec<(String, String)>>()?,
                                ),
                                "short" => <Arg>::short(arg, map.next_value::<char>()?),
                                "short_alias" => <Arg>::short_alias(arg, map.next_value::<char>()?),
                                "short_aliases" => {
                                    <Arg>::short_aliases(arg, map.next_value::<Vec<char>>()?)
                                }
                                "use_value_delimiter" => {
                                    <Arg>::use_value_delimiter(arg, map.next_value::<bool>()?)
                                }
                                "value_delimiter" => {
                                    <Arg>::value_delimiter(arg, map.next_value::<char>()?)
                                }
                                "value_name" => <Arg>::value_name(arg, map.next_value::<String>()?),
                                "value_names" => {
                                    <Arg>::value_names(arg, &map.next_value::<Vec<String>>()?)
                                }
                                "value_terminator" => {
                                    <Arg>::value_terminator(arg, map.next_value::<String>()?)
                                }
                                "visible_alias" => {
                                    <Arg>::visible_alias(arg, map.next_value::<String>()?)
                                }
                                "visible_aliases" => {
                                    <Arg>::visible_aliases(arg, &map.next_value::<Vec<String>>()?)
                                }
                                "visible_short_alias" => {
                                    <Arg>::visible_short_alias(arg, map.next_value::<char>()?)
                                }
                                "visible_short_aliases" => <Arg>::visible_short_aliases(
                                    arg,
                                    map.next_value::<Vec<char>>()?,
                                ),
                                "required_if_eq" => {
                                    let (v0, v1) = map.next_value::<(String, String)>()?;
                                    <Arg>::required_if_eq(arg, v0, v1)
                                }
                                "requires_if" => {
                                    let (v0, v1) = map.next_value::<(String, String)>()?;
                                    <Arg>::requires_if(arg, v0, v1)
                                }
                                "default_value_if" => {
                                    let (v0, v1, v2) =
                                        map.next_value::<(String, String, String)>()?;
                                    <Arg>::default_value_if(arg, v0, v1, v2)
                                }
                                "arg_action" => arg
                                    .action(clap::ArgAction::from(map.next_value::<ArgAction>()?)),
                                "env" => {
                                    #[cfg(not(feature = "env"))]
                                    {
                                        return Err(Error::custom("env feature disabled"));
                                    }
                                }
                                "hide_env" => {
                                    #[cfg(not(feature = "env"))]
                                    {
                                        return Err(Error::custom("env feature disabled"));
                                    }
                                }
                                "hide_env_values" => {
                                    #[cfg(not(feature = "env"))]
                                    {
                                        return Err(Error::custom("env feature disabled"));
                                    }
                                }
                                "value_hint" => arg.value_hint(clap::ValueHint::from(
                                    map.next_value::<ValueHint>()?,
                                )),
                                "value_parser" => {
                                    arg.value_parser(map.next_value::<ValueParser>()?)
                                }
                                depr @ ("case_insensitive"
                                | "empty_values"
                                | "from_usage"
                                | "hidden"
                                | "hidden_long_help"
                                | "hidden_short_help"
                                | "multiple"
                                | "required_if"
                                | "required_ifs"
                                | "required_unless"
                                | "required_unless_all"
                                | "required_unless_one"
                                | "set"
                                | "setting"
                                | "settings"
                                | "validator_regex"
                                | "with_name"
                                | "allow_invalid_utf8"
                                | "forbid_empty_values"
                                | "max_occurrences"
                                | "max_values"
                                | "min_values"
                                | "multiple_occurrences"
                                | "multiple_values"
                                | "possible_value"
                                | "possible_values"
                                | "require_value_delimiter"
                                | "takes_value") => {
                                    return Err(Error::custom(::core::fmt::Arguments::new_v1(
                                        &["deprecated key: "],
                                        &[::core::fmt::ArgumentV1::new_display(&depr)],
                                    )))
                                }
                                #[cfg(feature = "allow-deprecated")]
                                "name" => {
                                    const N_KEY: &str = "\"id\"";
                                    {
                                        key = N_KEY;
                                    };
                                    continue 'parse_value_jmp_loop;
                                }
                                #[cfg(feature = "allow-deprecated")]
                                "require_delimiter" => {
                                    const N_KEY: &str = "\"require_value_delimiter\"";
                                    {
                                        key = N_KEY;
                                    };
                                    continue 'parse_value_jmp_loop;
                                }
                                #[cfg(feature = "allow-deprecated")]
                                "use_delimiter" => {
                                    const N_KEY: &str = "\"use_value_delimiter\"";
                                    {
                                        key = N_KEY;
                                    };
                                    continue 'parse_value_jmp_loop;
                                }
                                unknown => {
                                    return Err(Error::unknown_field(
                                        unknown,
                                        &[
                                            "alias",
                                            "aliases",
                                            "allow_hyphen_values",
                                            "conflicts_with",
                                            "conflicts_with_all",
                                            "default_missing_value",
                                            "default_missing_values",
                                            "default_value",
                                            "default_value_ifs",
                                            "display_order",
                                            "exclusive",
                                            "global",
                                            "group",
                                            "groups",
                                            "help",
                                            "help_heading",
                                            "hide",
                                            "hide_default_value",
                                            "hide_long_help",
                                            "hide_possible_values",
                                            "hide_short_help",
                                            "id",
                                            "ignore_case",
                                            "index",
                                            "last",
                                            "long",
                                            "long_help",
                                            "id",
                                            "next_line_help",
                                            "number_of_values",
                                            "overrides_with",
                                            "overrides_with_all",
                                            "require_equals",
                                            "raw",
                                            "required",
                                            "required_if_eq_all",
                                            "required_if_eq_any",
                                            "required_unless_present",
                                            "required_unless_present_any",
                                            "required_unless_present_all",
                                            "requires",
                                            "requires_all",
                                            "requires_ifs",
                                            "short",
                                            "short_alias",
                                            "short_aliases",
                                            "use_value_delimiter",
                                            "value_delimiter",
                                            "value_name",
                                            "value_names",
                                            "value_terminator",
                                            "visible_alias",
                                            "visible_aliases",
                                            "visible_short_alias",
                                            "visible_short_aliases",
                                            "\"arg_action\"",
                                            "\"env\"",
                                            "\"hide_env\"",
                                            "\"hide_env_values\"",
                                            "\"value_hint\"",
                                            "\"value_parser\"",
                                        ],
                                    ))
                                }
                            };
                        }
                    };
                }
                Ok(ArgWrap { arg })
            }
        }
        impl<'de> DeserializeSeed<'de> for ArgVisitor {
            type Value = ArgWrap;
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_map(self)
            }
        }
        impl<'de> DeserializeSeed<'de> for ArgWrap {
            type Value = ArgWrap;
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_map(ArgVisitor(self.arg))
            }
        }
        pub(crate) struct Args<const USE_ARRAY: bool>(pub(crate) Command);
        impl<'de, const USE_ARRAY: bool> DeserializeSeed<'de> for Args<USE_ARRAY> {
            type Value = Command;
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                if USE_ARRAY {
                    deserializer.deserialize_seq(self)
                } else {
                    deserializer.deserialize_map(self)
                }
            }
        }
        impl<'de, const USE_ARRAY: bool> Visitor<'de> for Args<USE_ARRAY> {
            type Value = Command;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("args")
            }
            #[cfg(not(feature = "override-arg"))]
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let x = ArgKV;
                let mut com = self.0;
                while let Some(a) = seq.next_element_seed(x)? {
                    com = com.arg(a);
                }
                Ok(com)
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut app = self.0;
                while let Some(name) = map.next_key::<&str>()? {
                    #[cfg(not(feature = "override-arg"))]
                    {
                        app = app.arg(map.next_value_seed(ArgVisitor::new_str(name))?);
                    }
                }
                Ok(app)
            }
        }
    }
    mod group {
        use crate::ArgGroupWrap;
        use clap::{ArgGroup, Command};
        use serde::de::{DeserializeSeed, Error, Visitor};
        struct GroupVisitor<'de>(&'de str);
        impl<'de> Visitor<'de> for GroupVisitor<'de> {
            type Value = ArgGroupWrap;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("arg group map")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut group = ArgGroup::new(self.0);
                while let Some(key) = map.next_key::<&str>()? {
                    group = {
                        #[allow(unused_mut)]
                        let mut key;
                        {
                            key = key;
                        };
                        #[allow(unused_labels)]
                        'parse_value_jmp_loop: loop {
                            break 'parse_value_jmp_loop match key {
                                "arg" => <ArgGroup>::arg(group, map.next_value::<&str>()?),
                                "args" => <ArgGroup>::args(group, &map.next_value::<Vec<&str>>()?),
                                "conflicts_with" => {
                                    <ArgGroup>::conflicts_with(group, map.next_value::<&str>()?)
                                }
                                "conflicts_with_all" => <ArgGroup>::conflicts_with_all(
                                    group,
                                    &map.next_value::<Vec<&str>>()?,
                                ),
                                "id" => <ArgGroup>::id(group, map.next_value::<&str>()?),
                                "multiple" => {
                                    <ArgGroup>::multiple(group, map.next_value::<bool>()?)
                                }
                                "required" => {
                                    <ArgGroup>::required(group, map.next_value::<bool>()?)
                                }
                                "requires" => {
                                    <ArgGroup>::requires(group, map.next_value::<&str>()?)
                                }
                                "requires_all" => {
                                    <ArgGroup>::requires_all(group, &map.next_value::<Vec<&str>>()?)
                                }
                                #[cfg(feature = "allow-deprecated")]
                                "name" => {
                                    const N_KEY: &str = "\"id\"";
                                    {
                                        key = N_KEY;
                                    };
                                    continue 'parse_value_jmp_loop;
                                }
                                unknown => {
                                    return Err(Error::unknown_field(
                                        unknown,
                                        &[
                                            "arg",
                                            "args",
                                            "conflicts_with",
                                            "conflicts_with_all",
                                            "id",
                                            "multiple",
                                            "required",
                                            "requires",
                                            "requires_all",
                                        ],
                                    ))
                                }
                            };
                        }
                    };
                }
                Ok(ArgGroupWrap { group })
            }
        }
        impl<'de> DeserializeSeed<'de> for GroupVisitor<'de> {
            type Value = ArgGroupWrap;
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_map(self)
            }
        }
        pub(crate) struct Groups(pub(crate) Command);
        impl<'de> DeserializeSeed<'de> for Groups {
            type Value = Command;
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_map(self)
            }
        }
        impl<'de> Visitor<'de> for Groups {
            type Value = Command;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("arg groups")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut app = self.0;
                while let Some(name) = map.next_key::<&str>()? {
                    app = app.group(map.next_value_seed(GroupVisitor(name))?);
                }
                Ok(app)
            }
        }
    }
}
///
///Deserialize [`Command`] from [`Deserializer`].
///```
///const CLAP_TOML: &'static str = r#"
///name = "app_clap_serde"
///version = "1.0"
///author = "tester"
///about = "test-clap-serde"
///"#;
///let app = clap_serde::load(&mut toml::Deserializer::new(CLAP_TOML))
///    .expect("parse failed");
///assert_eq!(app.get_name(), "app_clap_serde");
///assert_eq!(app.get_about(), Some("test-clap-serde"));
///```
pub fn load<'de, D>(de: D) -> Result<Command, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::Deserialize;
    CommandWrap::deserialize(de).map(|a| a.into())
}
///
///Wrapper of [`Command`] to deserialize.
///```
///const CLAP_TOML: &'static str = r#"
///name = "app_clap_serde"
///version = "1.0"
///author = "tester"
///about = "test-clap-serde"
///"#;
///let app: clap::Command = toml::from_str::<clap_serde::CommandWrap>(CLAP_TOML)
///    .expect("parse failed")
///    .into();
///assert_eq!(app.get_name(), "app_clap_serde");
///assert_eq!(app.get_about(), Some("test-clap-serde"));
///```
pub struct CommandWrap {
    app: Command,
}
#[automatically_derived]
impl ::core::fmt::Debug for CommandWrap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "CommandWrap", "app", &&self.app)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for CommandWrap {
    #[inline]
    fn clone(&self) -> CommandWrap {
        CommandWrap {
            app: ::core::clone::Clone::clone(&self.app),
        }
    }
}
#[deprecated]
pub type AppWrap = CommandWrap;
impl From<CommandWrap> for Command {
    fn from(a: CommandWrap) -> Self {
        a.app
    }
}
impl From<Command> for CommandWrap {
    fn from(app: Command) -> Self {
        CommandWrap { app }
    }
}
impl Deref for CommandWrap {
    type Target = Command;
    fn deref(&self) -> &Self::Target {
        &self.app
    }
}
/// Wrapper of [`Arg`] to deserialize with [`DeserializeSeed`](`serde::de::DeserializeSeed`).
pub struct ArgWrap {
    arg: Arg,
}
#[automatically_derived]
impl ::core::fmt::Debug for ArgWrap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "ArgWrap", "arg", &&self.arg)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for ArgWrap {
    #[inline]
    fn clone(&self) -> ArgWrap {
        ArgWrap {
            arg: ::core::clone::Clone::clone(&self.arg),
        }
    }
}
impl From<ArgWrap> for Arg {
    fn from(arg: ArgWrap) -> Self {
        arg.arg
    }
}
impl From<Arg> for ArgWrap {
    fn from(arg: Arg) -> Self {
        ArgWrap { arg }
    }
}
impl Deref for ArgWrap {
    type Target = Arg;
    fn deref(&self) -> &Self::Target {
        &self.arg
    }
}
pub(crate) struct ArgGroupWrap {
    group: ArgGroup,
}
impl From<ArgGroupWrap> for ArgGroup {
    fn from(group: ArgGroupWrap) -> Self {
        group.group
    }
}
impl From<ArgGroup> for ArgGroupWrap {
    fn from(group: ArgGroup) -> Self {
        ArgGroupWrap { group }
    }
}
