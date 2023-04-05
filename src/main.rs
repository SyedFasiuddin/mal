use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::rc::Rc;
use regex::Regex;

#[derive(Debug)]
enum MalType {
    Nil,
    Bool(bool),
    Int(i32),
    Sym(String),
    List(Rc<Vec<MalType>>),
}

#[derive(Debug)]
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

impl MalType {
    fn pr_str(&self) -> String {
        match self {
            Self::Nil => "nil".to_string(),
            Self::Bool(true) => "true".to_string(),
            Self::Bool(false) => "false".to_string(),
            Self::Int(num) => format!("{num}"),
            Self::Sym(s) => s.clone(),
            Self::List(list) => {
                let ret: Vec<String> = list.iter().map(|x| x.pr_str()).collect();
                format!("{}{}{}", "(", ret.join(" "), ")")
            }
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

fn read_str(s: &str) -> Result<MalType, MalErr> {
    let tokens = tokenize(&s);
    // println!("{:?}", tokens);
    let mut reader = Reader {
        tokens,
        pos: 0,
    };
    read_form(&mut reader)
}

fn rpl() { // read print loop
    let mut buf = String::new();
    loop {
        print!("mal> ");
        stdout().flush().expect("Failed to flush prompt");
        stdin().read_line(&mut buf).expect("Failed to read stdin");

        if !buf.is_empty() {
            match read_str(&buf) {
                Ok(mal) => println!("{}", mal.pr_str()),
                Err(e) => {
                    eprintln!("Something went wrong: {e:?}");
                    exit(1);
                }
            }
        }

        buf.clear();
    }
}

fn main() {
    rpl();
}
