use std::hash::Hash;
use std::fmt::Display;

pub trait Game: Clone + PartialEq + Eq + Hash + Display {
    type Move: Copy + Display;

    fn turn(&self) -> i32;
    fn evaluate(&self) -> i32;
    fn is_terminal(&self) -> bool;
    fn generate_legal_moves(&self, buffer: &mut Vec<Self::Move>);
    fn push(&mut self, m: Self::Move);
    fn pop(&mut self, m: Self::Move);
    fn action_space_size(&self) -> usize;

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