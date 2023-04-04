use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::rc::Rc;
use regex::Regex;

enum MalType {
    Nil,
    Bool(bool),
    Int(i32),
    Sym(String),
    List(Rc<Vec<MalType>>),
}

enum MalErr {
    ParseErr(String),
}

struct Reader {
    tokens: Vec<String>,
    pos: usize,
}

impl Reader {
    fn next(&mut self) -> Option<String> {
        self.pos = self.pos + 1;
        match self.tokens.get(self.pos - 1) {
            Some(token) => Some(token.to_owned()),
            None => None,
        }
    }

    fn peek(&self) -> Option<String> {
        match self.tokens.get(self.pos) {
            Some(token) => Some(token.to_owned()),
            None => None,
        }
    }
}

fn tokenize(s: &str) -> Vec<String> {
    let reg = Regex::new(
       r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###
       ).expect("Invalid regular expression provided");

    let mut vec = vec![];
    for cap in reg.captures_iter(s) {
        // comment
        if cap[1].starts_with(";") {
            continue;
        }
        vec.push(String::from(&cap[1]))
    }
    vec
}

fn read_list(rd: &mut Reader, end: &str) -> Result<MalType, MalErr> {
    let mut vec: Vec<MalType> = vec![];
    loop {
        let token = match rd.peek() {
            Some(t) => t,
            None => {
                eprintln!("Expected {end} but found EOF");
                exit(1);
            }
        };
        if token == end {
            break;
        }
        vec.push(read_form(rd)?);
    }
    let _ = rd.next(); // skip ")"
    Ok(MalType::List(Rc::new(vec)))
}

fn read_atom(rd: &mut Reader) -> MalType {
    let num_re = Regex::new(r"^-?[0-9]+$").expect("Invalid regular expression for number");
    match rd.next() {
        Some(token) => match &token[..] {
            "nil" => MalType::Nil,
            "true" => MalType::Bool(true),
            "false" => MalType::Bool(false),
            _ => {
                if num_re.is_match(&token) {
                    MalType::Int(token.parse().unwrap())
                } else {
                    MalType::Sym(token.to_string())
                }
            }
            // any_thing => MalType::Int(any_thing.parse::<i32>().expect("unable to parse int")),
        }
        None => {
            eprintln!("Expected token found none");
            exit(1);
        }
    }
}

fn read_form(rd: &mut Reader) -> Result<MalType, MalErr> {
    match rd.peek() {
        Some(token) => match &token[..] {
            "(" => {
                let _ = rd.next();
                read_list(rd, ")")
            }
            _ => Ok(read_atom(rd)),
        }
        None => Err(MalErr::ParseErr("No tokens found".to_string())),
    }
}

fn read_str(s: &str) {
    let tokens = tokenize(&s);
    // println!("{:?}", tokens);
    let mut reader = Reader {
        tokens,
        pos: 0,
    };
    let _ = read_form(&mut reader);
}

fn rpl() { // read print loop
    let mut buf = String::new();
    loop {
        print!("mal> ");
        stdout().flush().expect("Failed to flush prompt");
        stdin().read_line(&mut buf).expect("Failed to read stdin");

        if !buf.is_empty() {
            read_str(&buf);
        }

        buf.clear();
    }
}

fn main() {
    rpl();
}
