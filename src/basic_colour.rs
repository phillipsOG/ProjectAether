use sdl2::pixels::Color;

#[derive(Clone)]
pub struct BasicColour {
    pub black: Color,
    pub white: Color,
    pub red: Color,
    pub green: Color,
    pub blue: Color,
    pub yellow: Color,
}

impl BasicColour {
    pub(crate) fn new() -> Self {
        Self::update_with(COLOUR)
    }

    pub fn update_with(other: BasicColour) -> Self {
        Self {
            black: other.black,
            white: other.white,
            red: other.red,
            green: other.green,
            blue: other.blue,
            yellow: other.yellow,
        }
    }
}

// Define your constant colors using RGBColor
pub const COLOUR: BasicColour = BasicColour {
    black: Color::RGB(0, 0, 0),
    white: Color::RGB(255, 255, 255),
    red: Color::RGB(255, 0, 0),
    green: Color::RGB(0, 255, 0),
    blue: Color::RGB(0, 0, 255),
    yellow: Color::RGB(255, 255, 0),
};
