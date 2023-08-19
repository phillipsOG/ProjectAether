use std::ops::Deref;
use std::fmt;

pub struct Space {
    pub tile: char,
    pub is_visible: bool,
}

impl Space {
    pub(crate) fn new(tile: char) -> Self {
        Space { tile, is_visible: false }
    }

    pub(crate) fn from_char(tile: char) -> Self {
        Space { tile, is_visible: false }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tile)
    }
}

impl Deref for Space {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.tile
    }
}