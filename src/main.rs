use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::rc::Rc;

#[derive(Clone)]
enum MalType {
    Nil,
    Bool(bool),
    Int(i32),
    Str(String),
    Sym(String),
    Keyword(String),
    List(Rc<Vec<MalType>>),
    Func(fn(&[MalType]) -> Result<MalType, MalErr>),
}

#[derive(Debug)]
enum MalErr {
    ParseErr(String),
    FuncNotFound,
    WrongNumberOfArguments,
    UnexpectedToken,
}

struct Reader {
    tokens: Vec<String>,
    pos: usize,
}

#[derive(Clone)]
struct Env<'a> {
    env: HashMap<String, MalType>,
    outer: Option<&'a Env<'a>>,
}

impl Reader {
    fn next(&mut self) -> Option<String> {
        self.pos += 1;
        self.tokens.get(self.pos - 1).map(|token| token.to_owned())
    }

    fn peek(&self) -> Option<String> {
        self.tokens.get(self.pos).map(|token| token.to_owned())
    }
}

impl MalType {
    fn pr_str(&self) -> String {
        match self {
            Self::Nil => "nil".to_string(),
            Self::Bool(true) => "true".to_string(),
            Self::Bool(false) => "false".to_string(),
            Self::Int(num) => format!("{num}"),
            Self::Str(s) => s.clone(),
            Self::Sym(s) => s.clone(),
            Self::Keyword(s) => s.clone(),
            Self::List(list) => {
                let ret: Vec<String> = list.iter().map(|x| x.pr_str()).collect();
                format!("{}{}{}", "(", ret.join(" "), ")")
            }
            Self::Func(_f) => "<fn>".to_string(),
        }
    }
}

impl<'a> Env<'a> {
    fn new() -> Self {
        let add = MalType::Func(|vec: &[MalType]| match vec[..] {
            [MalType::Int(a), MalType::Int(b)] => Ok(MalType::Int(a + b)),
            _ => Err(MalErr::WrongNumberOfArguments),
        });
        let sub = MalType::Func(|vec: &[MalType]| match vec[..] {
            [MalType::Int(a), MalType::Int(b)] => Ok(MalType::Int(a - b)),
            _ => Err(MalErr::WrongNumberOfArguments),
        });
        let mul = MalType::Func(|vec: &[MalType]| match vec[..] {
            [MalType::Int(a), MalType::Int(b)] => Ok(MalType::Int(a * b)),
            _ => Err(MalErr::WrongNumberOfArguments),
        });
        let div = MalType::Func(|vec: &[MalType]| match vec[..] {
            [MalType::Int(a), MalType::Int(b)] => Ok(MalType::Int(a / b)),
            _ => Err(MalErr::WrongNumberOfArguments),
        });

        let mut env = Env {
            env: HashMap::new(),
            outer: None,
        };

        env.set("+", add);
        env.set("-", sub);
        env.set("*", mul);
        env.set("/", div);

        env
    }

    fn set(&mut self, k: &str, v: MalType) {
        self.env.insert(k.to_string(), v);
    }

    fn find(&self, k: &str) -> Option<MalType> {
        match self.env.get(k) {
            Some(val) => Some(val.clone()),
            None => match self.outer {
                Some(env) => env.find(k),
                None => None,
            },
        }
    }

    fn get(&self, k: &str) -> Option<MalType> {
        self.find(k)
    }
}

fn tokenize(s: &str) -> Vec<String> {
    let reg =
        Regex::new(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###)
            .expect("Invalid regular expression provided");

    let mut vec = vec![];
    for cap in reg.captures_iter(s) {
        // comment
        if cap[1].starts_with(';') {
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
                return Err(MalErr::UnexpectedToken);
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
    let str_re = Regex::new(r#"^"(.)*"$"#).expect("Invalid regular expression for string");
    let key_re = Regex::new(r"^:(.*)*$").expect("Invalid regular expression for keyword");
    match rd.next() {
        Some(token) => match &token[..] {
            "nil" => MalType::Nil,
            "true" => MalType::Bool(true),
            "false" => MalType::Bool(false),
            _ => {
                if num_re.is_match(&token) {
                    MalType::Int(token.parse().unwrap())
                } else if str_re.is_match(&token) {
                    MalType::Str(token.to_string())
                } else if key_re.is_match(&token) {
                    MalType::Keyword(token.to_string())
                } else {
                    MalType::Sym(token.to_string())
                }
            } // any_thing => MalType::Int(any_thing.parse::<i32>().expect("unable to parse int")),
        },
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
        },
        None => Err(MalErr::ParseErr("No tokens found".to_string())),
    }
}

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

            match &l[0] {
                MalType::Sym(s) if s == "def!" => match &l.to_vec()[..] {
                    [_, MalType::Sym(x), y] => {
                        let evaluated = eval(y.clone(), env);
                        env.set(x, evaluated.clone());
                        evaluated
                    }
                    _ => unreachable!(
                        "'def!' cannot work this way, either the types or length provided is wrong"
                    ),
                },
                MalType::Sym(s) if s == "let*" => {
                    let mut new_env = Env::new();
                    match &l.to_vec()[..] {
                        [_, MalType::List(l), y] => {
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
                        _ => unreachable!("Something went wrong with 'let*'"),
                    }
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

fn rpl() {
    // read print loop
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
    rpl();
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::eval;
    use crate::read_str;
    use crate::Env;

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
