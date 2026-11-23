//! Compatibility layer for `setting` / `settings` keys.
//!
//! clap 4 removed `AppSettings`; each former setting is now a builder method taking `bool`.
//! The keys are mapped to these methods so that configurations written for clap 3 keep working.
use clap::Command;
use serde::{de::Visitor, Deserialize};

macro_rules! app_settings {
    ($($name:ident),* $(,)?) => {
        /// Names of the settings which can be used in `setting` / `settings`.
        pub(crate) const SETTING_NAMES: &[&str] = &[$(stringify!($name)),*];

        /// Apply the setting named `name` (already converted to snake_case).
        pub(crate) fn apply_setting(app: Command, name: &str) -> Option<Command> {
            Some(match name {
                $(stringify!($name) => app.$name(true),)*
                _ => return None,
            })
        }
    };
}

app_settings!(
    ignore_errors,
    allow_hyphen_values,
    allow_negative_numbers,
    args_override_self,
    allow_missing_positional,
    trailing_var_arg,
    dont_delimit_trailing_values,
    infer_long_args,
    infer_subcommands,
    subcommand_required,
    allow_external_subcommands,
    multicall,
    subcommand_negates_reqs,
    args_conflicts_with_subcommands,
    subcommand_precedence_over_arg,
    arg_required_else_help,
    dont_collapse_args_in_usage,
    next_line_help,
    disable_colored_help,
    disable_help_flag,
    disable_help_subcommand,
    disable_version_flag,
    propagate_version,
    hide,
    hide_possible_values,
    help_expected,
    no_binary_name,
);

/// One or more setting names.
pub(crate) struct SettingNames(pub(crate) Vec<String>);

impl<'de> Deserialize<'de> for SettingNames {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = SettingNames;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("setting name or array of setting names")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(SettingNames(vec![v.to_owned()]))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut v = Vec::new();
                while let Some(s) = seq.next_element::<String>()? {
                    v.push(s);
                }
                Ok(SettingNames(v))
            }
        }
        deserializer.deserialize_any(V)
    }
}

pub(crate) fn apply_settings<E: serde::de::Error>(
    mut app: Command,
    names: SettingNames,
) -> Result<Command, E> {
    for name in names.0 {
        let key: String = convert_case_to!(name);
        app = apply_setting(app, &key).ok_or_else(|| E::unknown_variant(&name, SETTING_NAMES))?;
    }
    Ok(app)
}
