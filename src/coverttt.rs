use std::fmt::Display;

use crate::game::Game;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoverTicTacToe {
    board: [u16; 6],
    moves: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Size {
    Big,
    Medium,
    Small,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoverTTTMove {
    idx: usize,
    size: Size,
}

impl CoverTTTMove {
    pub fn new(idx: usize, size: Size) -> Self {
        Self {
            idx,
            size
        }
    }
}

impl CoverTicTacToe {
    pub fn new() -> Self {
        Self {
            board: [0; 6],
            moves: 0,
        }
    }

    fn probe_spot(&self, spot: usize) -> bool {
        // returns true if the chosen location is occupied by
        // the side to move
        self.board[(self.moves + 1) & 1] & (1 << spot) != 0
            || self.board[(self.moves + 1) & 1 + 2] & (1 << spot) != 0
            || self.board[(self.moves + 1) & 1 + 4] & (1 << spot) != 0
    }

    fn pos_filled(&self, i: usize) -> bool {
        (self.board[0] | self.board[1]) & (1 << i) != 0
            || (self.board[0 + 2] | self.board[1 + 2]) & (1 << i) != 0
            || (self.board[0 + 4] | self.board[1 + 4]) & (1 << i) != 0
    }

    fn player_at(&self, i: usize) -> bool {
        assert!(self.pos_filled(i));
        if self.board[0] & (1 << i) != 0 {
            // top level X
            true
        } else if self.board[1] & (1 << i) != 0 {
            // top level O
            false
        } else if self.board[0 + 2] & (1 << i) != 0 {
            // mid level X
            true
        } else if self.board[1 + 2] & (1 << i) != 0 {
            // mid level O
            false
        } else if self.board[0 + 4] & (1 << i) != 0 {
            // bottom level X
            true
        } else {
            // bottom level O
            false
        }
    }

    fn size_at(&self, i: usize) -> Size {
        assert!(self.pos_filled(i));
        if (self.board[0] | self.board[1]) & (1 << i) != 0 {
            Size::Big
        } else if (self.board[0 + 2] | self.board[1 + 2]) != 0 {
            Size::Medium
        } else {
            Size::Small
        }
    }

    fn char_at(&self, x: usize, y: usize) -> char {
        use Size::{Big, Small, Medium};
        if self.pos_filled(x * 3 + y) {
            if self.player_at(x * 3 + y) {
                match self.size_at(x * 3 + y) {
                    Big => 'A',
                    Medium => 'B',
                    Small => 'C'
                }
            } else {
                match self.size_at(x * 3 + y) {
                    Big => 'a',
                    Medium => 'b',
                    Small => 'c'
                }
            }
        } else {
            ' '
        }
    }
}

impl Game for CoverTicTacToe {
    type Move = CoverTTTMove;

    fn turn(&self) -> i32 {
        if self.moves & 1 == 0 {
            1
        } else {
            -1
        }
    }

    fn evaluate(&self) -> i32 {
        // check first diagonal
        if self.probe_spot(0) && self.probe_spot(4) && self.probe_spot(8) {
            return -self.turn();
        }

        // check second diagonal
        if self.probe_spot(2) && self.probe_spot(4) && self.probe_spot(6) {
            return -self.turn();
        }

        // check rows
        for i in 0..3 {
            if self.probe_spot(i * 3) && self.probe_spot(i * 3 + 1) && self.probe_spot(i * 3 + 2) {
                return -self.turn();
            }
        }
        // check columns
        for i in 0..3 {
            if self.probe_spot(i) && self.probe_spot(i + 3) && self.probe_spot(i + 6) {
                return -self.turn();
            }
        }

        return 0;
    }

    fn is_terminal(&self) -> bool {
        self.board[0] | self.board[1] == 0b111_111_111 || self.evaluate() != 0
    }

    fn generate_legal_moves(&self, buffer: &mut Vec<Self::Move>) {
        let bb = self.board[0] | self.board[1];
        let mut bb = !bb & 0b111_111_111;
        while bb != 0 {
            buffer.push(CoverTTTMove::new(bb.trailing_zeros() as usize));
            bb &= bb - 1; // clear the least significant bit set
        }
    }

    fn push(&mut self, m: Self::Move) {
        self.board[self.moves & 1] |= 1 << m.0;
        self.moves += 1;
    }

    fn pop(&mut self, m: Self::Move) {
        self.moves -= 1;
        self.board[self.moves & 1] ^= 1 << m.0;
    }

    fn action_space_size(&self) -> usize {
        9
    }
}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..3 {
            for x in 0..3 {
                write!(
                    f,
                    "{} ",
                    self.char_at(x, y)
                )?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Game;

    use super::TicTacToe;

    fn perft(board: &mut TicTacToe, depth: u8) -> u64 {
        if depth == 0 || board.is_terminal() {
            return 1;
        }

        let mut moves = Vec::with_capacity(9);
        board.generate_legal_moves(&mut moves);

        let mut count = 0;
        for m in moves {
            board.push(m);
            count += perft(board, depth - 1);
            board.pop(m);
        }

        count
    }

    #[test]
    fn depth1() {
        let mut board = TicTacToe::new();
        assert_eq!(perft(&mut board, 1), 9);
    }

    #[test]
    fn depth2() {
        let mut board = TicTacToe::new();
        assert_eq!(perft(&mut board, 2), 72);
    }

    #[test]
    fn fullperft() {
        let mut board = TicTacToe::new();
        assert_eq!(perft(&mut board, 10), 255168);
    }
}
