use std::ops::Add;
use std::fmt::{Debug, Formatter, Display};

use ansi_term::Colour;
use rand::Rng;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub row: usize,
    pub column: usize,
}

impl Move {

    pub fn new(row: usize, column: usize) -> Move {
        Move { row, column }
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_column(&self) -> usize {
        self.column
    }

    pub fn all() -> Vec<Move> {
        let mut moves = Vec::new();
        for row in 0..3 {
            for column in 0..3 {
                moves.push(Move::new(row, column));
            }
        }
        moves
    }

    pub fn to_string(&self) -> String {
        format!("({}, {})", self.row, self.column)
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
        Ok(Move { row, column: col })
    }

}

#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    columns: Vec<Vec<Square>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.to_string(), f)
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.to_string(), f)
    }
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
        match self.columns[col][row] {
            Square::Empty => false,
            Square::Die(_) => true,
        }
    }

    pub fn make_move(&mut self, die: Die, m: Move) -> Result<(), String> {
        if self.is_set(m.get_row(), m.get_column()) {
            return Err(format!("Square already set: {:?}", m));
        }
        self.columns[m.get_column()][m.get_row()] = Square::Die(die);
        return Ok(());
    }

    pub fn with_move_made(&self, die: Die, m: Move) -> Result<Self, String> {
        let mut new_board = self.clone();
        let result = new_board.make_move(die, m);
        return result.map(|_| new_board);
    }

    pub fn eliminate(&self, die: Die, column_index: usize) -> Board {
        let mut new_board = self.clone();
        for square in new_board.columns[column_index].iter_mut() {
            if *square == Square::Die(die) {
                *square = Square::Empty;
            }
        }
        return new_board;
    }

    pub fn get_empty_squares(&self) -> Vec<(usize, usize)> {
        let mut rows = vec![
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];
        for (col_n, column) in self.columns.iter().enumerate() {
            for (row_n, square) in column.iter().enumerate() {
                if *square == Square::Empty {
                    rows[row_n].push((row_n, col_n));
                }
            }
        }
        return rows.into_iter().flatten().collect();
    }

    pub fn get_n_empty_squares(&self) -> usize {
        self.get_empty_squares().len()
    }

    pub fn get_empty_squares_up_to_row_symmetry(&self) -> Vec<(usize, usize)> {
        let mut empty_squares = Vec::new();
        for (col_n, column) in self.columns.iter().enumerate() {
            for (row_n, square) in column.iter().enumerate() {
                if *square == Square::Empty {
                    empty_squares.push((row_n, col_n));
                    break;
                }
            }
        }
        return empty_squares;
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

    pub fn all() -> Vec<Die> {
        vec![Die::One, Die::Two, Die::Three, Die::Four, Die::Five, Die::Six]
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

    pub fn random() -> Die {
        let mut rng = rand::thread_rng();
        let die_value: u8 = rng.gen_range(1..7);
        Die::new(die_value).unwrap()
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
}

impl Outcome {

    pub fn to_string(&self) -> String {
        match self {
            Outcome::Victory(player) => format!("{} wins", player.to_string()),
            Outcome::Draw => "Draw".to_string(),
            Outcome::InProgress => "Game in progress".to_string(),
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

    pub fn opponent(&self) -> Self {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }

    pub fn compare_evaluation(&self, evaluation: f32, other_evaluation: f32) -> Comparison {
        if evaluation == other_evaluation {
            return Comparison::Equal;
        }
        return match self {
            Player::Player1 => {if evaluation > other_evaluation {Comparison::Better} else {Comparison::Worse}},
            Player::Player2 => {if evaluation < other_evaluation {Comparison::Better} else {Comparison::Worse}},
        }
    }

    pub fn get_random() -> Self {
        if rand::random::<bool>() {
            Player::Player1
        } else {
            Player::Player2
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Comparison {
    Better,
    Worse,
    Equal,
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
    fn test_board_eliminates() {

        let board = Board::from_string("5__\n__2\n_32".to_string()).unwrap();
        let non_eliminated_board = board.eliminate(Die::Two, 1);

        assert_eq!(non_eliminated_board, board);

        let non_eliminated_board = board.eliminate(Die::Six, 2);

        assert_eq!(non_eliminated_board, board);

        let eliminated_board = board.eliminate(Die::Two, 2);

        assert_eq!(eliminated_board, Board::from_string("5__\n___\n_3_".to_string()).unwrap());
    }

    #[test]
    fn test_move_instantiates() {
        let m = Move::from_string("1 2").unwrap();
        assert_eq!(m, Move::new(1, 2));
    }

    #[test]
    fn test_board_debug_format() {
        let b = Board::from_string("5__\n__2\n_32".to_string()).unwrap();
        assert_eq!(format!("{:?}", b), "\"5__\\n__2\\n_32\"");
    }


}