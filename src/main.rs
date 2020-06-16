use ebnffuzzer::extensions::ebnf_to_bnf;
use ebnffuzzer::Grammar;
use ebnffuzzer::GrammarFuzzer;
use ebnffuzzer::Node;
use ebnffuzzer::{CloseStrategy, GrowthStrategy, RandomStrategy, Strategy};
use std::collections::HashMap;

use std::cmp::Ordering;
use std::time::Instant;

fn json_grammar() -> Grammar<()> {
    let expansios: HashMap<_, _> = [
        ("<start>", vec!["<assoc>"]),
        (
            "<value>",
            vec!["<assoc>", "<list>", "<bool>", "<string>", "<int>"],
        ),
        ("<assoc>", vec!["{(<string>: <value>, )*<string>: <value>}"]),
        ("<list>", vec!["[(<value>, )*<value>]"]),
        ("<bool>", vec!["true", "false"]),
        ("<string>", vec!["\"<char>+\""]),
        ("<char>", vec!["a", "b", "c", "d"]),
        ("<int>", vec!["<digit>+"]),
        (
            "<digit>",
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
        ),
    ]
    .iter()
    .cloned()
    .collect();

    Grammar::from(expansios)
}

fn main() {
    // Fuzzer
    let expansion = GrowthStrategy::new(0, 1000);
    let random = RandomStrategy::new(40, 8000);
    let close = CloseStrategy::new();
    let strategies: Vec<&dyn Strategy<()>> = vec![&expansion, &random, &close];

    let ebnf_json_grammar = json_grammar();
    let json_grammar = ebnf_to_bnf(&ebnf_json_grammar);
    assert_eq!(json_grammar.is_valid_grammar(None), true);

    let fuzzer = GrammarFuzzer::new(json_grammar, &strategies);

    // Sample
    let mut stat = Vec::new();
    for _ in 0..100 {
        let mut node = Node::N("<start>".to_owned());
        let now = Instant::now();
        fuzzer.expand_tree(&mut node);
        stat.push((format!("{}", node).len() as u32, now.elapsed().as_millis()));
        println!("{}\n", node);
    }
    stat.sort_by(|a, b| {
        if a < b {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    println!("{:?}", stat);
}
