//! # Grammar

use super::parser::{self, Token};
use super::shared::add_to_set;
use std::collections::{HashMap, HashSet};

pub type Expansions<'a> = HashMap<&'a str, Vec<&'a str>>;

#[derive(Debug)]
pub struct Grammar<'a> {
    expansions: Expansions<'a>,
}

impl<'a> Grammar<'a> {
    pub fn new(expansions: Expansions<'a>) -> Self {
        Grammar { expansions }
    }

    pub fn symbol_cost(&self, symbol: &str, seen: &HashSet<&str>) -> f64 {
        self[symbol]
            .iter()
            .map(|expansion| self.expansion_cost(expansion, &add_to_set(seen, symbol)))
            .fold(f64::INFINITY, |min, c| if min < c { min } else { c })
    }

    pub fn expansion_cost(&self, expansion: &str, seen: &HashSet<&str>) -> f64 {
        let nonterminals = nonterminal_tokens(expansion);
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

    pub fn is_valid_grammar(&self, start_symbol: Option<&str>) -> bool {
        let start_symbol = start_symbol.unwrap_or("<start>");
        let reachable_nonterminals = self.find_reachable_nonterminals(start_symbol);
        let defined_nonterminals: HashSet<&str> = self.keys().map(|t| *t).collect();
        let unreachable_nonterminals = &defined_nonterminals - &reachable_nonterminals;
        let undefined_nonterminals = &reachable_nonterminals - &defined_nonterminals;
        let cycle = self.find_unavoidable_cycle(start_symbol);
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

    fn find_reachable_nonterminals<'b>(&'b self, sym: &'b str) -> HashSet<&'b str> {
        let mut result = HashSet::new();
        let mut frontier = vec![sym];
        while !frontier.is_empty() {
            let sym = frontier.pop().unwrap();
            if result.contains(sym) {
                continue;
            }
            result.insert(sym);
            if let Some(expansions) = self.get(sym) {
                for expansion in expansions {
                    frontier.extend(nonterminal_tokens(expansion));
                }
            }
        }
        result
    }

    fn find_unavoidable_cycle<'b>(&'b self, _start_symbol: &'b str) -> Vec<&'b str> {
        let defined_nonterminals: Vec<&str> = self.keys().map(|t| *t).collect();
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

use std::ops::Deref;
impl<'a> Deref for Grammar<'a> {
    type Target = Expansions<'a>;

    fn deref(&self) -> &Self::Target {
        &self.expansions
    }
}

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
