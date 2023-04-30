use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Token {
    value: String,
}

impl Token {
    fn new(value: String) -> Self {
        Token { value: value }
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
        self.statements[self.statements_counter].push(Token::new(self.stack.iter().collect()));
        self.stack.clear();
    }

    fn run(&mut self) {
        println!("Running...");
        self.state = 1;
        self.input = self.input.to_ascii_uppercase();
        let a = self.input.clone();
        let all = a.chars();

        for c in all {
            if self.pos == 0 {
                self.stack.push(c);
                self.statements.push(vec![]);
                self.last = c;
                self.pos += 1;
                continue;
            }
            if c == ';' {
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
            let pair = (self.last, c);
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
                    } else if matches!(pair.1, '\'') {
                        self.stack.push(c);
                    } else if matches!(pair.1, '_') {
                        self.stack.push(c);
                    } else {
                        println!("Not allowed: {}\t{}", pair.0, pair.1);
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
                    println!("Not allowed: {}\t{}", pair.0, pair.1);
                } else if pair.1.is_ascii_digit() {
                    // digit -> digit
                    self.stack.push(c);
                } else if pair.1.is_ascii_punctuation() {
                    // digit -> punctuation
                    if matches!(pair.1, '.' | '(' | ')' | ';') {
                        self.add_token();
                        self.stack.push(c);
                    } else {
                        println!("Not allowed: {}\t{}", pair.0, pair.1);
                    }
                } else if pair.1.is_ascii_whitespace() {
                    // digit -> whitespace
                    self.add_token();
                } else {
                    println!("Unknown: {}, {}", pair.0, pair.1);
                }
            } else if pair.0.is_ascii_punctuation() {
                if pair.1.is_ascii_alphabetic() {
                    // punctuation -> alphabetic
                    if matches!(pair.0, '_' | '\'') {
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
                        println!("Not allowed: {}\t{}", pair.0, pair.1);
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
                        || pair == ('\'', '\'')
                    {
                        self.add_token();
                        self.stack.push(c);
                    } else {
                        println!("Not allowed: {}\t{}", pair.0, pair.1);
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

        let mut answer = "".to_string();

        for stmt in &self.statements {
            let mut line = "".to_string();
            let mut longest = 0;
            for tkn in stmt.iter() {
                if self.col == 0 {
                    line += format!("{}", &tkn.value).as_str();
                }
                //  else if tkn.value == "(" {
                //     line += format!("\n\t{}", &tkn.value).as_str();
                //     col = 0;
                // }
                else if self.col > longest {
                    line += format!("\n\t{}", &tkn.value).as_str();
                    self.col = 0;
                }
                // else if col > 40 {
                //     statement += format!("\n\t{}", &tkn.value).as_str();
                //     col = 0;
                // }
                else {
                    line += format!(" {}", &tkn.value).as_str();
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