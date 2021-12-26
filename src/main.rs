use crate::{
    coverttt::CoverTTT,
    game::Game,
    solver::{eval_to_string, solve},
    tictactoe::TicTacToe,
};

mod coverttt;
mod game;
mod perft;
mod solver;
mod tictactoe;

fn invert(val: i32) -> f64 {
    if val == 0 {
        0.0
    } else {
        1.0 / val as f64
    }
}

fn main() {
    println!("Solving Noughts and Crosses");
    solver::print_solve_info(TicTacToe::new());
    println!();
    println!("Solving Cover Tic-Tac-Toe");
    solver::print_solve_info(CoverTTT::new());

    println!("CTTT opening move evaluations (sorted worst-to-best):");
    let mut root = CoverTTT::new();
    let mut moves = Vec::with_capacity(27);
    root.generate_legal_moves(&mut moves);
    let mut evals = moves
        .into_iter()
        .map(|m| {
            root.push(m);
            let value = solve(root);
            root.pop(m);
            (m, value)
        })
        .collect::<Vec<_>>();
    evals.sort_by(|a, b| invert(a.1).partial_cmp(&invert(b.1)).unwrap());
    for (m, value) in evals {
        println!("move {}: {}", m, eval_to_string(value + value.signum()));
    }
}
