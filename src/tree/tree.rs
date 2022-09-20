use crate::board::board::{Board, Move, Outcome, Player};

pub struct Tree {
    root: Node,
}

impl Tree {

    pub fn from_board(board: Board) -> Self {
        let root = Node::from_board(board);
        Tree { root }
    }

    pub fn depth(&self) -> usize {
        self.root.get_max_depth()
    }

    pub fn get_root(&self) -> &Node {
        &self.root
    }

}

pub struct Node {
    board: Board,
    children: Vec<Node>,
}

impl Node {

    pub fn from_board(board: Board) -> Self {
        let children = match board.get_outcome() {
            Outcome::InProgress => {
                let player = board.get_active_player().expect("We've already checked that the game isn't already over.");
                let mut children = Vec::new();
                for row in 0..3 {
                    for col in 0..3 {
                        if let Ok(child_board) = board.with_move_made(player, Move::new(row, col)) {
                            children.push(Node::from_board(child_board));
                        }
                    }
                }
                children
            },
            _ => Vec::new(),
        };
        Node { board, children }
    }

    pub fn get_board(&self) -> Board {
        self.board
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        self.board.get_legal_moves()
    }

    pub fn get_active_player(&self) -> Option<Player> {
        self.board.get_active_player()
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
        if self.board.get_outcome() != Outcome::InProgress {
            return Err("Game is already over".to_string());
        }
        let player = self.board.get_active_player().expect("We've already checked that the game isn't already over.");
        self.children.iter()
            .find(|child| Ok(child.board) == self.board.with_move_made(player, Move::new(row, col)))
            .map_or(
                Err(format!("There is no child with the move {}", Move::new(row, col).to_string())),
                |x| Ok(x)
            )
    }

    pub fn get_outcome(&self) -> Outcome {
        self.board.get_outcome()
    }

}


#[cfg(test)]
mod test_tree {
    use super::*;

    #[test]
    fn test_tiny_tree_builds() {
        let tree = Tree::from_board(
            Board::from_position(
                "XOX
                O_O
                XOX",
            ).unwrap()
        );

        assert_eq!(
            tree.get_root().get_board(),
            Board::from_position(
                "XOX
                O_O
                XOX",
            ).unwrap()
        );

        assert_eq!(
            tree.depth(),
            2
        );

        assert_eq!(
            tree.get_root().n_children(),
            1
        );

        assert_eq!(
            tree.get_root().get_child(1, 1).unwrap().get_board(),
            Board::from_position(
                "XOX
                OXO
                XOX",
            ).unwrap()
        );

        assert_eq!(
            tree.get_root().get_child(1, 1).unwrap().get_outcome(),
            Outcome::Victory(Player::X)
        )

    }

    #[test]
    fn test_small_tree_builds() {
        let tree = Tree::from_board(
            Board::from_position(
                "XOX
                O__
                XOX",
            ).unwrap()
        );

        assert_eq!(
            tree.get_root().get_board(),
            Board::from_position(
                "XOX
                O__
                XOX",
            ).unwrap()
        );

        assert_eq!(
            tree.depth(),
            3
        );

        assert_eq!(
            tree.get_root().n_children(),
            2
        );

        assert_eq!(
            tree.get_root().get_child(1, 2).unwrap().get_board(),
            Board::from_position(
                "XOX
                O_O
                XOX",
            ).unwrap()
        );

        assert_eq!(
            tree.get_root().get_child(1, 2).unwrap().n_children(),
            1
        );

        assert_eq!(
            tree.get_root().get_child(1, 2).unwrap().get_max_depth(),
            2
        );

    }

    
    #[test]
    fn test_full_tree_builds() {
        let tree = Tree::from_board(
            Board::empty()
        );

        assert_eq!(
            tree.depth(),
            10
        );

    }


}