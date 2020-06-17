use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take, take_while1},
    multi::{many0, many_till},
    IResult,
};

// -------------------------------- Tokens ------------------------------------

#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {
    Terminal(&'a str),
    Nonterminal(&'a str),
}

fn terminal_token(input: &str) -> IResult<&str, Token> {
    let (input, token) = take_while1(|c| !"<>".contains(c))(input)?;
    Ok((input, Token::Terminal(token)))
}

fn nonterminal_token(input: &str) -> IResult<&str, Token> {
    let (_input, _) = tag("<")(input)?;
    let (_input, sym) = take_while1(|c| !"<> ".contains(c))(_input)?;
    let (_input, _) = tag(">")(_input)?;
    let len = '<'.len_utf8() + sym.len() + '>'.len_utf8();
    let nonterminal_token = Token::Nonterminal(&input[..len]);
    Ok((_input, nonterminal_token))
}

fn token(input: &str) -> IResult<&str, Token> {
    alt((nonterminal_token, terminal_token))(input)
}

/// Returns a sequence of terminal an nonterminal tokens
pub fn tokens(input: &str) -> Vec<Token> {
    // it should consume the whole input
    let (input, tokens) = many0(token)(input).unwrap();
    assert_eq!(input.is_empty(), true);
    tokens
}

// ----------------------------- Expressions ----------------------------------

#[derive(Debug, PartialEq, Eq)]
pub struct ParenthesizedExpression<'a> {
    pub token: &'a str,
    pub op: &'a str,
    pub content: &'a str,
}

fn parenthesized_expression(input: &str) -> IResult<&str, ParenthesizedExpression> {
    let (_input, _) = tag("(")(input)?;
    let (_input, content) = take_while1(|c| !"()".contains(c))(_input)?;
    let (_input, _) = tag(")")(_input)?;
    let (_input, op) = is_a("+*?")(_input)?;
    let len = '('.len_utf8() + content.len() + ')'.len_utf8() + op.len();
    let parenthesized_expression = ParenthesizedExpression {
        token: &input[..len],
        op,
        content,
    };

    Ok((_input, parenthesized_expression))
}

/// Returns the next paranthesized expression in the input string
pub fn next_parenthesized_expression(input: &str) -> Option<ParenthesizedExpression> {
    match many_till(take(1usize), parenthesized_expression)(input) {
        Ok((_, (_, pe))) => Some(pe),
        Err(_) => None,
    }
}

// -------------------------------- Extended ----------------------------------

#[derive(Debug, PartialEq, Eq)]
pub struct ExtendedNonterminal<'a> {
    pub token: &'a str,
    pub op: &'a str,
    pub symbol: &'a str,
}

fn extended_nonterminal(input: &str) -> IResult<&str, ExtendedNonterminal> {
    let (_input, _) = tag("<")(input)?;
    let (_input, symbol) = take_while1(|c| !"<> ".contains(c))(_input)?;
    let (_input, _) = tag(">")(_input)?;
    let (_input, op) = is_a("+*?")(_input)?;
    let len = '<'.len_utf8() + symbol.len() + '>'.len_utf8() + op.len();
    let extended_nonterminal = ExtendedNonterminal {
        token: &input[..len],
        op,
        symbol: &input[..len - op.len()],
    };

    Ok((_input, extended_nonterminal))
}

/// Returns the next extended nonterminal
pub fn next_extended_nonterminal(input: &str) -> Option<ExtendedNonterminal> {
    match many_till(take(1usize), extended_nonterminal)(input) {
        Ok((_, (_, en))) => Some(en),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens() {
        let result = tokens("<string>: <json>");
        let expected = vec![
            Token::Nonterminal("<string>"),
            Token::Terminal(": "),
            Token::Nonterminal("<json>"),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_next_parenthesized_expression() {
        let result = next_parenthesized_expression("[(<value>, )*<value>]");
        let expected = Some(ParenthesizedExpression {
            token: "(<value>, )*",
            op: "*",
            content: "<value>, ",
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_next_extended_nonterminal() {
        let result = next_extended_nonterminal("[<value>*]");
        let expected = Some(ExtendedNonterminal {
            token: "<value>*",
            op: "*",
            symbol: "<value>",
        });

        assert_eq!(result, expected);
    }
}
