use crate::types::MalErr;
use crate::MalType::{self, Bool, Func, Int, List, Nil, Str};

pub fn ns() -> Vec<(&'static str, MalType)> {
    vec![
        (
            "+",
            Func(|vec| match vec[..] {
                [Int(a), Int(b)] => Ok(Int(a + b)),
                _ => Err(MalErr::WrongNumberOfArguments),
            }),
        ),
        (
            "-",
            Func(|vec| match vec[..] {
                [Int(a), Int(b)] => Ok(Int(a - b)),
                _ => Err(MalErr::WrongNumberOfArguments),
            }),
        ),
        (
            "*",
            Func(|vec| match vec[..] {
                [Int(a), Int(b)] => Ok(Int(a * b)),
                _ => Err(MalErr::WrongNumberOfArguments),
            }),
        ),
        (
            "/",
            Func(|vec| match vec[..] {
                [Int(a), Int(b)] => Ok(Int(a / b)),
                _ => Err(MalErr::WrongNumberOfArguments),
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
    ]
}
