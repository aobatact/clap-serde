use super::{Entries, SerializeConfig};
use clap::{ArgGroup, Command};
use serde::{Serialize, Serializer};

pub(crate) struct GroupsWrap<'a, C> {
    command: &'a Command,
    config: C,
}

impl<'a, C> GroupsWrap<'a, C> {
    pub(crate) fn new(command: &'a Command, config: C) -> Self {
        Self { command, config }
    }
}

impl<C: SerializeConfig> Serialize for GroupsWrap<'_, C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(
            self.command
                .get_groups()
                .map(|g| (g.get_id().as_str(), GroupProps(g, self.config.clone()))),
        )
    }
}

struct GroupProps<'a, C>(&'a ArgGroup, C);

impl<C: SerializeConfig> Serialize for GroupProps<'_, C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let g = self.0;
        let mut m = Entries::new(serializer, self.1.serialize_all())?;
        m.seq("args", g.get_args().map(|a| a.as_str()))?;
        // `is_multiple` requires `&mut self`
        m.flag("multiple", g.clone().is_multiple())?;
        m.flag("required", g.is_required_set())?;
        m.end()
    }
}
