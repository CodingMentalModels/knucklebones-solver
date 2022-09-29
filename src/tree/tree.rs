use std::fmt::Display;

use crate::board::board::{Board, Move, Outcome, Player, Die, Comparison};

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    player_1_board: Board,
    player_2_board: Board,
    node_type: NodeType,
    children: Vec<Node>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player 1:\n{}\nPlayer 2:\n{}\nType: {:?}\nN Children: {}", self.player_1_board, self.player_2_board, self.node_type, self.children.len())
    }
}

impl Node {

    pub fn empty() -> Self {
        Node {
            player_1_board: Board::empty(),
            player_2_board: Board::empty(),
            node_type: NodeType::Roll(Player::Player1),
            children: Vec::new(),
        }
    }

    pub fn new(player_1_board: Board, player_2_board: Board, node_type: NodeType) -> Self {
        Node {
            player_1_board,
            player_2_board,
            node_type,
            children: Vec::new(),
        }
    }

    pub fn from_player_and_boards(active_player: Player, active_players_board: Board, opponents_board: Board, node_type: NodeType) -> Self {
        match active_player {
            Player::Player1 => Node::new(active_players_board, opponents_board, node_type),
            Player::Player2 => Node::new(opponents_board, active_players_board, node_type),
        }
    }

    pub fn get_boards(&self) -> (Board, Board) {
        (self.player_1_board.clone(), self.player_2_board.clone())
    }

    pub fn get_player_1_board(&self) -> Board {
        self.player_1_board.clone()
    }

    pub fn get_player_2_board(&self) -> Board {
        self.player_2_board.clone()
    }

    pub fn get_node_type(&self) -> NodeType {
        self.node_type
    }

    pub fn get_player_board(&self, player: Player) -> Board {
        match player {
            Player::Player1 => self.player_1_board.clone(),
            Player::Player2 => self.player_2_board.clone(),
        }
    }

    pub fn get_die(&self) -> Option<Die> {
        match self.node_type {
            NodeType::Roll(_) => None,
            NodeType::Move(_, die) => Some(die),
        }
    }

    pub fn get_active_player(&self) -> Player {
        match self.node_type {
            NodeType::Roll(player) => player,
            NodeType::Move(player, _) => player,
        }
    }

    pub fn to_string_from_perspective(&self, player: Player) -> String {
        let maybe_roll_string = match self.node_type {
            NodeType::Roll(_) => "".to_string(),
            NodeType::Move(_, die) => die.to_string() + "\n",
        };
        match player {
            Player::Player1 => format!("Player:\n{}\n\nOpponent:\n{}\n\nRoll: {}\n", self.player_1_board, self.player_2_board, maybe_roll_string),
            Player::Player2 => format!("Player:\n{}\n\nOpponent:\n{}\n\nRoll: {}\n", self.player_2_board, self.player_1_board, maybe_roll_string),
        }
    }

    pub fn is_legal_move(&self, m: Move) -> bool {
        match self.node_type {
            NodeType::Roll(_) => false,
            NodeType::Move(player, _) => self.get_legal_moves().map_or(false, |moves| moves.contains(&m)),
        }
    }

    pub fn get_legal_moves(&self) -> Result<Vec<Move>, String> {
        match self.node_type {
            NodeType::Roll(player) => {
                return Err("Cannot get legal moves from a roll node".to_string());
            },
            NodeType::Move(player, die) => {
                let board = self.get_player_board(player);
                return Ok(board.get_empty_squares().iter().map(|square| Move::new(square.0, square.1)).collect());
            },
        }
    }

    pub fn get_legal_moves_up_to_row_symmetry(&self) -> Result<Vec<Move>, String> {
        match self.node_type {
            NodeType::Roll(player) => {
                return Err("Cannot get legal moves from a roll node".to_string());
            },
            NodeType::Move(player, die) => {
                let board = self.get_player_board(player);
                return Ok(board.get_empty_squares_up_to_row_symmetry().iter().map(|square| Move::new(square.0, square.1)).collect());
            },
        }
    }

    pub fn get_n_empty_squares(&self) -> usize {
        self.player_1_board.get_n_empty_squares() + self.player_2_board.get_n_empty_squares()
    }

    pub fn equals_up_to_children(&self, other: &Node) -> bool {
        self.player_1_board == other.player_1_board &&
        self.player_2_board == other.player_2_board &&
        self.node_type == other.node_type
    }

    pub fn generate_children_up_to_symmetry(&mut self) {
        match self.node_type {
            NodeType::Roll(_) => {
                self.add_rolls().expect("Won't error because we're in a Roll node type.");
            },
            NodeType::Move(_, _) => {
                let legal_moves = self.get_legal_moves_up_to_row_symmetry().expect("Won't error because we're in a Move node type.");
                for m in legal_moves {
                    self.add_move(m).expect("Won't error because we know the moves are legal.");
                }
            },
        }
    }

    pub fn build_n_moves_up_to_symmetry(&mut self, n: usize) {
        if self.is_game_over() {
            return;
        };
        match self.node_type {
            NodeType::Roll(_) => {
                self.generate_children_up_to_symmetry();                
                for child in self.children.iter_mut() {
                    child.build_n_moves_up_to_symmetry(n);
                }
            },
            NodeType::Move(_, _) => {
                if n == 0 {
                    return;
                };
                self.generate_children_up_to_symmetry();
                for child in self.children.iter_mut() {
                    child.build_n_moves_up_to_symmetry(n - 1);
                }
            },
        }
        
    }

    pub fn build_entire_tree_up_to_symmetry(&mut self) {
        if self.is_game_over() {
            return;
        }
        self.generate_children_up_to_symmetry();
        for child in self.children.iter_mut() {
            child.build_entire_tree_up_to_symmetry();
        }
    }

    pub fn get_next_moves_and_evaluation(&self, objective_function: fn(&Node) -> f32) -> Result<(Vec<Move>, f32), String> {
        let player = match self.node_type {
            NodeType::Roll(_) => {
                return Err("Cannot get next moves and evaluation from a roll node".to_string());
            },
            NodeType::Move(player, _) => player,
        };
        if self.is_leaf() {
            return Ok((Vec::new(), objective_function(self)));
        }

        let legal_moves = self.get_legal_moves_up_to_row_symmetry()
            .expect("Won't error because we're in a Move node type.");
        let mut best_evaluation = match player {
            Player::Player1 => f32::NEG_INFINITY,
            Player::Player2 => f32::INFINITY,
        };
        let mut next_moves = Vec::new();
        for next_move in legal_moves {
            let child_roll_node = self.get_child_from_move(next_move)
                .expect("Won't error because we know the moves are legal.");
            if child_roll_node.is_game_over() {
                return Ok((vec![next_move], objective_function(child_roll_node)));
            }
            let mut average_evaluation = 0.;
            let average_denominator = child_roll_node.get_n_children() as f32;
            for child_move_node in child_roll_node.children.iter() {
                let (_, child_evaluation) = child_move_node
                    .get_next_moves_and_evaluation(objective_function)
                    .expect("Won't error because we're in a Move node type.");
                average_evaluation += child_evaluation / average_denominator;
            }
            match player.compare_evaluation(average_evaluation, best_evaluation) {
                Comparison::Equal => {
                    next_moves.push(next_move);
                },
                Comparison::Better => {
                    best_evaluation = average_evaluation;
                    next_moves = vec![next_move];
                },
                Comparison::Worse => {},
            }
        }
        return Ok((next_moves, best_evaluation));
    }

    pub fn with_move_made(&self, m: Move) -> Result<Node, String> {
        match self.node_type {
            NodeType::Roll(_) => {
                return Err("Cannot make a move from a roll node".to_string());
            },
            NodeType::Move(player, die) => {
                let next_player = player.opponent();
                let current_players_board = match self.get_player_board(player).with_move_made(die, m) {
                    Ok(board) => board,
                    Err(e) => return Err(e),
                };
                let next_players_board = self.get_player_board(next_player).eliminate(die, m.get_column());
                return Ok(
                    Node::from_player_and_boards(
                        next_player,
                        next_players_board,
                        current_players_board,
                        NodeType::Roll(next_player)
                    )
                );
            },
        }
    }

    pub fn with_rolls(&self, die: Die) -> Result<Node, String> {
        match self.node_type {
            NodeType::Roll(player) => {
                let mut to_return = self.clone();
                to_return.add_rolls();
                return Ok(to_return);
            },
            NodeType::Move(player, _) => {
                return Err("Cannot roll from a move node".to_string());
            },
        }
    }

    pub fn get_scores(&self) -> (u16, u16) {
        (self.player_1_board.sum(), self.player_2_board.sum())
    }

    pub fn get_score(&self, player: Player) -> u16 {
        match player {
            Player::Player1 => self.player_1_board.sum(),
            Player::Player2 => self.player_2_board.sum(),
        }
    }

    pub fn get_score_difference(&self) -> i16 {
        self.player_1_board.sum() as i16 - self.player_2_board.sum() as i16
    }

    pub fn get_outcome(&self) -> Outcome {
        match self.is_game_over() {
            true => {
                let player_1_score = self.player_1_board.sum();
                let player_2_score = self.player_2_board.sum();
                if player_1_score > player_2_score {
                    Outcome::Victory(Player::Player1)
                } else if player_2_score > player_1_score {
                    Outcome::Victory(Player::Player2)
                } else {
                    Outcome::Draw
                }
            },
            false => Outcome::InProgress
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.player_1_board.is_full() || self.player_2_board.is_full()
    }

    pub fn add_move(&mut self, next_move: Move) -> Result<(), String> {
        match self.node_type {
            NodeType::Roll(_) => {
                return Err("Cannot add move to a roll node".to_string());
            },
            NodeType::Move(_, _) => {
                match self.with_move_made(next_move) {
                    Ok(node) => {
                        self.children.push(node);
                        return Ok(());
                    },
                    Err(e) => return Err(e),
                }
            }
        }
    }

    pub fn add_rolls(&mut self) -> Result<(), String> {
        match self.node_type {
            NodeType::Roll(player) => {
                let opponent = player.opponent();
                for die in Die::all() {
                    let new_node = Node::from_player_and_boards(
                        player,
                        self.get_player_board(player),
                        self.get_player_board(opponent),
                        NodeType::Move(player, die)
                    );
                    self.children.push(new_node);
                }
                return Ok(())
            },
            NodeType::Move(_, _) => {
                return Err("Cannot add rolls to a move node".to_string());
            }
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.get_n_children() == 0
    }

    pub fn get_n_children(&self) -> usize {
        self.children.len()
    }

    pub fn get_max_depth(&self) -> usize {
        if self.get_n_children() == 0 {
            return 1;
        } else {
            return 1 + self.children.iter().map(|child| child.get_max_depth()).max().unwrap();
        }
    }

    pub fn get_children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn get_child_from_move(&self, m: Move) -> Result<&Node, String> {
        self.get_child(m.get_row(), m.get_column())
    }

    pub fn get_child_from_roll(&self, roll: Die) -> Result<&Node, String> {
        match self.node_type {
            NodeType::Roll(_) => {
                if self.get_n_children() != 6 {
                    return Err("Roll node does not have children".to_string());
                } else {
                    return Ok(&self.children[(roll.to_value() - 1) as usize]);
                }
            },
            NodeType::Move(_, _) => {
                return Err("Cannot get child from roll from a move node".to_string());
            }
        }
    }

    pub fn get_child(&self, row: usize, col: usize) -> Result<&Node, String> {
        let expected_node = match self.with_move_made(Move::new(row, col)) {
            Ok(node) => node,
            Err(e) => return Err(e),
        };
        match self.children.iter()
            .find(|child| child.equals_up_to_children(&expected_node)) {
                Some(child) => Ok(child),
                None => Err(format!("No child at row {} and column {}", row, col)),
        }
    }

}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NodeType {
    Roll(Player),
    Move(Player, Die),
}

#[cfg(test)]
mod test_tree {
    use super::*;

    #[test]
    fn test_tree_instantiates() {
        let player_1_board = Board::empty();
        let player_2_board = Board::empty();
        let root = Node::new(player_1_board, player_2_board, NodeType::Roll(Player::Player1));

        assert_eq!(root.get_player_1_board(), Board::empty());
        assert_eq!(root.get_player_2_board(), Board::empty());
        assert_eq!(root.get_die(), None);
        assert_eq!(root.get_active_player(), Player::Player1);

        let player_1_board = Board::from_string("2__\n___\n___".to_string()).unwrap();
        let player_2_board = Board::empty();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Five));

        assert_eq!(root.get_player_1_board().sum(), 2);
        assert_eq!(root.get_player_2_board(), Board::empty());
        assert_eq!(root.get_die(), Some(Die::Five));
        assert_eq!(root.get_active_player(), Player::Player2);
    }

    #[test]
    fn test_is_game_over() {
        let player_1_board = Board::from_string("2__\n___\n___".to_string()).unwrap();
        let player_2_board = Board::empty();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Five));

        assert_eq!(root.is_game_over(), false);

        let player_1_board = Board::from_string("255\n122\n352".to_string()).unwrap();
        let player_2_board = Board::from_string("15_\n333\n12_".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Five));

        assert_eq!(root.is_game_over(), true);
    }

    #[test]
    fn test_evaluates_outcome() {
        let player_1_board = Board::from_string("2__\n___\n___".to_string()).unwrap();
        let player_2_board = Board::empty();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Five));

        assert_eq!(root.get_outcome(), Outcome::InProgress);

        let player_1_board = Board::from_string("255\n122\n352".to_string()).unwrap();
        let player_2_board = Board::from_string("15_\n333\n12_".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Five));

        assert_eq!(root.get_scores(), (6 + 24 + 18, 10 + 10 + 3));
        assert_eq!(root.get_outcome(), Outcome::Victory(Player::Player1));

        let player_1_board = Board::from_string("111\n111\n111".to_string()).unwrap();
        let player_2_board = Board::from_string("24_\n25_\n2__".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Five));

        assert_eq!(root.get_scores(), (27, 27));
        assert_eq!(root.get_outcome(), Outcome::Draw);
    }

    #[test]
    fn test_node_makes_move_and_add_rolls() {
        let player_1_board = Board::from_string("2__\n___\n___".to_string()).unwrap();
        let player_2_board = Board::empty();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Five));

        assert_eq!(root.get_n_children(), 0);

        let m = Move::new(0, 0);
        let mut result = root.add_move(m).is_ok();
        assert!(result);
        assert_eq!(root.get_n_children(), 1);

        let mut new_node = root.get_children()[0].clone();
        let result = new_node.add_rolls();
        assert!(result.is_ok());
        assert_eq!(new_node.get_n_children(), 6);
        assert!(new_node.get_children().iter().all(|x| x.get_die().is_some()));

    }

    #[test]
    fn test_node_gets_child() {
        let player_1_board = Board::from_string("2__\n___\n___".to_string()).unwrap();
        let player_2_board = Board::empty();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Five));
        root.add_move(Move::new(0, 0)).unwrap();
        let actual_child = root.get_child(0, 0).unwrap();
        
        let expected_child = Node::new(
            Board::from_string("2__\n___\n___".to_string()).unwrap(),
            Board::from_string("5__\n___\n___".to_string()).unwrap(),
            NodeType::Roll(Player::Player1),
        );

        assert_eq!(*actual_child, expected_child);

        let player_1_board = Board::from_string("651\n142\n62_".to_string()).unwrap();
        let player_2_board = Board::from_string("256\n1_2\n62_".to_string()).unwrap();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Six));
        root.add_move(Move::new(1, 1)).unwrap();
        let actual_child = root.get_child(1, 1).unwrap();
        
        let expected_child = Node::new(
            Board::from_string("651\n142\n62_".to_string()).unwrap(),
            Board::from_string("256\n162\n62_".to_string()).unwrap(),
            NodeType::Roll(Player::Player1),
        );

        assert_eq!(*actual_child, expected_child);
    }

    #[test]
    fn test_node_handles_elimination() {
        let player_1_board = Board::from_string("2__\n__5\n2_3".to_string()).unwrap();
        let player_2_board = Board::from_string("___\n___\n___".to_string()).unwrap();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Two));

        let result = root.add_move(Move::new(0, 0)).is_ok();
        assert!(result);
        
        let mut new_node = root.get_children()[0].clone();
        assert_eq!(
            new_node.get_player_1_board(),
            Board::from_string("___\n__5\n__3".to_string()).unwrap()
        );
        assert_eq!(
            new_node.get_player_2_board(),
            Board::from_string("2__\n___\n___".to_string()).unwrap()
        );
        
    }

    #[test]
    fn test_node_gets_legal_moves() {
        let player_1_board = Board::from_string("2__\n___\n___".to_string()).unwrap();
        let player_2_board = Board::empty();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Five));

        assert_eq!(
            root.get_legal_moves(),
            Ok(
                vec![
                    Move::new(0, 0), Move::new(0, 1), Move::new(0, 2),
                    Move::new(1, 0), Move::new(1, 1), Move::new(1, 2),
                    Move::new(2, 0), Move::new(2, 1), Move::new(2, 2),
                ]
            )
        );
    
        let player_1_board = Board::from_string("2_5\n122\n352".to_string()).unwrap();
        let player_2_board = Board::from_string("15_\n333\n12_".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::Five));

        assert_eq!(
            root.get_legal_moves(),
            Ok(vec![Move::new(0, 1),])
        );

        let player_1_board = Board::from_string("255\n1_2\n352".to_string()).unwrap();
        let player_2_board = Board::from_string("15_\n333\n12_".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board, NodeType::Roll(Player::Player1));

        assert!(root.get_legal_moves().is_err());


    }

    #[test]
    fn test_node_with_move_made() {
        let player_1_board = Board::from_string("2_5\n122\n352".to_string()).unwrap();
        let player_2_board = Board::from_string("1__\n333\n12_".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board.clone(), NodeType::Move(Player::Player1, Die::Five));

        let m = Move::new(0, 1);
        let mut new_node = root.with_move_made(m).unwrap();
        assert_eq!(
            new_node.get_player_1_board(),
            Board::from_string("255\n122\n352".to_string()).unwrap()
        );
        assert_eq!(
            new_node.get_player_2_board(),
            player_2_board
        );
        assert_eq!(
            new_node.node_type,
            NodeType::Roll(Player::Player2)
        );

        let player_1_board = Board::from_string("651\n142\n62_".to_string()).unwrap();
        let player_2_board = Board::from_string("256\n1_2\n62_".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Six));
        let actual_node = root.with_move_made(Move::new(1, 1)).unwrap();

        let expected_player_1_board = Board::from_string("651\n142\n62_".to_string()).unwrap();
        let expected_player_2_board = Board::from_string("256\n162\n62_".to_string()).unwrap();
        let expected_node = Node::new(expected_player_1_board, expected_player_2_board, NodeType::Roll(Player::Player1));

        assert_eq!(actual_node, expected_node);

        
        let player_1_board = Board::from_string("651\n142\n62_".to_string()).unwrap();
        let player_2_board = Board::from_string("256\n1_2\n62_".to_string()).unwrap();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Six));
        root.generate_children_up_to_symmetry();

        assert_eq!(root.with_move_made(Move::new(1, 1)).unwrap(), *root.get_child(1, 1).unwrap());
        
    }

    #[test]
    fn test_node_gets_legal_moves_up_to_row_symmetry() {
        let player_1_board = Board::from_string("2__\n___\n___".to_string()).unwrap();
        let player_2_board = Board::empty();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Five));

        assert_eq!(
            root.get_legal_moves_up_to_row_symmetry(),
            Ok(
                vec![
                    Move::new(0, 0), Move::new(0, 1), Move::new(0, 2),
                ]
            )
        );
        
        let player_1_board = Board::from_string("235\n1_2\n3_2".to_string()).unwrap();
        let player_2_board = Board::from_string("15_\n333\n12_".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::Five));

        assert_eq!(
            root.get_legal_moves_up_to_row_symmetry(),
            Ok(vec![Move::new(1, 1),])
        );

        
        let player_1_board = Board::from_string("255\n1_2\n352".to_string()).unwrap();
        let player_2_board = Board::from_string("15_\n333\n12_".to_string()).unwrap();
        let root = Node::new(player_1_board, player_2_board, NodeType::Roll(Player::Player1));

        assert!(root.get_legal_moves_up_to_row_symmetry().is_err());
    }

    #[test]
    fn test_node_generates_children_up_to_row_symmetry() {
        let player_1_board = Board::from_string("2__\n___\n___".to_string()).unwrap();
        let player_2_board = Board::empty();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Five));
        root.generate_children_up_to_symmetry();

        assert_eq!(root.get_n_children(), 3);
                
        let player_1_board = Board::from_string("235\n1_2\n3_2".to_string()).unwrap();
        let player_2_board = Board::from_string("15_\n333\n12_".to_string()).unwrap();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::Five));
        root.generate_children_up_to_symmetry();

        assert_eq!(root.get_n_children(), 1);

        let player_1_board = Board::from_string("255\n1_2\n352".to_string()).unwrap();
        let player_2_board = Board::from_string("15_\n333\n12_".to_string()).unwrap();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Roll(Player::Player1));
        root.generate_children_up_to_symmetry();

        assert_eq!(root.get_n_children(), 6);
    }

    #[test]
    fn test_tree_builds_entire_tree_up_to_symmetry() {
        let player_1_board = Board::from_string("255\n1_2\n352".to_string()).unwrap();
        let player_2_board = Board::from_string("15_\n333\n12_".to_string()).unwrap();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::Six));
        root.build_entire_tree_up_to_symmetry();
        assert_eq!(root.get_n_children(), 1);
        assert_eq!(root.get_max_depth(), 2);

        let player_1_board = Board::from_string("235\n1_2\n3_2".to_string()).unwrap();
        let player_2_board = Board::from_string("156\n333\n12_".to_string()).unwrap();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Roll(Player::Player2));
        let mut expected_root = root.clone();
        expected_root.add_rolls();
        for child in expected_root.children.iter_mut() {
            child.get_legal_moves_up_to_row_symmetry().unwrap().iter().for_each(
                |&m| {
                    child.add_move(m);
                }
            );
        }
        root.build_entire_tree_up_to_symmetry();
        assert_eq!(root.get_max_depth(), 3);
        assert_eq!(root.get_n_children(), 6);
        assert_eq!(root, expected_root);

        let player_1_board = Board::from_string("251\n142\n32_".to_string()).unwrap();
        let player_2_board = Board::from_string("256\n1_2\n62_".to_string()).unwrap();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Six));
        root.build_entire_tree_up_to_symmetry();
        assert_eq!(root.get_max_depth(), 4);
        assert_eq!(root.get_n_children(), 2);
        
    }

    #[test]
    fn test_tree_builds_to_depth() {
        let player_1_board = Board::empty();
        let player_2_board = Board::empty();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::Six));
        root.build_n_moves_up_to_symmetry(1);

        assert_eq!(root.get_max_depth(), 3);
        assert_eq!(root.get_n_children(), 3);
        assert!(root.get_children().iter().all(|c| c.get_n_children() == 6));
        
        let player_1_board = Board::empty();
        let player_2_board = Board::empty();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Roll(Player::Player1));
        root.build_n_moves_up_to_symmetry(1);

        assert_eq!(root.get_max_depth(), 4);
        assert_eq!(root.get_n_children(), 6);
        assert!(root.get_children().iter().all(|c| c.get_n_children() == 3));

        let player_1_board = Board::from_string("651\n142\n62_".to_string()).unwrap(); // 40 before move.
        let player_2_board = Board::from_string("256\n1_2\n62_".to_string()).unwrap(); // 24 before move.
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player2, Die::Six));
        let mut expected_root = root.clone();

        root.build_n_moves_up_to_symmetry(5);
        expected_root.build_entire_tree_up_to_symmetry();
        assert_eq!(root, expected_root);

    }

}