use std::fs;

enum Line {
    Incomplete(i64),
    Corrupted(i32),
    Ok,
}

fn read_file(file: &str) -> Vec<String>{
    let mut lines = fs::read_to_string(file)
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    lines.pop();

    lines
}

fn solve_line(s: &str) -> Line{
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            _ => {
                let close = stack.pop().unwrap();
                if close != c {
                    match c {
                        ')' => return Line::Corrupted(3),
                        ']' => return Line::Corrupted(57),
                        '}' => return Line::Corrupted(1197),
                        '>' => return Line::Corrupted(25137),
                        _ => panic!("???"),
                    }
                }
            }
        }
    }

    if stack.is_empty() {
        return Line::Ok;
    }

    let mut score: i64 = 0;
    while !stack.is_empty() {
        match stack.pop().unwrap() {
            ')' => {score = score * 5 + 1;}, 
            ']' => {score = score * 5 + 2;}, 
            '}' => {score = score * 5 + 3;}, 
            '>' => {score = score * 5 + 4;}, 
            _ => panic!("???"),
        }
    }

    Line::Incomplete(score)
}

fn solve1(file: &str) -> i32 {
    let input = read_file(file);
    let mut sum = 0;
    for line in input {
        if let Line::Corrupted(score) = solve_line(&line) {
            sum += score;
        }
    }
    sum
}

fn solve2(file: &str) -> i64 {
    let input = read_file(file);
    let mut res = Vec::new();
    for line in input {
        if let Line::Incomplete(score) = solve_line(&line) {
            res.push(score);
        }
    }
    res.sort();

    println!("{:?}", res);
    res[res.len()/2]
}

fn main() {
    println!("p1: {}", solve1("input/day10.txt"));
    println!("p2: {}", solve2("input/day10.txt"));
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn part1() {
        assert_eq!(solve1("input/day10.test"), 26397);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2("input/day10.test"), 288957);
    }
}
