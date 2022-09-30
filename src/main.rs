mod board;
mod tree;
mod solver;

use std::io;
use clap::{App, SubCommand, Arg};
use rand::seq::SliceRandom;
use crate::board::board::Player;
use crate::tree::tree::Node;
use crate::board::board::{Board, Move, Outcome, Die};
use crate::solver::solver::{Solver, SolverMode, Evaluation, HeuristicDepthAndObjective};
use crate::tree::tree::NodeType;

const DEFAULT_DEPTH: usize = 4;
const DEFAULT_MAX_DEPTH_TO_BRUTE_FORCE: usize = 1;

fn main() {
    let matches = App::new("Knucklebones (Cult of the Lamb) Solver")
		.about("Solver for Knucklebones")
		.subcommand(
			SubCommand::with_name("solve")
				.about("Solve Knucklebones Position")
				.arg(
					Arg::with_name("Next to Act Board")
						.help("Board for the player who's next to act.")						
				).arg(
					Arg::with_name("Next to Act Opponent's Board")
						.help("Board for the player who's not next to act.")						
				).arg(
					Arg::with_name("Roll")
						.help("Latest Roll.")						
				).arg(
                    Arg::with_name("Full Tree")
                        .help("Print full tree.")
                        .short('t')
                        .long("tree")
                ).arg(
                    Arg::with_name("Heuristic Depth")
                        .help("Depth of the heuristic search.")
                        .short('d')
                        .long("depth")
                        .takes_value(true)
                ).arg(
                    Arg::with_name("Max Depth to Brute Force")
                        .help("Max depth to brute force.")
                        .short('b')
                        .long("max-brute-force-depth")
                        .takes_value(true)
                )
            )
        .subcommand(
            SubCommand::with_name("play")
                .about("Play Knucklebones against the solver")
                .arg(
                    Arg::with_name("Heuristic Depth")
                        .help("Depth of the heuristic search.")
                        .short('d')
                        .long("depth")
                        .takes_value(true)
                ).arg(
                    Arg::with_name("Max Depth to Brute Force")
                        .help("Max depth to brute force.")
                        .short('b')
                        .long("max-brute-force-depth")
                        .takes_value(true)
                )
            ).get_matches();
    
    if let Some(matches) = matches.subcommand_matches("solve") {
        match (matches.value_of("Next to Act Board"), matches.value_of("Next to Act Opponent's Board"), matches.value_of("Roll")) {
            (Some(player_board), Some(opponent_board), Some(roll)) => {
                match (Board::from_string(player_board.to_string()), Board::from_string(opponent_board.to_string())) {
                    (Ok(player_board), Ok(opponent_board)) => {
                        match roll.parse::<u8>() {
                            Ok(roll) => {
                                match Die::new(roll) {
                                    Ok(die) => {
                                        let depth = get_int_from_arg_or_else(matches.value_of("Heuristic Depth"), DEFAULT_DEPTH);
                                        let max_depth_to_brute_force = get_int_from_arg_or_else(matches.value_of("Max Depth to Brute Force"), DEFAULT_MAX_DEPTH_TO_BRUTE_FORCE);
                                        let mut game = Node::new(player_board, opponent_board, NodeType::Move(Player::Player1, die));
                                        let mut solver = Solver::from_root(game.clone());
                                        let (maybe_tree, evaluation) = solver
                                            .get_evaluation_tree(SolverMode::Hybrid(max_depth_to_brute_force, (depth, |x| Solver::difference_heuristic(x, 3.5))))
                                            .expect("Evaluation tree should be constructable.");
                                        println!("Evaluation: {}", evaluation.to_string());
                                        let evaluation_tree = maybe_tree.expect("Game should still be in progress.");
                                        let best_moves = evaluation_tree.get_moves().expect("Guaranteed to be on a move node.");
                                        println!("Best Moves: {}", best_moves.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "));
                                        if matches.is_present("Full Tree") {
                                            println!("Optimal Tree: {}", evaluation_tree.to_pretty_string(|x| Solver::difference_heuristic(x, 3.5)));
                                        }
                                    },
                                    Err(e) => {
                                        println!("Invalid roll: {}", e);
                                    }
                                }                                
                            },
                            Err(_) => println!("Invalid Roll"),
                        }
                    },
                    _ => println!("Invalid board.")
                }
            },
            (None, _, _) => {
                println!("Missing Next to Act Player's board!");
            },
            (_, None, _) => {
                println!("Missing Next to Act Opponent's board!");
            },
            (_, _, None) => {
                println!("Missing Roll!");
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("play") {
        let max_depth_to_brute_force = match matches.value_of("Max Depth to Brute Force") {
            Some(depth) => depth.parse::<usize>().unwrap(),
            None => DEFAULT_MAX_DEPTH_TO_BRUTE_FORCE
        };
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
                        let mut solver = Box::new(Solver::from_root(game.clone()));
                        let result = solver.get_best_moves_and_evaluation(
                            SolverMode::Hybrid(max_depth_to_brute_force, (heuristic_depth, |x| Solver::difference_heuristic(x, 3.5)))
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
        println!(
            "Game Over!  {}\nFinal Board: {}\nPlayer: {}\nSolver: {}",
            outcome,
            game.to_string_from_perspective(player),
            game.get_score(player),
            game.get_score(player.opponent())
        );
    } else {
        println!("Missing subcommand!");  
    }
}

fn get_int_from_arg_or_else(arg: Option<&str>, default: usize) -> usize {
    match arg {
        Some(arg) => arg.parse::<usize>().unwrap(),
        None => default
    }
}

#[cfg(test)]
mod test_integration_tests {
    use super::*;

}