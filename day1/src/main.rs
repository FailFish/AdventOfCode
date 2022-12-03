use std::{fs::File, io::{BufReader, BufRead, Error}};

fn main() -> Result<(), Error> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    // println!("{}", day1(&mut buf_reader)?);
    let v = day1(&mut buf_reader)?;
    println!("{:?}, max: {}, total: {}", v, v.last().unwrap(), v.iter().sum::<u64>());
    Ok(())
}

fn day1(buf_reader: &mut BufReader<File>) -> Result<Vec<u64>, Error> {
    let mut buf = String::new();
    let mut sum = 0u64;
    let mut maxsum = 0u64;
    let mut top_three: Vec<u64> = vec![0, 0, 0];
    loop {
        match buf_reader.read_line(&mut buf)? {
            0 => break,
            1 => {
                // update top_three
                maxsum = if maxsum > sum { maxsum } else { sum };
                update_n_top(sum, &mut top_three, 3);
                sum = 0;
            },
            _ => {
                sum += buf.trim_end().parse::<u64>().expect(&format!("parse error, input: {}", buf));
            }
        }
        buf.clear();
    }
    Ok(top_three)
}

fn update_n_top(new: u64, v: &mut Vec<u64>, n: usize) {
    v[0] = if new > v[0] { new } else { v[0] };
    for i in 0..n-1 {
        if v[i] > v[i + 1] {
            (v[i], v[i + 1]) = (v[i + 1], v[i]);
        }
    }
}
