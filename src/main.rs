use std::collections::{HashSet, HashMap};

// Now working on json branch

struct DFA {
    states: HashSet<String>, // Set of all states, Q
    alphabet: HashSet<char>, // Set of input symbols, Σ
    transitions: HashMap<(String, char), String>, // Transition functions, δ: Q × Σ → Q
    start: String, // Initial state, q_0 ∈ Q
    accept: HashSet<String> // Set of accepting/final states, F ⊆ Q
}

impl DFA {
    // Generate a basic DFA from the first example at:
    // https://en.wikipedia.org/wiki/Deterministic_finite_automaton
    fn even_zeros() -> DFA {
        DFA {
            states: HashSet::from([
                "S1".to_string(),
                "S2".to_string()
            ]),
            alphabet: HashSet::from(['0', '1']),
            transitions: HashMap::from([
                (("S1".to_string(), '0'), "S2".to_string()),
                (("S1".to_string(), '1'), "S1".to_string()),

                (("S2".to_string(), '0'), "S1".to_string()),
                (("S2".to_string(), '1'), "S2".to_string())
            ]),
            start: "S1".to_string(),
            accept: HashSet::from(["S1".to_string()])
        }
    }

    fn starts_ends_a() -> DFA {
        DFA {
            states: HashSet::from([
                "S0".to_string(),
                "S1".to_string(),
                "S2".to_string(),
                "S3".to_string()
            ]),
            alphabet: HashSet::from(['a', 'b']),
            transitions: HashMap::from([
                (("S0".to_string(), 'a'), "S1".to_string()),
                (("S0".to_string(), 'b'), "S3".to_string()),

                (("S1".to_string(), 'a'), "S1".to_string()),
                (("S1".to_string(), 'b'), "S2".to_string()),

                (("S2".to_string(), 'a'), "S1".to_string()),
                (("S2".to_string(), 'b'), "S2".to_string()),

                (("S3".to_string(), 'a'), "S3".to_string()),
                (("S3".to_string(), 'b'), "S3".to_string())
            ]),
            start: "S0".to_string(),
            accept: HashSet::from(["S1".to_string()])
        }
    }
}

fn simulate(
    dfa: DFA,
    mode: &str,
    test: Option<&str>
) -> bool {
    let mut state: String = dfa.start;

    match mode {
        "random" => {
            let end: u8 = rand::random_range(0..u8::MAX); // Maximum length for input stream

            for _ in 0..end {
                 // Generate the next value of the input stream
                let ind: usize = rand::random_range(0..dfa.alphabet.len());
                let next: char = *dfa.alphabet.iter().nth(ind).unwrap();

                println!("{:?} -> {}",
                    &(state.clone(),next),
                    dfa.transitions.get(&(state.clone(), next)).unwrap().clone()
                );
                
                state = dfa.transitions.get(&(state, next)).unwrap().clone();
            }
            println!();
        }
        "test" => {
            let end: usize = test.unwrap_or("").len();

            for i in 0..end {
                let next: char = test.unwrap_or("").chars().nth(i).unwrap();
                
                state = dfa.transitions.get(&(state, next)).unwrap().clone();
            }
        }
        _ => {}
    }

    if dfa.accept.contains(&state) {
        println!("TRUE");
        true
    } else {
        println!("FALSE");
        false
    }
}

fn main() {
    let dfa: DFA = DFA::starts_ends_a();

    simulate(dfa, "random", None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        let dfa: DFA = DFA::even_zeros();

        let result: bool = simulate(dfa, "test", None);

        assert_eq!(result, true)
    }

    #[test]
    fn even_input() {
        let dfa: DFA = DFA::even_zeros();

        let result: bool = simulate(dfa, "test", Some("00"));

        assert_eq!(result, true)
    }

    #[test]
    fn odd_input() {
        let dfa: DFA = DFA::even_zeros();

        let result: bool = simulate(dfa, "test", Some("01"));

        assert_eq!(result, false)
    }

    #[test]
    fn all_ones() {
        let dfa: DFA = DFA::even_zeros();

        let result: bool = simulate(dfa, "test", Some("111"));

        assert_eq!(result, true)
    }

    #[test]
    fn single_zero() {
        let dfa: DFA = DFA::even_zeros();

        let result: bool = simulate(dfa, "test", Some("0"));

        assert_eq!(result, false)
    }

    #[test]
    fn even_zeros_with_ones() {
        let dfa: DFA = DFA::even_zeros();

        let result: bool = simulate(dfa, "test", Some("0101010"));

        assert_eq!(result, true);
    }

    #[test]
    fn odd_zeros_with_ones() {
        let dfa: DFA = DFA::even_zeros();

        let result: bool = simulate(dfa, "test", Some("101010"));

        assert_eq!(result, false);
    }

    #[test]
    fn long_input_even_zeros() {
        let dfa: DFA = DFA::even_zeros();

        let result: bool = simulate(dfa, "test", Some("0".repeat(100).as_str()));
        
        assert_eq!(result, true);
    }
}