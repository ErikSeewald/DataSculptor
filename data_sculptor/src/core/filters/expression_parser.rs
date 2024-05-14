use crate::core::filters::command_parser;
use crate::core::filters::filter::FilterType;
use crate::core::filters::filter::FilterExpression;

#[derive(Debug, Clone)]
enum Token {
    OpenParen,
    CloseParen,
    Operator(String),
    FilterExpression(String),
}

pub fn parse(filter_type: &FilterType, input: &str) -> Option<FilterExpression>
{
    // First, attempt to parse normally
    let mut result = parse_tokenized(filter_type, input);

    // If that fails, wrap in {} and try again -> shortcut so that you do not need to wrap single
    // filter command expressions in {} manually.
    if result.is_none() && !input.contains("{") && !input.contains("}")
    {
        let mut wrapped_input = String::from("{");
        wrapped_input.push_str(input);
        wrapped_input.push_str("}");

        result = parse_tokenized(filter_type, wrapped_input.as_str())
    }

    result
}

fn parse_tokenized(filter_type: &FilterType, input: &str) -> Option<FilterExpression>
{
    let tokens = tokenize(input);
    let postfix = infix_to_postfix(tokens);
    build_logical_filter(postfix, filter_type)
}

fn tokenize(input: &str) -> Vec<Token>
{
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek()
    {
        match ch
        {
            '{' =>
                {
                    chars.next();
                    let mut content = String::new();
                    while let Some(&c) = chars.peek()
                    {
                        if c == '}'
                        {
                            chars.next();
                            break;
                        } else
                        {
                            content.push(c);
                            chars.next();
                        }
                    }
                    tokens.push(Token::FilterExpression(content));
                },
            '(' =>
                {
                    chars.next();
                    tokens.push(Token::OpenParen);
                },
            ')' =>
                {
                    chars.next();
                    tokens.push(Token::CloseParen);
                },
            ' ' | '\t' | '\n' | '\r' =>
                {
                    chars.next(); // Ignore whitespace
                },
            _ =>
                {
                    if ch.is_alphanumeric() || ch == '!'
                    {
                        let mut name = String::new();
                        while let Some(&c) = chars.peek()
                        {
                            if c.is_alphanumeric() || c == '-'
                            {
                                name.push(c);
                                chars.next();
                            } else
                            {
                                break;
                            }
                        }
                        tokens.push(Token::Operator(name));
                    } else
                    {
                        chars.next(); // Ignore unknown characters
                    }
                }
        }
    }

    tokens
}

fn precedence(op: &str) -> i32
{
    match op
    {
        "not" => 3,
        "and" | "nand" => 2,
        "or" | "nor" | "xor" | "xnor" => 1,
        _ => 0,
    }
}

fn infix_to_postfix(tokens: Vec<Token>) -> Vec<Token>
{
    let mut output: Vec<Token> = Vec::new();
    let mut ops_stack: Vec<Token> = Vec::new();

    for token in tokens
    {
        match token
        {
            Token::FilterExpression(_) => output.push(token),
            Token::Operator(op) =>
                {
                    while let Some(Token::Operator(top_op)) = ops_stack.last()
                    {
                        if precedence(&op) <= precedence(&top_op)
                        {
                            output.push(ops_stack.pop().unwrap());
                        } else
                        {
                            break;
                        }
                    }
                    ops_stack.push(Token::Operator(op));
                },
            Token::OpenParen => ops_stack.push(Token::OpenParen),
            Token::CloseParen =>
                {
                    while let Some(t) = ops_stack.pop() {
                        if let Token::OpenParen = t {
                            break;
                        } else {
                            output.push(t);
                        }
                    }
                },
        }
    }

    while let Some(t) = ops_stack.pop()
    {
        output.push(t);
    }

    output
}

fn build_logical_filter(tokens: Vec<Token>, filter_type: &FilterType) -> Option<FilterExpression>
{
    let mut stack: Vec<FilterExpression> = Vec::new();

    for token in tokens
    {
        match token
        {
            Token::FilterExpression(expr) =>
                {
                    let filter_command = command_parser::parse(filter_type, expr)?;
                    stack.push(FilterExpression::SingleCommand(filter_command));
                },
            Token::Operator(op) =>
                {
                    match op.as_str()
                    {
                        "not" => {
                            let a = stack.pop()?;
                            stack.push(FilterExpression::Not(Box::new(a)));
                        },
                        _ => {
                            let b = stack.pop()?;
                            let a = stack.pop()?;
                            stack.push(match op.as_str() {
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
            _ => return None,
        }
    }

    stack.pop()
}