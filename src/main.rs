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

use game::PartiallySolvable;
use solver::expectiminimax;

use crate::{rgu::Ur, game::Game, solver::expecti_best_move, tictactoe::TicTacToe, coverttt::CoverTTT};

fn play_human<G: Game + PartiallySolvable>(mut game: G) {
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
        game.generate_moves(&mut lmoves);
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
