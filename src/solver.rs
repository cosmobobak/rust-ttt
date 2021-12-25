use crate::game::Game;

const INF: i32 = std::i32::MAX;

// fn solve(game: &mut impl Game) {
//     let mut buffer = Vec::new();
//     game.generate_legal_moves(&mut buffer);

//     let move_scores = buffer.iter().map(|m| {
//         let mut game_copy = game.clone();
//         game_copy.push(m.clone());
//         (m, solve_internal(game_copy))
//     }).collect::<Vec<_>>();
// }

fn negamax<T>(node: &mut T, depth: usize, alpha: i32, beta: i32) -> i32 
    where T: Game
{
    if depth == 0 || node.is_terminal() {
        return node.turn() * node.evaluate();
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

fn solve(game: impl Game) -> i32 {
    let mut game = game;
    negamax(&mut game, 1000, -INF, INF) * game.turn()
}

#[cfg(test)]
mod tests {
    use crate::game::Game;

    use crate::tictactoe::{TicTacToe, TicTacToeMove};

    use super::solve;

    #[test]
    fn ttt_root() {        
        let value = solve(TicTacToe::new());

        assert_eq!(value, 0);
    }

    #[test]
    fn ttt_xwin() {
        let mut root = TicTacToe::new();
        root.push(TicTacToeMove::new(4));
        root.push(TicTacToeMove::new(1));

        assert_eq!(solve(root), 1);
    }


    #[test]
    fn ttt_owin() {
        let mut root = TicTacToe::new();
        root.push(TicTacToeMove::new(1));
        root.push(TicTacToeMove::new(4));
        root.push(TicTacToeMove::new(7));

        assert_eq!(solve(root), -1);
    }
}