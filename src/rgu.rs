use std::{hint::unreachable_unchecked, ops::Neg, fmt::{Display, Formatter, self}};

use rand::Rng;

use crate::{game::{Game, ToMove, PartiallySolvable, StochasticGame}, iter_bits::IterBits};

// an implementation of the Royal Game of Ur

const BATTLE_SQUARES: [usize; 7] = [4, 5, 6, 8, 9, 10, 11];
const ROSETTE_SQUARES: [usize; 3] = [3, 7, 13];
const FROM_POT: usize = 14;
const END_SQUARE: usize = 13;
const STARTING_PIECES: i32 = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
enum State {
    X = 1,
    O = -1,
    Empty = 0,
}

impl Neg for State {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            State::X => State::O,
            State::O => State::X,
            State::Empty => State::Empty,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Board {
    // the board is represented as a bitvector
    // the first 14 bits are the squares for player 1
    // then, the 14 bits from bit 32 are used 
    // for player 2's pieces.
    slots: u64,
}

impl Board {
    fn new() -> Board {
        Board { slots: 0 }
    }

    fn bits(&self) -> u64 {
        self.slots
    }

    fn advance(&mut self, m: Move, player: State) {
        assert!(player != State::Empty);
        let big_shift = match player {
            State::X => 0,
            State::O => 32,
            State::Empty => unreachable!()
        };
        if m.from != FROM_POT {
            // we're actually moving a piece, not just putting one
            // on the board for the first time.
            let from_bit = 1 << (m.from + big_shift);
            self.slots &= !from_bit;
        }
        if m.to > END_SQUARE {
            // getting off the end of the board
            return;
        }
        let to_bit = 1 << (m.to + big_shift);
        self.slots |= to_bit;
        if m.capture {
            let opposing_spot = 1 << (m.to + (32 - big_shift));
            self.slots &= !opposing_spot;
        }
    }

    fn revert(&mut self, m: Move, player: State) {
        assert!(player != State::Empty);
        let big_shift = match player {
            State::X => 32,
            State::O => 0,
            State::Empty => unreachable!()
        };
        if m.from != FROM_POT {
            // we're actually moving a piece, not just putting one
            // on the board for the first time.
            let from_bit = 1 << (m.from + big_shift);
            self.slots |= from_bit;
        }
        if m.to > END_SQUARE {
            // getting off the end of the board
            return;
        }
        let to_bit = 1 << (m.to + big_shift);
        self.slots &= !to_bit;
        if m.capture {
            let opposing_spot = 1 << (m.to + (32 - big_shift));
            self.slots |= opposing_spot;
        }
    }

    fn is_empty(&self) -> bool {
        self.slots == 0
    }

    fn is_side_empty(&self, player: State) -> bool {
        assert!(player != State::Empty);
        let big_shift = match player {
            State::X => 0,
            State::O => 32,
            State::Empty => unsafe { unreachable_unchecked() }
        };
        self.slots & (0xFFFF_FFFF << big_shift) == 0
    }

    fn filled_slots(&self, player: State) -> IterBits {
        assert!(player != State::Empty);
        let big_shift = match player {
            State::X => 0,
            State::O => 32,
            State::Empty => unsafe { unreachable_unchecked() }
        };
        IterBits { bitboard: (self.slots >> big_shift) & 0xFFFF_FFFF }
    }

    fn test(&self, slot: usize, player: State) -> bool {
        assert!(player != State::Empty);
        let big_shift = match player {
            State::X => 0,
            State::O => 32,
            State::Empty => unsafe { unreachable_unchecked() }
        };
        let bit = 1 << (slot + big_shift);
        (self.slots & bit) != 0
    }

    fn set(&mut self, slot: usize, player: State) {
        assert!(player != State::Empty);
        let big_shift = match player {
            State::X => 0,
            State::O => 32,
            State::Empty => unsafe { unreachable_unchecked() }
        };
        let bit = 1 << (slot + big_shift);
        self.slots |= bit;
    }

    fn count(&self, player: State) -> u32 {
        assert!(player != State::Empty);
        let big_shift = match player {
            State::X => 0,
            State::O => 32,
            State::Empty => unsafe { unreachable_unchecked() }
        };
        ((self.slots >> big_shift) & 0xFFFF_FFFF).count_ones()
    }

    fn char_at(&self, slot: usize) -> char {
        if self.test(slot, State::X) {
            'X'
        } else if self.test(slot, State::O) {
            'O'
        } else {
            '.'
        }
    }

    fn chars_at(&self, slot: usize) -> (char, char) {
        let left = if self.test(slot, State::X) {
            'X'
        } else {
            '.'
        };
        let right = if self.test(slot, State::O) {
            'O'
        } else {
            '.'
        };
        (left, right)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ur {
    slots: Board,
    pots: [u8; 2],
    moves: usize,
    last_roll: Option<usize>,
    rolls: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    from: usize,
    to: usize,
    capture: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UrMove {
    Move(Move),
    Roll(usize),
    Pass,
}

impl Ur {
    pub fn new() -> Self {
        Self {
            slots: Board::new(),
            moves: 0,
            pots: [STARTING_PIECES as u8; 2],
            last_roll: None,
            rolls: Vec::new(),
        }
    }

    fn dice_roll() -> usize {
        // Movements are determined by rolling a set of four-sided, tetrahedron-shaped dice.
        // Two of the four corners of each die are marked and the other two are not, 
        // giving each die an equal chance of landing with a marked or unmarked corner facing up.
        // The number of marked ends facing upwards after a roll of the dice indicates how many
        // spaces a player may move during that turn.

        let mut rng = rand::thread_rng();
        let mut sum = 0;
        for _ in 0..4 {
            sum += rng.gen_bool(0.5) as usize;
        }
        sum
    }
}

impl Display for UrMove {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            UrMove::Move(m) => write!(f, "{}-{}", m.from, m.to),
            UrMove::Roll(r) => write!(f, "roll {}", r),
            UrMove::Pass => write!(f, "pass"),
        }
    }
}

impl Game for Ur {
    type Move = UrMove;
    
    fn turn(&self) -> i8 {
        if self.moves & 1 == 0 {
            1
        } else {
            -1
        }
    }

    fn evaluate(&self) -> i8 {
        if self.pots[0] == 0 && self.slots.is_side_empty(State::X) {
            1
        } else if self.pots[1] == 0 && self.slots.is_side_empty(State::O) {
            -1
        } else {
            0
        }
    }

    fn is_terminal(&self) -> bool {
        self.evaluate() != 0
    }

    fn generate_moves(&self, buffer: &mut Vec<Self::Move>) {
        match self.last_roll {
            Some(roll) => {
                if roll == 0 {
                    // add a pass if we can't do anything
                    if buffer.is_empty() {
                        buffer.push(UrMove::Pass);
                    }
                    return;
                }
                let us = if self.turn() == 1 { State::X } else { State::O };
                // add all the moves of pieces on the board
                for from in self.slots.filled_slots(us) {
                    let to = std::cmp::min(from + roll, 14);
                    if !self.slots.test(to, us) {
                        let capture = self.slots.test(to, -us);
                        buffer.push(UrMove::Move(Move {
                            from,
                            to,
                            capture,
                        }));
                    }
                }
                // add a move for adding a new piece
                if self.pots[self.moves & 1] > 0 && roll != 0 {
                    let to = roll - 1;
                    if !self.slots.test(to, us) {
                        buffer.push(UrMove::Move(Move {
                            from: FROM_POT,
                            to,
                            capture: false,
                        }));
                    }
                }
            },
            None => {
                // add the roll moves
                for i in 0..=4 {
                    buffer.push(UrMove::Roll(i));
                }
            },
        }
    }

    fn push(&mut self, m: Self::Move) {
        match m {
            UrMove::Move(m) => {
                let us = self.turn();
                let us = if us == 1 { State::X } else { State::O };
                self.slots.advance(m, us);
                if m.capture {
                    self.pots[(self.moves & 1) ^ 1] += 1;
                }
                if m.from == FROM_POT {
                    self.pots[self.moves & 1] -= 1;
                }
                let landed_on_rosette = ROSETTE_SQUARES.contains(&m.to);
                if !landed_on_rosette {
                    // we don't count the move if we landed on a rosette
                    // so the player gets another turn.
                    self.moves += 1;
                }
                self.rolls.push(self.last_roll.unwrap());
                self.last_roll = None;
            },
            UrMove::Roll(roll) => {
                self.last_roll = Some(roll);
            },
            UrMove::Pass => {
                self.moves += 1;
                self.rolls.push(self.last_roll.unwrap());
                self.last_roll = None;
            },
        }
    }

    fn pop(&mut self, m: Self::Move) {
        match m {
            UrMove::Move(m) => {
                let landed_on_rosette = ROSETTE_SQUARES.contains(&m.to);
                if !landed_on_rosette {
                    // we don't count the move if we landed on a rosette
                    // so the player gets another turn.
                    self.moves -= 1;
                }
                let us = self.turn();
                let us = if us == 1 { State::O } else { State::X };
                self.slots.revert(m, us);
                if m.capture {
                    self.pots[(self.moves & 1) ^ 1] -= 1;
                }
                if m.from == FROM_POT {
                    self.pots[self.moves & 1] += 1;
                }
                self.last_roll = self.rolls.pop();
            },
            UrMove::Roll(_) => {
                self.last_roll = None;
            },
            UrMove::Pass => {
                self.moves -= 1;
                self.last_roll = self.rolls.pop();
            },
        }
    }

    fn action_space_size(&self) -> usize {
        21
    }

    fn to_move(&self) -> ToMove {
        match self.last_roll {
            Some(_) => {
                match self.turn() {
                    1 => ToMove::Max,
                    -1 => ToMove::Min,
                    _ => unreachable!(),
                }
            }
            None => ToMove::Chance,
        }
    }
}

impl PartiallySolvable for Ur {
    fn heuristic(&self) -> i32 {
        const MATE_SCORE: i32 = 1_000_000;
        const IN_POT_PENALTY: i32 = 50;
        const PROGRESS_SCORE: i32 = 100;
        const FINISHING_SCORE: i32 = 2000;

        if self.is_terminal() {
            return self.evaluate() as i32 * MATE_SCORE;
        }

        // it's good to have pieces further forward,
        // and it's good to have more pieces on the board
        // as you will have more choices to make.
        // it's even better to have moved pieces off the board.
        let mut score = 0;
        for x_pos in self.slots.filled_slots(State::X) {
            score += x_pos as i32 * PROGRESS_SCORE;
        }
        for o_pos in self.slots.filled_slots(State::O) {
            score -= o_pos as i32 * PROGRESS_SCORE;
        }
        score -= self.pots[0] as i32 * IN_POT_PENALTY;
        score += self.pots[1] as i32 * IN_POT_PENALTY;
        let total_xs = self.slots.count(State::X) as i32 + self.pots[0] as i32;
        let total_os = self.slots.count(State::O) as i32 + self.pots[1] as i32;
        score += (STARTING_PIECES - total_xs) * FINISHING_SCORE;
        score -= (STARTING_PIECES - total_os) * FINISHING_SCORE;
        score
    }
}

impl StochasticGame for Ur {
    fn generate_legal_moves_with_probabilities(&self, buffer: &mut Vec<(Self::Move, f32)>) {
        assert!(self.last_roll.is_none(), "generate_legal_moves_with_probabilities called on a non-chance node");
        const MOVE_PROBS: [(UrMove, f32); 5] = [
            (UrMove::Roll(0), 0.0625),
            (UrMove::Roll(1), 0.2500),
            (UrMove::Roll(2), 0.3750),
            (UrMove::Roll(3), 0.2500),
            (UrMove::Roll(4), 0.0625),
        ];
        buffer.extend_from_slice(&MOVE_PROBS);
    }
}

impl Display for Ur {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // the first line is x3 -> x0, then x13 -> x12.
        // the second line is 4 -> 11.
        // the third line is o3 -> o0, then o13 -> o12.
        // then we'll do dice roll and pots.
        writeln!(f, "{} {} {} {}     {} {}",
            self.slots.chars_at(3).0, self.slots.chars_at(2).0, self.slots.chars_at(1).0, self.slots.chars_at(0).0, self.slots.chars_at(13).0, self.slots.chars_at(12).0,
        )?;
        writeln!(f, "{} {} {} {} {} {} {} {}",
            self.slots.char_at(4), self.slots.char_at(5), self.slots.char_at(6), self.slots.char_at(7), self.slots.char_at(8), self.slots.char_at(9), self.slots.char_at(10), self.slots.char_at(11),
        )?;
        writeln!(f, "{} {} {} {}     {} {}",
            self.slots.chars_at(3).1, self.slots.chars_at(2).1, self.slots.chars_at(1).1, self.slots.chars_at(0).1, self.slots.chars_at(13).1, self.slots.chars_at(12).1,
        )?;
        writeln!(f)?;
        writeln!(f, "Move {}", self.moves + 1)?;
        match self.last_roll {
            Some(roll) => write!(f, "Roll: {} | ", roll)?,
            None => write!(f, "Roll: ? | ")?,
        }

        writeln!(f, "Pots: {} X, {} O", self.pots[0], self.pots[1])
    }
}

#[cfg(test)]
mod tests {
    use crate::perft::perft;

    #[test]
    fn startpos_legal_moves() {
        use super::*;
        let g = Ur::new();
        let mut moves = Vec::new();
        g.generate_moves(&mut moves);
        assert_eq!(moves.len(), 5);
    }

    #[test]
    fn moves_after_roll() {
        use super::*;
        let mut g = Ur::new();
        g.push(UrMove::Roll(0));
        let mut moves = Vec::new();
        g.generate_moves(&mut moves);
        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn perft_1() {
        use super::*;
        let mut board = Ur::new();
        let before = board.clone();
        let count = perft(&mut board, 1);
        assert_eq!(count, 5);
        assert_eq!(board, before);
    }

    #[test]
    fn perft_2() {
        use super::*;
        let mut board = Ur::new();
        let before = board.clone();
        let count = perft(&mut board, 2);
        assert_eq!(count, 5);
        assert_eq!(board, before);
    }

    #[test]
    fn perft_3() {
        use super::*;
        let mut board = Ur::new();
        let before = board.clone();
        let count = perft(&mut board, 3);
        assert_eq!(count, 25);
        assert_eq!(board, before);
    }

    #[test]
    fn perft_4() {
        use super::*;
        let mut board = Ur::new();
        let before = board.clone();
        let count = perft(&mut board, 4);
        assert_eq!(count, 28);
        assert_eq!(board, before);
    }

    #[test]
    fn print() {
        use super::*;
        use std::fmt::Write;
        let mut board = Ur::new();
        board.push(UrMove::Roll(1));
        board.push(UrMove::Move(
            Move {
                from: 14,
                to: 0,
                capture: false,
            }
        ));
        let mut s = String::new();
        write!(s, "{}", board).unwrap();
        assert_eq!(s, r". . . X     . .
. . . . . . . .
. . . .     . .

Move 2
Roll: ? | Pots: 6 X, 7 O
");
    }

    #[test]
    fn make_unmake() {
        use super::*;
        for r in 1..=4 {
            let mut g = Ur::new();
            g.push(UrMove::Roll(r));
            let before = g.clone();
            g.push(UrMove::Move(
                Move {
                    from: 14,
                    to: r - 1,
                    capture: false,
                }
            ));
            g.pop(UrMove::Move(
                Move {
                    from: 14,
                    to: r - 1,
                    capture: false,
                }
            ));
            assert_eq!(g, before);
        }
    }

    #[test]
    fn rosette() {
        use super::*;
        let mut g = Ur::new();
        let before = g.clone();
        let moves = [
            UrMove::Roll(4),
            UrMove::Move(
                Move {
                    from: 14,
                    to: 3,
                    capture: false,
                }
            ),
            UrMove::Roll(4),
            UrMove::Move(
                Move {
                    from: 3,
                    to: 7,
                    capture: false,
                }
            )];
        for m in &moves {
            g.push(*m);
        }
        for m in moves.iter().rev() {
            g.pop(*m);
        }
        assert_eq!(g, before);
    }
}