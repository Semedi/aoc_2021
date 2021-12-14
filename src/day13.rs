use std::fs;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum Position {
    X(i32),
    Y(i32),
}

impl Position {
    fn fold(&self, dots: &mut [(i32, i32)]) {
        dots.iter_mut().for_each(|dot| match *self {
            Position::X(x) => dot.0 = if dot.0 < x { dot.0 } else { x + x - dot.0 },
            Position::Y(y) => dot.1 = if dot.1 < y { dot.1 } else { y + y - dot.1 },
        })
    }
}

fn read_file(filename: &str) -> (Vec<(i32, i32)>, Vec<Position>) {
    let re = Regex::new(r"^fold (\w+) (\w)=(\d+)$").unwrap();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut dots  = Vec::new();
    let mut folds = Vec::new();
    for line in contents.split("\n") {
        if line == "" {continue;}

        if re.is_match(line) {
            for cap in re.captures_iter(line) {
                if &cap[2] == "x" {
                    folds.push(Position::X(cap[3].parse::<i32>().unwrap()));
                } else {
                    folds.push(Position::Y(cap[3].parse::<i32>().unwrap()));
                }

            }
        } else {
            let dot = line
                .split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            dots.push((dot[0], dot[1]));
        }
    }

    (dots, folds)
}

fn solve1() {
    let (mut page, folds) = read_file("input/day13.txt");

    folds.first().unwrap().fold(&mut page);
    let n1 = page.into_iter().collect::<HashSet<_>>().len();
    println!("p1: {:?}", n1);
}

fn solve2() {
    let (mut page, folds) = read_file("input/day13.txt");
    folds.iter().for_each(|pos| pos.fold(&mut page));
    let (w, h) = page.iter().fold((0, 0), |(w, h), &dot| (w.max(dot.0), h.max(dot.1)));

    for y in 0..h+1{
        for x in 0..w+1{
            if page.contains(&(x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");

    }
}

fn main() {
    solve1();
    solve2();
}
