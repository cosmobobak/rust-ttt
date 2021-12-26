use std::hash::Hash;

pub trait Game: Clone + PartialEq + Eq + Hash {
    type Move: Copy;

    fn turn(&self) -> i32;
    fn evaluate(&self) -> i32;
    fn is_terminal(&self) -> bool;
    fn generate_legal_moves(&self, buffer: &mut Vec<Self::Move>);
    fn push(&mut self, m: Self::Move);
    fn pop(&mut self, m: Self::Move);
    fn action_space_size(&self) -> usize;
}
