//! A grammar fuzzer following the ["Fuzzing with Grammars"](https://www.fuzzingbook.org/html/Grammars.html) and
//! ["Efficient Grammar Fuzzing"](https://www.fuzzingbook.org/html/GrammarFuzzer.html) chapters of ["The Fuzzing Book"](https://www.fuzzingbook.org/) as a starting point.
//!
//! ## Example
//!
//! ```
//! use grammar_fuzzer::extensions::ebnf_to_bnf;
//! use grammar_fuzzer::{CloseStrategy, GrowthStrategy, RandomStrategy, Strategy};
//! use grammar_fuzzer::{Grammar, GrammarFuzzer, Node};
//! use std::collections::HashMap;
//!
//! fn json_grammar() -> Grammar<()> {
//!     let expansios: HashMap<_, _> = [
//!         ("<start>", vec!["<assoc>"]),
//!         (
//!             "<value>",
//!             vec!["<assoc>", "<list>", "<bool>", "<string>", "<int>"],
//!         ),
//!         ("<assoc>", vec!["{(<string>: <value>, )*<string>: <value>}"]),
//!         ("<list>", vec!["[(<value>, )*<value>]"]),
//!         ("<bool>", vec!["true", "false"]),
//!         ("<string>", vec!["\"<char>+\""]),
//!         ("<char>", vec!["a", "b", "c", "d"]),
//!         ("<int>", vec!["<digit>+"]),
//!         (
//!             "<digit>",
//!             vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
//!         ),
//!     ]
//!     .iter()
//!     .cloned()
//!     .collect();
//!
//!     Grammar::from(&expansios)
//! }
//!
//! fn main() {
//!     // Fuzzer
//!     let expansion = GrowthStrategy::new(0, 1000);
//!     let random = RandomStrategy::new(40, 8000);
//!     let close = CloseStrategy::new();
//!     let strategies: Vec<&dyn Strategy<()>> = vec![&expansion, &random, &close];
//!
//!     let ebnf_json_grammar = json_grammar();
//!     let json_grammar = ebnf_to_bnf(&ebnf_json_grammar);
//!     assert_eq!(json_grammar.is_valid_grammar(None), true);
//!
//!     let fuzzer = GrammarFuzzer::new(json_grammar, &strategies);
//!
//!     // Sample
//!     for _ in 0..40 {
//!         let mut node = Node::N(String::from("<start>"));
//!         fuzzer.expand_tree(&mut node);
//!         println!("{}\n", node);
//!     }
//! }
//!
//! ```

mod parser;
mod shared;

pub mod derivation_tree;
pub mod extensions;
pub mod fuzzer;
pub mod grammar;
pub mod strategy;

pub use derivation_tree::{Children, Node};
pub use extensions::ebnf_to_bnf;
pub use fuzzer::GrammarFuzzer;
pub use grammar::{Alternatives, Expansion, Expansions, Grammar};
pub use strategy::{CloseStrategy, GrowthStrategy, RandomStrategy, Strategy};
