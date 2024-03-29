pub mod core;
pub mod env;
pub mod reader;
pub mod types;

use crate::env::Env;
use crate::reader::read_str;
use crate::types::MalErr;
use crate::types::MalType;
use itertools::Itertools;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::rc::Rc;

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
                    let mut new_env = Env::default();
                    for (key, val) in l.iter().cloned().tuples() {
                        match key {
                            MalType::Sym(s) => {
                                let evaluated = eval(val.clone(), &mut new_env);
                                new_env.set(&s, evaluated);
                            }
                            _ => todo!("Wrong type for symbol"),
                        }
                    }
                    new_env.outer = Some(Rc::new((*env).clone()));
                    eval(y.clone(), &mut new_env)
                }
                [MalType::Sym(s), ..] if s == "do" => {
                    match eval_ast(&MalType::List(Rc::new(l[1..].to_vec())), env).unwrap() {
                        MalType::List(l) => l.last().unwrap().clone(),
                        _ => unreachable!("Wrong do form"),
                    }
                }
                [MalType::Sym(s), ..] if s == "if" => match eval(l[1].clone(), env) {
                    MalType::Nil | MalType::Bool(false) => {
                        if l.len() > 3 {
                            eval(l[3].clone(), env)
                        } else {
                            MalType::Nil
                        }
                    }
                    _ => eval(l[2].clone(), env),
                },
                [MalType::Sym(s), MalType::List(params), body] if s == "fn*" => MalType::MalFunc {
                    env: Rc::new((*env).clone()),
                    params: params.clone().to_vec(),
                    body: Box::new(body.clone()),
                },
                _ => match eval_ast(&ast, env).unwrap() {
                    MalType::List(ref l) => match &l.to_vec()[..] {
                        [MalType::Func(f), ..] => match f(&l.to_vec()[1..]) {
                            Ok(val) => val,
                            Err(MalErr::E(s)) => {
                                eprintln!("{s}");
                                MalType::Nil
                            }
                            Err(_) => {
                                unreachable!("No other type of error can be returned by MalFunc")
                            }
                        },
                        [MalType::MalFunc { env, params, body }, ..] => {
                            let exprs = l[1..].to_vec();
                            let mut new_env = Env::new(params.to_vec(), exprs);
                            new_env.outer = Some(env.clone());
                            eval(*body.to_owned(), &mut new_env)
                        }
                        _ => eval_ast(&ast, env).unwrap(),
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
    let mut env = Env::default();
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
    use crate::env::Env;
    use crate::eval;
    use crate::reader::read_str;
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
        let mut env = Env::default();

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
        let mut env = Env::default();

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

    #[test]
    fn step4() {
        let hash = HashMap::from([
            ("(do (def! a 6) 7 (+ a 8))", "14"),
            ("(def! DO 7)", "7"),
            ("(if true 7 8)", "7"),
            ("(if false 7 8)", "8"),
            ("(if false 7)", "nil"),
            ("(if false 7 false)", "false"),
            ("(if true (+ 1 7) (+ 1 8))", "8"),
            ("(if false (+ 1 7) (+ 1 8))", "9"),
            ("(if nil 7 8)", "8"),
            ("(if 0 7 8)", "7"),
            ("(if false (+ 1 7))", "nil"),
            ("(if nil 8)", "nil"),
            ("(if nil 8 7)", "7"),
            ("(if true (+ 1 7))", "8"),
            ("(fn* (a) a)", "<user:fn>"),
            ("( (fn* () 4) )", "4"),
            ("( (fn* (a) a) 7)", "7"),
            ("( (fn* (a) (+ a 1)) 10)", "11"),
            ("( (fn* (a b) (+ b a)) 3 4)", "7"),
            ("( (fn* (a b) (+ a b)) 2 3)", "5"),
            ("( (fn* (f x) (f x)) (fn* (a) (+ 1 a)) 7)", "8"),
            ("( ( (fn* (a) (fn* (b) (+ a b))) 5) 7)", "12"),
        ]);
        let mut env = Env::default();

        for (input, output) in hash {
            let mal = read_str(input).unwrap();
            assert_eq!(output, eval(mal, &mut env).pr_str());
        }
    }

    #[test]
    fn step4_ex1() {
        let mut env = Env::default();
        let mal = read_str("(def! gen-plus5 (fn* () (fn* (b) (+ 5 b))))").unwrap();
        eval(mal, &mut env);
        let mal = read_str("(def! plus5 (gen-plus5))").unwrap();
        eval(mal, &mut env);
        let mal = read_str("(plus5 7)").unwrap();
        assert_eq!("12", eval(mal, &mut env).pr_str());
    }

    #[test]
    #[ignore = "fn* creates a new environment whose outer environment is a clone of the state of
        environment during time of creation of fn*, which means this def! has not yet defined the
        function `sumdown' in the outer environment referred to by the body of the function and so
        it cannot find `sumdown' and will panic."]
    fn step4_reccursive_fn() {
        let mut env = Env::default();
        let mal =
            read_str("(def! sumdown (fn* (N) (if (> N 0) (+ N (sumdown  (- N 1))) 0)))").unwrap();
        eval(mal, &mut env);
        let mal = read_str("(sumdown 6)").unwrap();
        assert_eq!("21", eval(mal, &mut env).pr_str());

        let mal = read_str(
            "(def! fib (fn* (N) (if (= N 0) 1 (if (= N 1) 1 (+ (fib (- N 1)) (fib (- N 2)))))))",
        )
        .unwrap();
        eval(mal, &mut env);
        let mal = read_str("(fib 4)").unwrap();
        assert_eq!("5", eval(mal, &mut env).pr_str());

        let mal = read_str("(def! sum2 (fn* (n acc) (if (= n 0) acc (sum2 (- n 1) (+ n acc)))))").unwrap();
        eval(mal, &mut env);
        let mal = read_str("(sum2 10 0)").unwrap();
        assert_eq!("55", eval(mal, &mut env).pr_str());

        let mal = read_str("(def! res2 nil)").unwrap();
        assert_eq!("nil", eval(mal, &mut env).pr_str());

        let mal = read_str("(def! res2 (sum2 10000 0))").unwrap();
        assert_eq!("res2", eval(mal, &mut env).pr_str());
        let mal = read_str("res2").unwrap();
        assert_eq!("50005000", eval(mal, &mut env).pr_str());
    }

    #[test]
    fn test_stdlib() {
        let hash = HashMap::from([
            (r#"(= "" "")"#, "true"),
            (r#"(= "abc" "abc")"#, "true"),
            (r#"(= "abc" "")"#, "false"),
            (r#"(= "abc" "")"#, "false"),
            (r#"(= "" "abc")"#, "false"),
            (r#"(= "abc" "def")"#, "false"),
            (r#"(= "abc" "ABC")"#, "false"),
            (r#"(if "" 9 4)"#, "9"),
            ("(= 1 1)", "true"),
            ("(= 1 2)", "false"),
            ("(< 81 94)", "true"),
            ("(<= 1 2)", "true"),
            ("(>= 1 1)", "true"),
            ("(>= 1 2)", "false"),
            ("(list 1 2 3)", "(1 2 3)"),
            ("(list)", "()"),
            ("(count (1 2 3))", "3"),
            ("(count (list))", "0"),
            ("(list? 1 2)", "false"),
            ("(list? (1) 2)", "true"),
            ("(list? (list))", "true"),
            ("(empty? (list))", "true"),
        ]);
        let mut env = Env::default();

        for (input, output) in hash {
            let mal = read_str(input).unwrap();
            assert_eq!(output, eval(mal, &mut env).pr_str());
        }
    }

    #[test]
    #[ignore = "not implemented"]
    fn step5() {
        let mut env = Env::default();
        let mal = read_str("(def! sum2 (fn* (n acc) (if (= n 0) acc (sum2 (- n 1) (+ n acc)))))").unwrap();
        eval(mal, &mut env);
        let mal = read_str("(sum2 10 0)").unwrap();
        assert_eq!("55", eval(mal, &mut env).pr_str());

        let mal = read_str("(def! res2 nil)").unwrap();
        assert_eq!("nil", eval(mal, &mut env).pr_str());

        let mal = read_str("(def! res2 (sum2 10000 0))").unwrap();
        assert_eq!("res2", eval(mal, &mut env).pr_str());
        let mal = read_str("res2").unwrap();
        assert_eq!("50005000", eval(mal, &mut env).pr_str());

        let mal = read_str("(def! foo (fn* (n) (if (= n 0) 0 (bar (- n 1)))))").unwrap();
        eval(mal, &mut env).pr_str();
        let mal = read_str("(def! bar (fn* (n) (if (= n 0) 0 (foo (- n 1)))))").unwrap();
        eval(mal, &mut env).pr_str();
        let mal = read_str("(foo 10000)").unwrap();
        assert_eq!("0", eval(mal, &mut env).pr_str());
    }
}
