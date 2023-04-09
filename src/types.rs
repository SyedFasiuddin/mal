use crate::env::Env;
use std::rc::Rc;

#[derive(Clone)]
pub enum MalType {
    Nil,
    Bool(bool),
    Int(i32),
    Str(String),
    Sym(String),
    Keyword(String),
    List(Rc<Vec<MalType>>),
    Func(fn(&[MalType]) -> Result<MalType, MalErr>),
    MalFunc {
        env: Rc<Env>,
        params: Vec<MalType>,
        body: Box<MalType>,
    },
}

#[derive(Debug)]
pub enum MalErr {
    ParseErr(String),
    E(String),
    FuncNotFound,
    WrongNumberOfArguments,
    UnexpectedToken,
}

impl MalType {
    pub fn pr_str(&self) -> String {
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
            Self::Func(_f) => "<std:fn>".to_string(),
            Self::MalFunc { .. } => "<user:fn>".to_string(),
        }
    }
}
