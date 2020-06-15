//! # Derivation Tree

use super::parser::{self, Token};
use std::cell::RefCell;
use std::ops::Deref;

#[derive(Debug)]
pub enum Node<'a> {
    T(&'a str),
    N(&'a str),
    EN(&'a str, Children<'a>),
}

#[derive(Debug)]
pub struct Children<'a> {
    pub roots: Vec<RefCell<Node<'a>>>,
}

impl<'a> std::fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Node::T(sym) => write!(f, "{}", sym),
            Node::N(sym) => write!(f, "{}", sym),
            Node::EN(_, Children { roots }) => {
                for r in roots {
                    write!(f, "{}", r.borrow())?;
                }
                Ok(())
            }
        }
    }
}

impl<'a> Deref for Children<'a> {
    type Target = Vec<RefCell<Node<'a>>>;

    fn deref(&self) -> &Self::Target {
        &self.roots
    }
}

impl<'a> Children<'a> {
    pub fn new_terminal(symbol: &'a str) -> Self {
        Children {
            roots: vec![RefCell::new(Node::new_terminal(symbol))],
        }
    }
}

impl<'a> From<&'a str> for Children<'a> {
    fn from(expansion: &'a str) -> Self {
        let tokens = parser::tokens(expansion);
        if tokens.is_empty() {
            return Children::new_terminal("");
        }

        let roots = tokens
            .iter()
            .map(|token| match token {
                Token::Nonterminal(t) => Node::new_nonterminal(t),
                Token::Terminal(t) => Node::new_terminal(t),
            })
            .map(|n| RefCell::new(n))
            .collect();

        Children { roots }
    }
}

impl<'a> Node<'a> {
    pub fn new_nonterminal(sym: &'a str) -> Self {
        Node::N(sym)
    }

    pub fn new_terminal(sym: &'a str) -> Self {
        Node::T(sym)
    }

    pub fn new_expanded(sym: &'a str, children: Children<'a>) -> Self {
        Node::EN(sym, children)
    }

    pub fn any_possible_expansions(&self) -> bool {
        match self {
            Node::T(_) => false,
            Node::N(_) => true,
            Node::EN(_, chl) => chl
                .iter()
                .any(|child| child.borrow().any_possible_expansions()),
        }
    }

    pub fn num_possible_expansions(&self) -> usize {
        match self {
            Node::T(_) => 0,
            Node::N(_) => 1,
            Node::EN(_, chl) => chl
                .iter()
                .map(|child| child.borrow().num_possible_expansions())
                .sum(),
        }
    }
}
