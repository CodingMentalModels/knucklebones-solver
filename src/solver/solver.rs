use crate::board::board::{Board, Move, Outcome, Player};
use crate::tree::tree::{Node};

pub struct Solver {
    root: Node,
}

impl Solver {
    pub fn from_root(root: Node) -> Self {
        Solver {
            root,
        }
    }

    pub fn get_best_moves_and_evaluation(&mut self, solver_mode: SolverMode) -> Result<(Vec<Move>, Evaluation), String> {
        match solver_mode {
            SolverMode::BruteForce => self.get_best_moves_and_evaluation_brute_force(),
            SolverMode::Heuristic => self.get_best_moves_and_evaluation_heuristic(),
        }
    }

    fn get_best_moves_and_evaluation_brute_force(&mut self) -> Result<(Vec<Move>, Evaluation), String> {
        self.root.build_entire_tree_up_to_symmetry();
        return self.root.get_next_moves_and_evaluation(
            |x| Evaluation::from_outcome(
                x.get_outcome()
            ).expect("Outcome will be known at all leaf nodes.")
            .get_evaluation(),
        ).map(|(moves, evaluation)| (moves, Evaluation::new(evaluation)));
    }

    fn get_best_moves_and_evaluation_heuristic(&self) -> Result<(Vec<Move>, Evaluation), String> {
        unimplemented!()
    }

    pub fn get_evaluation(&mut self, solver_mode: SolverMode) -> Result<Evaluation, String> {
        self.get_best_moves_and_evaluation(solver_mode).map(|(_, evaluation)| evaluation)
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

    fn from_outcome(outcome: Outcome) -> Result<Self, String> {
        match outcome {
            Outcome::Victory(Player::Player1) => Ok(Evaluation::new(1.0)),
            Outcome::Victory(Player::Player2) => Ok(Evaluation::new(-1.0)),
            Outcome::Draw => Ok(Evaluation::new(0.0)),
            Outcome::InProgress => Err("Cannot convert InProgress to Evaluation".to_string()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SolverMode {
    BruteForce,
    Heuristic,
}

#[cfg(test)]
mod test_solver {
    use crate::{tree::tree::NodeType, board::board::Die};

    use super::*;

    #[test]
    fn test_solver_solves_endgame_situations() {
        let player_1_board = Board::from_string("255\n1_2\n352".to_string()).unwrap();
        let player_2_board = Board::from_string("15_\n333\n12_".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::Six));
        let mut solver = Solver::from_root(root);
        let (best_moves, evaluation) = solver.get_best_moves_and_evaluation(SolverMode::BruteForce).unwrap();
        assert_eq!(best_moves, vec![Move::new(1, 1)]);
        assert_eq!(evaluation, Evaluation::new(1.0));

        let player_1_board = Board::from_string("255\n1_2\n352".to_string()).unwrap();
        let player_2_board = Board::from_string("255\n1_2\n652".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board, NodeType::Roll(Player::Player1));
        let mut solver = Solver::from_root(root);
        let result = solver.get_best_moves_and_evaluation(SolverMode::BruteForce);
        assert!(result.is_err());

        let player_1_board = Board::from_string("651\n142\n62_".to_string()).unwrap(); // 40 before move.
        let player_2_board = Board::from_string("256\n1_2\n62_".to_string()).unwrap(); // 24 before move.
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Six));
        let mut solver = Solver::from_root(root);
        let (best_moves, evaluation) = solver.get_best_moves_and_evaluation(SolverMode::BruteForce).unwrap();
        assert_eq!(best_moves, vec![Move::new(2, 2)]);
        assert_eq!(evaluation, Evaluation::new(4.*1. + 1.*0. + 1.*(-1.)));
    }

}