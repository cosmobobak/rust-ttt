#![allow(dead_code)]

mod iter_bits;
mod game;
mod tictactoe;
mod coverttt;
// mod connect4;
mod perft;
mod solver;
mod rgu;
mod adversarialknight;

use solver::expectiminimax;

use crate::{rgu::Ur, game::Game, solver::expecti_best_move, tictactoe::TicTacToe, coverttt::CoverTTT};

fn main() {
    // println!("Solving Noughts and Crosses");
    // solver::print_solve_info(TicTacToe::new());
    // println!();
    // println!("Solving Cover Tic-Tac-Toe");
    // solver::print_solve_info(CoverTTT::new());
    // println!();
    // println!("Solving the Adversarial Knight");
    // solver::print_solve_info(adversarialknight::AdversarialKnight::new());
    // println!();

    println!("Perft testing Noughts and Crosses");
    perft::perft_test(TicTacToe::new());
    println!();
    // println!("Perft testing Cover Tic-Tac-Toe");
    // perft::perft_test(CoverTTT::new());
    // println!();
    println!("Perft testing the Adversarial Knight");
    perft::perft_test(Ur::new());
    println!();

    let mut rgu = Ur::new();
    for depth in 1..20 {
        let before = rgu.clone();
        let start = std::time::Instant::now();
        let eval = expectiminimax(&mut rgu, depth);
        let time = start.elapsed().as_secs_f32();
        assert!(rgu == before);
        println!("eval {} at depth {}, done in {:.1}s", eval, depth, time);
    }

    // let mut rgu = Ur::new();
    // while !rgu.is_terminal() {
    //     let m = expecti_best_move(rgu.clone());
    //     println!("{}", rgu);
    //     let before = rgu.clone();
    //     println!("eval: {}", expectiminimax(&mut rgu, 5));
    //     println!("best move: {:?}", m);
    //     assert_eq!(rgu, before);
    //     rgu.push(m);
    // }
    // println!("{}", rgu);

}
