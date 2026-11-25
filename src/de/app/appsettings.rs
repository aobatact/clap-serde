//! Compatibility layer for `setting` / `settings` / `global_setting` / `global_settings` keys.
//!
//! clap 4 removed `AppSettings`; each former setting is now a builder method taking `bool`.
//! The setting names of clap 3 are mapped to these methods.
use clap::Command;
use serde::{de::Visitor, Deserialize};

/// Apply the setting named `name` (already converted to snake_case).
/// Returns `None` if the name is unknown.
fn apply_setting(app: Command, name: &str) -> Option<Command> {
    macro_rules! settings {
        ($($name:ident),* $(,)?) => {
            match name {
                $(stringify!($name) => return Some(app.$name(true)),)*
                _ => {}
            }
        };
    }
    settings!(
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
    Some(match name {
        // renamed in clap 3.1 / 4
        "hidden" => app.hide(true),
        "all_args_override_self" => app.args_override_self(true),
        "subcommands_negate_reqs" => app.subcommand_negates_reqs(true),
        "args_negate_subcommands" => app.args_conflicts_with_subcommands(true),
        "subcommand_required_else_help" => {
            app.subcommand_required(true).arg_required_else_help(true)
        }
        // removed in clap 4 (the behavior is the default or no longer exists)
        "wait_on_error"
        | "derive_display_order"
        | "use_long_format_for_help_subcommand"
        | "allow_invalid_utf8_for_external_subcommands"
        | "allow_invalid_utf_8_for_external_subcommands"
        | "no_auto_help"
        | "no_auto_version" => app,
        _ => return None,
    })
}

/// Names shown in the error of unknown settings.
const SETTING_NAMES: &[&str] = &[
    "ignore_errors",
    "allow_hyphen_values",
    "allow_negative_numbers",
    "args_override_self",
    "allow_missing_positional",
    "trailing_var_arg",
    "dont_delimit_trailing_values",
    "infer_long_args",
    "infer_subcommands",
    "subcommand_required",
    "allow_external_subcommands",
    "multicall",
    "subcommand_negates_reqs",
    "args_conflicts_with_subcommands",
    "subcommand_precedence_over_arg",
    "arg_required_else_help",
    "next_line_help",
    "disable_colored_help",
    "disable_help_flag",
    "disable_help_subcommand",
    "disable_version_flag",
    "propagate_version",
    "hide",
    "hide_possible_values",
    "help_expected",
    "no_binary_name",
];

/// One or more setting names.
#[derive(Default)]
pub(crate) struct SettingNames(Vec<String>);

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

impl SettingNames {
    /// Convert the names to snake_case and check that all of them are known.
    pub(crate) fn validate<E: serde::de::Error>(self) -> Result<Self, E> {
        let names = self
            .0
            .into_iter()
            .map(|name| {
                let key: String = convert_case_to!(name);
                if apply_setting(Command::new(""), &key).is_some() {
                    Ok(key)
                } else {
                    Err(E::unknown_variant(&name, SETTING_NAMES))
                }
            })
            .collect::<Result<_, _>>()?;
        Ok(SettingNames(names))
    }

    pub(crate) fn extend(&mut self, other: SettingNames) {
        self.0.extend(other.0);
    }

    /// Apply the settings (validated by [`Self::validate`]) to the command.
    pub(crate) fn apply(&self, mut app: Command) -> Command {
        for name in &self.0 {
            app = apply_setting(app, name).expect("validated setting name");
        }
        app
    }

    /// Apply the settings to the command and all of its subcommands recursively,
    /// like the global settings in clap 3.
    pub(crate) fn apply_global(&self, app: Command) -> Command {
        self.apply(app).mut_subcommands(|s| self.apply_global(s))
    }
}
