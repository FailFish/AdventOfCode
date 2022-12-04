use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    ops::RangeInclusive,
};

fn is_overlap(a: RangeInclusive<usize>, b: RangeInclusive<usize>) -> bool {
    if a.start() <= b.start() && b.end() <= a.end() || b.start() <= a.start() && a.end() <= b.end()
    {
        true
    } else if a.start() <= b.start() && b.start() <= a.end() || b.start() <= a.start() && a.start() <= b.end() {
        true
    } else {
        false
    }
}

fn main() -> Result<(), Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>();
    let mut total = 0;
    for line in lines {
        let (first, second) = line.split_once(',').unwrap();
        let (start, end) = first.split_once('-').unwrap();
        let first_range = RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap());
        let (start, end) = second.split_once('-').unwrap();
        let second_range = RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap());

        println!("{:?} {:?}", first_range, second_range);

        if is_overlap(first_range, second_range) {
            total += 1;
        }
    }
    println!("{total}");
    Ok(())
}
