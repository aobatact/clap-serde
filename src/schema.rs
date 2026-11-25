//! JSON Schema of the input of [`CommandWrap`](crate::CommandWrap).
use serde_json::{json, Map, Value};

/// Returns the [JSON Schema](https://json-schema.org/) (draft 2020-12) of the input of
/// [`CommandWrap`](crate::CommandWrap).
///
/// The keys and enum values follow the case-key feature, and the keys of the optional features
/// (`env`, `color`, `help`, `allow-deprecated`) are included only when the features are enabled.
/// Unknown keys are not allowed, as the deserializer rejects them.
///
/// The schema for the `snake-case-key` feature with all the optional features is also available
/// as `schema/clap-serde.schema.json` in the repository.
///
/// ```
/// let schema = clap_serde::json_schema();
/// assert_eq!(schema["$schema"], "https://json-schema.org/draft/2020-12/schema");
/// ```
pub fn json_schema() -> Value {
    let mut defs = Map::new();
    defs.insert("command".into(), command());
    defs.insert("arg".into(), arg());
    defs.insert("arg_group".into(), arg_group());
    defs.insert("args".into(), entries("arg", true));
    defs.insert("args_map".into(), entries("arg", false));
    defs.insert("subcommands".into(), entries("command", true));
    defs.insert("subcommands_map".into(), entries("command", false));
    defs.insert("groups".into(), entries("arg_group", false));
    defs.insert(
        "char".into(),
        json!({ "type": "string", "minLength": 1, "maxLength": 1 }),
    );
    defs.insert("usize".into(), json!({ "type": "integer", "minimum": 0 }));
    defs.insert("string_pair".into(), tuple(&[str_ty(), str_ty()]));
    defs.insert(
        "default_value_if".into(),
        tuple(&[str_ty(), nullable(str_ty()), nullable(str_ty())]),
    );
    defs.insert(
        "num_args".into(),
        json!({
            "anyOf": [
                { "$ref": "#/$defs/usize" },
                {
                    "type": "object",
                    "properties": {
                        "min": { "$ref": "#/$defs/usize" },
                        "max": { "$ref": "#/$defs/usize" },
                    },
                    "additionalProperties": false,
                },
            ],
        }),
    );
    defs.insert("arg_action".into(), enum_values(ARG_ACTIONS));
    defs.insert("value_hint".into(), enum_values(VALUE_HINTS));
    defs.insert("value_parser".into(), value_parser());
    #[cfg(feature = "color")]
    defs.insert(
        "color_choice".into(),
        enum_values(&["auto", "always", "never"]),
    );
    #[cfg(feature = "allow-deprecated")]
    defs.insert("settings".into(), {
        let names = enum_values(&crate::de::app::appsettings::all_setting_names());
        json!({ "anyOf": [names, { "type": "array", "items": names }] })
    });

    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "clap-serde Command",
        "description": "Definition of a clap Command for clap-serde",
        "$ref": "#/$defs/command",
        "required": [key("name")],
        "$defs": defs,
    })
}

const ARG_ACTIONS: &[&str] = &[
    "set",
    "append",
    "set_true",
    "set_false",
    "count",
    "help",
    "help_short",
    "help_long",
    "version",
];

const VALUE_HINTS: &[&str] = &[
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

const VALUE_PARSERS: &[&str] = &[
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

/// Convert the snake_case name to the case of the case-key feature.
#[cfg(any(
    feature = "snake-case-key",
    not(any(feature = "kebab-case-key", feature = "pascal-case-key"))
))]
pub(crate) fn key(s: &str) -> String {
    s.to_string()
}

#[cfg(all(not(feature = "snake-case-key"), feature = "kebab-case-key"))]
pub(crate) fn key(s: &str) -> String {
    s.replace('_', "-")
}

#[cfg(all(
    not(feature = "snake-case-key"),
    not(feature = "kebab-case-key"),
    feature = "pascal-case-key"
))]
pub(crate) fn key(s: &str) -> String {
    s.split('_')
        .map(|w| {
            let mut c = w.chars();
            c.next()
                .map(|f| f.to_ascii_uppercase().to_string() + c.as_str())
                .unwrap_or_default()
        })
        .collect()
}

fn str_ty() -> Value {
    json!({ "type": "string" })
}

fn reference(name: &str) -> Value {
    json!({ "$ref": format!("#/$defs/{name}") })
}

fn array(items: Value) -> Value {
    json!({ "type": "array", "items": items })
}

fn tuple(items: &[Value]) -> Value {
    json!({
        "type": "array",
        "prefixItems": items,
        "items": false,
        "minItems": items.len(),
    })
}

fn nullable(v: Value) -> Value {
    json!({ "anyOf": [v, { "type": "null" }] })
}

fn enum_values(names: &[&str]) -> Value {
    json!({ "enum": names.iter().map(|n| key(n)).collect::<Vec<_>>() })
}

/// `args`, `subcommands`, etc. : a sequence of single-entry maps (if `seq`) or a map.
fn entries(def: &str, seq: bool) -> Value {
    let map = json!({ "type": "object", "additionalProperties": reference(def) });
    if seq {
        let single = json!({
            "type": "object",
            "additionalProperties": reference(def),
            "minProperties": 1,
            "maxProperties": 1,
        });
        json!({ "anyOf": [array(single), map] })
    } else {
        map
    }
}

fn value_parser() -> Value {
    json!({
        "anyOf": [
            enum_values(VALUE_PARSERS),
            {
                "type": "object",
                "properties": {
                    "type": enum_values(VALUE_PARSERS),
                    "min": { "type": "integer" },
                    "max": { "type": "integer" },
                    "max_inclusive": { "type": "boolean", "default": true },
                },
                "required": ["type"],
                "additionalProperties": false,
            },
        ],
    })
}

/// Remove the links (`[text](url)` to `text`) and the backquotes from the markdown.
fn plain_text(md: &str) -> String {
    let mut out = String::new();
    let mut rest = md;
    while let Some(i) = rest.find('[') {
        out.push_str(&rest[..i]);
        let r = &rest[i + 1..];
        match r
            .find("](")
            .and_then(|j| r[j..].find(')').map(|k| (j, j + k)))
        {
            Some((j, k)) => {
                out.push_str(&r[..j]);
                rest = &r[k + 1..];
            }
            None => {
                out.push('[');
                rest = r;
            }
        }
    }
    out.push_str(rest);
    out.replace('`', "")
}

/// Builder of the properties of an object.
struct Props {
    /// `Command`, `Arg` or `ArgGroup`, for the links to the clap docs.
    clap_type: &'static str,
    props: Map<String, Value>,
}

impl Props {
    fn new(clap_type: &'static str) -> Self {
        Self {
            clap_type,
            props: Map::new(),
        }
    }

    fn doc_link(&self, method: &str) -> String {
        format!(
            "https://docs.rs/clap/4.6/clap/struct.{}.html#method.{method}",
            self.clap_type
        )
    }

    /// Add a key which calls the method of the same name.
    /// `null` is allowed as it is ignored.
    fn method(&mut self, name: &str, ty: Value) -> &mut Self {
        let desc = format!("[`{}::{name}`]({})", self.clap_type, self.doc_link(name));
        self.raw(name, nullable(ty), desc)
    }

    fn methods(&mut self, names: &[&str], ty: Value) -> &mut Self {
        for name in names {
            self.method(name, ty.clone());
        }
        self
    }

    /// Add a key with the schema and the markdown description.
    fn raw(&mut self, name: &str, mut ty: Value, desc: impl Into<String>) -> &mut Self {
        let desc = desc.into();
        let obj = ty.as_object_mut().expect("schema object");
        obj.insert("description".into(), plain_text(&desc).into());
        obj.insert("markdownDescription".into(), desc.into());
        self.props.insert(key(name), ty);
        self
    }

    /// Add a deprecated key which is converted to `to`.
    #[cfg(feature = "allow-deprecated")]
    fn deprecated(&mut self, name: &str, to: &str) -> &mut Self {
        let mut ty = self.props[&key(to)].clone();
        ty.as_object_mut()
            .expect("schema object")
            .insert("deprecated".into(), true.into());
        let desc = format!("Deprecated. Use `{}` instead.", key(to));
        self.raw(name, ty, desc)
    }

    fn build(&mut self) -> Value {
        json!({
            "type": "object",
            "properties": std::mem::take(&mut self.props),
            "additionalProperties": false,
        })
    }
}

fn command() -> Value {
    let mut p = Props::new("Command");
    p.methods(
        &[
            "about",
            "after_help",
            "after_long_help",
            "alias",
            "author",
            "before_help",
            "before_long_help",
            "bin_name",
            "display_name",
            "long_about",
            "long_flag",
            "long_flag_alias",
            "long_version",
            "name",
            "override_help",
            "override_usage",
            "subcommand_help_heading",
            "subcommand_value_name",
            "version",
            "visible_alias",
            "visible_long_flag_alias",
        ],
        str_ty(),
    )
    .methods(
        &[
            "aliases",
            "long_flag_aliases",
            "visible_aliases",
            "visible_long_flag_aliases",
        ],
        array(str_ty()),
    )
    .methods(
        &[
            "allow_external_subcommands",
            "allow_hyphen_values",
            "allow_missing_positional",
            "allow_negative_numbers",
            "arg_required_else_help",
            "args_conflicts_with_subcommands",
            "args_override_self",
            "disable_colored_help",
            "disable_help_flag",
            "disable_help_subcommand",
            "disable_version_flag",
            "dont_collapse_args_in_usage",
            "dont_delimit_trailing_values",
            "flatten_help",
            "help_expected",
            "hide",
            "hide_possible_values",
            "ignore_errors",
            "infer_long_args",
            "infer_subcommands",
            "multicall",
            "next_line_help",
            "no_binary_name",
            "propagate_version",
            "subcommand_negates_reqs",
            "subcommand_precedence_over_arg",
            "subcommand_required",
            "trailing_var_arg",
        ],
        json!({ "type": "boolean" }),
    )
    .methods(
        &["display_order", "max_term_width", "term_width"],
        reference("usize"),
    )
    .methods(
        &["short_flag", "short_flag_alias", "visible_short_flag_alias"],
        reference("char"),
    )
    .methods(
        &["short_flag_aliases", "visible_short_flag_aliases"],
        array(reference("char")),
    )
    .method("next_display_order", reference("usize"))
    .method("next_help_heading", str_ty())
    .raw(
        "args",
        reference("args"),
        "Args. A sequence of single-entry maps `[{ id: Arg }]` or a map `{ id: Arg }`.",
    )
    .raw(
        "args_map",
        reference("args_map"),
        "Args as a map `{ id: Arg }`.",
    )
    .raw(
        "subcommands",
        reference("subcommands"),
        "Subcommands. A sequence of single-entry maps `[{ name: Command }]` or a map `{ name: Command }`.",
    )
    .raw(
        "subcommands_map",
        reference("subcommands_map"),
        "Subcommands as a map `{ name: Command }`.",
    )
    .raw(
        "groups",
        reference("groups"),
        "ArgGroups as a map `{ id: ArgGroup }`.",
    );
    #[cfg(feature = "color")]
    p.method("color", reference("color_choice"));
    #[cfg(feature = "help")]
    p.method("help_template", str_ty());
    #[cfg(feature = "allow-deprecated")]
    {
        for (name, desc) in [
            ("setting", "clap 3 `AppSettings` (deprecated)."),
            ("settings", "clap 3 `AppSettings` (deprecated)."),
            (
                "global_setting",
                "clap 3 `AppSettings` applied to the subcommands too (deprecated).",
            ),
            (
                "global_settings",
                "clap 3 `AppSettings` applied to the subcommands too (deprecated).",
            ),
        ] {
            let mut ty = reference("settings");
            ty["deprecated"] = true.into();
            p.raw(name, ty, desc);
        }
        p.deprecated("help_heading", "next_help_heading");
    }
    p.build()
}

fn arg() -> Value {
    let mut p = Props::new("Arg");
    let required_if_eq = format!(
        "[`Arg::required_if_eq`]({}) : `[arg, value]`",
        p.doc_link("required_if_eq")
    );
    let requires_if = format!(
        "[`Arg::requires_if`]({}) : `[value, arg]`",
        p.doc_link("requires_if")
    );
    p.methods(
        &[
            "alias",
            "conflicts_with",
            "default_missing_value",
            "default_value",
            "group",
            "help",
            "id",
            "long",
            "long_help",
            "overrides_with",
            "required_unless_present",
            "requires",
            "value_name",
            "value_terminator",
            "visible_alias",
        ],
        str_ty(),
    )
    .methods(
        &[
            "aliases",
            "conflicts_with_all",
            "default_missing_values",
            "default_values",
            "groups",
            "overrides_with_all",
            "required_unless_present_all",
            "required_unless_present_any",
            "requires_all",
            "value_names",
            "visible_aliases",
        ],
        array(str_ty()),
    )
    .methods(
        &[
            "allow_hyphen_values",
            "allow_negative_numbers",
            "exclusive",
            "global",
            "hide",
            "hide_default_value",
            "hide_long_help",
            "hide_possible_values",
            "hide_short_help",
            "ignore_case",
            "last",
            "next_line_help",
            "raw",
            "require_equals",
            "required",
            "trailing_var_arg",
            "use_value_delimiter",
        ],
        json!({ "type": "boolean" }),
    )
    .methods(
        &["display_order", "index", "number_of_values"],
        reference("usize"),
    )
    .methods(
        &[
            "short",
            "short_alias",
            "value_delimiter",
            "visible_short_alias",
        ],
        reference("char"),
    )
    .methods(
        &["short_aliases", "visible_short_aliases"],
        array(reference("char")),
    )
    .methods(
        &["required_if_eq_all", "required_if_eq_any", "requires_ifs"],
        array(reference("string_pair")),
    )
    .raw("required_if_eq", reference("string_pair"), required_if_eq)
    .raw("requires_if", reference("string_pair"), requires_if)
    .method("action", reference("arg_action"))
    .method("default_value_if", reference("default_value_if"))
    .method("default_value_ifs", array(reference("default_value_if")))
    .method("help_heading", str_ty())
    .method("num_args", reference("num_args"))
    .method("value_hint", reference("value_hint"))
    .method("value_parser", reference("value_parser"))
    .raw(
        "possible_values",
        nullable(array(str_ty())),
        "Sets `PossibleValuesParser` as the value parser.",
    )
    .raw(
        "positional",
        nullable(json!({ "type": "boolean" })),
        "Ignored. Only output by serialization.",
    );
    #[cfg(feature = "env")]
    p.method("env", str_ty())
        .method("hide_env", json!({ "type": "boolean" }))
        .method("hide_env_values", json!({ "type": "boolean" }));
    #[cfg(feature = "allow-deprecated")]
    p.deprecated("name", "id")
        .deprecated("arg_action", "action")
        .deprecated("use_delimiter", "use_value_delimiter")
        .deprecated("require_delimiter", "use_value_delimiter");
    p.build()
}

fn arg_group() -> Value {
    let mut p = Props::new("ArgGroup");
    p.methods(&["arg", "conflicts_with", "id", "requires"], str_ty())
        .methods(
            &["args", "conflicts_with_all", "requires_all"],
            array(str_ty()),
        )
        .methods(&["multiple", "required"], json!({ "type": "boolean" }));
    #[cfg(feature = "allow-deprecated")]
    p.deprecated("name", "id");
    p.build()
}
