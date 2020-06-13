use super::derivation_tree::Node;
use super::grammar::Grammar;
use super::shared::{max_idx, min_idx};
use rand::Rng;

pub trait Strategy {
    fn cont(&self, dt_root: &Node, num_steps: usize) -> bool;
    fn choose<'a>(&self, grammar: &'a Grammar, node: &Node) -> Option<&'a str>;
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

impl Strategy for RandomStrategy {
    fn cont(&self, dt_root: &Node, num_steps: usize) -> bool {
        dt_root.num_possible_expansions() < self.nonterminals_threshold
            && num_steps < self.max_steps
    }

    fn choose<'a>(&self, grammar: &'a Grammar, node: &Node) -> Option<&'a str> {
        match node {
            Node::N(sym) => {
                let expansions = &grammar[&sym[..]];
                let rand_idx = rand::thread_rng().gen_range(0, expansions.len());
                let choosen_expansion = expansions[rand_idx];
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

impl Strategy for GrowthStrategy {
    fn cont(&self, dt_root: &Node, num_steps: usize) -> bool {
        dt_root.num_possible_expansions() < self.nonterminals_threshold
            && num_steps < self.max_steps
    }

    fn choose<'a>(&self, grammar: &'a Grammar, node: &Node) -> Option<&'a str> {
        match node {
            Node::N(sym) => {
                let expansions = &grammar[&sym[..]];
                let costs = costs(grammar, sym, expansions);
                let max_idx = max_idx(&costs);
                let choosen_expansion = expansions[max_idx];
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

impl Strategy for CloseStrategy {
    fn cont(&self, _dt_root: &Node, _num_steps: usize) -> bool {
        true
    }

    fn choose<'a>(&self, grammar: &'a Grammar, node: &Node) -> Option<&'a str> {
        match node {
            Node::N(sym) => {
                let expansions = &grammar[&sym[..]];
                let costs = costs(grammar, sym, expansions);
                let min_idx = min_idx(&costs);
                let choosen_expansion = expansions[min_idx];
                Some(choosen_expansion)
            }
            _ => None,
        }
    }
}

// ---------------------------------- Helpers ---------------------------------

fn costs(grammar: &Grammar, sym: &str, expansions: &Vec<&str>) -> Vec<f64> {
    let seen = [sym].iter().cloned().collect();
    expansions
        .iter()
        .map(|expansion| grammar.expansion_cost(expansion, &seen))
        .collect()
}
