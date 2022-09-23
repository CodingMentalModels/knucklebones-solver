use crate::board::board::{Board, Move, Outcome, Player, Die};

#[derive(Clone, Debug)]
pub struct Node {
    player_1_board: Board,
    player_2_board: Board,
    node_type: NodeType,
    children: Vec<Node>,
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

    pub fn generate_children_up_to_symmetry(&mut self) {
        match self.node_type {
            NodeType::Roll(_) => {
                self.add_rolls().expect("Won't error because we're in a Roll node type.");
            },
            NodeType::Move(player, die) => {
                let legal_moves = self.get_legal_moves_up_to_row_symmetry().expect("Won't error because we're in a Move node type.");
                for m in legal_moves {
                    self.add_move(m).expect("Won't error because we know the moves are legal.");
                }
            },
        }
    }

    pub fn get_scores(&self) -> (u16, u16) {
        (self.player_1_board.sum(), self.player_2_board.sum())
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
            NodeType::Roll(player) => {
                return Err("Cannot add move to a roll node".to_string());
            },
            NodeType::Move(player, die) => {
                let opponent = player.opponent();
                match self.get_player_board(player).with_move_made(die, next_move) {
                    Ok(new_board) => {
                        let new_node = Node::from_player_and_boards(
                            player,
                            new_board,
                            self.get_player_board(opponent),
                            NodeType::Roll(opponent)
                        );
                        self.children.push(new_node);
                        return Ok(())
                    },
                    Err(err) => return Err(err),
                };
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

    pub fn n_children(&self) -> usize {
        self.children.len()
    }

    pub fn get_max_depth(&self) -> usize {
        if self.n_children() == 0 {
            return 1;
        } else {
            return 1 + self.children.iter().map(|child| child.get_max_depth()).max().unwrap();
        }
    }

    pub fn get_children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn get_child(&self, row: usize, col: usize) -> Result<&Node, String> {
        unimplemented!()
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

        assert_eq!(root.n_children(), 0);

        let m = Move::new(0, 0);
        let mut result = root.add_move(m).is_ok();
        assert!(result);
        assert_eq!(root.n_children(), 1);

        let mut new_node = root.get_children()[0].clone();
        let result = new_node.add_rolls();
        assert!(result.is_ok());
        assert_eq!(new_node.n_children(), 6);
        assert!(new_node.get_children().iter().all(|x| x.get_die().is_some()));


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

        assert_eq!(root.n_children(), 3);
                
        let player_1_board = Board::from_string("235\n1_2\n3_2".to_string()).unwrap();
        let player_2_board = Board::from_string("15_\n333\n12_".to_string()).unwrap();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Move(Player::Player1, Die::Five));
        root.generate_children_up_to_symmetry();

        assert_eq!(root.n_children(), 1);

        let player_1_board = Board::from_string("255\n1_2\n352".to_string()).unwrap();
        let player_2_board = Board::from_string("15_\n333\n12_".to_string()).unwrap();
        let mut root = Node::new(player_1_board, player_2_board, NodeType::Roll(Player::Player1));
        root.generate_children_up_to_symmetry();

        assert_eq!(root.n_children(), 6);
    }

}