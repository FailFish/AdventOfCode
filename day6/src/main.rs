use std::{fs::File, io::{BufReader, BufRead, stdin}};

fn main() -> Result<(), std::io::Error>{
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let lines : Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let answer: Vec<usize> = lines.iter().map(|l| {
        find_marker(l, 14)
    }).collect();
    println!("{:?}", answer);
    Ok(())
}

fn find_marker(l: &str, n: usize) -> usize {
    let mut save_four = Vec::new();
    for (i, cb) in l.chars().map(|c| 1 << c as usize - 'a' as usize).enumerate() {
        save_four.insert(0, cb);
        if save_four.len() > n { save_four.pop(); }
        if bit_count(save_four.iter().fold(0, |accu, x| accu | x)) == n {
            return i + 1;
        }
    }
    0
}

fn bit_count(item: u64) -> usize {
    let mut count = 0;
    for i in 0..27 {
        if item & (1 << i) > 0 { count += 1; }
    }
    count
}
