mod board;
mod tree;
mod solver;

use clap::{App, SubCommand, Arg};
use crate::board::board::{Board, Move, Outcome};
use crate::solver::solver::Solver;



fn main() {
    let matches = App::new("Tic Tac Toe Solver")
		.about("Solver for Tic Tac Toe")
		.subcommand(
			SubCommand::with_name("solve")
				.about("Solve Tic Tac Toe Position")
				.arg(
					Arg::with_name("Position")
						.help("Tic Tac Toe Position")						
				).arg(
                    Arg::with_name("Show Line")
                        .help("Show an example optimal line from the best move")
                        .short('l')
                        .long("line")
                )
            ).get_matches();
    
    if let Some(matches) = matches.subcommand_matches("solve") {
        match matches.value_of("Position") {
            Some(position) => {
                match Board::from_position(position) {
                    Ok(board) => {
                        let mut solver = Solver::from_board(board);
                        match matches.is_present("Show Line") {
                            true => {
                                match solver.get_evaluation_and_line() {
                                    (evaluation, line) => {
                                        let mut moves_and_boards: Vec<(Option<Move>, Board)> = vec![(None, board)];
                                        for m in line.iter() {
                                            let last_board = moves_and_boards.last().unwrap().1;
                                            let next_board = last_board.with_move_made(
                                                last_board.get_active_player().unwrap(),
                                                *m
                                            ).unwrap();
                                            moves_and_boards.push((Some(*m), next_board));
                                        }
                                        let boards_string = moves_and_boards.iter().map(|(maybe_m, board)| {
                                            match maybe_m {
                                                Some(m) => {
                                                    format!("{}", board.to_string_with_square_highlighted(m.get_row(), m.get_column()))
                                                },
                                                None => {
                                                    format!("{}", board.to_string())
                                                }
                                            }
                                        }).collect::<Vec<String>>().join("\n\n");
                                        println!("\n\nEvaluation:\n{}\n\nLine:\n{}", evaluation.to_string(), boards_string);
                                    },
                                }
                            },
                            false => {
                                match solver.get_next_moves_and_evaluation() {
                                    Ok((next_moves, evaluation)) => {
                                        let next_moves_string = next_moves.iter()
                                            .map(|x| x.to_string())
                                            .collect::<Vec<String>>()
                                            .join("\n");
                                        
                                        println!("\n\nEvaluation: {}\nIndifferent between these moves:\n{}", evaluation.to_string(), next_moves_string);
                                    },
                                    Err(error) => {
                                        println!("{}", error);
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        println!("{}", error);
                    }
                }
            },
            None => {
                println!("Needs a Position!");
            }
        }
    } else {
        println!("Invalid command!");
    }
}

#[cfg(test)]
mod test_integration_tests {
    use crate::solver::solver::Evaluation;

    use super::*;

    #[test]
    fn test_solver_solves_tic_tac_toe() {
        let mut solver = Solver::from_board(
            Board::from_position(
                "XOX
                O_O
                XOX",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves(), Ok(vec![Move::new(1, 1)]));

        let mut solver = Solver::from_board(
            Board::from_position(
                "X_X
                O_O
                ___",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves(), Ok(vec![Move::new(0, 1), Move::new(1, 1)]));

        let mut solver = Solver::from_board(
            Board::from_position(
                "XO_
                _X_
                ___",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves().unwrap().len(), 6);
        
        let mut solver = Solver::from_board(
            Board::from_position(
                "XO_
                _X_
                __O",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves_and_evaluation(), Ok((vec![Move::new(1, 0), Move::new(2, 0)], Evaluation::new(1.))));

        let mut solver = Solver::from_board(
            Board::from_position(
                "XO_
                ___
                ___",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves_and_evaluation(), Ok((vec![Move::new(1, 0), Move::new(1, 1), Move::new(2, 0)], Evaluation::new(1.))));

        
        let mut solver = Solver::from_board(
            Board::from_position(
                "XOX
                _O_
                __X",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves_and_evaluation(), Ok((vec![Move::new(1, 2), Move::new(2, 1)], Evaluation::new(-1.))));

        let mut solver = Solver::from_board(
            Board::from_position(
                "XO_
                O__
                XXO",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves_and_evaluation(), Ok((vec![Move::new(0, 2), Move::new(1, 1), Move::new(1, 2)], Evaluation::new(0.))));

    }

}