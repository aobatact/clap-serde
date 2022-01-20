use crate::ArgGroupWrap;
use clap::ArgGroup;
use serde::de::{Error, Visitor};

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
                (bool, required),
                (bool, multiple),
                (&str, arg),
                (ref Vec<&str>, args),
                (&str, requires),
                (&str, name),
            });
        }

        Ok(ArgGroupWrap { group })
    }
}
