use std::error::Error;
use std::fmt::Display;
// use itertools::Itertools;

const BOARD_SIZE: usize = 9;

#[derive(Clone, Copy, Debug, PartialEq)]
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
    TileOccupied,
}

impl Error for GameError {}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::TileOccupied => write!(f, "Tile already occupied!"),
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
            _ => Err(GameError::TileOccupied),
        }
    }

    pub fn display_char(&self) -> &str {
        self.state.enum_str()
    }
}

#[derive(Clone)]
pub struct Player {
    pub team: TileState,
    pub magic_square_values: Vec<i32>,
}

impl Player {
    pub fn new(team: TileState) -> Player {
        Player { team, magic_square_values: vec![] }
    }

    pub fn player_team(&self) -> &str {
        self.team.enum_str()
    }

    // pub fn check_win_condition(&self) -> bool {
    //     // Check if the player's magic square values sum to 15 for any combination of three tiles
    //     if self.magic_square_values.len() < 3 {
    //         return false;
    //     }

    //     self.magic_square_values.iter().combinations(3).any(|combo| combo.iter().sum::<i32>() == 15)
    // }
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

            let selection = Self::input_selection();
            while let Err(e) = self.board[selection].update_state(current_player.team) {
                println!("{e} Please select another tile: ");
            }
        
            // TODO: Check for win condition after each turn
            if self.check_win_condition() {
                println!("Player {} wins!", current_player.player_team());
                return;
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

    fn check_win_condition(&self) -> bool {
        let win_combinations = [
            [0, 1, 2], // Row 1
            [3, 4, 5], // Row 2
            [6, 7, 8], // Row 3
            [0, 3, 6], // Column 1
            [1, 4, 7], // Column 2
            [2, 5, 8], // Column 3
            [0, 4, 8], // Diagonal \
            [2, 4, 6], // Diagonal /
        ];

        for combo in &win_combinations {
            let (a, b, c) = (self.board[combo[0]].state, self.board[combo[1]].state, self.board[combo[2]].state);
            if a != TileState::Empty && a == b && b == c {
                return true;
            }
        }

        false
    }
}