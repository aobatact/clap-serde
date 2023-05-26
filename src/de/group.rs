use crate::ArgGroupWrap;
use clap::{ArgGroup, Command};
use serde::de::{DeserializeSeed, Error, Visitor};

struct GroupVisitor(String);

impl<'de> Visitor<'de> for GroupVisitor {
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
            group = parse_value!(key, group, map, ArgGroup, {
                (arg, String),
                ref (args, Vec<String>),
                (conflicts_with, String),
                ref (conflicts_with_all, Vec<String>),
                (id, String),
                (multiple, bool),
                (required, bool),
                (requires, String),
                ref (requires_all, Vec<String>),
            }, deprecated:{
                "name" => "id",
            });
        }

        Ok(ArgGroupWrap { group })
    }
}

impl<'de> DeserializeSeed<'de> for GroupVisitor {
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
        while let Some(name) = map.next_key::<String>()? {
            app = app.group(map.next_value_seed(GroupVisitor(name))?);
        }
        Ok(app)
    }
}
