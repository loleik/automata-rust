use std::collections::{HashSet, HashMap};

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
    fn new_basic_dfa() -> DFA {
        DFA {
            states: HashSet::from(["S1".to_string(), "S2".to_string()]),
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
}

fn simulate(
    dfa: DFA,
    mode: &str,
    test: Option<&str>
) -> bool {
    let mut state: String = dfa.start;

    match mode {
        "random" => {
            let end: u8 = rand::random_range(0..10); // Maximum length for input stream    

            for _ in 0..end {
                let next: char = match rand::random_bool(1.0 / 2.0) {
                    true => '1',
                    false => '0'
                }; // Generate the next value of the input stream
                
                state = dfa.transitions.get(&(state, next)).unwrap().clone();
            }
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
        println!("Yes, input stream has an even number of 0's (or none)");
        true
    } else {
        println!("No, input stream as an odd number of 0's");
        false
    }
}

fn main() {
    let dfa: DFA = DFA::new_basic_dfa();

    //simulate(dfa, "random", None);

    simulate(dfa, "test", Some("111"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        let dfa: DFA = DFA::new_basic_dfa();

        let result: bool = simulate(dfa, "test", None);

        assert_eq!(result, true)
    }

    #[test]
    fn even_input() {
        let dfa: DFA = DFA::new_basic_dfa();

        let result: bool = simulate(dfa, "test", Some("00"));

        assert_eq!(result, true)
    }

    #[test]
    fn odd_input() {
        let dfa: DFA = DFA::new_basic_dfa();

        let result: bool = simulate(dfa, "test", Some("01"));

        assert_eq!(result, false)
    }

    #[test]
    fn all_ones() {
        let dfa: DFA = DFA::new_basic_dfa();

        let result: bool = simulate(dfa, "test", Some("111"));

        assert_eq!(result, true)
    }

    #[test]
    fn single_zero() {
        let dfa: DFA = DFA::new_basic_dfa();

        let result: bool = simulate(dfa, "test", Some("0"));

        assert_eq!(result, false)
    }

    #[test]
    fn even_zeros_with_ones() {
        let dfa: DFA = DFA::new_basic_dfa();

        let result: bool = simulate(dfa, "test", Some("0101010"));

        assert_eq!(result, true);
    }

    #[test]
    fn odd_zeros_with_ones() {
        let dfa: DFA = DFA::new_basic_dfa();

        let result: bool = simulate(dfa, "test", Some("101010"));

        assert_eq!(result, false);
    }

    #[test]
    fn long_input_even_zeros() {
        let dfa: DFA = DFA::new_basic_dfa();

        let result: bool = simulate(dfa, "test", Some("0".repeat(100).as_str()));
        
        assert_eq!(result, true);
    }
}