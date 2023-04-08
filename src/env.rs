use crate::core;
use crate::MalType;
use std::borrow::BorrowMut;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Env {
    env: HashMap<String, MalType>,
    pub outer: Option<Box<Env>>,
}

impl Default for Env {
    fn default() -> Self {
        let mut env = Env {
            env: HashMap::new(),
            outer: None,
        };

        for (key, val) in core::ns() {
            env.set(key, val);
        }

        env
    }
}

impl Env {
    pub fn new(binds: Vec<MalType>, exprs: Vec<MalType>) -> Self {
        let mut env = Env::default();

        for (key, val) in binds.iter().zip(&exprs) {
            env.set(&key.clone().pr_str(), val.clone());
        }
        env
    }

    pub fn set(&mut self, k: &str, v: MalType) {
        self.env.borrow_mut().insert(k.to_string(), v);
    }

    fn find(&self, k: &str) -> Option<MalType> {
        match self.env.get(k) {
            Some(val) => Some(val.clone()),
            None => match &self.outer {
                Some(env) => env.find(k),
                None => None,
            },
        }
    }

    pub fn get(&self, k: &str) -> Option<MalType> {
        self.find(k)
    }
}
