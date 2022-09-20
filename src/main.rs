mod board;
mod tree;
mod solver;

use clap::{App, SubCommand, Arg};
use crate::board::board::{Board, Move, Outcome};
use crate::solver::solver::Solver;



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
    } else {
        println!("Invalid command!");
    }
}

#[cfg(test)]
mod test_integration_tests {
    use super::*;

}