use std::{path::{Path}, io::{self, BufReader, BufRead}, fs::{File}};

#[derive(Debug)]
struct Machine {
    x: isize,
    cycle: usize,
    signal_strength: isize,
}
impl Machine {
    fn update(&mut self, dx: isize) {
        self.cycle += 1;
        self.x += dx;
        // println!("{}, {}", self.cycle, self.x);
        if self.cycle % 40 == 20 {
            self.signal_strength += self.cycle as isize * self.x;
        }
    }
}

#[derive(Debug)]
struct Machine2 {
    x: isize,
    cycle: usize,
}
impl Machine2 {
    fn update(&mut self, dx: isize) -> bool {
        let is_draw = if ((self.cycle as isize - 1) % 40 - self.x % 40).abs() <= 1 { true } else { false };
        self.cycle += 1;
        self.x += dx;
        is_draw
    }
}

#[derive(Debug)]
struct Job {
    cycle_left: usize,
    added: isize,
}

impl Job {
    fn new(s: &str) -> Result<Job, io::Error> {
        let mut tokens = s.split_whitespace();
        let e = io::Error::new(io::ErrorKind::InvalidData, "invalid CPU instruction");
        let cycle_left = match tokens.next() {
            Some("addx") => 2,
            Some("noop") => return Ok(Job { cycle_left: 1, added: 0 }),
            _ => return Err(e),
        };
        let added = tokens.next().ok_or(e)?.parse::<isize>().unwrap();
        Ok(Job {
            cycle_left,
            added
        })
    }

    fn perform(&mut self, state: &mut Machine) {
        let mut added = 0;
        self.cycle_left -= 1;
        if self.cycle_left == 0 {
            added = self.added;
        }
        state.update(added)
    }

    fn perform2(&mut self, state: &mut Machine2) -> bool {
        let mut added = 0;
        self.cycle_left -= 1;
        if self.cycle_left == 0 {
            added = self.added;
        }
        state.update(added)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_lines("input")?;
    let result = solve_p1(&lines)?;
    println!("problem 1: {result}");
    println!("problem 2");
    solve_p2(&lines)?;
    Ok(())
}

fn solve_p1(lines: &Vec<String>) -> Result<isize, io::Error> {
    let mut state = Machine {
        x: 1,
        cycle: 1,
        signal_strength: 0
    };
    for line in lines {
        let mut job = Job::new(&line)?;
        while job.cycle_left != 0{
            job.perform(&mut state);
        }
    }
    Ok(state.signal_strength)
}

fn solve_p2(lines: &Vec<String>) -> Result<(), io::Error> {
    let mut state = Machine2 {
        x: 1,
        cycle: 1,
    };
    let mut crt = vec![];
    for line in lines {
        let mut job = Job::new(&line)?;
        while job.cycle_left != 0{
            crt.push(job.perform2(&mut state));
        }
    }
    print_crt(&crt);
    Ok(())
}

fn print_crt(crt: &Vec<bool>) {
    let crt: String = crt.iter().map(|&p| if p { '#' } else { '.' }).collect();
    for i in 0..6 {
        let start = 40 * i;
        let end = start + 40;
        println!("{}", &crt[start..end]);
    }
}

fn read_lines<P: AsRef<Path>>(filename: P) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}
