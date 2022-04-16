use std::{collections::HashMap, fmt::Display};

use crate::game::Game;

pub fn perft<G: Game + Display>(board: &mut G, depth: u8) -> u64 {
    eprintln!("depth {}, board \n{}", depth, board);
    if board.is_terminal() {
        return 1;
    }

    let mut moves = Vec::with_capacity(27);
    board.generate_moves(&mut moves);

    if depth == 1 {
        return moves.len() as u64;
    }

    let mut nodes = 0;
    for m in moves {
        board.push(m);
        nodes += perft(board, depth - 1);
        board.pop(m);
    }

    nodes
}

fn perft_cached_internal<G: Game>(board: &mut G, depth: u8, seen: &mut HashMap<G, u64>) -> u64 {
    if board.is_terminal() {
        return 1;
    }

    let mut moves = Vec::with_capacity(27);
    board.generate_moves(&mut moves);

    if depth == 1 {
        return moves.len() as u64;
    }

    if let Some(nodes) = seen.get(board) {
        return *nodes;
    }

    let mut nodes = 0;
    for m in moves {
        board.push(m);
        nodes += perft_cached_internal(board, depth - 1, seen);
        board.pop(m);
    }

    if depth > 3 {
        seen.insert(board.clone(), nodes);
    }

    nodes
}

fn perft_cached<G: Game>(board: &mut G, depth: u8) -> u64 {
    let mut seen = HashMap::new();
    perft_cached_internal(board, depth, &mut seen)
}

pub fn perft_test<G: Game>(board: G) {
    let total_start = std::time::Instant::now();
    let mut count;
    for d in 1..50 {
        let start = std::time::Instant::now();
        let board_ref = &mut board.clone();
        count = perft_cached(board_ref, d);
        println!(
            "depth {}: {:>14} nodes, done in {:.1}s, at {:.0} Mnps", 
            d, 
            count,
            start.elapsed().as_secs_f32(),
            count as f32 / start.elapsed().as_secs_f32() / 1_000_000.0);
        if total_start.elapsed().as_secs_f32() > 20.0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::coverttt::CoverTTT;
    use crate::tictactoe::TicTacToe;

    use super::{perft, perft_cached};

    #[test]
    fn ttt_root() {
        let mut board = TicTacToe::new();
        let count = perft(&mut board, 1);
        assert_eq!(count, 9);
    }

    #[test]
    fn cttt_root() {
        let mut board = CoverTTT::new();
        let count = perft(&mut board, 1);
        assert_eq!(count, 27);
    }

    #[test]
    fn validate_cached_perft() {
        let mut board = CoverTTT::new();
        assert_eq!(perft_cached(&mut board, 1), perft(&mut board, 1));
        assert_eq!(perft_cached(&mut board, 2), perft(&mut board, 2));
        assert_eq!(perft_cached(&mut board, 3), perft(&mut board, 3));
        assert_eq!(perft_cached(&mut board, 4), perft(&mut board, 4));
    }
}