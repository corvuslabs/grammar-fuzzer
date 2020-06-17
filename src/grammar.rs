//! # Grammar

use super::parser::{self, Token};
use super::shared::add_to_set;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::Deref;

pub struct Expansion<T> {
    pub string: String,
    pub opts: Option<T>,
}

impl<T> Expansion<T> {
    pub fn new(string: &str, opts: Option<T>) -> Self {
        Expansion {
            string: String::from(string),
            opts,
        }
    }
}

impl<T> fmt::Debug for Expansion<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

pub type Alternatives<T> = Vec<Expansion<T>>;

pub type Expansions<T> = HashMap<String, Alternatives<T>>;

#[derive(Debug)]
pub struct Grammar<T> {
    expansions: Expansions<T>,
}

impl<T> Deref for Grammar<T> {
    type Target = Expansions<T>;

    fn deref(&self) -> &Self::Target {
        &self.expansions
    }
}

impl From<HashMap<&str, Vec<&str>>> for Grammar<()> {
    fn from(input: HashMap<&str, Vec<&str>>) -> Self {
        let expansions = input
            .iter()
            .map(|(symbol, expansions)| {
                (
                    String::from(*symbol),
                    expansions
                        .iter()
                        .map(|exp| Expansion {
                            string: String::from(*exp),
                            opts: None,
                        })
                        .collect(),
                )
            })
            .collect();

        Grammar::new(expansions)
    }
}

impl<T> Grammar<T> {
    pub fn new(expansions: Expansions<T>) -> Self {
        Grammar { expansions }
    }

    /// symbol_cost is the minimum of the potential expansion costs
    pub fn symbol_cost(&self, symbol: &str, seen: &HashSet<&str>) -> f64 {
        self[symbol]
            .iter()
            .map(|expansion| self.expansion_cost(expansion, &add_to_set(seen, symbol)))
            .fold(f64::INFINITY, |min, c| if min < c { min } else { c })
    }

    /// expansion_cost is sum of the nonterminal symbol costs plus 1
    /// if we have visited one of the nonterminal symbols in the expansion before, the cost is infinity
    pub fn expansion_cost(&self, expansion: &Expansion<T>, seen: &HashSet<&str>) -> f64 {
        let nonterminals = nonterminal_tokens(&expansion.string);
        if nonterminals.is_empty() {
            return 1.0;
        }
        if nonterminals.iter().any(|token| seen.contains(token)) {
            return f64::INFINITY;
        }
        let step_cost: f64 = 1.0;
        let expansion_cost: f64 = nonterminals
            .iter()
            .map(|token| self.symbol_cost(token, seen))
            .sum();
        let cost: f64 = expansion_cost + step_cost;
        cost
    }

    /// is_valid_grammar looks for unreachable nonterminals, reachable nonterminals and unavoidable cycles
    pub fn is_valid_grammar(&self, start_symbol: Option<&str>) -> bool {
        let start_symbol = start_symbol.unwrap_or("<start>");
        let reachable_nonterminals = self.find_reachable_nonterminals(start_symbol);
        let defined_nonterminals: HashSet<&str> = self.keys().map(|t| t.as_str()).collect();
        let unreachable_nonterminals = &defined_nonterminals - &reachable_nonterminals;
        let undefined_nonterminals = &reachable_nonterminals - &defined_nonterminals;
        let cycle = self.find_unavoidable_cycle();
        if !unreachable_nonterminals.is_empty() {
            println!("unreachable nonterminals: {:?}", unreachable_nonterminals);
        }
        if !undefined_nonterminals.is_empty() {
            println!("undefined nonterminals: {:?}", undefined_nonterminals);
        }
        if !cycle.is_empty() {
            println!("tokens in unavoidable cycles: {:?}", cycle);
        }
        undefined_nonterminals.is_empty() & cycle.is_empty()
    }

    /// find_reachable_nonterminals returns reachable nonterminal symbols from a start symbol
    fn find_reachable_nonterminals<'a>(&'a self, symbol: &'a str) -> HashSet<&'a str> {
        let mut result = HashSet::new();
        let mut frontier = vec![symbol];
        while !frontier.is_empty() {
            let sym = frontier.pop().unwrap();
            if result.contains(sym) {
                continue;
            }
            result.insert(sym);
            if let Some(expansions) = self.get(sym) {
                for expansion in expansions {
                    frontier.extend(nonterminal_tokens(&expansion.string));
                }
            }
        }
        result
    }

    /// find_unavoidable_cycle returns the nonterminal symbols that appear in any unavoidable cycles
    fn find_unavoidable_cycle(&self) -> Vec<&str> {
        let defined_nonterminals: Vec<&str> = self.keys().map(|t| t.as_str()).collect();
        let costs: Vec<f64> = defined_nonterminals
            .iter()
            .map(|token| self.symbol_cost(token, &HashSet::new()))
            .collect();
        let mut tokens_in_cycle = Vec::new();
        for (idx, cost) in costs.iter().enumerate() {
            if *cost == f64::INFINITY {
                tokens_in_cycle.push(defined_nonterminals[idx]);
            }
        }
        tokens_in_cycle
    }
}

/// nonterminal_tokens returns the nonterminal symbols in the same order as the input string
fn nonterminal_tokens(input: &str) -> Vec<&str> {
    parser::tokens(input)
        .iter()
        .filter(|t| match t {
            Token::Nonterminal(_) => true,
            Token::Terminal(_) => false,
        })
        .map(|t| match t {
            Token::Nonterminal(t) => *t,
            Token::Terminal(_) => panic!(),
        })
        .collect()
}
