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
    pub team: TileState,
}

impl Player {
    pub fn new(team: TileState) -> Player {
        Player { team }
    }

    pub fn player_team(&self) -> &str {
        self.team.enum_str()
    }
}

const BOARD_SIZE: usize = 9;

pub struct Game {
    pub board: Vec<Tile>,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: vec![Tile::new(); BOARD_SIZE],
            players: vec![
                Player::new(TileState::X),
                Player::new(TileState::O),
            ],
        }
    }
}