pub mod env;
pub mod reader;
pub mod types;

use crate::env::Env;
use crate::reader::read_form;
use crate::reader::tokenize;
use crate::reader::Reader;
use crate::types::MalErr;
use crate::types::MalType;
use itertools::Itertools;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::rc::Rc;

fn read_str(s: &str) -> Result<MalType, MalErr> {
    let tokens = tokenize(s);
    // println!("{:?}", tokens);
    let mut reader = Reader { tokens, pos: 0 };
    read_form(&mut reader)
}

fn eval_ast(ast: &MalType, env: &mut Env) -> Result<MalType, MalErr> {
    match ast {
        MalType::Sym(s) => match env.get(&s[..]) {
            Some(f) => Ok(f),
            None => {
                eprintln!("Unable to find {s} in current environment");
                Err(MalErr::FuncNotFound)
            }
        },
        MalType::List(l) => {
            let mut vec = vec![];
            for each in l.iter() {
                vec.push(eval(each.clone(), env));
            }
            Ok(MalType::List(Rc::new(vec)))
        }
        _ => Ok(ast.clone()),
    }
}

fn eval(ast: MalType, env: &mut Env) -> MalType {
    match ast.clone() {
        MalType::List(l) => {
            if l.is_empty() {
                return MalType::List(l);
            }

            match &l.to_vec()[..] {
                [MalType::Sym(s), MalType::Sym(x), y] if s == "def!" => {
                    let evaluated = eval(y.clone(), env);
                    env.set(&x, evaluated.clone());
                    evaluated
                }
                [MalType::Sym(s), MalType::List(l), y] if s == "let*" => {
                    let mut new_env = Env::new();
                    for (key, val) in l.iter().cloned().tuples() {
                        match key {
                            MalType::Sym(s) => {
                                let evaluated = eval(val.clone(), &mut new_env);
                                new_env.set(&s, evaluated);
                            }
                            _ => todo!("Wrong type for symbol"),
                        }
                    }
                    new_env.outer = Some(env);
                    eval(y.clone(), &mut new_env)
                }
                _ => match eval_ast(&ast, env).unwrap() {
                    MalType::List(ref l) => match l.to_vec()[..] {
                        [MalType::Func(f), _, _] => match f(&l.to_vec()[1..=2]) {
                            Ok(val) => val,
                            Err(MalErr::WrongNumberOfArguments) => {
                                eprintln!("Wrong number of arguments provided");
                                todo!("Handle err when wrong number of arguments were provided");
                            }
                            Err(_) => {
                                unreachable!("No other type of error can be returned by MalFunc")
                            }
                        },
                        _ => unreachable!("You are trying to do something wrong"),
                    },
                    _ => {
                        eprintln!("Unexpected token at first position of list");
                        exit(1);
                    }
                },
            }
        }
        _ => eval_ast(&ast, env).unwrap(),
    }
}

fn repl() {
    let mut env = Env::new();
    let mut buf = String::new();

    loop {
        print!("mal> ");
        stdout().flush().expect("Failed to flush prompt");
        stdin().read_line(&mut buf).expect("Failed to read stdin");

        if !buf.is_empty() {
            match read_str(&buf) {
                Ok(mal) => println!("{}", eval(mal, &mut env).pr_str()),
                Err(e) => {
                    eprintln!("Something went wrong: {e:?}");
                }
            }
        }

        buf.clear();
    }
}

fn main() {
    repl();
}

#[cfg(test)]
mod tests {
    use crate::eval;
    use crate::read_str;
    use crate::Env;
    use std::collections::HashMap;

    #[test]
    fn step1() {
        let hash = HashMap::from([
            ("()", "()"),
            ("1", "1"),
            ("    1", "1"),
            ("    -123    ", "-123"),
            ("+", "+"),
            ("    abc", "abc"),
            ("    abc123    ", "abc123"),
            ("abc-def", "abc-def"),
            ("( * 1   2   )", "(* 1 2)"),
            ("(1, 2, 3,,,,),,,", "(1 2 3)"),
            ("  ( +   1 (+  2 3  )  )", "(+ 1 (+ 2 3))"),
            ("(def! x 3)", "(def! x 3)"),
            ("(1 2 3 4 5 6)", "(1 2 3 4 5 6)"),
        ]);

        for (input, output) in hash {
            let mal = read_str(input).unwrap();
            assert_eq!(output, mal.pr_str());
        }
    }

    #[test]
    fn step2() {
        let hash = HashMap::from([
            ("(+ 1 2)", "3"),
            ("(+ 5 (* 2 3))", "11"),
            ("(- (+ 5 (* 2 3)) 3)", "8"),
            ("(/ (- (+ 5 (* 2 3)) 3) 4)", "2"),
            ("(/ (- (+ 515 (* 87 311)) 302) 27)", "1010"),
            ("(* -3 6)", "-18"),
            ("(/ (- (+ 515 (* -87 311)) 296) 27)", "-994"),
        ]);
        let mut env = Env::new();

        for (input, output) in hash {
            let mal = read_str(input).unwrap();
            assert_eq!(output, eval(mal, &mut env).pr_str());
        }
    }

    #[test]
    fn step3() {
        let hash = HashMap::from([
            ("(def! x 3)", "3"),
            ("(def! x (+ 1 7))", "8"),
            ("(def! y (let* (z 7) z))", "7"),
            ("(let* (z 9) z)", "9"),
            ("(let* (z (+ 2 3)) (+ 1 z))", "6"),
            ("(let* (p (+ 2 3) q (+ 2 p)) (+ p q))", "12"),
            ("(let* (x 2 x 3) x)", "3"),
        ]);
        let mut env = Env::new();

        for (input, output) in hash {
            let mal = read_str(input).unwrap();
            assert_eq!(output, eval(mal, &mut env).pr_str());
        }

        let mal = read_str("(def! a 4)").unwrap();
        assert_eq!("4", eval(mal, &mut env).pr_str());

        let mal = read_str("(let* (z 2) (let* (q 9) a))").unwrap();
        assert_eq!("4", eval(mal, &mut env).pr_str());

        /*
           Hashmaps donot store items in the order of their insertion and so they are not
           accessed in order in the for loop, the order is arbitrary. So this makes it very
           diffucult to test the values are added to the environment properly and so had to
           write the test one after the other.
        */
    }
}
