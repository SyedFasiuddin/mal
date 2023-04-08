use std::rc::Rc;

use crate::types::MalErr;
use crate::MalType::{self, Bool, Func, Int, List, Nil, Str};

pub fn ns() -> Vec<(&'static str, MalType)> {
    vec![
        (
            "+",
            Func(|vec| match vec[..] {
                [Int(a), Int(b)] => Ok(Int(a + b)),
                _ => Err(MalErr::E(
                    "Wrong number or type of arguments provided to operator `+'".to_string(),
                )),
            }),
        ),
        (
            "-",
            Func(|vec| match vec[..] {
                [Int(a), Int(b)] => Ok(Int(a - b)),
                _ => Err(MalErr::E(
                    "Wrong number or type of arguments provided to operator `-'".to_string(),
                )),
            }),
        ),
        (
            "*",
            Func(|vec| match vec[..] {
                [Int(a), Int(b)] => Ok(Int(a * b)),
                _ => Err(MalErr::E(
                    "Wrong number or type of arguments provided to operator `*'".to_string(),
                )),
            }),
        ),
        (
            "/",
            Func(|vec| match vec[..] {
                [Int(a), Int(b)] => Ok(Int(a / b)),
                _ => Err(MalErr::E(
                    "Wrong number or type of arguments provided to operator `/'".to_string(),
                )),
            }),
        ),
        (
            "=",
            Func(|vec| match &vec[..] {
                [Nil, Nil] => Ok(Bool(true)),
                [Bool(a), Bool(b)] => Ok(Bool(a == b)),
                [Int(a), Int(b)] => Ok(Bool(a == b)),
                [Str(a), Str(b)] => Ok(Bool(a == b)),
                [List(_l1), List(_l2)] => {
                    todo!();
                }
                _ => Err(MalErr::E(
                    "Wrong number or type of arguments for `=' operator".to_string(),
                )),
            }),
        ),
        (
            "<",
            Func(|vec| match vec[..] {
                [Int(a), Int(b)] => Ok(Bool(a < b)),
                _ => Err(MalErr::E(
                    "Wrong type of arguments for `<' operator".to_string(),
                )),
            }),
        ),
        (
            "<=",
            Func(|vec| match vec[..] {
                [Int(a), Int(b)] => Ok(Bool(a <= b)),
                _ => Err(MalErr::E(
                    "Wrong type of arguments for `<=' operator".to_string(),
                )),
            }),
        ),
        (
            ">",
            Func(|vec| match vec[..] {
                [Int(a), Int(b)] => Ok(Bool(a > b)),
                _ => Err(MalErr::E(
                    "Wrong type of arguments for `>' operator".to_string(),
                )),
            }),
        ),
        (
            ">=",
            Func(|vec| match vec[..] {
                [Int(a), Int(b)] => Ok(Bool(a >= b)),
                _ => Err(MalErr::E(
                    "Wrong type of arguments for `>=' operator".to_string(),
                )),
            }),
        ),
        (
            "count",
            Func(|vec| {
                if vec.len() < 1 {
                    return Err(MalErr::E(
                        "Possibly wrong number of arguments provided to `count'".to_string(),
                    ));
                }
                match &vec[0] {
                    List(l) => Ok(Int(l.iter().count() as i32)),
                    _ => Ok(Int(1)),
                }
            }),
        ),
        (
            "list",
            Func(|vec| {
                let mut ret = Vec::new();
                for each in vec {
                    ret.push(each.clone());
                }
                Ok(List(Rc::new(ret)))
            }),
        ),
        (
            "list?",
            Func(|vec| {
                if vec.len() < 1 {
                    return Err(MalErr::E(
                        "Possibly wrong number of arguments provided to `list?'".to_string(),
                    ));
                }
                match &vec[0] {
                    List(_) => Ok(Bool(true)),
                    _ => Ok(Bool(false)),
                }
            }),
        ),
        (
            "empty?",
            Func(|vec| {
                if vec.len() < 1 {
                    return Err(MalErr::E(
                        "Possibly wrong number of arguments provided to `empty?'".to_string(),
                    ));
                }
                match &vec[0] {
                    List(l) => Ok(Bool(l.is_empty())),
                    _ => Ok(Bool(false)),
                }
            }),
        ),
    ]
}
