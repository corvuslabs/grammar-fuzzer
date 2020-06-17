//! A grammar fuzzer that supports different expansion strategies
//!
//! # Example
//!
//! ```
//! use grammar_fuzzer::{Grammar, GrammarFuzzer, Node};
//! use grammar_fuzzer::{RandomStrategy, Strategy};
//! use std::collections::HashMap;
//! 
//! // Strategies
//! let random = RandomStrategy::new(10, 8000);
//! let strategies: Vec<&dyn Strategy<()>> = vec![&random];
//! // Grammar
//! let expansios: HashMap<_, _> = [
//!         ("<string>", vec!["<string><char>", "<char>"]),
//!         ("<char>", vec!["a", "b", "c", "d"]),
//!     ]
//!     .iter()
//!     .cloned()
//!     .collect();
//! let grammar = Grammar::from(expansios);
//! assert_eq!(grammar.is_valid_grammar(Some("<string>")), true);
//! // Fuzzer
//! let fuzzer = GrammarFuzzer::new(grammar, &strategies);
//! // Expand the derivation tree
//! let mut node = Node::N(String::from("<string>"));
//! fuzzer.expand_tree(&mut node);
//! println!("{}\n", node);
//! ```
use super::derivation_tree::{Children, Node};
use super::grammar::Grammar;
use super::shared::random_element;
use super::strategy::Strategy;

pub struct GrammarFuzzer<'a, T> {
    grammar: Grammar<T>,
    steps: &'a Vec<&'a dyn Strategy<T>>,
}

impl<'a, T> GrammarFuzzer<'a, T> {
    pub fn new(grammar: Grammar<T>, steps: &'a Vec<&'a dyn Strategy<T>>) -> GrammarFuzzer<'a, T> {
        GrammarFuzzer { grammar, steps }
    }

    /// Selects an expansion given a strategy and divides the expansion-string into
    /// a sequence of terminal and nonterminal child nodes
    fn expand_nonterminal(&self, node: &Node, strategy: &dyn Strategy<T>) -> Children {
        match node {
            Node::N(_) => {
                let chosen_expantion = strategy.choose(&self.grammar, &node).unwrap();
                let children = Children::from(chosen_expantion.as_str());
                children
            }
            _ => panic!(),
        }
    }

    /// Expands a nonterminal leaf node in the derivation tree
    fn expand_tree_once(&self, node: &mut Node, strategy: &dyn Strategy<T>) {
        match node {
            Node::T(_) => (),
            Node::N(sym) => {
                let children = self.expand_nonterminal(&Node::N(sym.to_owned()), strategy);
                let new_subtree = Node::new_expanded(sym, children);
                std::mem::replace(node, new_subtree);
            }
            Node::EN(_, Children { roots }) => {
                let random_root = random_element(&roots, |r| r.borrow().any_possible_expansions());
                if let Some(root) = random_root {
                    let root: &mut Node = &mut *root.borrow_mut();
                    self.expand_tree_once(root, strategy);
                }
                ()
            }
        }
    }

    /// Expands the derivation tree following a strategy
    /// it terminates when `strategy.cont` returns false or when all the nonterminal nodes have been expanded
    fn expand_tree_with_strategy(&self, root: &mut Node, strategy: &dyn Strategy<T>) {
        let mut step = 0;
        loop {
            if !root.any_possible_expansions() {
                break;
            }

            if !strategy.cont(root, step) {
                break;
            }

            self.expand_tree_once(root, strategy);
            step += 1;
        }
    }

    /// Applies a sequence of strategies
    pub fn expand_tree(&self, root: &mut Node) {
        for strategy in self.steps {
            self.expand_tree_with_strategy(root, *strategy);
        }
    }
}
