//! Grammar extensions, specifically EBNF
//!
//! # Example
//!
//! ```
//! use grammar_fuzzer::{Grammar, ebnf_to_bnf};
//! use std::collections::HashMap;
//! 
//! let ebnf_grammar: HashMap<&str, Vec<&str>> = [
//!     ("<list>", vec!["[(<string>, )*<string>]"]),
//!     ("<assoc>", vec!["{(<string>: <string>, )+}"]),
//!     ("<string>", vec!["<string>?<char>"]),
//!     ("<char>", vec!["a", "b", "c", "d"]),
//! ]
//! .iter()
//! .cloned()
//! .collect();
//!
//! let ebnf_grammar = Grammar::from(&ebnf_grammar);
//!
//! let bnf_grammar = ebnf_to_bnf(&ebnf_grammar);
//! ```

use super::grammar::{Alternatives, Expansion, Expansions, Grammar};
use super::parser;
use std::collections::HashSet;

/// Converts a grammar in EBNF to BNF, the only supported EBNF operators are: `*+?`
pub fn ebnf_to_bnf<T: Copy>(grammar: &Grammar<T>) -> Grammar<T> {
    let grammar = convert_grammar(grammar, convert_ebnf_parentheses);
    let grammar = convert_grammar(&grammar, convert_ebnf_operators);
    grammar
}

/// Invokes `apply` function with all expansions in a grammar and returns a new grammar
fn convert_grammar<T: Copy, F>(grammar: &Grammar<T>, apply: F) -> Grammar<T>
where
    F: Fn(&Expansion<T>, &mut Symbols) -> (Expansion<T>, Expansions<T>),
{
    let mut expansions_for_new_grammar = Expansions::new();
    let mut new_symbol = Symbols::from(grammar);
    // Sort the keys to ensure convert_grammar is deterministic
    let mut tokens: Vec<String> = grammar.keys().cloned().collect();
    tokens.sort();

    for token in tokens.iter() {
        for expansion in &grammar[token] {
            let (converted_expansion, new_expansions) = apply(expansion, &mut new_symbol);
            expansions_for_new_grammar
                .entry(token.to_owned())
                .or_insert(Vec::new())
                .push(converted_expansion);
            expansions_for_new_grammar.extend(new_expansions);
        }
    }

    Grammar::new(expansions_for_new_grammar)
}

/// Converts parenthesized expressions, ex: `(<json>)+`
fn convert_ebnf_parentheses<T: Copy>(
    expansion: &Expansion<T>,
    symbols: &mut Symbols,
) -> (Expansion<T>, Expansions<T>) {
    let mut expansion_symbol = expansion.string.clone();
    let mut new_expansions = Expansions::new();
    loop {
        if let Some(expression) = parser::next_parenthesized_expression(&expansion_symbol.clone()) {
            let new_symbol = symbols.new(None);

            expansion_symbol = expansion_symbol.replacen(
                expression.token,
                &format!("{}{}", new_symbol, expression.op),
                1,
            );
            new_expansions.insert(new_symbol, vec![Expansion::new(expression.content, None)]);
        } else {
            break;
        }
    }

    (
        Expansion::new(&expansion_symbol, expansion.opts),
        new_expansions,
    )
}

/// Converts extended nonterminals, ex: `<json>+`
fn convert_ebnf_operators<T: Copy>(
    expansion: &Expansion<T>,
    symbols: &mut Symbols,
) -> (Expansion<T>, Expansions<T>) {
    let mut expansion_symbol = expansion.string.clone();
    let mut new_expansions = Expansions::new();
    loop {
        if let Some(extension) = parser::next_extended_nonterminal(&expansion_symbol.clone()) {
            let new_symbol = symbols.new(None);

            expansion_symbol = expansion_symbol.replacen(extension.token, &new_symbol, 1);
            new_expansions.insert(
                new_symbol.clone(),
                operator_expansions(&extension, &new_symbol),
            );
        } else {
            break;
        }
    }

    (
        Expansion::new(&expansion_symbol, expansion.opts),
        new_expansions,
    )
}

fn operator_expansions<T>(
    extension: &parser::ExtendedNonterminal,
    new_symbol: &str,
) -> Alternatives<T> {
    let original_symbol = String::from(extension.symbol);
    match extension.op {
        "?" => vec![format!(""), original_symbol],
        "*" => vec![format!(""), format!("{}{}", original_symbol, new_symbol)],
        "+" => vec![
            format!("{}", original_symbol),
            format!("{}{}", original_symbol, new_symbol),
        ],
        _ => panic!(),
    }
    .iter()
    .map(|e| Expansion::new(e, None))
    .collect()
}

// -------------------------------- NewSymbols --------------------------------

struct Symbols {
    existing_nonterminals: HashSet<String>,
}

impl Symbols {
    /// Returns a unique nonterminal symbol on every invokation
    fn new(&mut self, nonterminal_symbol: Option<&str>) -> String {
        let mut tentative_symbol = nonterminal_symbol.unwrap_or("<symbol>").to_owned();
        let symbol_name = &tentative_symbol.clone()[1..tentative_symbol.len() - 1];
        if self.existing_nonterminals.contains(&tentative_symbol) {
            let mut count = 0;
            loop {
                count += 1;
                tentative_symbol = format!("<{}-{}>", symbol_name, count);
                if !self.existing_nonterminals.contains(&tentative_symbol) {
                    break;
                }
            }
        }

        self.existing_nonterminals.insert(tentative_symbol.clone());
        tentative_symbol
    }
}

impl<T> From<&Grammar<T>> for Symbols {
    /// Uses defined nonterminal symbol in a grammar to create a Symbols struct
    fn from(input: &Grammar<T>) -> Self {
        let nonterminal_tokens: HashSet<String> = input.keys().cloned().collect();
        Symbols {
            existing_nonterminals: nonterminal_tokens,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_ebnf_to_bnf() {
        let ebnf_grammar: HashMap<&str, Vec<&str>> = [
            ("<list>", vec!["[(<string>, )*<string>]"]),
            ("<assoc>", vec!["{(<string>: <string>, )+}"]),
            ("<string>", vec!["<string>?<char>"]),
            ("<char>", vec!["a", "b", "c", "d"]),
        ]
        .iter()
        .cloned()
        .collect();

        let ebnf_grammar = Grammar::from(&ebnf_grammar);

        let expected_bnf_grammar: HashMap<&str, Vec<&str>> = [
            ("<list>", vec!["[<symbol-3><string>]"]),
            ("<assoc>", vec!["{<symbol-2>}"]),
            ("<string>", vec!["<symbol-4><char>"]),
            ("<char>", vec!["a", "b", "c", "d"]),
            ("<symbol>", vec!["<string>: <string>, "]),
            ("<symbol-1>", vec!["<string>, "]),
            ("<symbol-2>", vec!["<symbol>", "<symbol><symbol-2>"]),
            ("<symbol-3>", vec!["", "<symbol-1><symbol-3>"]),
            ("<symbol-4>", vec!["", "<string>"]),
        ]
        .iter()
        .cloned()
        .collect();

        let expected_bnf_grammar = Grammar::from(&expected_bnf_grammar);

        assert_eq!(ebnf_to_bnf(&ebnf_grammar), expected_bnf_grammar);
    }
}
