#![allow(clippy::unusual_byte_groupings)]

use std::fmt::Display;

use crate::game::Game;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoverTTT {
    board: [u16; 6],
    moves: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Big = 0,
    Medium = 2,
    Small = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoverTTTMove {
    idx: usize,
    size: Size,
}

impl CoverTTTMove {
    pub fn new(idx: usize, size: Size) -> Self {
        Self { idx, size }
    }
}

impl Display for CoverTTTMove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let size_char = match self.size {
            Size::Big => 'B',
            Size::Medium => 'M',
            Size::Small => 'S',
        };
        write!(f, "{}{}", size_char, self.idx)
    }
}

impl CoverTTT {
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
            || (self.board[((self.moves + 1) & 1) + 2] & (1 << spot) != 0
                && self.board[self.moves & 1] & (1 << spot) == 0)
            || (self.board[((self.moves + 1) & 1) + 4] & (1 << spot) != 0
                && self.board[self.moves & 1] & (1 << spot) == 0
                && self.board[(self.moves & 1) + 2] & (1 << spot) == 0)
    }

    fn pos_filled(&self, i: usize) -> bool {
        (self.board[0]
            | self.board[1]
            | self.board[2]
            | self.board[3]
            | self.board[4]
            | self.board[5])
            & (1 << i)
            != 0
    }

    fn player_at(&self, i: usize) -> bool {
        assert!(self.pos_filled(i));
        if self.board[0] & (1 << i) != 0 {
            // top level X
            true
        } else if self.board[1] & (1 << i) != 0 {
            // top level O
            false
        } else if self.board[2] & (1 << i) != 0 {
            // mid level X
            true
        } else if self.board[3] & (1 << i) != 0 {
            // mid level O
            false
        } else {
            self.board[4] & (1 << i) != 0
        }
    }

    fn size_at(&self, i: usize) -> Size {
        assert!(self.pos_filled(i));
        if (self.board[0] | self.board[1]) & (1 << i) != 0 {
            Size::Big
        } else if (self.board[2] | self.board[3]) & (1 << i) != 0 {
            Size::Medium
        } else {
            Size::Small
        }
    }

    fn char_at(&self, x: usize, y: usize) -> char {
        use Size::{Big, Medium, Small};
        if self.pos_filled(x * 3 + y) {
            if self.player_at(x * 3 + y) {
                match self.size_at(x * 3 + y) {
                    Big => 'B',
                    Medium => 'M',
                    Small => 'S',
                }
            } else {
                match self.size_at(x * 3 + y) {
                    Big => 'b',
                    Medium => 'm',
                    Small => 's',
                }
            }
        } else {
            '.'
        }
    }

    fn layer_bitmask(&self, size: Size) -> u16 {
        match size {
            Size::Big => self.board[0] | self.board[1],
            Size::Medium => self.board[2] | self.board[3],
            Size::Small => self.board[4] | self.board[5],
        }
    }

    fn legal_moves_exist(&self) -> bool {
        let tops_used = self.board[self.moves & 1].count_ones() as usize;
        let top_bb = self.board[0] | self.board[1];
        if tops_used < 3 {
            let options = !top_bb & 0b111_111_111;
            if options != 0 {
                return true;
            }
        }
        let mids_used = self.board[(self.moves & 1) + 2].count_ones() as usize;
        let mid_bb = self.board[2] | self.board[3];
        if mids_used < 3 {
            let options = !mid_bb & !top_bb & 0b111_111_111;
            if options != 0 {
                return true;
            }
        }
        let bottoms_used = self.board[(self.moves & 1) + 4].count_ones() as usize;
        let bottom_bb = self.board[4] | self.board[5];
        if bottoms_used < 3 {
            let options = !bottom_bb & !mid_bb & !top_bb & 0b111_111_111;
            if options != 0 {
                return true;
            }
        }
        false
    }
}

impl Game for CoverTTT {
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

        0
    }

    fn is_terminal(&self) -> bool {
        (self.layer_bitmask(Size::Small) == 6
            && self.layer_bitmask(Size::Medium) == 6
            && self.layer_bitmask(Size::Big) == 6)
            || self.evaluate() != 0
            || !self.legal_moves_exist()
    }

    fn generate_legal_moves(&self, buffer: &mut Vec<Self::Move>) {
        let tops_used = self.board[self.moves & 1].count_ones() as usize;
        let top_bb = self.board[0] | self.board[1];
        if tops_used < 3 {
            let mut options = !top_bb & 0b111_111_111;
            while options != 0 {
                buffer.push(CoverTTTMove::new(
                    options.trailing_zeros() as usize,
                    Size::Big,
                ));
                options &= options - 1; // clear the least significant bit set
            }
        }
        let mids_used = self.board[(self.moves & 1) + 2].count_ones() as usize;
        let mid_bb = self.board[2] | self.board[3];
        if mids_used < 3 {
            let mut options = !mid_bb & !top_bb & 0b111_111_111;
            while options != 0 {
                buffer.push(CoverTTTMove::new(
                    options.trailing_zeros() as usize,
                    Size::Medium,
                ));
                options &= options - 1; // clear the least significant bit set
            }
        }
        let bottoms_used = self.board[(self.moves & 1) + 4].count_ones() as usize;
        let bottom_bb = self.board[4] | self.board[5];
        if bottoms_used < 3 {
            let mut options = !bottom_bb & !mid_bb & !top_bb & 0b111_111_111;
            while options != 0 {
                buffer.push(CoverTTTMove::new(
                    options.trailing_zeros() as usize,
                    Size::Small,
                ));
                options &= options - 1; // clear the least significant bit set
            }
        }
    }

    fn push(&mut self, m: Self::Move) {
        self.board[(self.moves & 1) + m.size as usize] |= 1 << m.idx;
        self.moves += 1;
    }

    fn pop(&mut self, m: Self::Move) {
        self.moves -= 1;
        self.board[(self.moves & 1) + m.size as usize] &= !(1 << m.idx);
    }

    fn action_space_size(&self) -> usize {
        9 * 3
    }
}

impl Display for CoverTTT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..3 {
            for y in 0..3 {
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

    use super::CoverTTT;

    #[test]
    fn depth1() {
        let mut board = CoverTTT::new();
        assert_eq!(perft(&mut board, 1), 27);
    }

    #[test]
    fn depth2() {
        let mut board = CoverTTT::new();
        dbg!(perft(&mut board, 2));
        assert_eq!(perft(&mut board, 2), 675);
    }

    // #[test]
    // fn depth6() {
    //     let mut board = CoverTTT::new();
    //     dbg!(perft(&mut board, 6));
    //     assert_eq!(perft(&mut board, 6), 103735800);
    // }
}
