mod dfa;

use clap::{arg, Command};
use std::io;
use dfa::{DFA, simulate};

use crate::dfa::EXAMPLES;

fn cli() -> Command {
    Command::new("automata")
        .about("Several different automata based simulations")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("dfa")
            .about("Deterministic Finite Automata")
            .arg(arg!(<TYPE> "[json] input or pre-defined [example]"))
            .arg_required_else_help(true)
        )
}

fn main() {
    let matches: clap::ArgMatches = cli().get_matches();

    match matches.subcommand() {
        Some(("dfa", sub_matches)) => {
            let mode: &String = sub_matches.get_one::<String>("TYPE").unwrap();

            if mode.as_str() == "json" {}
            else if mode == "example" {
                for (x, _, z) in EXAMPLES {
                    println!("{x} : {z}")
                }

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed");

                let number: usize = match input.trim().parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid input, please input a number");
                        return;
                    }
                };

                let (_, constructor, description) = EXAMPLES
                        .iter()
                        .find(|(id, _, _)| *id == number)
                        .expect("Invalid choice");

                let dfa: DFA = constructor();

                DFA::visualize(&dfa);
                simulate(dfa, "test", Some("00"));
            }
            else { println!("Please enter [json] or [example]") }
        },
        _ => {}
    }
}
