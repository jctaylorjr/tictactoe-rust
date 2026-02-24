use std::{io, usize};

// use rand;
// use std::time::{Duration, Instant};
use tictactoe::{Player, Tile, TileState};

fn main() {
    let board_size = 9;
    let mut tiles: Vec<Tile> = vec![Tile::new(); board_size];

    // dot_product_test();
    let player_x = Player::new(TileState::X);
    let player_o = Player::new(TileState::O);

    let mut players = vec![player_x, player_o].into_iter().cycle();
    for _ in 0..board_size {
        let current_player = players.next().unwrap();
        println!(
            "Player {}'s turn! Select a tile (1-9): ",
            current_player.player_team()
        );
        let tile_choice = tile_select();
        println!("{tile_choice}")
    }
}

fn tile_select() -> usize {
    let mut input = String::new();

    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if let Ok(n) = input.trim().parse() {
            if (1..=9).contains(&n) {
                return (n - 1) as usize;
            }
        }
    }
}

// fn dot_product_test() {
//     let arr0: Vec<f64> = rand::random_iter().take(100000).collect();
//     let arr1: Vec<f64> = rand::random_iter().take(100000).collect();
//     let mut dot_product: f64 = 0.0;

//     let start = Instant::now();
//     for _ in 0..1000 {
//         dot_product = calc_dot_product(&arr0, &arr1)
//     }
//     let duration = start.elapsed();

//     println!("{:?}", dot_product);
//     println!("Time in seconds: {:?}", duration.as_secs_f64());
// }

// fn calc_dot_product(arr0: &[f64], arr1: &[f64]) -> f64 {
//     arr0.iter()
//         .zip(arr1.iter())
//         .map(|(&x, &y)| x * y)
//         .fold(0.0, |acc, x| acc + x)
// }
