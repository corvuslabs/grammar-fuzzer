//! # Grammar Fuzzer

use super::derivation_tree::{Children, Node};
use super::grammar::Grammar;
use super::shared::random_element;
use super::strategy::Strategy;

pub struct GrammarFuzzer<'a, T> {
    grammar: Grammar<'a, T>,
    steps: &'a Vec<&'a dyn Strategy<T>>,
}

impl<'a, T> GrammarFuzzer<'a, T> {
    pub fn new(grammar: Grammar<'a, T>, steps: &'a Vec<&'a dyn Strategy<T>>) -> Self {
        GrammarFuzzer { grammar, steps }
    }

    fn expand_nonterminal(&self, node: &Node<'a>, strategy: &dyn Strategy<T>) -> Children<'a> {
        match node {
            Node::N(_) => {
                let chosen_expantion = strategy.choose(&self.grammar, &node).unwrap();
                let children = Children::from(chosen_expantion);
                children
            }
            _ => panic!(),
        }
    }

    fn expand_tree_once(&self, node: &mut Node<'a>, strategy: &dyn Strategy<T>) {
        match node {
            Node::T(_) => (),
            Node::N(sym) => {
                let children = self.expand_nonterminal(&Node::N(sym), strategy);
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

    fn expand_tree_with_strategy(&self, root: &mut Node<'a>, strategy: &dyn Strategy<T>) {
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

    pub fn expand_tree(&self, root: &mut Node<'a>) {
        for strategy in self.steps {
            self.expand_tree_with_strategy(root, *strategy);
        }
    }
}
