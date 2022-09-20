use ansi_term::Colour;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub row: usize,
    pub col: usize,
}

impl Move {

    pub fn new(row: usize, col: usize) -> Move {
        Move { row, col }
    }

    pub fn to_string_on_board(&self, board: &Board) -> Result<String, String> {
        let player = match board.get_active_player() {
            Some(player) => player,
            None => return Err("Game is already over".to_string()),
        };
        let maybe_board = board.with_move_made(player, *self);
        let final_board = match maybe_board {
            Ok(b) => b,
            Err(_) => {return Err("Invalid move".to_string());},
        };
        let mut to_print = "".to_string();
        for row in 0..3 {
            for col in 0..3 {
                if row == self.row && col == self.col {
                    to_print.push_str(&(Colour::Green.prefix().to_string() + &player.to_string() + &Colour::Green.suffix().to_string()));
                } else if final_board.x_bitboard.is_set(row, col) {
                    to_print.push_str("X");
                } else if final_board.o_bitboard.is_set(row, col) {
                    to_print.push_str("O");
                } else {
                    to_print.push_str("_");
                }
            }
            to_print.push_str("\n");
        }
        return Ok(to_print.trim_end().to_string());
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_column(&self) -> usize {
        self.col
    }

    pub fn to_string(&self) -> String {
        format!("({}, {})", self.row, self.col)
    }

    pub fn from_string(s: &str) -> Result<Move, String> {
        let stripped_s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        if stripped_s.len() != 2 {
            return Err(format!("Invalid move string: {}", s));
        }
        let mut chars = stripped_s.chars();
        let row = match chars.next() {
            Some('0') => 0,
            Some('1') => 1,
            Some('2') => 2,
            _ => return Err(format!("Invalid move string: {}", s)),
        };
        let col = match chars.next() {
            Some('0') => 0,
            Some('1') => 1,
            Some('2') => 2,
            _ => return Err(format!("Invalid move string: {}", s)),
        };
        Ok(Move { row, col })
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    x_bitboard: Bitboard,
    o_bitboard: Bitboard,
}

impl Board {

    pub fn empty() -> Board {
        Board {
            x_bitboard: Bitboard::empty(),
            o_bitboard: Bitboard::empty(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut to_return = "".to_string();
        for row in 0..3 {
            for col in 0..3 {
                if self.x_bitboard.is_set(row, col) {
                    to_return += "X";
                } else if self.o_bitboard.is_set(row, col) {
                    to_return += "O";
                } else {
                    to_return += "_";
                }
            }
            to_return += "\n";
        }
        return to_return.trim_end().to_string();
    }

    pub fn to_string_with_square_highlighted(&self, row: usize, col: usize) -> String {
        let mut to_return = "".to_string();
        for r in 0..3 {
            for c in 0..3 {
                let to_add = if self.x_bitboard.is_set(r, c) {
                    "X"
                } else if self.o_bitboard.is_set(r, c) {
                    "O"
                } else {
                    "_"
                };
                if r == row && c == col {
                    to_return += &(Colour::Green.prefix().to_string() + &to_add.to_string() + &Colour::Green.suffix().to_string());
                } else {
                    to_return += &to_add;
                }
            }
            to_return += "\n";
        }
        return to_return.trim_end().to_string();
    }
    
    pub fn new(x_bitboard: Bitboard, o_bitboard: Bitboard) -> Board {
        Board { x_bitboard, o_bitboard }
    }

    pub fn from_position(position: &str) -> Result<Self, String> {
        let mut x_bitboard = Bitboard::empty();
        let mut o_bitboard = Bitboard::empty();
        let mut row = 0;
        let mut col = 0;
        let stripped_position = position.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        if stripped_position.len() != 9 {
            return Err(format!("Invalid position string: {}", position));
        }
        for c in stripped_position.chars() {
            match c {
                'X' => x_bitboard.set(col, row),
                'O' => o_bitboard.set(col, row),
                '_' => (),
                _ => return Err(format!("Invalid character: {}", c)),
            };
            if row == 2 {
                row = 0;
                col += 1;
            } else {
                row += 1;
            }
        }
        Ok(Board::new(x_bitboard, o_bitboard))
    }

    pub fn get_outcome(&self) -> Outcome {
        let mut x_victory = false;
        let mut o_victory = false;

        x_victory = self.x_bitboard.is_victory();
        o_victory = self.o_bitboard.is_victory();

        match (self.is_full(), x_victory, o_victory) {
            (_, true, true) => Outcome::Ambiguous,
            (_, true, false) => Outcome::Victory(Player::X),
            (_, false, true) => Outcome::Victory(Player::O),
            (true, false, false) => Outcome::Draw,
            (false, false, false) => Outcome::InProgress,
        }
    }

    pub fn is_full(&self) -> bool {
        self.x_bitboard.union(&self.o_bitboard) == Bitboard::full()
    }

    fn is_set(&self, row: usize, col: usize) -> bool {
        self.x_bitboard.is_set(row, col) || self.o_bitboard.is_set(row, col)
    }

    pub fn make_move(&mut self, player: Player, m: Move) -> Result<(), String> {
        if self.get_active_player() != Some(player) {
            return Err(format!("It is not {}'s turn", player.to_string()));
        };
        if self.is_set(m.get_row(), m.get_column()) {
            return Err(format!("Move {} has already been made", m.to_string()));
        }
        match player {
            Player::X => self.x_bitboard.set(m.row, m.col),
            Player::O => self.o_bitboard.set(m.row, m.col),
        }
        return Ok(());
    }

    pub fn with_move_made(&self, player: Player, m: Move) -> Result<Self, String> {
        let mut new_board = self.clone();
        new_board.make_move(player, m)?;
        Ok(new_board)
    }

    pub fn get_active_player(&self) -> Option<Player> {
        if self.get_outcome() != Outcome::InProgress {
            return None;
        }
        
        if self.x_bitboard.n_set() == self.o_bitboard.n_set() {
            Some(Player::X)
        } else {
            Some(Player::O)
        }
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for row in 0..3 {
            for col in 0..3 {
                if !self.is_set(row, col) {
                    moves.push(Move::new(row, col));
                }
            }
        }
        moves
    }
    
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Bitboard(u16);

impl Bitboard {

    pub fn empty() -> Self {
        Bitboard(0)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn full() -> Self {
        Bitboard(0b111111111)
    }

    pub fn is_victory(&self) -> bool {
        if self.contains(Bitboard(0b111000000)) { return true;}
        if self.contains(Bitboard(0b000111000)) { return true;}
        if self.contains(Bitboard(0b000000111)) { return true;}
        if self.contains(Bitboard(0b100100100)) { return true;}
        if self.contains(Bitboard(0b010010010)) { return true;}
        if self.contains(Bitboard(0b001001001)) { return true;}
        if self.contains(Bitboard(0b100010001)) { return true;}
        if self.contains(Bitboard(0b001010100)) { return true;}
        return false;
    }

    pub fn from_binary(binary: &str) -> Result<Self, String> {
        if binary.len() != 9 {
            return Err(format!("Binary string must be 9 characters long, got {}", binary.len()));
        }
        let mut bitboard = Bitboard::empty();
        let mut index = 0;
        let mut i = 0;
        let mut j = 0;
        for c in binary.chars() {
            match c {
                '0' => (),
                '1' => bitboard.set(i, j),
                _ => return Err(format!("Invalid character in binary string: {}", c)),
            }
            if j == 2 {
                j = 0;
                i += 1;
            } else {
                j += 1;
            }
        }
        Ok(bitboard)
    }

    pub fn union(&self, other: &Self) -> Self {
        Bitboard(self.0 | other.0)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Bitboard(self.0 & other.0)
    }

    pub fn difference(&self, other: &Self) -> Self {
        Bitboard(self.0 & !other.0)
    }

    pub fn contains(&self, other: Self) -> bool {
        self.intersection(&other) == other
    }

    pub fn set(&mut self, row: usize, col: usize) {
        self.0 |= 1 << ((2 - row) * 3 + (2 - col));
    }

    pub fn n_set(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn is_set(&self, row: usize, col: usize) -> bool {
        self.0 & (1 << ((2 - row) * 3 + (2 - col))) != 0
    }

}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    Victory(Player),
    Draw,
    InProgress,
    Ambiguous,
}

impl Outcome {

    pub fn to_string(&self) -> String {
        match self {
            Outcome::Victory(player) => format!("{} wins", player.to_string()),
            Outcome::Draw => "Draw".to_string(),
            Outcome::InProgress => "Game in progress".to_string(),
            Outcome::Ambiguous => "Ambiguous".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Player {

    pub fn to_string(&self) -> String {
        match self {
            Player::X => "X".to_string(),
            Player::O => "O".to_string(),
        }
    }
}

#[cfg(test)]
mod test_board_tests {
    use super::*;

    #[test]
    fn test_board_instantiates() {
        
        let board = Board::from_position(
            "___
            ___
            _O_",
        ).unwrap();
        assert_eq!(board, Board::new(Bitboard::empty(), Bitboard::from_binary("000000010").unwrap()));
    }

    #[test]
    fn test_board_determines_winner() {
        assert_eq!(Board::from_position(
            "XOX
            OXO
            XOX",
        ).unwrap().get_outcome(), Outcome::Victory(Player::X));

        assert_eq!(Board::from_position(
            "XOX
            OOO
            _XX",
        ).unwrap().get_outcome(), Outcome::Victory(Player::O));

        
        assert_eq!(Board::from_position(
            "XOX
            XXO
            OXO",
        ).unwrap().get_outcome(), Outcome::Draw);

        
        assert_eq!(Board::from_position(
            "XOX
            ___
            ___",
        ).unwrap().get_outcome(), Outcome::InProgress);

        assert_eq!(Board::from_position(
            "XXX
            ___
            OOO",
        ).unwrap().get_outcome(), Outcome::Ambiguous);

    }

    #[test]
    fn test_bitboard_instantiates() {
        let mut bitboard = Bitboard::empty();
        assert_eq!(bitboard, Bitboard(0));
        bitboard.set(1, 2);
        bitboard.set(2, 2);
        assert_eq!(bitboard, Bitboard::from_binary("000001001").unwrap());

        let mut bitboard_2 = Bitboard::empty();
        bitboard_2.set(2, 1);
        assert_eq!(bitboard_2, Bitboard(2));

        let mut other_bitboard = Bitboard::from_binary("110000001").unwrap();
        let combined_bitboard = bitboard.union(&other_bitboard);
        assert_eq!(combined_bitboard, Bitboard::from_binary("110001001").unwrap());

        let intersected_bitboard = bitboard.intersection(&other_bitboard);
        assert_eq!(intersected_bitboard, Bitboard::from_binary("000000001").unwrap());
    }

    #[test]
    fn test_board_moves() {
        let mut board = Board::from_position(
            "___
            ___
            ___",
        ).unwrap();
        assert_eq!(board.make_move(Player::X, Move::new(2, 1)), Ok(()));
        assert_eq!(
            board,
            Board::from_position(
                "___
                ___
                _X_",
            ).unwrap()
        );
    }

    #[test]
    fn test_board_pretty_prints() {
        let board = Board::from_position(
            "___
            ___
            ___",
        ).unwrap();
        assert_eq!(board.to_string(),
        "___\n___\n___".to_string()
        );

        let board = Board::from_position(
            "XOX
            OXO
            XOX",
        ).unwrap();
        assert_eq!(
            board.to_string(),
            "XOX\nOXO\nXOX".to_string());
    }

    fn test_board_gets_active_player() {
        let board = Board::from_position(
            "___
            ___
            ___",
        ).unwrap();
        assert_eq!(board.get_active_player(), Some(Player::X));

        let board = Board::from_position(
            "___
            ___
            _X_",
        ).unwrap();
        assert_eq!(board.get_active_player(), Some(Player::O));

        let board = Board::from_position(
            "XOX
            O_O
            XOX",
        ).unwrap();
        assert_eq!(board.get_active_player(), Some(Player::X));

        let board = Board::from_position(
            "X_X
            OOO
            X__",
        ).unwrap();
        assert_eq!(board.get_active_player(), None);

        
        let board = Board::from_position(
            "XOX
            OXO
            XOX",
        ).unwrap();
        assert_eq!(board.get_active_player(), None);

    }

    #[test]
    fn test_board_gets_legal_moves() {
        let board = Board::from_position(
            "___
            ___
            ___",
        ).unwrap();
        assert_eq!(board.get_legal_moves(), vec![Move::new(0, 0), Move::new(0, 1), Move::new(0, 2), Move::new(1, 0), Move::new(1, 1), Move::new(1, 2), Move::new(2, 0), Move::new(2, 1), Move::new(2, 2)]);

        let board = Board::from_position(
            "XOX
            OXO
            XOX",
        ).unwrap();
        assert_eq!(board.get_legal_moves(), vec![]);

        let board = Board::from_position(
            "XOX
            OXO
            O_X",
        ).unwrap();
        assert_eq!(board.get_legal_moves(), vec![Move::new(2, 1)]);

        let board = Board::from_position(
            "XOX
            _X_
            __O",
        ).unwrap();
        assert_eq!(board.get_legal_moves(), vec![Move::new(1, 0), Move::new(1, 2), Move::new(2, 0), Move::new(2, 1)]);
    }

    #[test]
    fn test_move_instantiates() {
        let m = Move::from_string("1 2").unwrap();
        assert_eq!(m, Move::new(1, 2));
    }

    #[test]
    fn test_board_pretty_prints_with_square_highlighted() {
        let board = Board::from_position(
            "__X
            ___
            ___",
        ).unwrap();
        assert_eq!(board.to_string_with_square_highlighted(0, 2), 
            "__\u{1b}[32mX\u{1b}[0m\n___\n___".to_string()
        );
    }


}