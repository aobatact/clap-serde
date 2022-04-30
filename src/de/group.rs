use crate::ArgGroupWrap;
use clap::{Command, ArgGroup};
use serde::de::{DeserializeSeed, Error, Visitor};

struct GroupVisitor<'a>(&'a str);

impl<'de> Visitor<'de> for GroupVisitor<'de> {
    type Value = ArgGroupWrap<'de>;

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
                (arg, &str),
                ref (args, Vec<&str>),
                (conflicts_with, &str),
                ref (conflicts_with_all, Vec<&str>),
                (multiple, bool),
                (name, &str),
                (required, bool),
                (requires, &str),
                ref (requires_all, Vec<&str>),
            });
        }

        Ok(ArgGroupWrap { group })
    }
}

impl<'de> DeserializeSeed<'de> for GroupVisitor<'de> {
    type Value = ArgGroupWrap<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

pub(crate) struct Groups<'a>(pub(crate) Command<'a>);
impl<'de> DeserializeSeed<'de> for Groups<'de> {
    type Value = Command<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de> Visitor<'de> for Groups<'de> {
    type Value = Command<'de>;

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
