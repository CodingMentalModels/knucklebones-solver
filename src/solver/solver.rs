use crate::board::board::{Board, Move, Outcome, Player};
use crate::tree::tree::{Tree, Node};

pub struct Solver {
    tree: Tree,
}

impl Solver {
    pub fn from_board(board: Board) -> Self {
        let tree = Tree::from_board(board);
        Solver { tree }
    }

    pub fn get_evaluation(&self) -> Evaluation {
        let root = self.tree.get_root();
        self.get_evaluation_and_line_for_node(root).0
    }

    fn get_evaluation_and_line_for_node(&self, node: &Node) -> (Evaluation, Vec<Move>) {
        let children = node.get_children().iter();
        if children.len() == 0 {
            return (Solver::get_evaluation_for_outcome(node.get_board().get_outcome()), Vec::new());
        }
        let active_player = node.get_active_player();
        let mut best_evaluation = Evaluation(
            match active_player {
                Some(Player::X) => -1.,
                Some(Player::O) => 1.,
                None => panic!("There's no active player even though there the node has children."),
            }
        );
        
        let mut best_move_line: Vec<Move> = Vec::new();
        for child_move in node.get_legal_moves().iter() {
            let child = node.get_child(child_move.get_row(), child_move.get_column()).expect("Move is legal by definition of get_legal_moves().");
            let (child_evaluation, child_line) = self.get_evaluation_and_line_for_node(child);
            match active_player {
                Some(Player::X) => {
                    if child_evaluation >= best_evaluation {
                        best_evaluation = child_evaluation;
                        best_move_line = vec![*child_move];
                        best_move_line.append(&mut child_line.clone());
                    }
                },
                Some(Player::O) => {
                    if child_evaluation <= best_evaluation {
                        best_evaluation = child_evaluation;
                        best_move_line = vec![*child_move];
                        best_move_line.append(&mut child_line.clone());
                    }
                },
                None => panic!("There's no active player even though there the node has children."),
            }
        }
        return (best_evaluation, best_move_line);
    }

    fn get_evaluation_for_outcome(outcome: Outcome) -> Evaluation {
        let raw_evaluation = match outcome {
            Outcome::InProgress => 0.,
            Outcome::Ambiguous => 0.,
            Outcome::Draw => 0.,
            Outcome::Victory(Player::X) => 1.,
            Outcome::Victory(Player::O) => -1.,
        };
        return Evaluation(raw_evaluation);
    }

    pub fn get_next_moves(&self) -> Result<Vec<Move>, String> {
        let (next_moves, _) = self.get_next_moves_and_evaluation()?;
        return Ok(next_moves);
    }

    pub fn get_next_moves_and_evaluation(&self) -> Result<(Vec<Move>, Evaluation), String> {

        let active_player = match self.tree.get_root().get_active_player() {
            Some(player) => player,
            None => return Err("The game is already over.".to_string()),
        };

        let root = self.tree.get_root();
        let mut next_moves = Vec::new();
        let mut best_evaluation = Evaluation(
            match active_player {
                Player::X => -2.,
                Player::O => 2.,
            }
        );

        for m in root.get_legal_moves().iter() {
            let child = root.get_child(m.get_row(), m.get_column()).unwrap();
            let (evaluation, _line) = self.get_evaluation_and_line_for_node(child);
            if ((evaluation > best_evaluation) && (active_player == Player::X)) ||
                ((evaluation < best_evaluation) && (active_player == Player::O)) {
                best_evaluation = evaluation;
                next_moves.clear();
                next_moves.push(*m);
            } else if evaluation == best_evaluation {
                next_moves.push(*m);
            }
        }
        return Ok((next_moves, best_evaluation));
    }

    pub fn get_evaluation_and_line(&self) -> (Evaluation, Vec<Move>) {
        let root = self.tree.get_root();
        self.get_evaluation_and_line_for_node(root)
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

    #[test]
    fn test_solver_gets_evaluation() {
        assert_eq!(
            Solver::from_board(
                Board::from_position(
                    "XOX
                    OXO
                    XOX",
                ).unwrap()
            ).get_evaluation(),
            Evaluation(1.)
        );
        
        assert_eq!(
            Solver::from_board(
                Board::from_position(
                    "XOX
                    O_O
                    XOX",
                ).unwrap()
            ).get_evaluation(),
            Evaluation(1.)
        );

        assert_eq!(
            Solver::from_board(
                Board::from_position(
                    "XOX
                    OO_
                    XXO",
                ).unwrap()
            ).get_evaluation(),
            Evaluation(0.)
        );

        assert_eq!(
            Solver::from_board(
                Board::from_position(
                    "XO_
                    OO_
                    XXO",
                ).unwrap()
            ).get_evaluation(),
            Evaluation(-1.)
        );

        
        assert!(
            Solver::from_board(
                Board::from_position(
                    "XO_
                    O__
                    XXO",
                ).unwrap()
            ).get_evaluation() < Evaluation(0.0001)
        );
    }

    #[test]
    fn test_solver_gets_line() {

        let board = Board::from_position(
            "XO_
            XOX
            O__"
        ).unwrap();
        assert_eq!(board.get_active_player(), Some(Player::X));
        let solver = Solver::from_board(board);

        let (evaluation, line) = solver.get_evaluation_and_line();
        assert_eq!(evaluation, Evaluation(-1.));
        assert_eq!(line, vec![Move { row: 2, col: 2 }, Move { row: 2, col: 1} ]);

        let board = Board::from_position(
            "XO_
            XOX
            ___"
        ).unwrap();
        assert_eq!(board.get_active_player(), Some(Player::O));
        let solver = Solver::from_board(board);

        let (evaluation, line) = solver.get_evaluation_and_line();
        assert_eq!(evaluation, Evaluation(-1.));
        assert_eq!(line, vec![Move { row: 2, col: 1 }]);

        let (best_moves, _) = solver.get_next_moves_and_evaluation().unwrap();
        assert_eq!(best_moves, vec![Move { row: 2, col: 0 }, Move { row: 2, col: 1 }]);

        let board = Board::from_position(
            "XOO
            _X_
            ___"
        ).unwrap();
        assert_eq!(board.get_active_player(), Some(Player::X));
        let solver = Solver::from_board(board);
        
        let (evaluation, line) = solver.get_evaluation_and_line();
        assert_eq!(evaluation, Evaluation(1.));
        assert_eq!(line, vec![Move::new(2, 2)]);

        let (best_moves, _) = solver.get_next_moves_and_evaluation().unwrap();
        assert_eq!(best_moves, vec![Move { row: 1, col: 0 }, Move { row: 1, col: 2 }, Move { row: 2, col: 0 }, Move { row: 2, col: 2 }]);

    }
}