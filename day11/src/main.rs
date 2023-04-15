use anyhow::{anyhow, Result};
use regex::Regex;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone, Debug)]
struct Monkey {
    id: usize,
    start_items: VecDeque<usize>,
    operator: Operator,
    operand: Option<usize>,
    test: ThrowAction,
    inspect_count: usize,
}

impl Monkey {
    fn from_lines(lines: &[String]) -> Result<Self> {
        let pattern = r"Monkey (?P<id>\d+):
  Starting items: (?P<items>(\d+, )*\d+)
  Operation: new = old (?P<operator>\*|/|-|\+) (?P<operand>\d+|old)
  Test: divisible by (?P<div>\d+)
    If true: throw to monkey (?P<true>\d+)
    If false: throw to monkey (?P<false>\d+)";

        let input = lines.join("\n");
        let re = Regex::new(pattern).unwrap();
        let captures = re.captures(&input).unwrap();

        match (
            captures
                .name("id")
                .map(|m| m.as_str().parse())
                .transpose()?,
            captures.name("items").map(|m| m.as_str()),
            captures.name("operator").map(|m| m.as_str()),
            captures.name("operand").map(|m| m.as_str()),
            captures
                .name("div")
                .map(|m| m.as_str().parse())
                .transpose()?,
            captures
                .name("true")
                .map(|m| m.as_str().parse())
                .transpose()?,
            captures
                .name("false")
                .map(|m| m.as_str().parse())
                .transpose()?,
        ) {
            (
                Some(id),
                Some(items),
                Some(operator),
                Some(operand),
                Some(divisor),
                Some(id_true),
                Some(id_false),
            ) => {
                let operator = Operator::from_input(operator)?;
                let operand = if operand == "old" {
                    None
                } else {
                    Some(operand.parse()?)
                };
                let target = (id_true, id_false);
                let start_items = items
                    .split(',')
                    .map(|s| s.trim().parse::<usize>())
                    .collect::<Result<VecDeque<usize>, _>>()?;
                Ok(Self {
                    id,
                    start_items,
                    operator,
                    operand,
                    test: ThrowAction { divisor, target },
                    inspect_count: 0,
                })
            }
            _ => Err(anyhow!("Error in parsing: {}", input)),
        }
    }

    fn inspect(&mut self, div: usize) -> Option<(usize, usize)> {
        self.inspect_count += 1;
        self.start_items.pop_front().and_then(|i| {
            let x = if let Some(x) = self.operand { x } else { i };
            let worry = self.operator.calculate(i, x);
            /* println!(
                "
  Monkey {} inspects an item with a worry level of {}.
    Worry level is multiplied by {} to {}.
    Monkey gets bored with item. Worry level is divided by 3 to {}.
    Current worry level is {} divisible by {}.
    Item with worry level 500 is thrown to monkey {}.
    ",
    self.id,
                i,
                x,
                worry,
                worry / 3,
                if worry / 3 % self.test.divisor == 0 {
                    "not"
                } else {
                    ""
                },
                self.test.divisor,
                if worry / 3 % self.test.divisor == 0 {
                    self.test.target.0
                } else {
                    self.test.target.1
                }
            ); */

            let worry = worry % div;

            if worry % self.test.divisor == 0 {
                Some((self.test.target.0, worry))
            } else {
                Some((self.test.target.1, worry))
            }
        })
    }

    fn receive(&mut self, item: usize) {
        self.start_items.push_back(item);
    }
}

#[derive(Clone, Copy, Debug)]
struct ThrowAction {
    divisor: usize,
    target: (usize, usize),
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn from_input(ops: &str) -> Result<Self> {
        match ops {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            _ => Err(anyhow!("invalid operator: {}", ops)),
        }
    }

    fn calculate(&self, old: usize, x: usize) -> usize {
        match self {
            Operator::Add => old + x,
            Operator::Sub => old - x,
            Operator::Mul => old * x,
            Operator::Div => old / x,
        }
    }
}

fn main() -> Result<()> {
    let lines = read_lines("input")?;
    let nmonkey = (lines.len() + 1) / 7;
    let mut monkeys: Vec<RefCell<Monkey>> = vec![];
    for i in 0..nmonkey {
        let monkey = Monkey::from_lines(&lines[i * 7..i * 7 + 6])?;
        monkeys.push(RefCell::new(monkey));
    }
    let monkeys2 = monkeys.clone();
    let result = solve(&monkeys, 3, 20);
    println!("{result}");
    let div: usize = monkeys2.iter().map(|m| m.borrow().test.divisor).product();
    let result = solve(&monkeys2, div, 10000);
    println!("{result}");
    Ok(())
}

fn solve(monkeys: &Vec<RefCell<Monkey>>, div: usize, repeats: usize) -> usize {
    for _ in 0..repeats {
        // println!("{monkeys:?}");
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].borrow_mut();
            let nitems = monkey.start_items.len();
            for _ in 0..nitems {
                let (idx, item) = match monkey.inspect(div) {
                    Some(p) => p,
                    _ => break,
                };
                monkeys[idx].borrow_mut().receive(item);
            }
        }
    }

    let mut toptwo = vec![0, 0];
    for monkey in monkeys.iter() {
        let inspect_count = monkey.borrow().inspect_count;
        print!("{}, ", inspect_count);
        if toptwo[0] < inspect_count {
            toptwo[1] = toptwo[0];
            toptwo[0] = inspect_count;
        } else if toptwo[1] < inspect_count {
            toptwo[1] = inspect_count;
        }
    }
    toptwo[0] * toptwo[1]
}

fn read_lines<P: AsRef<Path>>(filename: P) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}
