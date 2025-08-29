// I'll be writing up some stuff for docs in this branch

mod dfa;

use clap::{arg, Command};
use std::io::{self, Write};
use std::fs;
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

fn cls() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    io::stdout().flush().unwrap();
}

fn grab_number() -> usize {
    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed");

        match input.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => {
                println!("Invalid input, please input a number");
                continue;
            }
        };
    };
}

fn grab_string(dfa: Option<&DFA>) -> String {
    'outer: loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed");
        if dfa.is_some() {
            for c in input.trim().chars() {
                if !dfa.unwrap().alphabet.contains(&c) {
                    println!("Input must be in the DFA alphabet {c}");
                    println!("{:?}", dfa.unwrap().alphabet);
                    continue 'outer;
                }
            }
        }

        return input.trim().to_string()
    };
}

fn main() -> io::Result<()> {
    let matches: clap::ArgMatches = cli().get_matches();

    cls();

    match matches.subcommand() {
        Some(("dfa", sub_matches)) => {
            let mode: &String = sub_matches.get_one::<String>("TYPE").unwrap();

            let dfa: DFA = match mode.as_str() {
                "json" => {
                    println!("Give me a file name!");
                    let json_data: String = fs::read_to_string(grab_string(None))?;

                    match DFA::de_json(&json_data) {
                        Ok(dfa) => dfa,
                        Err(errors) => {
                            for e in errors {
                                eprintln!(" - {e}");
                            };

                            return Err(
                                io::Error::new(
                                    io::ErrorKind::InvalidInput, 
                                    "There were errors with the provided file"
                                )
                            );
                        }
                    }
                }
                "example" => {
                    for (x, _, z) in EXAMPLES {
                        println!("{x} : {z}")
                    }

                    let dfa_selector: usize = grab_number();

                    let (_, constructor, _) = EXAMPLES
                            .iter()
                            .find(|(id, _, _)| *id == dfa_selector)
                            .expect("Invalid choice");

                    constructor()
                }
                _ => {
                    return Err(
                        io::Error::new(
                            io::ErrorKind::InvalidInput, 
                            "Please enter [json] or [example]"
                        )
                    );
                }
            };

            println!();

            DFA::visualize(&dfa);

            println!();
            println!("DFA constructed. Please select input type:");
            println!("1 : Generate a random input stream\n2 : Specify input");

            let input_type: usize = grab_number();

            println!();

            match input_type {
                1 => {
                    simulate(dfa, "random", None);
                },
                2 => {
                    println!("Please enter input:");
                    
                    let input: String = grab_string(Some(&dfa));

                    simulate(dfa, "test", Some(&input.trim()));
                },
                _ => { println!("Invalid input") }
            }
        },
        _ => {}
    }

    Ok(())
}
