#![allow(deprecated)]

use std::str::FromStr;
use visdom::Vis;
use crate::{parser::ParseError, structures::EventPane};

impl FromStr for EventPane {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;

        let event = root.find("#eventpane");
        let value = event.outer_html();

        Ok(EventPane { value })
    }
}
