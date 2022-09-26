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
            SolverMode::Heuristic(depth, f) => self.get_best_moves_and_evaluation_heuristic(depth, f),
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

    fn get_best_moves_and_evaluation_heuristic(&mut self, depth: usize, objective_function: fn(&Node) -> f32) -> Result<(Vec<Move>, Evaluation), String> {
        self.root.build_n_moves_up_to_symmetry(depth);
        self.root.get_next_moves_and_evaluation(objective_function)
            .map(|(moves, evaluation)| (moves, Evaluation::new(evaluation)))
    }

    pub fn get_evaluation(&mut self, solver_mode: SolverMode) -> Result<Evaluation, String> {
        self.get_best_moves_and_evaluation(solver_mode).map(|(_, evaluation)| evaluation)
    }

    pub fn difference_heuristic(node: &Node, empty_square_fill: f32) -> f32 {
        let difference = node.get_score_difference();
        let player_1_empty_squares = node.get_player_1_board().get_n_empty_squares() as f32;
        let player_2_empty_squares = node.get_player_2_board().get_n_empty_squares() as f32;
        return (difference as f32) + empty_square_fill * (player_1_empty_squares - player_2_empty_squares) as f32;
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

#[derive(Copy, Clone)]
pub enum SolverMode {
    BruteForce,
    Heuristic(usize, fn(&Node) -> f32),
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
        let player_2_board = Board::from_string("256\n1_2\n62_".to_string()).unwrap(); // 28 before move.
        // Player 2 has two moves:
        // (1, 1) => 30
        // (2, 2) => 44
        // Player 1 has one move, (1, 1):
        // Based on rolls, that means the score is:
        // 1 => 45 => Player 1 wins
        // 2 => 47 => Player 1 wins
        // 3 => 43 => Player 2 wins iff Player 2 played (2, 2), else Player 1 wins
        // 4 => 44 => Draws iff Player 2 played (2, 2), else wins
        // 5 => 45 => Player 1 wins
        // 6 => 46 => PLayer 1 wins
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Six));
        let mut solver = Solver::from_root(root);
        let (best_moves, evaluation) = solver.get_best_moves_and_evaluation(SolverMode::BruteForce).unwrap();
        assert_eq!((best_moves, evaluation), (vec![Move::new(2, 2)], Evaluation::new((4.*1. + 1.*0. + 1.*(-1.))/6.)));
    }

    #[test]
    fn test_solver_solves_heuristically() {
        let player_1_board = Board::empty();
        let player_2_board = Board::empty();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::Six));
        let mut solver = Solver::from_root(root);
        let result = solver
            .get_best_moves_and_evaluation(
                SolverMode::Heuristic(1, |x| Solver::difference_heuristic(x, 3.5)),
            ).unwrap();
        assert_eq!(
            result,
            (
                vec![Move::new(0, 0), Move::new(0, 1), Move::new(0, 2)],
                Evaluation::new(2.5)
            )
        );
    }

}