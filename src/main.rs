use std::io::{stdin, stdout, Write};

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
