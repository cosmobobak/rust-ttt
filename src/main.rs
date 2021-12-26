#![allow(unused_imports, dead_code)]

use coverttt::CoverTTT;
use crate::{perft::perft, tictactoe::TicTacToe, game::Game, solver::{INF, solve, eval_to_string}, coverttt::{CoverTTTMove, Size}};


mod game;
mod solver;
mod perft;
mod coverttt;
mod tictactoe;

fn main() {
    // println!("Solving Noughts and Crosses");
    // solver::print_solve_info(TicTacToe::new());
    // println!();
    // println!("Solving Cover Tic-Tac-Toe");
    // solver::print_solve_info(CoverTTT::new());

    // println!("CTTT opening move evaluations (sorted worst-to-best):");
    // let mut root = CoverTTT::new();
    // let mut moves = Vec::with_capacity(27);
    // root.generate_legal_moves(&mut moves);
    // let mut evals = moves.into_iter().map(|m| {
    //     root.push(m);
    //     let value = solve(root);
    //     root.pop(m);
    //     (m, value)
    // }).collect::<Vec<_>>();
    // evals.sort_by(|a, b| (1.0 / a.1 as f32).partial_cmp(&(1.0 / b.1 as f32)).unwrap());
    // for (m, value) in evals {
    //     println!("move {}: {}", m, eval_to_string(value + value.signum()));
    // }

    let mut game = CoverTTT::new();
    // game.push(CoverTTTMove::new(4, Size::Big));
    // game.push(CoverTTTMove::new(8, Size::Big));
    // game.push(CoverTTTMove::new(2, Size::Big));
    // game.push(CoverTTTMove::new(6, Size::Big));
    // game.push(CoverTTTMove::new(7, Size::Big));
    // game.push(CoverTTTMove::new(1, Size::Medium));
    // game.push(CoverTTTMove::new(3, Size::Medium));
    // game.push(CoverTTTMove::new(7, Size::Medium));
    // game.push(CoverTTTMove::new(0, Size::Medium));
    while !game.is_terminal() {
        println!("{}", game);
        game.push(solver::best_move(game));
    }
    println!("{}", game);
}
