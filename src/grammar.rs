//! # Grammar

use super::parser::{self, Token};
use super::shared::add_to_set;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::Deref;

// ------------------------------------ Types ---------------------------------

pub type Symbol<'a> = &'a str;

pub struct Expansion<'a, T> {
    pub symbol: Symbol<'a>,
    pub opts: Option<T>,
}

pub type Expansions<'a, T> = HashMap<Symbol<'a>, Vec<Expansion<'a, T>>>;

#[derive(Debug)]
pub struct Grammar<'a, T> {
    expansions: Expansions<'a, T>,
}

// ---------------------------------- Expansion -------------------------------

impl<'a, T> fmt::Debug for Expansion<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl<'a, T> Deref for Expansion<'a, T> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.symbol
    }
}

// ---------------------------------- Grammar ---------------------------------

impl<'a, T> Grammar<'a, T> {
    pub fn new(expansions: Expansions<'a, T>) -> Self {
        Grammar { expansions }
    }

    pub fn symbol_cost(&self, symbol: Symbol, seen: &HashSet<Symbol>) -> f64 {
        self[symbol]
            .iter()
            .map(|expansion| self.expansion_cost(expansion, &add_to_set(seen, symbol)))
            .fold(f64::INFINITY, |min, c| if min < c { min } else { c })
    }

    pub fn expansion_cost(&self, expansion: &Expansion<T>, seen: &HashSet<Symbol>) -> f64 {
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

    pub fn is_valid_grammar(&self, start_symbol: Option<Symbol>) -> bool {
        let start_symbol = start_symbol.unwrap_or("<start>");
        let reachable_nonterminals = self.find_reachable_nonterminals(start_symbol);
        let defined_nonterminals: HashSet<&str> = self.keys().map(|t| *t).collect();
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

    fn find_reachable_nonterminals(&self, sym: Symbol<'a>) -> HashSet<Symbol<'a>> {
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

    fn find_unavoidable_cycle(&self) -> Vec<Symbol> {
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

impl<'a, T> Deref for Grammar<'a, T> {
    type Target = Expansions<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.expansions
    }
}

fn nonterminal_tokens<'a, T>(input: &Expansion<'a, T>) -> Vec<Symbol<'a>> {
    parser::tokens(input.symbol)
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
