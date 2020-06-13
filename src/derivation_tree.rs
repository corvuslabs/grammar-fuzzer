//! # Derivation Tree

use super::parser::{self, Token};
use std::cell::RefCell;
use std::ops::Deref;

#[derive(Debug)]
pub enum Node {
    T(String),
    N(String),
    EN(String, Children),
}

#[derive(Debug)]
pub struct Children {
    pub roots: Vec<RefCell<Node>>,
}

impl std::fmt::Display for Node {
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

impl Deref for Children {
    type Target = Vec<RefCell<Node>>;

    fn deref(&self) -> &Self::Target {
        &self.roots
    }
}

impl Children {
    pub fn epsilon() -> Self {
        Children {
            roots: vec![RefCell::new(Node::new_terminal(""))],
        }
    }
}

impl From<&str> for Children {
    fn from(expansion: &str) -> Self {
        let tokens = parser::tokens(expansion);
        if tokens.is_empty() {
            return Children::epsilon();
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

impl Node {
    pub fn new_nonterminal(sym: &str) -> Self {
        Node::N(sym.to_owned())
    }

    pub fn new_terminal(sym: &str) -> Self {
        Node::T(sym.to_owned())
    }

    pub fn new_expanded(sym: &str, children: Children) -> Self {
        Node::EN(sym.to_owned(), children)
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
