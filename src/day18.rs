use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Token {
    Number(u64),
    Operator(Operator),
    LeftParenthesis,
    RightParenthesis,
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '0'..='9' => Self::Number(c.to_digit(10).unwrap().into()),
            '+' => Self::Operator(Operator::Add),
            '*' => Self::Operator(Operator::Multiply),
            '(' => Self::LeftParenthesis,
            ')' => Self::RightParenthesis,
            _ => panic!("bad token"),
        }
    }
}

fn lex(s: &str) -> Vec<Token> {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .map(Token::from)
        .collect()
}

fn shunting_yard(precedence: &HashMap<Operator, u8>, tokens: &[Token]) -> Vec<Token> {
    let mut operator_stack: Vec<Token> = vec![];
    let mut output_queue: VecDeque<Token> = VecDeque::new();
    for token in tokens {
        match token {
            Token::Number(_) => output_queue.push_back(*token),
            Token::Operator(new_op) => {
                let new_prec = precedence[new_op];
                while let Some(t) = operator_stack.pop() {
                    match t {
                        Token::Operator(op) if precedence[&op] >= new_prec => {
                            output_queue.push_back(t)
                        }
                        _ => {
                            operator_stack.push(t);
                            break;
                        }
                    }
                }
                operator_stack.push(*token);
            }
            Token::LeftParenthesis => operator_stack.push(*token),
            Token::RightParenthesis => {
                while let Some(op) = operator_stack.pop() {
                    if op != Token::LeftParenthesis {
                        output_queue.push_back(op)
                    } else {
                        break;
                    }
                }
            }
        }
    }
    while let Some(token) = operator_stack.pop() {
        output_queue.push_back(token);
    }
    output_queue.into_iter().collect()
}

fn eval_rpn(tokens: &[Token]) -> u64 {
    let mut stack: Vec<u64> = vec![];
    for token in tokens {
        match token {
            Token::Number(n) => stack.push(*n),
            Token::Operator(operator) => {
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                let result = match operator {
                    Operator::Add => op1 + op2,
                    Operator::Multiply => op1 * op2,
                };
                stack.push(result)
            }
            _ => panic!("something went wrong"),
        }
    }

    stack[0]
}

pub(crate) fn day18() {
    let input = std::fs::read_to_string("data/day18.txt").unwrap();
    let problems: Vec<Vec<Token>> = input.lines().map(lex).collect();

    let mut precedence: HashMap<Operator, u8> = HashMap::new();
    precedence.insert(Operator::Add, 0);
    precedence.insert(Operator::Multiply, 0);

    let total: u64 = problems
        .iter()
        .map(|problem| {
            let parsed = shunting_yard(&precedence, &problem);
            eval_rpn(&parsed)
        })
        .sum();
    println!("Part one answer is {}", total);

    precedence.insert(Operator::Add, 1);
    let total: u64 = problems
        .iter()
        .map(|problem| {
            let parsed = shunting_yard(&precedence, &problem);
            eval_rpn(&parsed)
        })
        .sum();
    println!("Part two answer is {}", total);
}
