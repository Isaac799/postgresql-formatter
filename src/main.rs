use std::env;
use std::fs;
mod data;

#[derive(Debug)]
struct Token {
    value: String,
    kind: TokenKind,
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    Identifier,
    Keyword,
    DataType,
    ArithmeticOperator,
    BooleanOperator,
    ComparisonOperator,
    LogicalOperator,
    End,
}

impl Token {
    fn new(value: String, kind: TokenKind) -> Self {
        Token {
            value: value,
            kind: kind,
        }
    }
}

#[derive(Debug)]
struct Lexer {
    input: String,
    state: usize,
    last: char,
    stack: Vec<char>,
    statements: Vec<Vec<Token>>,
    statements_counter: usize,
    commenting: bool,
    stringing: bool,
    row: usize,
    col: usize,
    pos: usize,
    // tokens: Vec<Token>,
    output: String,
}

impl Lexer {
    fn new(a: String) -> Self {
        println!("Creating new lexer...");
        Lexer {
            input: a,
            state: 0,
            last: ' ',
            stack: vec![],
            commenting: false,
            stringing: false,
            row: 0,
            col: 0,
            pos: 0,
            statements: vec![],
            statements_counter: 0,
            // tokens: vec![],
            output: "".to_string(),
        }
    }

    fn add_token(&mut self) {
        let mut value: String = self.stack.iter().collect();
        let value_upper: String = value.to_ascii_uppercase();
        let mut kind: TokenKind = TokenKind::Identifier;

        if data::KEYWORDS.contains(&value_upper.as_str()) {
            kind = TokenKind::Keyword;
            value = value_upper;
        } else if data::DATA_TYPES.contains(&value_upper.as_str()) {
            kind = TokenKind::DataType;
            value = value_upper;
        } else {
            match value.to_ascii_uppercase().as_str() {
                "+" | "-" | "*" | "/" | "%" => kind = TokenKind::ArithmeticOperator,
                "AND" | "OR" | "NOT" => kind = TokenKind::BooleanOperator,
                "<" | "<=" | "=" | ">=" | ">" => kind = TokenKind::ComparisonOperator,
                "THEN" | "ELSE" => kind = TokenKind::LogicalOperator,
                ";" => kind = TokenKind::End,
                _ => {}
            }
        }

        if kind == TokenKind::Identifier && !(value.starts_with('\'') && value.ends_with('\'')) {
            value = value.to_ascii_lowercase();
        }

        self.statements[self.statements_counter].push(Token::new(value, kind));
        self.stack.clear();
    }

    fn run(&mut self) {
        println!("Running...");
        self.state = 1;
        // self.input = self.input.to_ascii_uppercase();
        let a: String = self.input.clone();
        let all: std::str::Chars = a.chars();

        for c in all {
            if self.pos == 0 {
                self.stack.push(c);
                self.statements.push(vec![]);
                self.last = c;
                self.pos += 1;
                continue;
            }
            if c == ';' && !self.commenting {
                self.add_token();
                self.stack.push(c);
                self.add_token();

                if self.statements.len() != self.statements_counter {
                    self.statements.push(vec![])
                }
                self.statements_counter += 1;
                self.last = c;
                self.pos += 1;
                continue;
            }
            if self.commenting {
                if c == '\n' {
                    self.commenting = false;
                    self.add_token();
                    if self.statements.len() != self.statements_counter {
                        self.statements.push(vec![])
                    }
                    self.statements_counter += 1;
                } else {
                    self.last = c;
                    self.pos += 1;
                    self.stack.push(c);
                    continue;
                }
            }
            let pair: (char, char) = (self.last, c);
            if pair.0.is_ascii_alphabetic() {
                if pair.1.is_ascii_alphabetic() {
                    // alphabetic -> alphabetic
                    self.stack.push(c);
                } else if pair.1.is_ascii_digit() {
                    // alphabetic -> digit
                    self.stack.push(c);
                } else if pair.1.is_ascii_punctuation() {
                    // alphabetic -> punctuation
                    if matches!(pair.1, '(' | ')' | ';' | ',') {
                        self.add_token();
                        self.stack.push(c);
                    } else if matches!(pair.1, '\'' | '%') {
                        if self.stringing {
                            self.stack.push(c);
                        }
                    } else if matches!(pair.1, '_') {
                        self.stack.push(c);
                    } else {
                        println!("\tNot allowed: {}\t{}", pair.0, pair.1);
                    }
                } else if pair.1.is_ascii_whitespace() {
                    // alphabetic -> whitespace
                    self.add_token();
                } else {
                    println!("Unknown: {}, {}", pair.0, pair.1);
                }
            } else if pair.0.is_ascii_digit() {
                if pair.1.is_ascii_alphabetic() {
                    // digit -> alphabetic
                    println!("\tNot allowed: {}\t{}", pair.0, pair.1);
                } else if pair.1.is_ascii_digit() {
                    // digit -> digit
                    self.stack.push(c);
                } else if pair.1.is_ascii_punctuation() {
                    // digit -> punctuation
                    if matches!(pair.1, '.' | '(' | ')' | ';') {
                        self.add_token();
                        self.stack.push(c);
                    } else {
                        println!("\tNot allowed: {}\t{}", pair.0, pair.1);
                    }
                } else if pair.1.is_ascii_whitespace() {
                    // digit -> whitespace
                    self.add_token();
                } else {
                    println!("Unknown: {}, {}", pair.0, pair.1);
                }
            } else if pair.0.is_ascii_punctuation() {
                self.stringing = !(pair != ('\'', '\'') && (pair.1 == '\'' && !self.stringing));
                if pair.1.is_ascii_alphabetic() {
                    // punctuation -> alphabetic
                    if matches!(pair.0, '_' | '\'' | '%') {
                        // println!("\t{}", c);
                        self.stack.push(c);
                    } else {
                        self.add_token();
                        self.stack.push(c);
                    }
                } else if pair.1.is_ascii_digit() {
                    // punctuation -> digit
                    if matches!(pair.0, '.' | '(' | ')') {
                        self.add_token();
                        self.stack.push(c);
                    } else {
                        println!("\tNot allowed: {}\t{}", pair.0, pair.1);
                    }
                } else if pair.1.is_ascii_punctuation() {
                    // punctuation -> punctuation
                    if pair == ('(', ')')
                        || pair == (')', ',')
                        || pair == (')', '\'')
                        || pair == (')', ';')
                        || pair == ('(', '\'')
                        || pair == ('\'', '(')
                        || pair == ('\'', ')')
                        || pair == ('\'', ',')
                    {
                        self.add_token();
                        self.stack.push(c);
                    } else if pair == ('\'', '%')
                        || pair == ('%', '\'')
                        || pair == ('>', '=')
                        || pair == ('<', '=')
                    {
                        self.stack.push(c);
                    } else if pair == ('\'', '\'') {
                        if self.stringing {
                            self.stack.push(c);
                        } else {
                            self.add_token();
                            self.stack.push(c);
                        }
                    } else if pair == ('-', '-') {
                        self.commenting = true;
                        self.stack.push(c);
                    } else {
                        println!("\tNot allowed: {}\t{}", pair.0, pair.1);
                    }
                } else if pair.1.is_ascii_whitespace() {
                    // punctuation -> whitespace
                    self.add_token();
                } else {
                    println!("Unknown: {}, {}", pair.0, pair.1);
                }
            } else if pair.0.is_ascii_whitespace() {
                if pair.1.is_ascii_alphabetic() {
                    // whitespace => alphabetic
                    self.stack.push(c);
                } else if pair.1.is_ascii_digit() {
                    // whitespace => digit
                    self.stack.push(c);
                } else if pair.1.is_ascii_punctuation() {
                    // whitespace => punctuation
                    self.stack.push(c);
                } else if pair.1.is_ascii_whitespace() {
                    // whitespace => whitespace
                } else {
                    println!("Unknown: {}, {}", pair.0, pair.1);
                }
            }
            self.last = c;
            self.pos += 1;
        }

        if self.stack.len() > 0 {
            println!("Stack remains!");
            self.add_token();
        }

        let mut answer: String = "".to_string();

        // println!("Processing Statements: {:?}", &self.statements);

        for stmt in &self.statements {
            let mut line: String = "".to_string();
            for tkn in stmt.iter() {
                if self.col == 0 {
                    line += format!("{}", &tkn.value).as_str();
                } else {
                    line += format!("\n\t{}\t--{:?}", &tkn.value, &tkn.kind).as_str();
                }
                self.col += tkn.value.len();
            }
            answer += format!("{}\n", line).as_str();
            self.col = 0;
        }

        match fs::write("./formatted.sql", answer) {
            Ok(_) => {
                println!("Finished!")
            }
            Err(e) => {
                println!("Something went wrong... {}", e)
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file: Result<String, std::io::Error> = fs::read_to_string(&args[1]);

    match file {
        Ok(f) => {
            let mut lexer: Lexer = Lexer::new(f);
            lexer.run();
        }
        Err(_) => {
            println!("Please specify a target file")
        }
    }
    Ok(())
}
