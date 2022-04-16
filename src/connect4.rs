#![allow(clippy::unusual_byte_groupings)]

use std::fmt::Display;

use crate::game::Game;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;
const ROW_LENGTH: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connect4 {
    // * Example of bit order to encode for a 7x6 board
    //    * .  .  .  .  .  .  .
    //    * 5 12 19 26 33 40 47
    //    * 4 11 18 25 32 39 46
    //    * 3 10 17 24 31 38 45
    //    * 2  9 16 23 30 37 44
    //    * 1  8 15 22 29 36 43
    //    * 0  7 14 21 28 35 42 
    // 
    // Position is stored as
    // - a bitboard "filled" with 1 on any color stones
    // - a bitboard "current" with 1 on stones of current player
    
    filled: u64,
    // "board" has ones for the current player's pieces
    // and zeros for the opponent's pieces, flipped every turn
    current: u64,
    // track the number of moves played
    moves: usize
}


impl Connect4 {
    pub fn new() -> Self {
        Self {
            filled: 0,
            current: 0,
            moves: 0,
        }
    }

    fn is_won(&self) -> bool {
        let mask = 1u64 << self.last_played_loc;

        for dir in 0..4 {
            if self.win_dir(mask, dir) {
                return true;
            }
        }

        false
    }

    fn win_dir(&self, mask: u64, dir: usize) -> bool {
        const SHIFTS: [usize; 4] = [1, WIDTH, WIDTH + 1, WIDTH - 1];
        let shift = SHIFTS[dir];
        let board = self.current;
        let starting_row = Self::row(mask);
        // MUST TRACK: shifts that go past the edge.
        let mut on_count = 0;

        let mut mut_mask = mask;
        while on_count != 4
            && (board & mut_mask) != 0
            && mut_mask != 0
            && Self::row(mut_mask) == starting_row
        {
            on_count += 1;
            mut_mask <<= shift;
        }
        let mut mut_mask = mask >> shift;
        while on_count != 4
            && (board & mut_mask) != 0
            && mut_mask != 0
            && Self::row(mut_mask) == starting_row
        {
            on_count += 1;
            mut_mask >>= shift;
        }
        
        on_count == ROW_LENGTH
    }

    #[inline(always)]
    fn row(mask: u64) -> usize {
        (mask.trailing_zeros() / WIDTH as u32) as usize
    }

    fn bottom_mask(col: usize) -> u64 {
        1u64 << (col * (HEIGHT + 1))
    }
}

impl Game for Connect4 {
    type Move = usize;

    fn turn(&self) -> i32 {
        if self.moves & 1 == 0 {
            1
        } else {
            -1
        }
    }

    fn evaluate(&self) -> i32 {
        if self.is_won() {
            return -self.turn();
        }

        0
    }

    fn is_terminal(&self) -> bool {
        self.moves == WIDTH * HEIGHT || self.evaluate() != 0
    }

    fn generate_legal_moves(&self, buffer: &mut Vec<Self::Move>) {
        let filled = self.filled;
        let mut empty = !filled & 0b1111111;
        while empty != 0 {
            buffer.push(empty.trailing_zeros() as usize);
            empty &= empty - 1; // clear the least significant bit set
        }
    }

    fn action_space_size(&self) -> usize {
        WIDTH
    }

    fn push(&mut self, m: Self::Move) {
        assert!(m < WIDTH);
        self.current ^= self.filled;
        self.filled |= self.filled + Self::bottom_mask(m);
        self.moves += 1;
    }

    fn pop(&mut self, m: Self::Move) {
        todo!()
    }
}

impl Display for Connect4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let mask = 1u64 << (row * WIDTH + col);
                if (self.current[0] & mask) != 0 {
                    write!(f, "X")?;
                } else if (self.current[1] & mask) != 0 {
                    write!(f, "O")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_correct() {
        let bitboard = 1u64 << 0;
        assert_eq!(0, Connect4::row(bitboard));
        let bitboard = 1u64 << (WIDTH - 1);
        assert_eq!(0, Connect4::row(bitboard));
        let bitboard = 1u64 << WIDTH;
        assert_eq!(1, Connect4::row(bitboard));
        let bitboard = 1u64 << (WIDTH * HEIGHT - 1);
        assert_eq!(5, Connect4::row(bitboard));
    }
}
