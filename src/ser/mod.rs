use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

use crate::util::case_key;

pub(crate) mod app;
pub(crate) mod arg;
pub(crate) mod group;

/// Config to serialize [`Command`](`clap::Command`) and [`Arg`](`clap::Arg`).
pub trait SerializeConfig: Clone {
    /// Serialize all the fields in [`Command`](`clap::Command`) and [`Arg`](`clap::Arg`).
    /// If this returns false, the flags (getter begin with `is_`) with `false`
    /// and values (getter begin with `get_`) with `None` will be skipped.
    fn serialize_all(&self) -> bool;
}

impl SerializeConfig for () {
    #[inline]
    fn serialize_all(&self) -> bool {
        false
    }
}

impl<S: SerializeConfig> SerializeConfig for &S {
    #[inline]
    fn serialize_all(&self) -> bool {
        (*self).serialize_all()
    }
}

/// Serialize all the fields in [`Command`](`clap::Command`) and [`Arg`](`clap::Arg`).
///
/// If not set, the flags (getter begin with `is_`) with `false`
/// and values (getter begin with `get_`) with `None` will be skipped.
/// When set, `None` values are serialized as `null` (unit),
/// which are ignored on deserialization.
/// ```
/// # use clap::Command;
/// # use clap_serde::*;
/// # let command = Command::new("cmd");
/// let wrap = CommandWrapRef::new(&command).with_config(NoSkip);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct NoSkip;
impl SerializeConfig for NoSkip {
    #[inline]
    fn serialize_all(&self) -> bool {
        true
    }
}

/// Helper which writes the entries of a map, honoring [`SerializeConfig`].
/// The keys are converted to the case of the case-key feature.
pub(crate) struct Entries<M> {
    map: M,
    all: bool,
}

impl<M: SerializeMap> Entries<M> {
    pub(crate) fn new<S>(serializer: S, all: bool) -> Result<Self, S::Error>
    where
        S: Serializer<SerializeMap = M, Error = M::Error>,
    {
        Ok(Self {
            map: serializer.serialize_map(None)?,
            all,
        })
    }

    /// Write the entry always.
    pub(crate) fn value<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), M::Error> {
        self.map.serialize_entry(&case_key(key), value)
    }

    /// Write the flag if it is `true` (or when serializing all).
    pub(crate) fn flag(&mut self, key: &'static str, value: bool) -> Result<(), M::Error> {
        if self.all || value {
            self.map.serialize_entry(&case_key(key), &value)?;
        }
        Ok(())
    }

    /// Write the value if it is `Some` (or `null` when serializing all).
    pub(crate) fn opt<T: Serialize>(
        &mut self,
        key: &'static str,
        value: Option<T>,
    ) -> Result<(), M::Error> {
        match value {
            Some(v) => self.map.serialize_entry(&case_key(key), &v),
            None if self.all => self.map.serialize_entry(&case_key(key), &()),
            None => Ok(()),
        }
    }

    /// Write the sequence if it is not empty (or when serializing all).
    pub(crate) fn seq<T: Serialize>(
        &mut self,
        key: &'static str,
        value: impl IntoIterator<Item = T>,
    ) -> Result<(), M::Error> {
        let v: Vec<T> = value.into_iter().collect();
        if self.all || !v.is_empty() {
            self.map.serialize_entry(&case_key(key), &v)?;
        }
        Ok(())
    }

    pub(crate) fn end(self) -> Result<M::Ok, M::Error> {
        self.map.end()
    }
}

/// Serialize as a string using [`Display`](std::fmt::Display).
pub(crate) struct DisplayStr<T>(pub(crate) T);

impl<T: std::fmt::Display> Serialize for DisplayStr<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.0)
    }
}
