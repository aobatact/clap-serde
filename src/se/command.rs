use crate::CommandWrap;
use serde::{Serialize, ser::SerializeMap};


impl<'se> Serialize for CommandWrap<'se> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let app = &self.app;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("name", app.get_name())?;
        serialize_keys!(app, map, get: {
            get_about,
            get_bin_name,
            //get_color
            get_long_about,
            get_long_flag,
            get_short_flag,
        });
        map.end()
    }
}
