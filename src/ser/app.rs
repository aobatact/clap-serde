use crate::CommandWrap;
use serde::ser::SerializeMap;
use serde::Serialize;

impl<'de> Serialize for CommandWrap<'de> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let command = &self.app;
        let r = ser_value!(command, serializer, [
            (name, get_name),
            ref [
                #[cfg(feature = "color")]
                {crate::de::app::color::to_ser} (color, get_color),
                (no_binary_name_set, is_no_binary_name_set),
                (dont_delimit_trailing_values_set,is_dont_delimit_trailing_values_set),
                (disable_version_flag_set,is_disable_version_flag_set),
                (propagate_version_set,is_propagate_version_set),
                (next_line_help_set,is_next_line_help_set),
                (disable_help_flag_set,is_disable_help_flag_set),
                (disable_help_subcommand_set,is_disable_help_subcommand_set),
                (disable_colored_help_set,is_disable_colored_help_set),
                (dont_collapse_args_in_usage_set,is_dont_collapse_args_in_usage_set),
                (arg_required_else_help_set,is_arg_required_else_help_set),
                (allow_negative_numbers_set,is_allow_negative_numbers_set),
                (trailing_var_arg_set,is_trailing_var_arg_set),
                (allow_missing_positional_set,is_allow_missing_positional_set),
                (hide_set,is_hide_set),
                (subcommand_required_set,is_subcommand_required_set),
                (allow_external_subcommands_set,is_allow_external_subcommands_set),
                (allow_invalid_utf8_for_external_subcommands_set,is_allow_invalid_utf8_for_external_subcommands_set),
                (args_conflicts_with_subcommands_set,is_args_conflicts_with_subcommands_set),
                (subcommand_precedence_over_arg_set,is_subcommand_precedence_over_arg_set),
                (subcommand_negates_reqs_set,is_subcommand_negates_reqs_set),
                (multicall_set,is_multicall_set)
            ],
            opt [ 
                (display_name, get_display_name),
                (bin_name, get_bin_name),
                (version, get_version),
                (long_version, get_long_version),
                (author, get_author),
                (long_flag, get_long_flag),
                [&] (short_flag, get_short_flag),
                (about, get_about),
                (long_about, get_long_about),
                (next_help_heading, get_next_help_heading),
                // visible alias
                // subcommands
                (subcommand_help_heading, get_subcommand_help_heading),
                (subcommand_value_name, get_subcommand_value_name),
                (before_help, get_before_help),
                (before_long_help, get_before_long_help),
                (after_help, get_after_help),
                (after_long_help, get_after_long_help),
            ]
            //groups
            //args

        ]);
        r
    }
}
