use super::grammar::{self, Grammar};
use super::parser;
use std::collections::{HashMap, HashSet};

pub type Symbol = String;

pub struct Expansion<T: Copy> {
    pub symbol: Symbol,
    pub opts: Option<T>,
}

pub type Expansions<T> = HashMap<Symbol, Vec<Expansion<T>>>;

impl<'a, T: Copy> From<&'a Expansions<T>> for Grammar<'a, T> {
    fn from(input: &'a Expansions<T>) -> Self {
        let mut grammar = grammar::Expansions::new();
        for (token, expansions) in input.iter() {
            let token = &token[..];
            let expansion = expansions
                .iter()
                .map(|expansion| grammar::Expansion {
                    symbol: &expansion.symbol[..],
                    opts: expansion.opts,
                })
                .collect();
            grammar.insert(token, expansion);
        }
        Grammar::new(grammar)
    }
}

impl<'a, T: Copy> From<&Grammar<'a, T>> for Expansions<T> {
    fn from(input: &Grammar<'a, T>) -> Self {
        let mut new_expansions = Expansions::new();
        for (token, expansions) in input.iter() {
            let token = token.to_string();
            let expansions = expansions
                .iter()
                .map(|expansion| Expansion {
                    symbol: expansion.symbol.to_owned(),
                    opts: expansion.opts,
                })
                .collect();
            new_expansions.insert(token, expansions);
        }

        new_expansions
    }
}

pub fn ebnf_to_bnf<T: Copy>(ebnf_grammar: &Expansions<T>) -> Expansions<T> {
    let grammar = convert_grammar(ebnf_grammar, convert_ebnf_parentheses);
    let grammar = convert_grammar(&grammar, convert_ebnf_operators);
    grammar
}

fn convert_grammar<T: Copy, F>(ebnf_grammar: &Expansions<T>, apply: F) -> Expansions<T>
where
    F: Fn(&Expansion<T>, &mut NewSymbols) -> (Expansion<T>, Expansions<T>),
{
    let mut new_grammar = Expansions::new();
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

fn convert_ebnf_parentheses<T: Copy>(
    expansion: &Expansion<T>,
    symbols: &mut NewSymbols,
) -> (Expansion<T>, Expansions<T>) {
    let mut converted_expansion = expansion.symbol.to_owned();
    let mut new_expansions = Expansions::new();
    loop {
        if let Some(expresion) = parser::next_parenthesized_expression(&converted_expansion.clone())
        {
            let new_symbol = symbols.new(None);

            converted_expansion = converted_expansion.replacen(
                expresion.token,
                &format!("{}{}", new_symbol, expresion.op),
                1,
            );
            new_expansions.insert(
                new_symbol,
                vec![Expansion {
                    symbol: expresion.content.to_owned(),
                    opts: None,
                }],
            );
        } else {
            break;
        }
    }

    (
        Expansion {
            symbol: converted_expansion,
            opts: expansion.opts,
        },
        new_expansions,
    )
}

fn convert_ebnf_operators<T: Copy>(
    expansion: &Expansion<T>,
    symbols: &mut NewSymbols,
) -> (Expansion<T>, Expansions<T>) {
    let mut converted_expansion = expansion.symbol.to_owned();
    let mut new_expansions = Expansions::new();
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

    (
        Expansion {
            symbol: converted_expansion,
            opts: expansion.opts,
        },
        new_expansions,
    )
}

fn operator_expansions<T: Copy>(
    extension: &parser::ExtendedNonterminal,
    new_symbol: &str,
) -> Vec<Expansion<T>> {
    let original_symbol = extension.symbol.to_owned();
    let expansions = match extension.op {
        "?" => vec![format!(""), original_symbol],
        "*" => vec![format!(""), format!("{}{}", original_symbol, new_symbol)],
        "+" => vec![
            format!("{}", original_symbol),
            format!("{}{}", original_symbol, new_symbol),
        ],
        _ => panic!(),
    };
    expansions
        .iter()
        .map(|e| Expansion {
            symbol: e.to_owned(),
            opts: None,
        })
        .collect()
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

impl<T: Copy> From<&Expansions<T>> for NewSymbols {
    fn from(input: &Expansions<T>) -> Self {
        let nonterminal_tokens: HashSet<String> = input.keys().map(|k| k.clone()).collect();
        NewSymbols {
            existing_nonterminals: nonterminal_tokens,
        }
    }
}
