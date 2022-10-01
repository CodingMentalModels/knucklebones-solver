use crate::board::board::{Board, Move, Outcome, Player};
use crate::tree::tree::{Node, NodeType};

pub struct Solver {
    root: Node,
}

impl Solver {
    pub fn from_root(root: Node) -> Self {
        Solver {
            root,
        }
    }

    pub fn get_root(&self) -> &Node {
        &self.root
    }

    pub fn get_evaluation_tree(&mut self, solver_mode: SolverMode) -> Result<(Option<Node>, Evaluation), String> {
        match solver_mode {
            SolverMode::BruteForce => self.get_evaluation_tree_brute_force(),
            SolverMode::Heuristic((depth, f)) => 
                self.get_evaluation_tree_heuristic(depth, f),
            SolverMode::Hybrid(max_moves_left_before_brute_force, (depth, f)) => 
                self.get_evaluation_tree_hybrid(max_moves_left_before_brute_force, depth, f),
        }
    }

    pub fn get_best_moves_and_evaluation(&mut self, solver_mode: SolverMode) -> Result<(Vec<Move>, Evaluation), String> {
        self.get_evaluation_tree(solver_mode).map(
            |(maybe_tree, evaluation)|
            (Self::get_best_moves_from_evaluation_tree(maybe_tree).expect("Guaranteed to be a Move Node"), evaluation)
        )
    }

    fn get_evaluation_tree_brute_force(&mut self) -> Result<(Option<Node>, Evaluation), String> {
        self.root.build_entire_tree_up_to_symmetry();
        return self.root.get_evaluation_tree(
            |x| Evaluation::from_outcome(
                x.get_outcome()
            ).expect("Outcome will be known at all leaf nodes.")
            .get_evaluation(),
        ).map(
            |(maybe_tree, evaluation)|
            (maybe_tree, Evaluation::new(evaluation))
        );
    }

    fn get_evaluation_tree_heuristic(&mut self, depth: usize, objective_function: fn(&Node) -> f32) -> Result<(Option<Node>, Evaluation), String> {
        self.root.build_n_moves_up_to_symmetry(depth);
        self.root.get_evaluation_tree(objective_function)
            .map(
                |(maybe_tree, evaluation)|
                (maybe_tree, Evaluation::new(evaluation))
            )
    }

    fn get_evaluation_tree_hybrid(&mut self, max_moves_left_before_brute_force: usize, depth: usize, objective_function: fn(&Node) -> f32) -> Result<(Option<Node>, Evaluation), String> {
        if self.root.get_moves_left_ignoring_elimination() <= max_moves_left_before_brute_force {
            self.get_evaluation_tree_brute_force()
        } else {
            self.get_evaluation_tree_heuristic(depth, objective_function)
        }
    }

    fn get_best_moves_from_evaluation_tree(maybe_tree: Option<Node>) -> Result<Vec<Move>, String> {
        match maybe_tree {
            Some(tree) => {
                match tree.get_node_type() {
                    NodeType::Roll(_) => Err("Roll node can't be the root of an evaluation tree.".to_string()),
                    NodeType::Move(_, _) => Ok(tree.get_moves().expect("Guaranteed to be a move node.")),
                }
            },
            None => Ok(vec![]),
        }
    }

    pub fn get_evaluation(&mut self, solver_mode: SolverMode) -> Result<Evaluation, String> {
        self.get_best_moves_and_evaluation(solver_mode).map(|(_, evaluation)| evaluation)
    }

    pub fn difference_heuristic(node: &Node, empty_square_fill: f32) -> f32 {
        // Heuristic == Difference in current score, attributing empty_square_fill to squares that will get played assuming no eliminations.
        let difference = node.get_score_difference();
        if node.is_game_over() {
            return difference as f32;
        };
        let mut player_1_empty_squares = node.get_player_1_board().get_n_empty_squares() as f32;
        let mut player_2_empty_squares = node.get_player_2_board().get_n_empty_squares() as f32;
        let finishing_first = if player_1_empty_squares > player_2_empty_squares {
            Player::Player2
        } else if player_1_empty_squares < player_2_empty_squares {
            Player::Player1
        } else {
            node.get_active_player()
        };
        let finishing_first_bonus = if node.get_active_player() == finishing_first {
            1.0
        } else {
            -1.0
        };
        let empty_square_raw_difference = match finishing_first {
            Player::Player1 => (player_2_empty_squares - player_1_empty_squares + finishing_first_bonus),
            Player::Player2 => -(player_1_empty_squares - player_2_empty_squares + finishing_first_bonus),
        };
        let empty_square_heuristic = empty_square_raw_difference * empty_square_fill;
        return (difference as f32) + empty_square_heuristic;
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

    pub fn from_perspective(&self, perspective: Player) -> Self {
        match perspective {
            Player::Player1 => Evaluation(self.0),
            Player::Player2 => Evaluation(-self.0),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:.2}", self.0)
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
    Heuristic(HeuristicDepthAndObjective),
    Hybrid(BruteForceMaxMovesRemaining, HeuristicDepthAndObjective),
}

pub type HeuristicDepthAndObjective = (usize, fn(&Node) -> f32);
pub type BruteForceMaxMovesRemaining = usize;

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
                SolverMode::Heuristic((1, |x| Solver::difference_heuristic(x, 3.5))),
            ).unwrap();
        assert_eq!(
            result,
            (
                vec![Move::new(0, 0), Move::new(0, 1), Move::new(0, 2)],
                Evaluation::new(6.0)
            )
        );

        let player_1_board = Board::from_string("651\n142\n62_".to_string()).unwrap(); // 40 before move.
        let player_2_board = Board::from_string("256\n1_2\n62_".to_string()).unwrap(); // 24 before move.
        // Player 2 has two moves:
        // (1, 1) => 30
        // (2, 2) => 44
        // Player 1 has one move, (1, 1):
        // Based on rolls, that means the score is:
        // 1 => 45 => Player 1 wins and diff = 1
        // 2 => 47 + 4 (due to elimination of the 2) => Player 1 wins and diff = 7
        // 3 => 43 => Player 2 wins iff Player 2 played (2, 2), else Player 1 wins.  diff = -1
        // 4 => 44 => Draws iff Player 2 played (2, 2), else wins.  Diff = 0.
        // 5 => 45 => Player 1 wins.  Diff = 1.
        // 6 => 46 + 26 (due to elimination of 2 6's) => PLayer 1 wins.  Diff = 28
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Six));
        let mut solver = Solver::from_root(root);
        let (best_moves, evaluation) = solver.get_best_moves_and_evaluation(SolverMode::Heuristic((5, |x| Solver::difference_heuristic(x, 3.5)))).unwrap();
        assert_eq!((best_moves, evaluation), (vec![Move::new(2, 2)], Evaluation::new((1. + 7. - 1. + 0. + 1. + 28.)/6.)));
    }

    #[test]
    fn test_solver_solves_hybrid() {
        let player_1_board = Board::from_string("255\n1_2\n352".to_string()).unwrap();
        let player_2_board = Board::from_string("15_\n333\n12_".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::Six));
        let mut solver = Solver::from_root(root);
        let (best_moves, evaluation) = solver.get_best_moves_and_evaluation(SolverMode::Hybrid(5, (4, |x| Solver::difference_heuristic(x, 3.5)))).unwrap();
        assert_eq!(best_moves, vec![Move::new(1, 1)]);
        assert_eq!(evaluation, Evaluation::new(1.0));


        let player_1_board = Board::empty();
        let player_2_board = Board::empty();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::Six));
        let mut solver = Solver::from_root(root);
        let result = solver
            .get_best_moves_and_evaluation(
                SolverMode::Hybrid(5, (1, |x| Solver::difference_heuristic(x, 3.5))),
            ).unwrap();
        assert_eq!(
            result,
            (
                vec![Move::new(0, 0), Move::new(0, 1), Move::new(0, 2)],
                Evaluation::new(6.0)
            )
        );

        let player_1_board = Board::from_string("35_\n___\n___".to_string()).unwrap();
        let player_2_board = Board::from_string("__4\n__4\n___".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::Six));
        let mut solver = Solver::from_root(root);
        let (evaluation_tree, evaluation) = solver.get_evaluation_tree(SolverMode::Hybrid(1, (2, |x| Solver::difference_heuristic(x, 3.5)))).unwrap();
        assert_eq!(evaluation_tree.clone().unwrap().get_max_depth(), 5);
        assert_eq!(evaluation_tree.unwrap().get_child_from_move(Move::new(1, 1)).unwrap().get_child_from_roll(Die::Six).unwrap().get_n_children(), 1);

        // Perf Test -- Any more moves then this and it stack overflows.
        let player_1_board = Board::from_string("62_\n1_2\n62_".to_string()).unwrap();
        let player_2_board = Board::from_string("356\n122\n62_".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Six));
        let mut solver = Solver::from_root(root);
        let (best_moves, evaluation) = solver.get_best_moves_and_evaluation(SolverMode::Hybrid(5, (4, |x| Solver::difference_heuristic(x, 3.5)))).unwrap();
        assert!(evaluation >= Evaluation::new(-1.0));
        assert!(evaluation <= Evaluation::new(1.0));

    }

    #[test]
    fn test_difference_heuristic() {

        let player_1_board = Board::from_string("111\n111\n11_".to_string()).unwrap(); // 22
        let player_2_board = Board::from_string("222\n222\n22_".to_string()).unwrap(); // 44
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::One));
        assert_eq!(Solver::difference_heuristic(&root, 0.), -22.0);
        assert_eq!(Solver::difference_heuristic(&root, 3.5), -22.0 + 3.5);

        root.add_move(Move::new(2, 2));
        let after_move = root.get_child_from_move(Move::new(2, 2)).unwrap();
        assert!(after_move.is_game_over());
        assert_eq!(Solver::difference_heuristic(&after_move, 0.), 27.0 - 44.0);
        assert_eq!(Solver::difference_heuristic(&after_move, 3.5), 27.0 - 44.0);

        let player_1_board = Board::from_string("111\n111\n11_".to_string()).unwrap(); // 22
        let player_2_board = Board::from_string("222\n222\n22_".to_string()).unwrap(); // 44
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Two));
        assert_eq!(Solver::difference_heuristic(&root, 0.), 22.0 - 44.0);
        assert_eq!(Solver::difference_heuristic(&root, 3.5), 22.0 - 44.0 - 3.5);

        root.add_move(Move::new(2, 2));
        let after_move = root.get_child_from_move(Move::new(2, 2)).unwrap();
        assert!(after_move.is_game_over());
        assert_eq!(Solver::difference_heuristic(&after_move, 0.), 22.0 - 54.0);
        assert_eq!(Solver::difference_heuristic(&after_move, 3.5), 22.0 - 54.0);

        let player_1_board = Board::empty();
        let player_2_board = Board::empty();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::Six));
        assert_eq!(Solver::difference_heuristic(&root, 0.), 0.0);
        assert_eq!(Solver::difference_heuristic(&root, 3.5), 3.5);

        root.add_move(Move::new(0, 0));
        let after_move = root.get_child_from_move(Move::new(0, 0)).unwrap();
        assert_eq!(Solver::difference_heuristic(&after_move, 0.), 6.0);
        assert_eq!(Solver::difference_heuristic(&after_move, 3.5), 6.0);

        let player_1_board = Board::empty();
        let player_2_board = Board::empty();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::One));
        assert_eq!(Solver::difference_heuristic(&root, 0.), 0.0);
        assert_eq!(Solver::difference_heuristic(&root, 3.5), 3.5);

        root.add_move(Move::new(0, 0));
        let after_move = root.get_child_from_move(Move::new(0, 0)).unwrap();
        assert_eq!(Solver::difference_heuristic(&after_move, 0.), 1.0);
        assert_eq!(Solver::difference_heuristic(&after_move, 3.5), 1.0);

        let player_1_board = Board::from_string("__1\n___\n___".to_string()).unwrap();
        let player_2_board = Board::empty();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Six));
        assert_eq!(Solver::difference_heuristic(&root, 0.), 1.0);
        assert_eq!(Solver::difference_heuristic(&root, 3.5), 1.0);

        root.add_move(Move::new(0, 0));
        let after_move = root.get_child_from_move(Move::new(0, 0)).unwrap();
        assert_eq!(Solver::difference_heuristic(&after_move, 0.), -5.0);
        assert_eq!(Solver::difference_heuristic(&after_move, 3.5), -1.5);

        let player_1_board = Board::from_string("651\n142\n62_".to_string()).unwrap(); // 40 before move.
        let player_2_board = Board::from_string("256\n1_2\n62_".to_string()).unwrap(); // 24 before move.
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Six));
        assert_eq!(Solver::difference_heuristic(&root, 0.), 40.0 - 24.0);
        assert_eq!(Solver::difference_heuristic(&root, 3.5), 40.0 - 24.0);

        root.build_n_moves_up_to_symmetry(5);
        let after_final_move = root
            .get_child_from_move(Move::new(2, 2)).unwrap()
            .get_child_from_roll(Die::Two).unwrap()
            .get_child_from_move(Move::new(2, 2)).unwrap();
        assert_eq!(Solver::difference_heuristic(&after_final_move, 0.0), 7.0);
        assert_eq!(Solver::difference_heuristic(&after_final_move, 3.5), 7.0);
    }

}