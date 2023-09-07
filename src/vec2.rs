#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord, Debug)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl Vec2 {
    pub const ZERO: Self = Self::new(0, 0);

    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
