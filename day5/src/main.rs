use std::{io::{BufReader, BufRead}, fs::File, error::Error, str::FromStr};

struct Order {
    quantity: usize,
    from: usize,
    to: usize,
}

impl FromStr for Order {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = line.split_whitespace().collect();
        let from = splits[3].parse::<usize>()?;
        let to = splits[5].parse::<usize>()?;
        Ok(Order {
            quantity: splits[1].parse()?,
            from: from - 1,
            to: to - 1,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().map(|s| s.unwrap()).collect();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];

    for line in &lines[0..8] {
        let mut line_with_delim = String::new();
        for (i, c) in line.char_indices() {
            if i % 4 == 3 {
                line_with_delim.push('_');
            } else {
                line_with_delim.push(c);
            }
        }
        // println!("{line_with_delim}");
        let column: Vec<_> = line_with_delim.split_terminator('_').collect();
        for (i, &elem) in column.iter().enumerate() {
            let elem: Vec<_> = elem.chars().collect(); //trim_start_matches('[').trim_end_matches(']');
            if elem[1].is_alphabetic() {
                stacks[i].insert(0, elem[1]);
            }
        }
    }
    println!("{stacks:#?}");

    // solution for first prob
    // for line in &lines[10..] {
    //     let order = Order::from_str(line)?;
    //     for i in 0..order.quantity {
    //         let last = stacks[order.from].pop().unwrap();
    //         stacks[order.to].push(last);
    //     }
    // }
    // let lasts = stacks.iter().map(|v| v.last().unwrap()).collect::<String>();
    // println!("{lasts:?}");

    // solution for second prob
    for line in &lines[10..] {
        let order = Order::from_str(line)?;
        let len = stacks[order.from].len();
        let mut suffix = stacks[order.from].split_off(len - order.quantity);
        stacks[order.to].append(&mut suffix);
    }
    let lasts = stacks.iter().map(|v| v.last().unwrap()).collect::<String>();
    println!("{lasts:?}");
    Ok(())
}
