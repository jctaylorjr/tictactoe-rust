use std::error::Error;
use std::fmt::Display;

const BOARD_SIZE: usize = 9;

#[derive(Clone, Copy, Debug)]
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

#[derive(Debug)]
enum GameError {
    InvalidTileSelection,
}

impl Error for GameError {}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::InvalidTileSelection => write!(f, "Invalid tile selection!"),
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

    fn update_state(&mut self, state: TileState) -> Result<(), GameError> {
        match self.state {
            TileState::Empty => {
                self.state = state;
                Ok(())
            }
            _ => Err(GameError::InvalidTileSelection),
        }
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

    pub fn start_game(&mut self) {
        let mut player_iter = self.players.iter().cycle();
        for _ in 0..BOARD_SIZE {
            let current_player = player_iter.next().unwrap();
            println!(
                "Player {}'s turn! Select a tile (1-{}): ",
                current_player.player_team(), BOARD_SIZE
            );
            // println!("{tile_choice}")
            while let Err(_) = self.board[Self::input_selection()].update_state(current_player.team) {
                println!("Tile already occupied! Please select another tile: ");
            }
        }
        println!("{:?}", self.board);
    }

    fn input_selection() -> usize {
        let mut input = String::new();

        loop {
            input.clear();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            if let Ok(n) = input.trim().parse() {
                if (1..=BOARD_SIZE).contains(&n) {
                    return (n - 1) as usize;
                }
                println!("Invalid input! Please enter a number between 1 and {}: ", BOARD_SIZE);
            }
        }
    }
}