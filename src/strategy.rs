use super::derivation_tree::Node;
use super::grammar::{Expansion, Grammar, Symbol};
use super::shared::{max_idx, min_idx};
use rand::Rng;

pub trait Strategy<T> {
    fn cont(&self, dt_root: &Node, num_steps: usize) -> bool;
    fn choose<'a>(&self, grammar: &Grammar<'a, T>, node: &Node) -> Option<Symbol<'a>>;
}

// -------------------------------- Random ------------------------------------

pub struct RandomStrategy {
    nonterminals_threshold: usize,
    max_steps: usize,
}

impl RandomStrategy {
    pub fn new(nonterminals_threshold: usize, max_steps: usize) -> Self {
        RandomStrategy {
            nonterminals_threshold,
            max_steps,
        }
    }
}

impl<T> Strategy<T> for RandomStrategy {
    fn cont(&self, dt_root: &Node, num_steps: usize) -> bool {
        dt_root.num_possible_expansions() < self.nonterminals_threshold
            && num_steps < self.max_steps
    }

    fn choose<'a>(&self, grammar: &Grammar<'a, T>, node: &Node) -> Option<Symbol<'a>> {
        match node {
            Node::N(sym) => {
                let expansions = &grammar[*sym];
                let rand_idx = rand::thread_rng().gen_range(0, expansions.len());
                let choosen_expansion = expansions[rand_idx].symbol;
                Some(choosen_expansion)
            }
            _ => None,
        }
    }
}

// -------------------------------- Growth ------------------------------------

pub struct GrowthStrategy {
    nonterminals_threshold: usize,
    max_steps: usize,
}

impl GrowthStrategy {
    pub fn new(nonterminals_threshold: usize, max_steps: usize) -> Self {
        GrowthStrategy {
            nonterminals_threshold,
            max_steps,
        }
    }
}

impl<T> Strategy<T> for GrowthStrategy {
    fn cont(&self, dt_root: &Node, num_steps: usize) -> bool {
        dt_root.num_possible_expansions() < self.nonterminals_threshold
            && num_steps < self.max_steps
    }

    fn choose<'a>(&self, grammar: &Grammar<'a, T>, node: &Node) -> Option<Symbol<'a>> {
        match node {
            Node::N(sym) => {
                let expansions = &grammar[*sym];
                let costs = costs(grammar, sym, expansions);
                let max_idx = max_idx(&costs);
                let choosen_expansion = expansions[max_idx].symbol;
                Some(choosen_expansion)
            }
            _ => None,
        }
    }
}

// -------------------------------- Close -------------------------------------

pub struct CloseStrategy {}

impl CloseStrategy {
    pub fn new() -> Self {
        CloseStrategy {}
    }
}

impl<T> Strategy<T> for CloseStrategy {
    fn cont(&self, _dt_root: &Node, _num_steps: usize) -> bool {
        true
    }

    fn choose<'a>(&self, grammar: &Grammar<'a, T>, node: &Node) -> Option<Symbol<'a>> {
        match node {
            Node::N(sym) => {
                let expansions = &grammar[*sym];
                let costs = costs(grammar, sym, expansions);
                let min_idx = min_idx(&costs);
                let choosen_expansion = expansions[min_idx].symbol;
                Some(choosen_expansion)
            }
            _ => None,
        }
    }
}

// ---------------------------------- Helpers ---------------------------------

fn costs<T>(grammar: &Grammar<T>, sym: Symbol, expansions: &Vec<Expansion<T>>) -> Vec<f64> {
    let seen = [sym].iter().cloned().collect();
    expansions
        .iter()
        .map(|expansion| grammar.expansion_cost(expansion, &seen))
        .collect()
}
