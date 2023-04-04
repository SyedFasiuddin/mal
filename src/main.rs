use std::io::{stdin, stdout, Write};

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
