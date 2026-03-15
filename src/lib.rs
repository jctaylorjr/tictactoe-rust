use std::error::Error;
use std::fmt::Display;

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
            players: vec![Player::new(TileState::X), Player::new(TileState::O)],
        }
    }

    pub fn start_game(&mut self) {
        self.print_board();
        let mut player_iter = self.players.iter().cycle();
        for _ in 0..BOARD_SIZE {
            let current_player = player_iter.next().unwrap();
            println!(
                "Player {}'s turn! Select a tile (1-{}): ",
                current_player.player_team(),
                BOARD_SIZE
            );

            while let Err(e) = self.board[Self::input_selection()].update_state(current_player.team)
            {
                println!("{e} Please select another tile: ");
            }

            self.print_board();

            if self.check_win_condition() {
                println!("Player {} wins!", current_player.player_team());
                return;
            }
        }
        println!("It's a draw!");
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
                println!(
                    "Invalid input! Please enter a number between 1 and {}: ",
                    BOARD_SIZE
                );
            }
        }
    }

    fn check_win_condition(&self) -> bool {
        let win_combinations = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for combo in &win_combinations {
            let (a, b, c) = (
                self.board[combo[0]].state,
                self.board[combo[1]].state,
                self.board[combo[2]].state,
            );
            if a != TileState::Empty && a == b && b == c {
                return true;
            }
        }

        false
    }

    fn print_board(&self) {
        self.clear_terminal();
        for (i, tile) in self.board.iter().enumerate() {
            if tile.state == TileState::Empty {
                print!(" {} ", i + 1);
            } else if tile.state == TileState::X {
                print!("\x1b[31m {} \x1b[0m", tile.display_char());
            } else {
                print!("\x1b[32m {} \x1b[0m", tile.display_char());
            }
            if (i + 1) % 3 == 0 {
                println!();
            }
        }
    }

    fn clear_terminal(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }
}
