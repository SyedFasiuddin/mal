use std::io::{stdin, stdout, Write};

use regex::Regex;

struct Reader {
    tokens: Vec<String>,
    pos: usize,
}

impl Reader {
    fn next(&mut self) -> Option<String> {
        self.pos = self.pos + 1;
        match self.tokens.get(self.pos) {
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

fn read_str() {
}

fn rpl() { // read print loop
    let mut buf = String::new();
    loop {
        print!("mal> ");
        stdout().flush().expect("Failed to flush prompt");
        stdin().read_line(&mut buf).expect("Failed to read stdin");
        print!("{buf}");
        buf.clear();
    }
}

fn main() {
    rpl();
}
