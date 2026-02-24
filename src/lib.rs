#[derive(Clone, Debug)]
pub enum TileState {
    Empty,
    X,
    O,
}

impl TileState {
    pub fn enum_str(&self) -> &str {
        match self {
            TileState::Empty => " ",
            TileState::X => "X",
            TileState::O => "O",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tile {
    state: TileState,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            state: TileState::Empty,
        }
    }

    pub fn update_state(&mut self, state: TileState) {
        self.state = state;
    }

    pub fn display_char(&self) -> &str {
        self.state.enum_str()
    }
}

#[derive(Clone)]
pub struct Player {
    pub default_tile: TileState,
}

impl Player {
    pub fn new(default_tile: TileState) -> Player {
        Player { default_tile }
    }

    pub fn player_team(&self) -> &str {
        self.default_tile.enum_str()
    }
}
