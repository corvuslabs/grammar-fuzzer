use super::grammar::{Expansions, Grammar};
use super::parser;
use std::collections::{HashMap, HashSet};

pub type OExpansions = HashMap<String, Vec<String>>;

impl<'a> From<&'a OExpansions> for Grammar<'a> {
    fn from(input: &'a OExpansions) -> Self {
        let mut grammar = Expansions::new();
        for (token, expansions) in input.iter() {
            let token = &token[..];
            let expansion = expansions.iter().map(|expansion| &expansion[..]).collect();
            grammar.insert(token, expansion);
        }
        Grammar::new(grammar)
    }
}

impl<'a> From<&Grammar<'a>> for OExpansions {
    fn from(input: &Grammar<'a>) -> Self {
        let mut new_expansions = OExpansions::new();
        for (token, expansions) in input.iter() {
            let token = token.to_string();
            let expansions = expansions
                .iter()
                .map(|expansion| expansion.to_string())
                .collect();
            new_expansions.insert(token, expansions);
        }

        new_expansions
    }
}

pub fn ebnf_to_bnf(ebnf_grammar: &OExpansions) -> OExpansions {
    let grammar = convert_grammar(ebnf_grammar, convert_ebnf_parentheses);
    let grammar = convert_grammar(&grammar, convert_ebnf_operators);
    grammar
}

fn convert_grammar<F>(ebnf_grammar: &OExpansions, apply: F) -> OExpansions
where
    F: Fn(&str, &mut NewSymbols) -> (String, OExpansions),
{
    let mut new_grammar = OExpansions::new();
    let mut symbols = NewSymbols::from(ebnf_grammar);
    for (token, expansions) in ebnf_grammar.iter() {
        for expansion in expansions {
            let (converted_expansion, new_expansions) = apply(expansion, &mut symbols);
            new_grammar
                .entry(token.to_owned())
                .or_insert(Vec::new())
                .push(converted_expansion);
            new_grammar.extend(new_expansions);
        }
    }

    new_grammar
}

fn convert_ebnf_parentheses(expansion: &str, symbols: &mut NewSymbols) -> (String, OExpansions) {
    let mut converted_expansion = expansion.to_owned();
    let mut new_expansions = OExpansions::new();
    loop {
        if let Some(expresion) = parser::next_parenthesized_expression(&converted_expansion.clone())
        {
            let new_symbol = symbols.new(None);

            converted_expansion = converted_expansion.replacen(
                expresion.token,
                &format!("{}{}", new_symbol, expresion.op),
                1,
            );
            new_expansions.insert(new_symbol, vec![expresion.content.to_owned()]);
        } else {
            break;
        }
    }

    (converted_expansion, new_expansions)
}

fn convert_ebnf_operators(expansion: &str, symbols: &mut NewSymbols) -> (String, OExpansions) {
    let mut converted_expansion = expansion.to_owned();
    let mut new_expansions = OExpansions::new();
    loop {
        if let Some(extension) = parser::next_extended_nonterminal(&converted_expansion.clone()) {
            let new_symbol = symbols.new(None);

            converted_expansion = converted_expansion.replacen(extension.token, &new_symbol, 1);
            new_expansions.insert(
                new_symbol.clone(),
                operator_expansions(&extension, &new_symbol),
            );
        } else {
            break;
        }
    }

    (converted_expansion, new_expansions)
}

fn operator_expansions(extension: &parser::ExtendedNonterminal, new_symbol: &str) -> Vec<String> {
    let original_symbol = extension.symbol.to_owned();
    match extension.op {
        "?" => vec![format!(""), original_symbol],
        "*" => vec![format!(""), format!("{}{}", original_symbol, new_symbol)],
        "+" => vec![
            format!("{}", original_symbol),
            format!("{}{}", original_symbol, new_symbol),
        ],
        _ => panic!(),
    }
}

// -------------------------------- NewSymbols --------------------------------

struct NewSymbols {
    existing_nonterminals: HashSet<String>,
}

impl NewSymbols {
    fn new(&mut self, nonterminal_symbol: Option<&str>) -> String {
        let symbol = nonterminal_symbol.unwrap_or("<symbol>");
        if self.existing_nonterminals.contains(symbol) {
            let mut count = 0;
            loop {
                count += 1;
                let tentative_symbol_name = format!("<{}-{}>", &symbol[1..symbol.len() - 1], count);
                if !self
                    .existing_nonterminals
                    .contains(&tentative_symbol_name[..])
                {
                    self.existing_nonterminals
                        .insert(tentative_symbol_name.to_owned());
                    return tentative_symbol_name;
                }
            }
        }

        self.existing_nonterminals.insert(symbol.to_owned());
        symbol.to_owned()
    }
}

impl From<&OExpansions> for NewSymbols {
    fn from(input: &OExpansions) -> Self {
        let nonterminal_tokens: HashSet<String> = input.keys().map(|k| k.clone()).collect();
        NewSymbols {
            existing_nonterminals: nonterminal_tokens,
        }
    }
}
