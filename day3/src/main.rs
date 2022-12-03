use std::{fs::File, io::{BufReader, Error, BufRead, stdin}};

fn get_priority(c: char) -> usize {
    assert!(c.is_ascii_alphabetic());
    match c {
        'a'..='z' => c as usize - 'a' as usize,
        'A'..='Z' => c as usize - 'A' as usize + 26,
        _ => panic!(),
    }
}

fn into_bits(s: &str) -> usize{
    let mut ret = 0;
    for c in s.chars() {
        let priority = get_priority(c);
        // println!("{c} {priority}");
        ret |= 1 << priority;
    }
    ret
}

fn from_bits(b: usize) -> usize {
    for i in 0..52usize {
        if (b >> i) & 1 == 1 {
            return i + 1;
        }
    }
    0
}

fn main() -> Result<(), Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut sum_prio = 0;
    let mut sum_prio2 = 0;
    let mut strike_count = 0;
    let mut three_elves_bit = usize::MAX;
    while let Some(Ok(line)) = lines.next() {
        let whole = into_bits(&line);
        // println!("line : {whole:#052b}");
        three_elves_bit = three_elves_bit & whole;
        strike_count += 1;
        if strike_count == 3 {
            // println!("{three_elves_bit:#052b}");
            sum_prio2 += from_bits(three_elves_bit);
            three_elves_bit = usize::MAX;
            strike_count = 0;
        }
        let (first, second) = line.split_at(line.len() / 2);
        let redundants = into_bits(first) & into_bits(second);
        // println!("{redundants:#052b}");
        sum_prio += from_bits(redundants);
    }
    println!("{sum_prio}");
    println!("{sum_prio2}");
    Ok(())
}
