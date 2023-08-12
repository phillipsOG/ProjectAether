use std::string::ToString;

pub struct TileSet {
    pub player: char,
    pub wall: char,
    pub closed_door_side: char,
    pub closed_door_top: char,
    pub open_door: char,
    pub key: char,
    pub floor: char,
    pub ladder: &'static str,
    pub previous_tile: char,
}

pub const TILE_SET: TileSet = TileSet {
    player: '@',
    wall: '#',
    closed_door_side: '|',
    closed_door_top: '-',
    open_door: '/',
    key: 'k',
    floor: '.',
    ladder: "|-|",
    previous_tile: '.',
};
