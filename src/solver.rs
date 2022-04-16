use std::collections::{HashMap, hash_map::Entry};

use crate::game::{Game, StochasticGame, PartiallySolvable, ToMove, Keyed};

pub const INF: i32 = std::i32::MAX;

// fn solve(game: &mut impl Game) {
//     let mut buffer = Vec::new();
//     game.generate_legal_moves(&mut buffer);

//     let move_scores = buffer.iter().map(|m| {
//         let mut game_copy = game.clone();
//         game_copy.push(m.clone());
//         (m, solve_internal(game_copy))
//     }).collect::<Vec<_>>();
// }

pub fn negamax<T>(node: &mut T, depth: usize, alpha: i32, beta: i32) -> i32
where
    T: Game,
{
    if depth == 0 || node.is_terminal() {
        return (node.turn() * node.evaluate()) as i32 * depth as i32;
    }

    let mut buffer = Vec::with_capacity(node.action_space_size());
    node.generate_moves(&mut buffer);

    let mut alpha = alpha;
    for m in buffer {
        node.push(m);
        let value = -negamax(node, depth - 1, -beta, -alpha);
        node.pop(m);
        alpha = std::cmp::max(alpha, value);
        if alpha >= beta {
            break;
        }
    }

    alpha
}

pub enum TTScore {
    Exact(i32),
    LowerBound(i32),
    UpperBound(i32),
}

pub struct TTEntry {
    pub depth: usize,
    pub value: TTScore,
}

pub fn negamax_tt<T>(node: &mut T, depth: usize, alpha: i32, beta: i32, tt: &mut HashMap<u64, TTEntry>) -> i32
where
    T: Game + Keyed,
{
    if depth == 0 || node.is_terminal() {
        return (node.turn() * node.evaluate()) as i32 * depth as i32;
    }

    let (mut alpha, mut beta) = (alpha, beta);

    if let Some(entry) = tt.get(&node.hashkey()) {
        if entry.depth >= depth {
            match entry.value {
                TTScore::Exact(v) => return v,
                TTScore::LowerBound(v) => alpha = std::cmp::max(v, alpha),
                TTScore::UpperBound(v) => beta = std::cmp::min(v, beta),
            }

            if alpha >= beta {
                return alpha;
            }
        }
    }

    let mut buffer = Vec::with_capacity(node.action_space_size());
    node.generate_moves(&mut buffer);

    let mut alpha = alpha;
    let inital_alpha = alpha;
    for m in buffer {
        node.push(m);
        let value = -negamax(node, depth - 1, -beta, -alpha);
        node.pop(m);
        alpha = std::cmp::max(alpha, value);
        if alpha >= beta {
            break;
        }
    }

    let entry_to_save = TTEntry {
        depth,
        value: if alpha <= inital_alpha {
            TTScore::UpperBound(alpha)
        } else if alpha >= beta {
            TTScore::LowerBound(alpha)
        } else {
            TTScore::Exact(alpha)
        },
    };

    tt.insert(node.hashkey(), entry_to_save);

    alpha
}

pub fn expectiminimax<T>(node: &mut T, depth: usize) -> i32
where
    T: StochasticGame + PartiallySolvable,
{
    use ToMove::*;
    // if node is a terminal node or depth = 0
    //    return the heuristic value of node
    if depth == 0 || node.is_terminal() {
        return node.heuristic();
    }

    match node.to_move() {
        Max => {
            // Return value of maximum-valued child node
            let mut max_value = -INF;
            let mut buffer = Vec::with_capacity(node.action_space_size());
            node.generate_moves(&mut buffer);
            for m in buffer {
                node.push(m);
                let value = expectiminimax(node, depth - 1);
                node.pop(m);
                max_value = std::cmp::max(max_value, value);
            }
            max_value
        },
        Min => {
            // Return value of minimum-valued child node
            let mut min_value = INF;
            let mut buffer = Vec::with_capacity(node.action_space_size());
            node.generate_moves(&mut buffer);
            for m in buffer {
                node.push(m);
                let value = expectiminimax(node, depth - 1);
                node.pop(m);
                min_value = std::cmp::min(min_value, value);
            }
            min_value
        },
        Chance => {
            // Return the average value of the child nodes,
            // weighted by the probability of the child nodes.
            const SCALE_FACTOR: i32 = 1_000_000;
            let mut scaled_value = 0;
            let mut buffer = Vec::with_capacity(node.action_space_size());
            node.generate_legal_moves_with_probabilities(&mut buffer);
            for (m, prob) in buffer {
                node.push(m);
                // don't reduce depth
                let value = expectiminimax(node, depth);
                node.pop(m);
                scaled_value += ((value * SCALE_FACTOR) as f32 * prob) as i32;
            }
            scaled_value / SCALE_FACTOR
        },
    }
}

pub fn solve(game: impl Game + Keyed) -> i32 {
    let mut game = game;
    let value = negamax(&mut game, 1000, -INF, INF) * game.turn() as i32;
    // 0 for a draw, N for mate-in-n for X, -N for mate-in-n for O
    (1000 - value.abs()) * value.signum()
}

pub fn eval_to_string(solution: i32) -> String {
    match solution.signum() {
        0 => "Draw".to_string(),
        1 => format!("X wins in {} moves", solution),
        -1 => format!("O wins in {} moves", -solution),
        _ => panic!("Invalid solution"),
    }
}

pub fn print_solve_info(game: impl Game + Keyed) {
    let start = std::time::Instant::now();
    let solution = solve(game);
    let time = start.elapsed().as_secs_f32();
    println!("Solved in {} seconds.", time);
    print!("Solution: ");
    println!("{}", eval_to_string(solution));
}

pub fn best_move<G: Game>(game: G) -> G::Move {
    let mut game = game;
    let mut moves = Vec::with_capacity(27);
    game.generate_moves(&mut moves);
    *moves
        .iter()
        .max_by_key(|&&m| {
            game.push(m);
            let value = -negamax(&mut game, 1000, -INF, INF);
            game.pop(m);
            value
        })
        .unwrap()
}

pub fn expecti_best_move<G>(game: G) -> G::Move
where
    G: StochasticGame + PartiallySolvable,
{
    let mut game = game;
    let mut moves = Vec::with_capacity(27);
    game.generate_moves(&mut moves);
    *moves
        .iter()
        .max_by_key(|&&m| {
            game.push(m);
            let value = expectiminimax(&mut game, 5);
            game.pop(m);
            match game.to_move() {
                ToMove::Max => value,
                ToMove::Min => -value,
                ToMove::Chance => rand::random::<i32>().abs(),
            }
        })
        .unwrap()
}

pub fn principal_variation<G: Game>(game: G) -> Vec<G::Move> {
    let mut game = game;
    let mut out = Vec::new();
    while !game.is_terminal() {
        let m = best_move(game.clone());
        game.push(m);
        out.push(m);
    }
    out
}

pub fn expecti_principal_variation<G>(game: G) -> Vec<G::Move>
where
    G: StochasticGame + PartiallySolvable,
{
    let mut game = game;
    let mut out = Vec::new();
    while !game.is_terminal() {
        let m = expecti_best_move(game.clone());
        game.push(m);
        out.push(m);
    }
    out
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::coverttt::{CoverTTT, CoverTTTMove, Size};
    use crate::game::Game;

    use crate::tictactoe::{TicTacToe, TicTacToeMove};

    use super::{solve, negamax, INF, negamax_tt};

    #[test]
    fn ttt_root() {
        let value = solve(TicTacToe::new());

        // this position is drawn, so the value is 0
        assert_eq!(value, 0);
    }

    #[test]
    fn ttt_xwin() {
        let mut root = TicTacToe::new();
        root.push(TicTacToeMove::new(4));
        root.push(TicTacToeMove::new(1));

        // X wins, so the sign of the value is positive
        assert_eq!(solve(root).signum(), 1);
        // X wins in five moves, so the value is 5
        assert_eq!(solve(root), 5);
    }

    #[test]
    fn ttt_owin() {
        let mut root = TicTacToe::new();
        root.push(TicTacToeMove::new(1));
        root.push(TicTacToeMove::new(4));
        root.push(TicTacToeMove::new(7));

        // O wins, so the sign of the value is negative
        assert_eq!(solve(root).signum(), -1);
        // O wins in five moves, so the value is -5
        assert_eq!(solve(root), -5);
    }

    #[test]
    fn cttt_xwin() {
        let mut root = CoverTTT::new();
        root.push(CoverTTTMove::new(4, Size::Big));
        root.push(CoverTTTMove::new(1, Size::Small));

        // X wins, so the sign of the value is positive
        assert_eq!(solve(root).signum(), 1);
        // X wins in five moves, so the value is 5
        assert_eq!(solve(root), 5);
    }

    #[test]
    fn hashtable_equivalence() {
        let game = TicTacToe::new();
        let without_table = negamax(&mut game.clone(), 6, -INF, INF);
        let with_table = negamax_tt(&mut game.clone(), 6, -INF, INF, &mut HashMap::new());
        assert_eq!(without_table, with_table);
        let without2 = negamax(&mut game.clone(), 100, -INF, INF);
        let with2 = negamax_tt(&mut game.clone(), 100, -INF, INF, &mut HashMap::new());
        assert_eq!(without2, with2);
    }
}
