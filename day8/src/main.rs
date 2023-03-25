use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn read_lines(path: &str) -> Result<Lines<BufReader<File>>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    Ok(lines)
}

// fn mark_visibility(map: &mut Vec<Vec<_>>, xrange: Range, yrange: Range) {
// }

fn main() -> Result<(), std::io::Error> {
    let lines = read_lines("input")?;

    let mut forest = Vec::<Vec<usize>>::new();
    for line in lines.map(|l| l.unwrap()) {
        let mut row = vec![];
        line.chars()
            .for_each(|c| row.push(c.to_digit(10).unwrap() as usize));
        forest.push(row);
    }

    let (n, m) = (forest.len(), forest.first().unwrap().len());
    let mut visible = vec![vec![0usize; m]; n];
    visible.iter_mut().enumerate().for_each(|(i, row)| {
        if i == 0 || i == n - 1 {
            row.iter_mut().for_each(|e| *e = 1usize);
        } else {
            row[0] = 1;
            row[m - 1] = 1;
        }
    });

    let mut colmaxs = vec![0; m];
    for i in 0..n {
        let mut rowmax = 0;
        for j in 0..m {
            let current_height = forest[i][j];

            if current_height > rowmax || current_height > colmaxs[j] {
                visible[i][j] = 1;
                colmaxs[j] = current_height.max(colmaxs[j]);
                rowmax = current_height.max(rowmax);
            }
        }
    }

    let mut colmaxs = vec![0; m];
    for i in (0..n).rev() {
        let mut rowmax = 0;
        for j in (0..m).rev() {
            let current_height = forest[i][j];
            if current_height > rowmax || current_height > colmaxs[j] {
                visible[i][j] = 1;
                colmaxs[j] = current_height.max(colmaxs[j]);
                rowmax = current_height.max(rowmax);
            }
        }
    }

    println!(
        "{}",
        visible
            .iter()
            .map(|r| r.iter().sum::<usize>())
            .sum::<usize>()
    );

    // calculate scenic score
    let result2 = part_2_solution(&forest);
    //
    println!("{}", result2);

    Ok(())
    // abcde
    // fghij
    // jklmn
    //
    // i will be visible if height(i) is larger than max(f, g, h) or max(d, i)
}

fn part_2_solution(forest: &Vec<Vec<usize>>) -> usize {
    let (n, m) = (forest.len(), forest.first().unwrap().len());
    let mut scores = vec![vec![1usize; m]; n];
    calculate_total_score(&forest, &mut scores);
    print_2d_vector(&scores);
    scores
        .iter()
        .map(|l| *l.iter().max().unwrap())
        .max()
        .unwrap()
}

fn calculate_total_score(forest: &Vec<Vec<usize>>, scores: &mut Vec<Vec<usize>>) {
    let (n, m) = (forest.len(), forest.first().unwrap().len());
    for (i, line) in forest.iter().enumerate() {
        calculate_line_score(line, i, scores, false);
    }

    for j in 0..m {
        let vertical_line = forest.iter().map(|line| line[j]).collect::<Vec<usize>>();
        calculate_line_score(&vertical_line, j, scores, true);
    }
}

fn calculate_line_score(
    line: &Vec<usize>,
    pos: usize,
    scores: &mut Vec<Vec<usize>>,
    is_vertical: bool,
) {
    calculate_directional_score(&line, pos, scores, is_vertical, true);
    // print_2d_vector(scores);
    calculate_directional_score(&line, pos, scores, is_vertical, false);
    // println!("{} {}", is_vertical, is_forward);
    // print_2d_vector(scores);
}

fn calculate_directional_score(
    line: &Vec<usize>,
    i: usize,
    scores: &mut Vec<Vec<usize>>,
    is_vertical: bool,
    is_forward: bool,
) {
    let iterator: Box<dyn Iterator<Item = _>> = if is_forward {
        Box::new(line.iter().enumerate())
    } else {
        Box::new(line.iter().enumerate().rev()) // this reverses index too
    };

    let mut suffix = vec![];
    for (j, &height) in iterator {
        let current_score = calculate_element_score(&suffix, height);
        if is_vertical {
            scores[j][i] *= current_score;
        } else {
            scores[i][j] *= current_score;
        }
        suffix.push(height);
    }
}

fn calculate_element_score(suffix: &Vec<usize>, current_height: usize) -> usize {
    suffix.iter().rev().enumerate().find_map(|(idx, &past)| {
        if past >= current_height {
            Some(idx + 1)
        } else {
            None
        }
    }).unwrap_or(suffix.len())
}

fn print_2d_vector<T: Debug>(v: &Vec<Vec<T>>) {
    println!("");
    for i in v {
        println!("{:?}", i);
    }
    println!("");
}
