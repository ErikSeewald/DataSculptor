//! Module for parsing [`String`]s into [`FilterExpression`]s.

use std::iter::Peekable;
use std::str::Chars;
use crate::core::filters::command_parser;
use crate::core::filters::filter::{FilterType};
use crate::core::filters::filter_expression::FilterExpression;

/// Enum representing a single token within an expression [`String`]
#[derive(Debug, Clone)]
enum Token
{
    Operator(String),
    FilterCommand(String),
    LeftParenthesis,
    RightParenthesis,
    Invalid
}

/// Tries to parse the given input string to a [`FilterExpression`]
///
/// # Returns
/// - `Some`([`FilterExpression`]) if the command was successfully parsed
/// - `None` if parsing was unsuccessful
pub fn parse(filter_type: &FilterType, input: &str) -> Option<FilterExpression>
{
    // First, attempt to parse normally
    let mut result = parse_tokenized(filter_type, input);

    // If that fails, wrap in {} and try again -> shortcut so that you do not need to wrap single
    // filter command expressions in {} manually.
    if result.is_none() && !input.contains("{") && !input.contains("}")
    {
        let mut wrapped_input = String::from("{") + input;
        wrapped_input += "}";

        result = parse_tokenized(filter_type, wrapped_input.as_str())
    }

    result
}

/// Attempts to parse the given input to a [`FilterExpression`] using a tokenized postfix
/// approach.
fn parse_tokenized(filter_type: &FilterType, input: &str) -> Option<FilterExpression>
{
    let tokens = tokenize(input);
    let postfix = infix_to_postfix(tokens);
    expression_from_tokenized_postfix(postfix, filter_type)
}

fn tokenize(input: &str) -> Vec<Token>
{
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut parenthesis_count = 0;

    while let Some(&ch) = chars.peek()
    {
        match ch
        {
            '{' =>{tokenize_command(&mut chars, &mut tokens);},

            '(' =>
            {
                chars.next();
                tokens.push(Token::LeftParenthesis);
                parenthesis_count += 1;

                // Disallow empty ()
                if let Some(&')') = chars.peek()
                {
                   tokens.push(Token::Invalid)
                }
            },

            ')' =>
            {
                chars.next();
                if parenthesis_count == 0
                {
                    tokens.push(Token::Invalid);
                }

                else
                {
                    tokens.push(Token::RightParenthesis);
                    parenthesis_count -= 1;
                }
            },

            ' ' | '\t' | '\n' | '\r' =>
                {
                    chars.next(); // Ignore whitespace
                },

            _ =>
            {
                tokenize_logical_operator(&mut chars, &mut tokens);
            }
        }
    }

    if parenthesis_count != 0
    {
        tokens.push(Token::Invalid);
    }

    tokens
}

fn tokenize_command(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>)
{
    chars.next();
    let mut content = String::new();
    while let Some(&ch) = chars.peek()
    {
        if ch == '}'
        {
            chars.next();
            break;
        }

        else
        {
            content.push(ch);
            chars.next();
        }
    }
    tokens.push(Token::FilterCommand(content));
}

fn tokenize_logical_operator(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>)
{
    if let Some(&ch) = chars.peek()
    {
        if ch.is_alphanumeric()
        {
            let mut name = String::new();
            while let Some(&c) = chars.peek()
            {
                if c.is_alphanumeric()
                {
                    name.push(c);
                    chars.next();
                }
                else {break;}
            }

            tokens.push(Token::Operator(name));
        }

        else
        {
            tokens.push(Token::Invalid);
            chars.next();
        }
    }
}

/// Precedence declaration for all supported logical operators
/// (Greater precedence -> greater number)
fn logical_precedence(operator: &str) -> i32
{
    match operator
    {
        "not" => 3,
        "and" | "nand" => 2,
        "or" | "nor" | "xor" | "xnor" => 1,
        _ => 0,
    }
}

/// Converts a vector of tokens from infix notation to postfix notation using the
/// shunting yard algorithm.
fn infix_to_postfix(tokens: Vec<Token>) -> Vec<Token>
{
    let mut output: Vec<Token> = Vec::new();
    let mut ops_stack: Vec<Token> = Vec::new();

    for token in tokens
    {
        match token
        {
            Token::FilterCommand(_) => output.push(token),

            Token::LeftParenthesis => ops_stack.push(token),

            Token::RightParenthesis =>
            {
                while let Some(top) = ops_stack.pop()
                {
                    if let Token::LeftParenthesis = top
                    {
                        break;
                    }
                    else {output.push(top);}
                }
            },

            Token::Operator(op) =>
            {
                while let Some(Token::Operator(top_op)) = ops_stack.last()
                {
                    if logical_precedence(&op) <= logical_precedence(&top_op)
                    {
                        output.push(ops_stack.pop().unwrap());
                    }
                    else {break;}
                }
                ops_stack.push(Token::Operator(op));
            },

            Token::Invalid => output.push(token)
        }
    }

    while let Some(t) = ops_stack.pop()
    {
        output.push(t);
    }

    output
}

fn expression_from_tokenized_postfix(tokens: Vec<Token>, filter_type: &FilterType)
    -> Option<FilterExpression>
{
    let mut stack: Vec<FilterExpression> = Vec::new();

    for token in tokens
    {
        match token
        {
            Token::FilterCommand(expr) =>
            {
                let filter_command = command_parser::parse(filter_type, expr)?;
                stack.push(FilterExpression::SingleCommand(filter_command));
            },

            Token::Operator(operator) =>
            {
                match operator.as_str()
                {
                    "not" =>
                    {
                        let a = stack.pop()?;
                        stack.push(FilterExpression::Not(Box::new(a)));
                    },

                    _ =>
                    {
                        let b = stack.pop()?;
                        let a = stack.pop()?;
                        stack.push(match operator.as_str()
                        {
                            "and" => FilterExpression::And(Box::new(a), Box::new(b)),
                            "or" => FilterExpression::Or(Box::new(a), Box::new(b)),
                            "xor" => FilterExpression::Xor(Box::new(a), Box::new(b)),
                            "nor" => FilterExpression::Nor(Box::new(a), Box::new(b)),
                            "nand" => FilterExpression::Nand(Box::new(a), Box::new(b)),
                            "xnor" => FilterExpression::Xnor(Box::new(a), Box::new(b)),
                            _ => return None,
                        });
                    }
                }
            },

            Token::Invalid => return None,

            // `LeftParenthesis` and `RightParenthesis` only serve to manage the operator
            // precedence in the `infix_to_postfix` function => They should not appear by the time
            // this function is called
            _ => {},
        }
    }

    stack.pop()
}