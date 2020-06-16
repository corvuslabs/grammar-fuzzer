use super::derivation_tree::Node;
use super::grammar::{Alternatives, Grammar};
use super::shared::{max_idx, min_idx};
use rand::Rng;

pub trait Strategy<T> {
    /// cont defines wheather to continue expanding the derivation tree following the current strategy
    /// dt_root: is the root node of the derivation-tree
    /// num_steps: is how many times the derivation tree was expanded following the current strategy
    fn cont(&self, dt_root: &Node, num_steps: usize) -> bool;

    /// choose selects an expansion-string for a given nonterminal node
    fn choose(&self, grammar: &Grammar<T>, node: &Node) -> Option<String>;
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
    /// continue until reaching the expected number of nonterminal nodes or passing the expansions limit
    fn cont(&self, dt_root: &Node, num_steps: usize) -> bool {
        dt_root.num_possible_expansions() < self.nonterminals_threshold
            && num_steps < self.max_steps
    }

    /// choose a random expansion
    fn choose(&self, grammar: &Grammar<T>, node: &Node) -> Option<String> {
        match node {
            Node::N(symbol) => {
                let expansions = &grammar[symbol];
                let rand_idx = rand::thread_rng().gen_range(0, expansions.len());
                let choosen_expansion = expansions[rand_idx].string.clone();
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
    /// continue until reaching the expected number of nonterminal nodes or passing the expansions limit
    fn cont(&self, dt_root: &Node, num_steps: usize) -> bool {
        dt_root.num_possible_expansions() < self.nonterminals_threshold
            && num_steps < self.max_steps
    }

    /// choose an expansion that maximizes the cost
    fn choose(&self, grammar: &Grammar<T>, node: &Node) -> Option<String> {
        match node {
            Node::N(symbol) => {
                let expansions = &grammar[symbol];
                let costs = costs(grammar, symbol, expansions);
                let max_idx = max_idx(&costs);
                let choosen_expansion = expansions[max_idx].string.clone();
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
    /// continue until all the nodes have been expanded
    fn cont(&self, _dt_root: &Node, _num_steps: usize) -> bool {
        true
    }

    /// choose an expansion that minimizes the cost
    fn choose(&self, grammar: &Grammar<T>, node: &Node) -> Option<String> {
        match node {
            Node::N(symbol) => {
                let expansions = &grammar[symbol];
                let costs = costs(grammar, symbol, expansions);
                let min_idx = min_idx(&costs);
                let choosen_expansion = expansions[min_idx].string.clone();
                Some(choosen_expansion)
            }
            _ => None,
        }
    }
}

// ---------------------------------- Helpers ---------------------------------

fn costs<T>(grammar: &Grammar<T>, sym: &str, expansions: &Alternatives<T>) -> Vec<f64> {
    let seen = [sym].iter().cloned().collect();
    expansions
        .iter()
        .map(|expansion| grammar.expansion_cost(expansion, &seen))
        .collect()
}

#[cfg(test)]
mod strategy_cont_tests {
    use super::*;
    use crate::derivation_tree::{
        Children,
        Node::{EN, N},
    };
    use std::cell::RefCell;

    fn sample_derivation_tree(num_nonterminals: usize) -> Node {
        if num_nonterminals > 1 {
            EN(
                String::from("<int>"),
                Children {
                    roots: vec![
                        RefCell::new(sample_derivation_tree(num_nonterminals - 1)),
                        RefCell::new(N(String::from("<digit>"))),
                    ],
                },
            )
        } else {
            N(String::from("<digit>"))
        }
    }

    #[test]
    fn test_random_strategy_cont() {
        let nonterminals_threshold = 10;
        let max_steps = 10;
        let below_max_steps = max_steps - 1;
        let below_nonterminal_threshold = sample_derivation_tree(nonterminals_threshold - 1);
        let at_nonterminal_threshold = sample_derivation_tree(nonterminals_threshold);
        let strategy: &dyn Strategy<()> = &RandomStrategy::new(nonterminals_threshold, max_steps);

        assert_eq!(
            strategy.cont(&below_nonterminal_threshold, below_max_steps),
            true
        );
        assert_eq!(
            strategy.cont(&below_nonterminal_threshold, max_steps),
            false
        );
        assert_eq!(
            strategy.cont(&at_nonterminal_threshold, below_max_steps),
            false
        );
    }

    #[test]
    fn test_growth_strategy_cont() {
        let nonterminals_threshold = 10;
        let max_steps = 10;
        let below_max_steps = max_steps - 1;
        let below_nonterminal_threshold = sample_derivation_tree(nonterminals_threshold - 1);
        let at_nonterminal_threshold = sample_derivation_tree(nonterminals_threshold);
        let strategy: &dyn Strategy<()> = &GrowthStrategy::new(nonterminals_threshold, max_steps);

        assert_eq!(
            strategy.cont(&below_nonterminal_threshold, below_max_steps),
            true
        );
        assert_eq!(
            strategy.cont(&below_nonterminal_threshold, max_steps),
            false
        );
        assert_eq!(
            strategy.cont(&at_nonterminal_threshold, below_max_steps),
            false
        );
    }

    #[test]
    fn test_close_strategy_cont() {
        let ten_nonterminal_nodes = sample_derivation_tree(10);
        let strategy: &dyn Strategy<()> = &CloseStrategy::new();

        assert_eq!(strategy.cont(&ten_nonterminal_nodes, 9), true);
    }
}

#[cfg(test)]
mod strategy_choose_tests {
    use super::*;
    use crate::derivation_tree::Node;
    use std::collections::HashMap;

    fn sample_grammar() -> Grammar<()> {
        let expansions: HashMap<_, _> = [
            ("<int>", vec!["<int><digit>", "<digit>"]),
            (
                "<digit>",
                vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
            ),
        ]
        .iter()
        .cloned()
        .collect();

        Grammar::from(expansions)
    }

    #[test]
    fn test_random_strategy_choose() {
        let nonterminals_threshold = 10;
        let max_steps = 10;
        let strategy: &dyn Strategy<()> = &GrowthStrategy::new(nonterminals_threshold, max_steps);
        let grammar = sample_grammar();
        let derivation_tree = Node::N(String::from("<int>"));
        let expected_expansions = vec![
            Some(String::from("<int><digit>")),
            Some(String::from("<digit>")),
        ];
        assert_eq!(
            expected_expansions.contains(&strategy.choose(&grammar, &derivation_tree)),
            true
        );
    }

    #[test]
    fn test_growth_strategy_choose() {
        let nonterminals_threshold = 10;
        let max_steps = 10;
        let strategy: &dyn Strategy<()> = &GrowthStrategy::new(nonterminals_threshold, max_steps);
        let grammar = sample_grammar();
        let derivation_tree = Node::N(String::from("<int>"));
        assert_eq!(
            strategy.choose(&grammar, &derivation_tree),
            Some(String::from("<int><digit>"))
        );
    }

    #[test]
    fn test_close_strategy_choose() {
        let strategy: &dyn Strategy<()> = &CloseStrategy::new();
        let grammar = sample_grammar();
        let derivation_tree = Node::N(String::from("<int>"));
        assert_eq!(
            strategy.choose(&grammar, &derivation_tree),
            Some(String::from("<digit>"))
        );
    }
}
