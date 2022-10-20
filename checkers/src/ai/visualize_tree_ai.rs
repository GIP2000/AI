use crate::board::Moves;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RTTree {
    pub h_val: i32,
    pub alpha: i32,
    pub beta: i32,
    pub mv: Moves,
    pub is_max: bool,
    pub pruned: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Tree<T: Serialize> {
    pub val: T,
    next: Vec<Option<Tree<T>>>,
}

impl<T: Serialize> Tree<T> {
    pub fn new(val: T) -> Self {
        Self { val, next: vec![] }
    }

    pub fn push(&mut self, next: Self) {
        self.next.push(Some(next));
    }
}
