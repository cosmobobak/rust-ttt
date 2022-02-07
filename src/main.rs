#![allow(unused_imports, dead_code)]

use crate::{
    coverttt::CoverTTT,
    game::Game,
    solver::{best_move, eval_to_string, solve},
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

fn play_human<G: Game>(mut game: G) {
    use std::io::{stdin,stdout,Write};
    let mut userinput = String::new();
    while !game.is_terminal() {
        println!("{}", game);
        if game.turn() == -1 {
            let m = solver::best_move(game.clone());
            println!("computer plays {}", m);
            game.push(m);
            continue;
        }
        let mut lmoves = Vec::new();
        game.generate_legal_moves(&mut lmoves);
        println!("legal moves:");
        for &m in &lmoves {
            print!("{}, ", m);
        }
        println!();
        let _ = stdout().flush();
        stdin().read_line(&mut userinput).expect("Did not enter a correct string.");
        let str_move = userinput.trim();
        let user_move = *lmoves.iter().find(|m| {
            format!("{}", m) == str_move
        }).expect("Move not found.");
        game.push(user_move);
        userinput.clear();
    }
    println!("{}", game);
    game.print_outcome();
}

fn main() {
    // println!("Solving Noughts and Crosses");
    // solver::print_solve_info(TicTacToe::new());
    // println!();
    // println!("Solving Cover Tic-Tac-Toe");
    // solver::print_solve_info(CoverTTT::new());

    play_human(CoverTTT::new());

    // println!("CTTT opening move evaluations (sorted worst-to-best):");
    // let mut root = CoverTTT::new();
    // let mut moves = Vec::with_capacity(27);
    // root.generate_legal_moves(&mut moves);
    // let mut evals = moves
    //     .into_iter()
    //     .map(|m| {
    //         root.push(m);
    //         let value = solve(root);
    //         root.pop(m);
    //         (m, value)
    //     })
    //     .collect::<Vec<_>>();
    // evals.sort_by(|a, b| invert(a.1).partial_cmp(&invert(b.1)).unwrap());
    // for (m, value) in evals {
    //     println!("move {}: {}", m, eval_to_string(value + value.signum()));
    // }
}
