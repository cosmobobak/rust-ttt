use std::collections::HashMap;

use crate::game::Game;

fn perft_cached_internal<G: Game>(board: &mut G, depth: u8, seen: &mut HashMap<G, u64>) -> u64 {
    if depth == 0 || board.is_terminal() {
        return 1;
    }

    if depth > 3 && seen.contains_key(board) {
        return seen[board];
    }

    let mut moves = Vec::with_capacity(27);
    board.generate_legal_moves(&mut moves);

    let mut count = 0;
    for m in moves {
        board.push(m);
        count += perft_cached_internal(board, depth - 1, seen);
        board.pop(m);
    }

    if depth > 3 {
        seen.insert(board.clone(), count);
    }

    count
}

pub fn perft<G: Game>(board: &mut G, depth: u8) -> u64 {
    let mut seen = HashMap::new();
    perft_cached_internal(board, depth, &mut seen)
}
