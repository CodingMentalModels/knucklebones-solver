use std::ops::Add;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    columns: Vec<Vec<Square>>,
}

impl Board {

    pub fn empty() -> Board {
        Board {
            columns: vec![
                vec![Square::Empty, Square::Empty, Square::Empty],
                vec![Square::Empty, Square::Empty, Square::Empty],
                vec![Square::Empty, Square::Empty, Square::Empty],
            ],
        }
    }

    pub fn is_full(&self) -> bool {
        self.get_elements().iter().all(|square| *square != Square::Empty)
    }

    fn get_elements(&self) -> Vec<Square> {
        let mut elements = Vec::new();
        for column in self.columns.iter() {
            for element in column {
                elements.push(*element);
            }
        }
        elements
    }

    pub fn to_string(&self) -> String {
        let mut row_strings: Vec<String> = vec![
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ];
        for column in self.columns.iter() {
            let mut row_n = 0;
            for element in column {
                row_strings[row_n] += &element.to_string();
                if row_n == 2 {
                    row_n = 0;
                } else {
                    row_n += 1;
                }
            }
        }
        return row_strings.join("\n");
    }

    pub fn from_string(s: String) -> Result<Self, String> {
        let mut board = Board::empty();
        let mut row_n = 0;
        let stripped_s = s.chars().filter(|c| !(c == &' ' || c == &'\t')).collect::<String>();
        for row in stripped_s.split("\n") {
            let mut col_n = 0;
            for element in row.chars() {
                match Square::from_char(element) {
                    Ok(square) => board.columns[col_n][row_n] = square,
                    Err(e) => return Err(e),
                };
                col_n += 1;
            }
            row_n += 1;
        }
        return Ok(board)
    }

    pub fn sum(&self) -> u16 {
        let mut sum = 0;
        let mut column_index = 0;
        for column in self.columns.iter() {
            let mut column_sum = 0;
            for element in column {
                match element {
                    Square::Empty => (),
                    Square::Die(die) => column_sum += die.to_value(),
                }
            }
            column_sum = column_sum * self.get_column_multiplicity(column_index);
            sum += column_sum;
            column_index += 1;
        }
        return sum;
    }

    pub fn get_column_multiplicity(&self, column_index: usize) -> u16 {
        let column = &self.columns[column_index];
        if (column[0] != Square::Empty) && (column[0] == column[1]) && (column[1] == column[2]) {
            return 3;
        }
        if  ((column[0] != Square::Empty) && (column[0] == column[1])) ||
            ((column[1] != Square::Empty) && (column[1] == column[2])) ||
            ((column[0] != Square::Empty) && (column[0] == column[2])) {
            return 2;
        }
        return 1;
    }

    pub fn to_string_with_square_highlighted(&self, row: usize, col: usize) -> String {
        unimplemented!()
    }
    
    pub fn new(x_bitboard: Bitboard, o_bitboard: Bitboard) -> Board {
        unimplemented!()
    }

    fn is_set(&self, row: usize, col: usize) -> bool {
        unimplemented!()
    }

    pub fn make_move(&mut self, player: Player, m: Move) -> Result<(), String> {
        unimplemented!()
    }

    pub fn with_move_made(&self, player: Player, m: Move) -> Result<Self, String> {
        unimplemented!()
    }

    pub fn get_active_player(&self) -> Option<Player> {
        unimplemented!()
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        unimplemented!()
    }
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Empty,
    Die(Die),
}

impl Square {

    pub fn to_string(&self) -> String {
        match self {
            Self::Empty => "_".to_string(),
            Self::Die(die) => die.to_string(),
        }
    }

    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            '_' => Ok(Self::Empty),
            c => Die::from_char(c).map(|x| Self::Die(x)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Die {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Add for Die {
    type Output = u16;

    fn add(self, rhs: Self) -> Self::Output {
        self.to_value() + rhs.to_value()
    }
}

impl Die {

    pub fn new(value: u8) -> Result<Die, String> {
        match value {
            1 => Ok(Die::One),
            2 => Ok(Die::Two),
            3 => Ok(Die::Three),
            4 => Ok(Die::Four),
            5 => Ok(Die::Five),
            6 => Ok(Die::Six),
            _ => return Err(format!("Invalid die value: {}", value)),
        }
    }

    pub fn to_value(&self) -> u16 {
        match self {
            Die::One => 1,
            Die::Two => 2,
            Die::Three => 3,
            Die::Four => 4,
            Die::Five => 5,
            Die::Six => 6,
        }
    }

    pub fn to_string(&self) -> String {
        self.to_value().to_string()
    }

    pub fn from_char(c: char) -> Result<Die, String> {
        match c {
            '1' => Ok(Die::One),
            '2' => Ok(Die::Two),
            '3' => Ok(Die::Three),
            '4' => Ok(Die::Four),
            '5' => Ok(Die::Five),
            '6' => Ok(Die::Six),
            _ => Err(format!("Invalid die character: {}", c).to_string()),
        }
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
    Player1,
    Player2,
}

impl Player {

    pub fn to_string(&self) -> String {
        match self {
            Player::Player1 => "Player 1".to_string(),
            Player::Player2 => "Player 2".to_string(),
        }
    }
}

#[cfg(test)]
mod test_board_tests {
    use super::*;

    #[test]
    fn test_dice_add() {
        let die_1 = Die::new(4).unwrap();
        let die_2 = Die::new(5).unwrap();
        assert_eq!(die_1 + die_2, 9);
    }

    #[test]
    fn test_board_instantiates() {
        let b = Board::empty();
        assert_eq!(b.to_string(), "___\n___\n___");

        let b = Board::from_string("5__\n__2\n___".to_string()).unwrap();
        assert_eq!(b.to_string(), "5__\n__2\n___");
    }

    #[test]
    fn test_board_sums() {
        let b = Board::empty();
        assert_eq!(b.sum(), 0);

        let b = Board::from_string("5__\n__2\n___".to_string()).unwrap();
        assert_eq!(b.sum(), 7);

        let b = Board::from_string("5__\n5_2\n1__".to_string()).unwrap();
        assert_eq!(b.sum(), 24);

        let b = Board::from_string("4_2\n5_2\n1_2".to_string()).unwrap();
        assert_eq!(b.sum(), 28);

        let b = Board::from_string("412\n542\n162".to_string()).unwrap();
        assert_eq!(b.sum(), 39);
        
    }

    #[test]
    fn test_board_is_full() {
        let b = Board::empty();
        assert_eq!(b.is_full(), false);

        let b = Board::from_string("5__\n__2\n___".to_string()).unwrap();
        assert_eq!(b.is_full(), false);

        let b = Board::from_string("5__\n5_2\n1__".to_string()).unwrap();
        assert_eq!(b.is_full(), false);

        let b = Board::from_string("412\n542\n162".to_string()).unwrap();
        assert_eq!(b.is_full(), true);
    }

    #[test]
    fn test_move_instantiates() {
        let m = Move::from_string("1 2").unwrap();
        assert_eq!(m, Move::new(1, 2));
    }


}