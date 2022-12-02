use std::{fs::File, io::{BufReader, BufRead, Error}};

fn main() -> Result<(), Error> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut buf = String::new();
    let mut sum = 0u64;
    let mut maxsum = 0u64;
    loop {
        match buf_reader.read_line(&mut buf)? {
            0 => break,
            1 => {
                maxsum = if maxsum > sum { maxsum } else { sum };
                sum = 0;
            },
            _ => sum += buf.trim_end().parse::<u64>().expect(&format!("buf: {}", buf)),
        }
        buf.clear();
    }
    println!("{}", maxsum);
    Ok(())
}
