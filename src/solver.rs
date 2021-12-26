use crate::game::Game;

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
        return node.turn() * node.evaluate() * depth as i32;
    }

    let mut alpha = alpha;

    let mut buffer = Vec::with_capacity(node.action_space_size());
    node.generate_legal_moves(&mut buffer);

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

pub fn solve(game: impl Game) -> i32 {
    let mut game = game;
    let value = negamax(&mut game, 1000, -INF, INF) * game.turn();
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

pub fn print_solve_info(game: impl Game) {
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
    game.generate_legal_moves(&mut moves);
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

#[cfg(test)]
mod tests {
    use crate::coverttt::{CoverTTT, CoverTTTMove, Size};
    use crate::game::Game;

    use crate::tictactoe::{TicTacToe, TicTacToeMove};

    use super::solve;

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
}
