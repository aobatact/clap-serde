use clap::App;
use serde::{
    de::{DeserializeSeed, Error, Visitor},
    Deserialize,
};
mod de;

#[cfg(test)]
mod tests;

#[repr(transparent)]
pub struct AppWrap<'a> {
    app: App<'a>,
}

impl<'a> From<AppWrap<'a>> for App<'a> {
    fn from(a: AppWrap<'a>) -> Self {
        a.app
    }
}


