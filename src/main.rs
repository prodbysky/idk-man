use core::fmt;
use std::collections::HashMap;
use std::env;
use std::process::exit;
use std::sync::RwLock;
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq)]
enum StackValue {
    Number(i32),
    Text(String),
}

impl fmt::Display for StackValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackValue::Number(n) => write!(f, "{}", n)?,
            StackValue::Text(t) => write!(f, "{}", t)?,
        }; Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    PushInt(i32),

    // Arithmetics
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Logic operators
    Eq,
    Lt,
    Mt,

    // Control flow
    If(usize),
    While,
    Do(usize),
    End,
    WEnd(usize),

    // Stack manipulators
    Dup,
    
    // THIS IS FOR THE FUTURE IMPL OF MACROS
    UserDefinedWord(String),

    Print,
}


// TODO: This does not seem optimal from the aesthetics point of view.
lazy_static! {
    static ref KEYWORDS: RwLock<HashMap<&'static str, TokenType>> = {
        let mut map = HashMap::new();
        map.insert("+", TokenType::Add);
        map.insert("-", TokenType::Sub);
        map.insert("*", TokenType::Mul);
        map.insert("/", TokenType::Div);
        map.insert("%", TokenType::Mod);
        map.insert("=", TokenType::Eq);
        map.insert(">", TokenType::Lt);
        map.insert("<", TokenType::Mt);
        map.insert("if", TokenType::If(0));
        map.insert("while", TokenType::While);
        map.insert("do", TokenType::Do(0));
        map.insert("wend", TokenType::WEnd(0));
        map.insert("end", TokenType::End);
        map.insert("dup", TokenType::Dup);
        map.insert("print", TokenType::Print);
        RwLock::new(map)
    };
}

fn tokenize(src: String) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = vec![];
    let chars: Vec<char> = src.chars().collect(); 
    let mut i = 0;
    
    // Parse a word out of the source
    let curr_word = |mut i: usize| {
        let current_char = |i: usize| {chars[i]};
        let mut word = String::new();
        while !current_char(i).is_whitespace() {
            word.push(current_char(i));
            i += 1;
        }
        (word, i)
    };

    let kws = KEYWORDS.read().expect("Failed to acquire KEYWORDS...");

    while i < src.len() {
        let (current_word, temp_i) = curr_word(i);
        i = temp_i;
        if current_word != "" {
            if let Ok(num) = current_word.parse::<i32>() {
                tokens.push(TokenType::PushInt(num));
            } else {
                // If the word is a existing keyword
                if let Some(token) = kws.get(current_word.as_str()) {
                    tokens.push(token.clone());
                } else {
                    tokens.push(TokenType::UserDefinedWord(current_word));
                }
            }
        }
        
        i += 1;
    }
    tokens
}

fn crossref_blocks(program: Vec<TokenType>) -> Vec<TokenType> {
    let mut stack: Vec<usize> = vec![];
    let mut res = program.clone();
    for (ip, op) in program.iter().enumerate() {
         
        match op {
            TokenType::End => {
                let if_ip = stack.pop().expect("Empty stack at crossref_blocks (Encountered end)...");
                res[if_ip] = TokenType::If(ip);
            },
            TokenType::WEnd(_) => {
                let do_ip = stack.pop().expect("Empty stack at crossref_blocks (Encountered WEnd)...");
                res[do_ip] = TokenType::Do(ip);
                let while_ip = stack.pop().expect("Empty stack at crossref_blocks (Encountered WEnd)...");
                res[ip] = TokenType::WEnd(while_ip);
            },
            TokenType::Do(_) => {
                stack.push(ip);
            },
            TokenType::If(_) => {
                stack.push(ip);
            },
            TokenType::While => {
                stack.push(ip);
            },

            _ => {},
        }
    }
    res
}

fn run(program: Vec<TokenType>) {
    println!("{:?}", program);
    let mut stack: Vec<StackValue> = vec![];
    let mut i: usize = 0;
    while i < program.len() {
        let token = program[i].clone(); 
        match token {
            TokenType::PushInt(num) => {
                stack.push(StackValue::Number(num));
                i += 1;
            },
            TokenType::Add => {
                let (a, b) = (stack.pop().expect("Empty stack when interpreting TokenType::Add..."), stack.pop().expect("Empty stack when interpreting TokenType::Add..."));

                match (a, b) {
                    (StackValue::Number(a), StackValue::Number(b)) => {
                        stack.push(StackValue::Number(a + b)) 
                    },
                    _ => {},
                }
                i += 1;
            },
            TokenType::Sub => {
                let (a, b) = (stack.pop().expect("Stack underflow when interpreting TokenType::Sub"), stack.pop().expect("Stack underflow when interpreting TokenType::Sub"));
                match (a, b) {
                    (StackValue::Number(a), StackValue::Number(b)) => {
                        stack.push(StackValue::Number(b - a)) 
                    },
                    _ => {},
                }
                i += 1;
            },
            TokenType::Mul => {
                let (a, b) = (stack.pop().expect("Stack underflow when interpreting TokenType::Mul"), stack.pop().expect("Stack underflow when interpreting TokenType::Mul"));
                match (a, b) {
                    (StackValue::Number(a), StackValue::Number(b)) => {
                        stack.push(StackValue::Number(b * a)) 
                    },
                    _ => {},
                }
                i += 1;
            },
            TokenType::Div => {
                let (a, b) = (stack.pop().expect("Stack underflow when interpreting TokenType::Div"), stack.pop().expect("Stack underflow when interpreting TokenType::Div"));
                match (a, b) {
                    (StackValue::Number(a), StackValue::Number(b)) => {
                        stack.push(StackValue::Number(b / a)) 
                    },
                    _ => {},
                }
                i += 1;
            },
            TokenType::Mod => {
                let (a, b) = (stack.pop().expect("Stack underflow when interpreting TokenType::Mod"), stack.pop().expect("Stack underflow when interpreting TokenType::Mod"));
                match (a, b) {
                    (StackValue::Number(a), StackValue::Number(b)) => {
                        stack.push(StackValue::Number(b % a)) 
                    },
                    _ => {},
                }
                i += 1;
            },
            TokenType::Eq => {
                let (a, b) = (stack.pop().expect("Stack undeflow when interpreting TokenType::Eq"), stack.pop().expect("Stack undeflow when interpreting TokenType::Eq"));
                match (a, b) {
                    (StackValue::Number(a), StackValue::Number(b)) => {
                        stack.push(StackValue::Number((b == a) as i32)) 
                    },
                    _ => {},
                }
                i += 1; 
            },
            TokenType::Lt => {
                let (a, b) = (stack.pop().expect("Stack undeflow when interpreting TokenType::Lt"), stack.pop().expect("Stack undeflow when interpreting TokenType::Lt"));
                match (a, b) {
                    (StackValue::Number(a), StackValue::Number(b)) => {
                        stack.push(StackValue::Number((b > a) as i32)) 
                    },
                    _ => {},
                }
                i += 1; 
            },
            TokenType::Mt => {
                let (a, b) = (stack.pop().expect("Stack undeflow when interpreting TokenType::Mt"), stack.pop().expect("Stack undeflow when interpreting TokenType::Mt"));
                match (a, b) {
                    (StackValue::Number(a), StackValue::Number(b)) => {
                        stack.push(StackValue::Number((b < a) as i32)) 
                    },
                    _ => {},
                }
                i += 1; 
            }, 
            TokenType::If(ip) => {
                let a = stack.pop().expect("Stack underflow when interpreting TokenType::If");
                match a {
                    StackValue::Number(num) => {
                        if num > 0 { i += 1; } else { i = ip; }
                    },
                    _ => {},
                }
            },
            TokenType::Do(ip) => {
                let a = stack.pop().expect("Stack underflow when interpreting TokenType::Do(i32)");
                match a {
                    StackValue::Number(num) => { 
                        if num > 0 { 
                            i += 1;
                        } 
                        else { 
                            i = ip + 1;
                        } 
                    }, 
                    _ => {},
                }
            },
            TokenType::WEnd(ip) => { i = ip; },
            TokenType::Dup => {
                let a = stack.pop().expect("Stack underflow when interpreting TokenType::Dup"); 
                stack.push(a.clone());
                stack.push(a.clone());
                i += 1;
            }
            TokenType::Print => {
                let a = stack.pop().expect("Stack underflow when interpreting TokenType::Print");
                println!("{}", a);
                i += 1;
            },
            // Words that do not effect the execution of the program
            TokenType::While | TokenType::End => {i += 1;}
            TokenType::UserDefinedWord(_) => {},
        }
    }
}

fn usage() {
    println!("./idk-man <Program file name>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        exit(1);
    }
    let filename = args[1].clone();
    if let Ok(src) = std::fs::read_to_string(filename) {
        run(crossref_blocks(tokenize(src)));
    } 
}
