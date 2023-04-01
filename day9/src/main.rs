use std::{io::{BufReader, Lines, BufRead}, fs::File, path::Path, collections::HashMap, ops::Index};

#[derive(Debug)]
struct Command {
    direction: (i32, i32),
    distance: usize,

}

fn main() -> Result<(), std::io::Error> {
    let lines = read_lines("input")?;

    let commands = lines.map(|l| {
        let l = l.unwrap();
        let mut iter = l.split_ascii_whitespace();
        let (direction, distance) = (iter.next().unwrap(), iter.next().unwrap());
        let distance = distance.parse::<usize>().unwrap();
        let direction = match direction {
            "R" => (1, 0),
            "D" => (0, -1),
            "L" => (-1, 0),
            "U" => (0, 1),
            _ => panic!(),
        };
        Command { direction, distance }
    }).collect::<Vec<Command>>();

    let result = solve(&commands, 2);
    println!("p1 result: {}", result);

    let result = solve(&commands, 10);
    println!("p2 result: {}", result);

    Ok(())
}

fn solve(commands: &Vec<Command>, nknots: usize) -> usize {
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); nknots];
    let mut visited = HashMap::new();
    for command in commands {
        let distance = command.distance;
        for _ in 0..distance {
            let (mut dx, mut dy) = command.direction;
            knots[0].0 += dx;
            knots[0].1 += dy;

            let mut front_knot = knots[0];
            for knot in knots[1..].iter_mut() {
                let x_diff = front_knot.0 - knot.0;
                let y_diff = front_knot.1 - knot.1;
                if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
                    dx = 0;
                    dy = 0;
                } else if x_diff.abs() == 2 || y_diff.abs() == 2 {
                    dx = x_diff.signum();
                    dy = y_diff.signum();
                }

                knot.0 += dx;
                knot.1 += dy;

                front_knot = *knot;
            }
            visited.insert(front_knot, true);
        }
    }
    visited.len()
}



fn read_lines<P: AsRef<Path>>(path: P) -> Result<Lines<BufReader<File>>, std::io::Error> {
    let reader = BufReader::new(File::open(path)?);
    Ok(reader.lines())
}
