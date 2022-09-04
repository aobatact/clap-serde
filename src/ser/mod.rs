#[macro_use]
mod macros;

pub(crate) mod app;
pub(crate) mod arg;

pub trait SerializeConfig {
    /// Serialize all the fields in [`Command`] and [`Arg`](`clap::Arg`).
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
    fn serialize_all(&self) -> bool {
        (*self).serialize_all()
    }
}

/// Serialize all the fields in [`Command`] and [`Arg`](`clap::Arg`).
/// If not set, the flags (getter begin with `is_`) with `false`
/// and values (getter begin with `get_`) with `None` will be skipped.
/// ```
/// # use clap::Command;
/// # use clap_serde::*;
/// # let command = Command::default();
/// let wrap = CommandWrapRef::new(&command).with_setting(NoSkip);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct NoSkip;
impl SerializeConfig for NoSkip {
    #[inline]
    fn serialize_all(&self) -> bool {
        true
    }
}
