mod board;
mod tree;
mod solver;

use std::io;
use clap::{App, SubCommand, Arg};
use rand::seq::SliceRandom;
use crate::board::board::Player;
use crate::tree::tree::Node;
use crate::board::board::{Board, Move, Outcome, Die};
use crate::solver::solver::{Solver, SolverMode, Evaluation};
use crate::tree::tree::NodeType;

const DEFAULT_DEPTH: usize = 4;

fn main() {
    let matches = App::new("Knucklebones (Cult of the Lamb) Solver")
		.about("Solver for Knucklebones")
		.subcommand(
			SubCommand::with_name("solve")
				.about("Solve Knucklebones Position")
				.arg(
					Arg::with_name("Position")
						.help("Knucklebones Position")						
				)
            )
        .subcommand(
            SubCommand::with_name("play")
                .about("Play Knucklebones against the solver")
                .arg(
                    Arg::with_name("Heuristic Depth")
                        .help("Depth of the heuristic search.")
                )
            ).get_matches();
    
    if let Some(matches) = matches.subcommand_matches("solve") {
        match matches.value_of("Position") {
            Some(position) => {
                unimplemented!()
            },
            None => {
                println!("Missing Position!");
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("play") {
        let heuristic_depth = match matches.value_of("Heuristic Depth") {
            Some(depth) => depth.parse::<usize>().expect("Invalid depth!"),
            None => DEFAULT_DEPTH
        };
        let player = Player::get_random();
        let mut rng = rand::thread_rng();
        let mut game = Node::empty();
        while !game.is_game_over() {
            match game.get_node_type() {
                NodeType::Roll(_) => {
                    let roll = Die::random();
                    game = game.with_rolls(roll).expect("Roll is guaranteed to be legal.")
                        .get_child_from_roll(roll).expect("Roll is guaranteed to be legal.").clone();
                },
                NodeType::Move(p, roll) => {
                    if p == player {
                        println!("Current Position:\n{}", game.to_string_from_perspective(player));
                        let mut valid_move = false;
                        while !valid_move {
                            let mut input = String::new();
                            println!("Enter move: ");
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            let input = input.trim();
                            if let Ok(m) = Move::from_string(input) {
                                if game.is_legal_move(m) {
                                    game = game.with_move_made(m).expect("Move is guaranteed to be valid");
                                    valid_move = true;
                                } else {
                                    println!("Invalid move!");
                                }
                            } else {
                                println!("Invalid move!");
                            }
                        }
                    } else {
                        let mut solver = Solver::from_root(game.clone());
                        let result = solver.get_best_moves_and_evaluation(
                            SolverMode::Heuristic(heuristic_depth, |x| Solver::difference_heuristic(x, 3.5))
                        );
                        match result {
                            Ok((best_moves, evaluation)) => {
                                let selected_move = best_moves.choose(&mut rng).unwrap();
                                println!("Solver rolls a {} and plays {}.  Evaluation: {}", roll.to_string(), selected_move.to_string(), evaluation.from_perspective(player).to_string());
                                game = game.with_move_made(*selected_move).expect("Move is guaranteed to be legal.");
                            },
                            Err(e) => {
                                println!("Solver failed: {}", e);
                                break;
                            }
                        }
                    }
                },
            }
        }
        let outcome = match game.get_outcome() {
            Outcome::Draw => "Draw",
            Outcome::Victory(p) => {
                if p == player {
                    "You Win!"
                } else {
                    "Solver Wins..."
                }
            },
            Outcome::InProgress => panic!("Game is over, but outcome is in progress.")
        };
        println!("Game Over!  {}\nPlayer: {}\nSolver: {}", outcome, game.get_score(player), game.get_score(player.opponent()));
    } else {
        println!("Missing subcommand!");  
    }
}

#[cfg(test)]
mod test_integration_tests {
    use super::*;

}