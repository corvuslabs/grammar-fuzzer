//! # Derivation Tree

use super::parser::{self, Token};
use std::cell::RefCell;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    /// T is a `Terminal Node`
    T(String),
    /// N is a `Nonterminal Node` that has not been expanded yet
    N(String),
    /// EN is an `Expanded Nonterminal Node`
    EN(String, Children),
}

#[derive(Debug, PartialEq, Eq)]
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
    /// splits an expansion-string into terminal and nonterminal symbols and lift them into Node::T and Node::N
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
        Node::N(String::from(sym))
    }

    pub fn new_terminal(sym: &str) -> Self {
        Node::T(String::from(sym))
    }

    pub fn new_expanded(sym: &str, children: Children) -> Self {
        Node::EN(String::from(sym), children)
    }

    /// any_possible_expansions returns true when there is a Node::N in a subtree
    pub fn any_possible_expansions(&self) -> bool {
        match self {
            Node::T(_) => false,
            Node::N(_) => true,
            Node::EN(_, chl) => chl
                .iter()
                .any(|child| child.borrow().any_possible_expansions()),
        }
    }

    /// any_possible_expansions returns the number of Node::N in a subtree
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

#[cfg(test)]
mod tests {
    use super::Node::{EN, N, T};
    use super::*;

    fn unexpanded_digit_derivation_tree() -> Node {
        EN(
            String::from("<int>"),
            Children {
                roots: vec![RefCell::new(N(String::from("<digit>")))],
            },
        )
    }

    fn digit_derivation_tree(d: usize) -> Node {
        EN(
            String::from("<digit>"),
            Children {
                roots: vec![RefCell::new(T(format!("{}", d)))],
            },
        )
    }

    fn int_derivation_tree(d: usize) -> Node {
        if d > 1 {
            EN(
                String::from("<int>"),
                Children {
                    roots: vec![
                        RefCell::new(int_derivation_tree(d - 1)),
                        RefCell::new(digit_derivation_tree(d)),
                    ],
                },
            )
        } else {
            T(format!("{}", d))
        }
    }

    #[test]
    fn test_display_for_node() {
        let derivation_tree = int_derivation_tree(9);
        assert_eq!(format!("{}", derivation_tree), "123456789");
    }

    #[test]
    fn test_from_str_for_children() {
        let result = Children::from("<string>: <value>");
        let expected = Children {
            roots: vec![
                RefCell::new(N(String::from("<string>"))),
                RefCell::new(T(String::from(": "))),
                RefCell::new(N(String::from("<value>"))),
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_any_possible_expansions() {
        assert_eq!(int_derivation_tree(9).any_possible_expansions(), false);
        assert_eq!(
            unexpanded_digit_derivation_tree().any_possible_expansions(),
            true
        );
    }

    #[test]
    fn test_num_possible_expansions() {
        assert_eq!(int_derivation_tree(9).num_possible_expansions(), 0);
        assert_eq!(
            unexpanded_digit_derivation_tree().num_possible_expansions(),
            1
        );
    }
}
