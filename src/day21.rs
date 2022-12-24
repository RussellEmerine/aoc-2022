use crate::day21::Op::*;
use crate::day21::Yell::*;
use crate::Oopsie;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Hash)]
pub enum Yell {
    Number(i64),
    Op(Op, String, String),
}

impl FromStr for Yell {
    type Err = Oopsie;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<_> = s.split(' ').collect();
        if s.len() == 1 {
            Ok(Number(s[0].parse().map_err(|_| Oopsie)?))
        } else if s.len() == 3 {
            let a = s[0].to_string();
            let b = s[2].to_string();
            match s[1] {
                "+" => Ok(Op(Add, a, b)),
                "-" => Ok(Op(Sub, a, b)),
                "*" => Ok(Op(Mul, a, b)),
                "/" => Ok(Op(Div, a, b)),
                _ => Err(Oopsie),
            }
        } else {
            Err(Oopsie)
        }
    }
}

fn eval(monkeys: &HashMap<String, Yell>, cache: &mut HashMap<String, i64>, monkey: String) -> i64 {
    cache
        .get(&monkey)
        .copied()
        .unwrap_or_else(|| match &monkeys[&monkey] {
            &Number(x) => x,
            Op(op, a, b) => {
                let a = eval(monkeys, cache, a.clone());
                let b = eval(monkeys, cache, b.clone());
                match *op {
                    Add => a + b,
                    Sub => a - b,
                    Mul => a * b,
                    Div => a / b,
                }
            }
        })
}

pub fn solve(monkeys: &HashMap<String, Yell>) -> i64 {
    eval(monkeys, &mut HashMap::new(), "root".to_string())
}

fn check(monkeys: &HashMap<String, Yell>) -> bool {
    let mut uses = HashSet::new();
    for yell in monkeys.values() {
        match yell {
            Number(_) => {}
            Op(_, a, b) => {
                if uses.contains(a) {
                    return false;
                }
                uses.insert(a.clone());
                if uses.contains(b) {
                    return false;
                }
                uses.insert(b.clone());
            }
        }
    }
    true
}

#[derive(Clone, Debug)]
struct Root(Tree, Tree);

impl Root {
    fn solve(self) -> Option<i64> {
        let Root(mut a, mut b) = self;
        a.collapse();
        b.collapse();
        if let Tree::Number(a) = a {
            Some(b.solve(a))
        } else if let Tree::Number(b) = b {
            Some(a.solve(b))
        } else {
            None
        }
    }

    fn from_monkeys(monkeys: &HashMap<String, Yell>) -> Self {
        match &monkeys["root"] {
            Op(_, a, b) => Root(
                Tree::from_monkeys(monkeys, a),
                Tree::from_monkeys(monkeys, b),
            ),
            Number(_) => {
                panic!("no root?")
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Tree {
    Number(i64),
    Human,
    Op(Op, Box<Tree>, Box<Tree>),
}

impl Tree {
    fn from_monkeys(monkeys: &HashMap<String, Yell>, name: &str) -> Self {
        if name == "humn" {
            Tree::Human
        } else {
            match &monkeys[name] {
                Op(op, a, b) => Tree::Op(
                    *op,
                    Box::new(Tree::from_monkeys(monkeys, a)),
                    Box::new(Tree::from_monkeys(monkeys, b)),
                ),
                Number(x) => Tree::Number(*x),
            }
        }
    }

    fn collapse(&mut self) {
        match self {
            Tree::Op(op, a, b) => {
                a.collapse();
                b.collapse();
                if let Tree::Number(a) = a.as_ref() {
                    if let Tree::Number(b) = b.as_ref() {
                        *self = Tree::Number(match op {
                            Add => a + b,
                            Sub => a - b,
                            Mul => a * b,
                            Div => a / b,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    fn solve(&self, x: i64) -> i64 {
        match self {
            Tree::Human => x,
            Tree::Op(op, a, b) => {
                if let Tree::Number(a) = a.as_ref() {
                    b.solve(match op {
                        Add => x - a,
                        Sub => a - x,
                        Mul => x / a,
                        Div => a / x,
                    })
                } else if let Tree::Number(b) = b.as_ref() {
                    a.solve(match op {
                        Add => x - b,
                        Sub => x + b,
                        Mul => x / b,
                        Div => x * b,
                    })
                } else {
                    panic!()
                }
            }
            Tree::Number(_) => unreachable!(),
        }
    }
}

pub fn more(monkeys: &HashMap<String, Yell>) -> i64 {
    assert!(check(monkeys));
    let root = Root::from_monkeys(monkeys);
    root.solve().unwrap()
}
