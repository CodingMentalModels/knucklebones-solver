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
        unimplemented!()
    }

    pub fn get_board(&self) -> Board {
        unimplemented!()
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
        unimplemented!()
    }

}


#[cfg(test)]
mod test_tree {
    use super::*;


}