#![allow(clippy::unusual_byte_groupings)]

use std::fmt::Display;

use crate::game::Game;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TicTacToe {
    board: [u16; 2],
    moves: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TicTacToeMove(usize);

impl TicTacToeMove {
    pub fn new(idx: usize) -> Self {
        TicTacToeMove(idx)
    }
}

impl Display for TicTacToeMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [0; 2],
            moves: 0,
        }
    }

    fn probe_spot(&self, spot: usize) -> bool {
        // returns true if the chosen location is occupied by
        // the side to move
        self.board[(self.moves + 1) & 1] & (1 << spot) != 0
    }

    fn pos_filled(&self, i: usize) -> bool {
        (self.board[0] | self.board[1]) & (1 << i) != 0
    }

    fn player_at(&self, i: usize) -> bool {
        assert!(self.pos_filled(i));
        self.board[0] & (1 << i) != 0
    }

    fn char_at(&self, x: usize, y: usize) -> char {
        if self.pos_filled(x * 3 + y) {
            if self.player_at(x * 3 + y) {
                'X'
            } else {
                '0'
            }
        } else {
            '.'
        }
    }
}

impl Game for TicTacToe {
    type Move = TicTacToeMove;

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

        0
    }

    fn is_terminal(&self) -> bool {
        self.moves == 9 || self.evaluate() != 0
    }

    fn generate_legal_moves(&self, buffer: &mut Vec<Self::Move>) {
        let bb = self.board[0] | self.board[1];
        let mut bb = !bb & 0b111_111_111;
        while bb != 0 {
            buffer.push(TicTacToeMove::new(bb.trailing_zeros() as usize));
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
                write!(f, "{} ", self.char_at(x, y))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::perft::perft;

    use super::TicTacToe;

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
