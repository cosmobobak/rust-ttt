use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToMove {
    Max,
    Min,
    Chance,
}

pub trait Game: Clone + Eq + Hash {
    type Move: Copy;

    fn turn(&self) -> i8;
    fn evaluate(&self) -> i8;
    fn is_terminal(&self) -> bool;
    fn generate_moves(&self, buffer: &mut Vec<Self::Move>);
    fn push(&mut self, m: Self::Move);
    fn pop(&mut self, m: Self::Move);
    fn action_space_size(&self) -> usize;
    fn to_move(&self) -> ToMove;
}

pub trait Keyed: Game {
    fn hashkey(&self) -> u64;
}

pub trait StochasticGame: Game {
    fn generate_legal_moves_with_probabilities(&self, buffer: &mut Vec<(Self::Move, f32)>);
}

pub trait PartiallySolvable: Game {
    fn heuristic(&self) -> i32;
}