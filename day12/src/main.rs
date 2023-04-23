use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

type height = usize;
type cord = (usize, usize);

struct HeightMap {
    hmap: Vec<Vec<height>>,
    nrow: usize,
    ncol: usize,
}

const FOUR_DIRECTION: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() -> Result<(), Error> {
    let lines = read_lines("input")?;

    let result = solve_p1(&lines);
    println!("{result}");
    let result = solve_p2(&lines);
    println!("{result}");

    Ok(())
}

fn read_lines<P: AsRef<Path>>(filename: P) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn solve_p1(lines: &Vec<String>) -> usize {
    let (heightmap, starts, end) = read_map(lines);
    let start = starts.range(0..1).copied().collect();
    let visitmap = vec![vec![false; heightmap.ncol]; heightmap.nrow];
    visit(heightmap, start, visitmap, end).unwrap()
}

fn solve_p2(lines: &Vec<String>) -> usize {
    let (heightmap, starts, end) = read_map(lines);
    let visitmap = vec![vec![false; heightmap.ncol]; heightmap.nrow];
    visit(heightmap, starts, visitmap, end).unwrap()
}

fn visit(hmap: HeightMap, mut queue: VecDeque<(cord, usize)>, mut vmap: Vec<Vec<bool>>, end: cord) -> Option<usize> {
    let HeightMap { hmap, nrow, ncol } = hmap;
    for (start, _) in queue.iter() {
        vmap[start.0][start.1] = true;
    }
    while let Some((current, dist)) = queue.pop_front() {
        // println!("{current:?}, {dist}");
        if current == end {
            return Some(dist);
        }
        for (dx, dy) in FOUR_DIRECTION {
            if let (Some(nx), Some(ny)) = (
                checked_add_signed(current.0, dx),
                checked_add_signed(current.1, dy),
            ) {
                if nx < nrow
                    && ny < ncol
                    && !vmap[nx][ny]
                    && hmap[nx][ny] <= 1 + hmap[current.0][current.1]
                {
                    vmap[nx][ny] = true;
                    queue.push_back(((nx, ny), dist + 1));
                }
            }
        }
    }
    None
}

fn read_map(lines: &Vec<String>) -> (HeightMap, VecDeque<(cord, usize)>, cord) {
    let mut start: cord = (0, 0);
    let mut starts: VecDeque<(cord, usize)> = VecDeque::new();
    let mut end: cord = (0, 0);
    let (nrow, ncol) = (lines.len(), lines[0].len());
    let mut hmap = vec![vec![0; ncol]; nrow];
    for (i, line) in lines.iter().enumerate() {
        for (j, height) in line.chars().enumerate() {
            if height == 'S' {
                start = (i, j);
            } else if height == 'a' {
                starts.push_back(((i, j), 0));
                hmap[i][j] = 0;
            } else if height == 'E' {
                end = (i, j);
                hmap[i][j] = 'z' as height - 'a' as height;
            } else {
                hmap[i][j] = height as height - 'a' as height;
            }
        }
    }
    starts.push_front((start, 0)); // 'S' at the front

    (HeightMap { hmap, nrow, ncol }, starts, end)
}

fn checked_add_signed(lhs: usize, rhs: isize) -> Option<usize> {
    if rhs < 0 {
        let rhs = -rhs as usize;
        lhs.checked_sub(rhs)
    } else {
        lhs.checked_add(rhs as usize)
    }
}
