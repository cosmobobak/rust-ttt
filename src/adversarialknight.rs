use std::fmt::Display;

use crate::game::{Game, ToMove, Keyed};

macro_rules! cfor {
    ($init: stmt; $cond: expr; $step: expr; $body: block) => {
        {
            $init
            while $cond {
                $body;

                $step;
            }
        }
    }
}

const fn init_hash_keys() -> [u64; 64] {
    let mut keys = [0; 64];
    
    cfor!(let mut sq = 0; sq < 64; sq += 1; {
        keys[sq] = 1 << sq;
    });

    keys
}

static KNIGHTLOC_HASHKEYS: [u64; 64] = init_hash_keys();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdversarialKnight {
    knightloc: isize,
    moves: usize,
    visited: u64,
}

impl AdversarialKnight {
    pub fn new() -> Self {
        Self {
            knightloc: 32,
            moves: 0,
            visited: KNIGHTLOC_HASHKEYS[32],
        }
    }

    pub fn visited(&self, loc: isize) -> bool {
        self.visited & (1 << loc) != 0
    }
}

impl Display for AdversarialKnight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.knightloc)
    }
}

const fn col_of(loc: isize) -> isize {
    loc % 8
}
const fn row_of(loc: isize) -> isize {
    loc / 8
}
static KNIGHT_MOVE_OFFSETS: [isize; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];

impl Game for AdversarialKnight {
    type Move = usize;

    fn turn(&self) -> i8 {
        if self.moves & 1 == 0 {
            1
        } else {
            -1
        }
    }

    fn evaluate(&self) -> i8 {
        let mut buf = Vec::with_capacity(8);
        self.generate_moves(&mut buf);
        if buf.is_empty() {
            return -self.turn();
        }
        0
    }

    fn is_terminal(&self) -> bool {
        self.moves == 64 || self.evaluate() != 0
    }

    fn generate_moves(&self, buffer: &mut Vec<Self::Move>) {
        let knightloc = self.knightloc;
        for offset in KNIGHT_MOVE_OFFSETS.iter() {
            let newloc = knightloc + *offset;
            if (0..64).contains(&newloc) && !self.visited(newloc) && col_of(newloc).abs_diff(col_of(knightloc)) <= 2 {
                buffer.push(newloc as usize);
            }
        }
    }

    fn push(&mut self, m: Self::Move) {
        self.knightloc = m as isize;
        self.moves += 1;
        self.visited ^= KNIGHTLOC_HASHKEYS[m as usize];
    }

    fn pop(&mut self, m: Self::Move) {
        self.visited ^= KNIGHTLOC_HASHKEYS[self.knightloc as usize];
        self.moves -= 1;
    }

    fn action_space_size(&self) -> usize {
        8
    }

    fn to_move(&self) -> crate::game::ToMove {
        if self.moves & 1 == 0 {
            ToMove::Max
        } else {
            ToMove::Min
        }
    }
}

impl Keyed for AdversarialKnight {
    fn hashkey(&self) -> u64 {
        self.visited
    }
}