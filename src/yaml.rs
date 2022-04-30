use serde::{
    de::{
        value::{MapDeserializer, SeqDeserializer},
        Error as _, IntoDeserializer, Unexpected,
    },
    Deserializer,
};
use yaml_rust::Yaml;

/**
Deserializing from [`Yaml`]
```
const YAML_STR: &'static str = r#"
name: app_clap_serde
version : "1.0"
about : yaml_support!
author : yaml_supporter

args:
    - apple :
        - short: a
    - banana:
        - short: b
        - long: banana
        - aliases :
            - musa_spp

subcommands:
    - sub1:
        - about : subcommand_1
    - sub2:
        - about : subcommand_2

"#;
let yaml = yaml_rust::Yaml::Array(yaml_rust::YamlLoader::load_from_str(YAML_STR).expect("not a yaml"));
let app = clap_serde::yaml_to_app(&yaml).expect("parse failed from yaml");
assert_eq!(app.get_name(), "app_clap_serde");
```
*/
pub fn yaml_to_app<'a>(yaml: &'a Yaml) -> Result<clap::Command<'a>, Error> {
    let wrap = YamlWrap { yaml };
    use serde::Deserialize;
    crate::CommandWrap::deserialize(wrap).map(|x| x.into())
}

/// Wrapper to use [`Yaml`] as [`Deserializer`].
///
/// Currently this implement functions in [`Deserializer`] that is only needed in deserializing into `Command`.
/// Recommend to use [`yaml_to_app`] instead.
pub struct YamlWrap<'a> {
    yaml: &'a yaml_rust::Yaml,
}

impl<'a> YamlWrap<'a> {
    pub fn new(yaml: &'a yaml_rust::Yaml) -> Self {
        Self { yaml }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Custom(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

macro_rules! de_num {
    ($sig : ident, $sig_v : ident) => {
        fn $sig<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            visitor.$sig_v(match self.yaml.as_i64().map(|i| i.try_into()) {
                Some(Ok(i)) => i,
                _ => return Err(as_invalid(self.yaml, "Intger")),
            })
        }
    };
}

fn as_invalid(y: &Yaml, expected: &str) -> Error {
    Error::invalid_type(
        match y {
            Yaml::Real(r) => r
                .parse()
                .map(|r| Unexpected::Float(r))
                .unwrap_or(Unexpected::Other(r)),
            Yaml::Integer(i) => Unexpected::Signed(*i),
            Yaml::String(s) => Unexpected::Str(s),
            Yaml::Boolean(b) => Unexpected::Bool(*b),
            Yaml::Array(_) => Unexpected::Seq,
            Yaml::Hash(_) => Unexpected::Map,
            Yaml::Alias(_) => todo!(),
            Yaml::Null => Unexpected::Unit,
            Yaml::BadValue => Unexpected::Other("BadValue"),
        },
        &expected,
    )
}

impl<'de> IntoDeserializer<'de, Error> for YamlWrap<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> Deserializer<'de> for YamlWrap<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.yaml {
            yaml_rust::Yaml::Real(s) => {
                visitor.visit_f64(s.parse::<f64>().map_err(|e| Error::Custom(e.to_string()))?)
            }
            yaml_rust::Yaml::Integer(i) => visitor.visit_i64(*i),
            yaml_rust::Yaml::String(s) => visitor.visit_str(s),
            yaml_rust::Yaml::Boolean(b) => visitor.visit_bool(*b),
            yaml_rust::Yaml::Array(_) => self.deserialize_seq(visitor), //visitor.visit_seq(a),
            yaml_rust::Yaml::Hash(_) => self.deserialize_map(visitor),
            yaml_rust::Yaml::Alias(_) => todo!(),
            yaml_rust::Yaml::Null => visitor.visit_none(),
            yaml_rust::Yaml::BadValue => return Err(as_invalid(self.yaml, "any")),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(self.yaml.as_bool().ok_or(as_invalid(self.yaml, "bool"))?)
    }

    de_num!(deserialize_i8, visit_i8);
    de_num!(deserialize_i16, visit_i16);
    de_num!(deserialize_i32, visit_i32);
    de_num!(deserialize_i64, visit_i64);
    de_num!(deserialize_u8, visit_i8);
    de_num!(deserialize_u16, visit_u16);
    de_num!(deserialize_u32, visit_u32);
    de_num!(deserialize_u64, visit_u64);

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f32(self.yaml.as_f64().ok_or(as_invalid(self.yaml, "f32"))? as f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f64(self.yaml.as_f64().ok_or(as_invalid(self.yaml, "f64"))?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_char(
            self.yaml
                .as_str()
                .ok_or(as_invalid(self.yaml, "char"))?
                .chars()
                .next()
                .ok_or(as_invalid(self.yaml, "char"))?,
        )
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let s = self.yaml.as_str();
        visitor.visit_borrowed_str(s.ok_or_else(|| as_invalid(self.yaml, "str"))?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(
            self.yaml
                .as_str()
                .ok_or(as_invalid(self.yaml, "string"))?
                .to_string(),
        )
    }

    ///not supported
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    ///not supported
    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if matches!(self.yaml, yaml_rust::Yaml::Null) {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if matches!(self.yaml, yaml_rust::Yaml::Null) {
            visitor.visit_unit()
        } else {
            Err(as_invalid(self.yaml, "unit"))
        }
    }

    ///unimplemented
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    ///unimplemented
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    ///unimplemented
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Some(n) = self.yaml.as_vec() {
            let seq = SeqDeserializer::new(n.iter().map(|y| YamlWrap { yaml: y }));
            visitor.visit_seq(seq)
        } else {
            Err(as_invalid(self.yaml, "seq"))
        }
    }

    ///unimplemented
    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    ///unimplemented
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.yaml {
            Yaml::Hash(h) => {
                let m = MapDeserializer::new(
                    h.iter()
                        .map(|(k, v)| (YamlWrap { yaml: k }, YamlWrap { yaml: v })),
                );
                visitor.visit_map(m)
            }
            Yaml::Array(a) => {
                let x = a
                    .iter()
                    .map(|y| y.as_hash().ok_or_else(|| as_invalid(self.yaml, "map")))
                    .collect::<Result<Vec<_>, _>>()?;
                let m = MapDeserializer::new(
                    x.into_iter()
                        .map(|x| x.iter())
                        .flatten()
                        .map(|(k, v)| (YamlWrap { yaml: k }, YamlWrap { yaml: v })),
                );
                visitor.visit_map(m)
            }
            _ => Err(as_invalid(self.yaml, "map")),
        }
    }

    ///unimplemented
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Some(s) = self.yaml.as_str() {
            visitor.visit_enum(s.into_deserializer())
        } else {
            Err(as_invalid(self.yaml, "enum"))
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}
