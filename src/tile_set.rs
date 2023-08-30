#[derive(Clone)]
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
    pub name: &'static str,
}

impl TileSet {
    pub(crate) fn new(&mut self) {
        self.update_with(DEFAULT_TILE_SET)
    }

    pub fn update_with(&mut self, other: TileSet) {
        self.player = other.player;
        self.wall = other.wall;
        self.closed_door_side = other.closed_door_side;
        self.closed_door_top = other.closed_door_top;
        self.open_door = other.open_door;
        self.key = other.key;
        self.floor = other.floor;
        self.ladder = other.ladder;
        self.previous_tile = other.previous_tile;
        self.name = other.name;
    }
}

pub const DEFAULT_TILE_SET: TileSet = TileSet {
    player: '@',
    wall: '#',
    closed_door_side: '|',
    closed_door_top: '-',
    open_door: '/',
    key: 'k',
    floor: '.',
    ladder: &"|-|",
    previous_tile: '.',
    name: &"Default Tile Set",
};

pub const LADDER_TILE_SET: TileSet = TileSet {
    player: '@',
    wall: '#',
    closed_door_side: '#',
    closed_door_top: '#',
    open_door: '#',
    key: ' ',
    floor: '-',
    ladder: &"|-|",
    previous_tile: ' ',
    name: &"Ladder Tile Set",
};

pub struct MonsterTileSet {
    pub player: char,
    pub snake: char,
}

impl MonsterTileSet {
    pub(crate) fn new(&mut self) {
        self.update_with(MONSTER_TILE_SET)
    }

    pub fn update_with(&mut self, other: MonsterTileSet) {
        self.player = other.player;
        self.snake = other.snake;
    }
}

pub const MONSTER_TILE_SET: MonsterTileSet = MonsterTileSet {
    player: '@',
    snake: 'S',
};
