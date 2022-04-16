use std::hash::Hash;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToMove {
    Max,
    Min,
    Chance,
}

pub trait Game: Clone + PartialEq + Eq + Hash + Display {
    type Move: Copy + Display;

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

    fn print_outcome(&self) {
        if !self.is_terminal() {
            println!("nonterminal state.");
            return;
        }

        let eval = self.evaluate();
        match eval {
            0 => println!("1/2-1/2"),
            1 => println!("1-0"),
            -1 => println!("0-1"),
            _ => println!("nonstandard terminal state value: {}", eval)
        }
    }
}