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

fn simulate() {
    let end: u8 = rand::random_range(0..10); // Maximum length for input stream
    let dfa: DFA = DFA::new_basic_dfa();

    let mut i: u8 = 0;
    
    let mut state: String = dfa.start;

    for _ in 0..end {
        let next: char = match rand::random_bool(1.0 / 2.0) {
            true => '1',
            false => '0'
        }; // Generate the next value of the input stream

        println!("{:?} -> {:?}", &(state.clone(),next), dfa.transitions.get(&(state.clone(), next)));

        state = dfa.transitions.get(&(state, next)).unwrap().clone();

        i += 1;
    }

    println!();
    if dfa.accept.contains(&state) {
        println!("Yes, input stream has an even number of 0's");
    } else {
        println!("No, input stream as an odd number of 0's");
    }
}

fn main() {
    simulate()
}