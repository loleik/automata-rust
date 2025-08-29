mod dfa;

use clap::{arg, Command};
use std::io::{self, stdin};
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
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

                for (x, _, z) in EXAMPLES {
                    println!("{x} : {z}")
                }

                let mut input: String = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed");

                let dfa_selector: usize = match input.trim().parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid input, please input a number");
                        return;
                    }
                };

                let (_, constructor, _) = EXAMPLES
                        .iter()
                        .find(|(id, _, _)| *id == dfa_selector)
                        .expect("Invalid choice");

                let dfa: DFA = constructor();

                DFA::visualize(&dfa);

                println!("DFA constructed. Please select input type:");
                println!("1 : Generate a random input stream\n2 : Specify input");

                input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed");

                let input_type: usize = match input.trim().parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid input, please input a number");
                        return;
                    }
                };

                match input_type {
                    1 => {
                        simulate(dfa, "random", None);
                    },
                    2 => {
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed");

                        simulate(dfa, "test", Some(&input));
                    },
                    _ => {println!("Invalid input")}
                }

                //println!("----------");
                //DFA::de_json();   
            }
            else { println!("Please enter [json] or [example]") }
        },
        _ => {}
    }
}
