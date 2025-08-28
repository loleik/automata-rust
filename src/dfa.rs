use core::fmt;
use std::collections::{HashSet, HashMap};
use serde::{Serialize, Deserialize};
use serde::de::{self, Visitor};
use serde_json::to_string_pretty;

// Now working on json branch
// This branch is for integrating json in a number of ways

#[derive(Serialize, Deserialize)]
pub struct DFA {
    states: HashSet<String>, // Set of all states, Q
    alphabet: HashSet<char>, // Set of input symbols, Σ
    transitions: HashMap<TransitionKey, String>, // Transition functions, δ: Q × Σ → Q
    start: String, // Initial state, q_0 ∈ Q
    accept: HashSet<String>, // Set of accepting/final states, F ⊆ Q
    description: Option<String>
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct TransitionKey(pub String, pub char);

impl Serialize for TransitionKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let key: String = format!("{}:{}", self.0, self.1);
        serializer.serialize_str(&key)
    }
}

impl<'de> Deserialize<'de> for TransitionKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        struct TransitionKeyVisitor;

        impl<'de> Visitor<'de> for TransitionKeyVisitor {
            type Value = TransitionKey;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a string in the format state:char")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut parts = v.splitn(2, ':');
                let state: String = parts 
                    .next()
                    .ok_or_else(|| E::custom("missing state"))?
                    .to_string();
                let ch: char = parts
                    .next()
                    .ok_or_else(|| E::custom("missing char"))?
                    .chars()
                    .next()
                    .ok_or_else(|| E::custom("empty char"))?;

                Ok(TransitionKey(state, ch))
            }
        }

        deserializer.deserialize_str(TransitionKeyVisitor)
    }
}

pub static EXAMPLES: [(usize, fn() -> DFA, &str); 2] = [
    (1, DFA::even_zeros, "Check if a binary string contains an even number of 0's"),
    (2, DFA::starts_ends_a, "Check if a string starts and ends with an a")
];

impl DFA {
    // Generate a basic DFA from the first example at:
    // https://en.wikipedia.org/wiki/Deterministic_finite_automaton
    pub fn even_zeros() -> DFA {
        let transitions: HashMap<TransitionKey, String> = HashMap::from([
            (TransitionKey("S1".to_string(), '0'), "S2".to_string()),
            (TransitionKey("S1".to_string(), '1'), "S1".to_string()),
            (TransitionKey("S2".to_string(), '0'), "S1".to_string()),
            (TransitionKey("S2".to_string(), '1'), "S2".to_string())  
        ]);

        DFA {
            states: HashSet::from([
                "S1".to_string(),
                "S2".to_string()
            ]),
            alphabet: HashSet::from(['0', '1']),
            transitions: transitions,
            start: "S1".to_string(),
            accept: HashSet::from(["S1".to_string()]),
            description: Some("Check if a binary string contains an even number of 0's".to_string())
        }
    }

    pub fn starts_ends_a() -> DFA {
        let transitions: HashMap<TransitionKey, String> = HashMap::from([
            (TransitionKey("S0".to_string(), 'a'), "S1".to_string()),
            (TransitionKey("S0".to_string(), 'b'), "S3".to_string()),

            (TransitionKey("S1".to_string(), 'a'), "S1".to_string()),
            (TransitionKey("S1".to_string(), 'b'), "S2".to_string()),

            (TransitionKey("S2".to_string(), 'a'), "S1".to_string()),
            (TransitionKey("S2".to_string(), 'b'), "S2".to_string()),

            (TransitionKey("S3".to_string(), 'a'), "S3".to_string()),
            (TransitionKey("S3".to_string(), 'b'), "S3".to_string())
        ]);

        DFA {
            states: HashSet::from([
                "S0".to_string(),
                "S1".to_string(),
                "S2".to_string(),
                "S3".to_string()
            ]),
            alphabet: HashSet::from(['a', 'b']),
            transitions: transitions,
            start: "S0".to_string(),
            accept: HashSet::from(["S1".to_string()]),
            description: Some("Check if a string starts and ends with an a".to_string())
        }
    }

    pub fn visualize(dfa: &DFA) {
        println!("{}", to_string_pretty(dfa).unwrap())
    }

    pub fn de_json() {
        let test: &'static str =  r#"
        {
          "states": ["q0", "q1"],
          "alphabet": ["a", "b"],
          "transitions": {
            "q0:a": "q1",
            "q0:b": "q0",
            "q1:a": "q1",
            "q1:b": "q0"
          },
          "start": "q0",
          "accept": ["q1"],
          "description": "Example DFA"
        }
        "#;

        let dfa: DFA = serde_json::from_str(test).unwrap();

        DFA::visualize(&dfa);
    }
}

pub fn simulate(
    dfa: DFA,
    mode: &str,
    test: Option<&str>
) -> bool {
    let mut state: TransitionKey = TransitionKey(dfa.start, 'x');

    match mode {
        "random" => {
            let end: u8 = rand::random_range(0..u8::MAX); // Maximum length for input stream

            for _ in 0..end {
                 // Generate the next value of the input stream
                let ind: usize = rand::random_range(0..dfa.alphabet.len());
                state.1 = *dfa.alphabet.iter().nth(ind).unwrap();

                /*println!("{:?} -> {}",
                    state,
                    dfa.transitions.get(&state).unwrap().clone()
                );*/
                
                state.0 = dfa.transitions.get(&state).unwrap().clone();
            }
            println!();
        }
        "test" => {
            let end: usize = test.unwrap_or("").len();
            println!("{test:?}");

            for i in 0..end {
                state.1 = test.unwrap_or("").chars().nth(i).unwrap();
                
                state.0 = dfa.transitions.get(&state).unwrap().clone();
            }
        }
        _ => {}
    }

    if dfa.accept.contains(&state.0) {
        println!("TRUE");
        true
    } else {
        println!("FALSE");
        false
    }
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