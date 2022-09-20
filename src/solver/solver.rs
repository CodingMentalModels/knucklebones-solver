use crate::board::board::{Board, Move, Outcome, Player};
use crate::tree::tree::{Tree, Node};

pub struct Solver {
    tree: Tree,
}

impl Solver {
    pub fn from_board(board: Board) -> Self {
        unimplemented!()
    }

    pub fn get_evaluation(&self) -> Evaluation {
        unimplemented!()
    }

}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Evaluation(f32);

impl Evaluation {

    pub fn new(evaluation: f32) -> Self {
        Evaluation(evaluation)
    }

    pub fn get_evaluation(&self) -> f32 {
        self.0
    }

    pub fn to_string(&self) -> String {
        if self.0 > 0.9999 {
            return "X is Winning".to_string();
        } else if self.0 < -0.9999 {
            return "O is Winning".to_string();
        } else if self.0.abs() < 0.0001 {
            return "Drawn".to_string();
        } else {
            return "Ambiguous".to_string();
        }
    }
}


#[cfg(test)]
mod test_solver {
    use super::*;

}