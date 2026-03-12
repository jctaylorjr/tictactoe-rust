// use rand;
// use std::time::{Duration, Instant};
use tictactoe::Game;

fn main() {
    let mut game = Game::new();

    println!("Welcome to Tic Tac Toe!");
    game.start_game();
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
