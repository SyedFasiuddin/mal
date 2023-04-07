use crate::MalErr;
use crate::MalType;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Env<'a> {
    env: HashMap<String, MalType>,
    pub outer: Option<&'a Env<'a>>,
}

impl<'a> Default for Env<'a> {
    fn default() -> Self {
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
}

impl<'a> Env<'a> {
    pub fn set(&mut self, k: &str, v: MalType) {
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

    pub fn get(&self, k: &str) -> Option<MalType> {
        self.find(k)
    }
}
