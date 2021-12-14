use std::fs;
use regex::Regex;

static WIDTH: i32 = 11;
static HEIGHT: i32 = 15;

#[derive(Debug, PartialEq)]
enum Position {
    X(i32),
    Y(i32),
}

fn idx(x: i32, y: i32) -> usize {
    (y * WIDTH + x) as usize
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


fn fold_y(page: &Vec<(i32, i32)>, render: Vec<String>, fold: i32) -> Vec<String> {
    let mut result = Vec::with_capacity(render.len()/2);
    for y in 0..HEIGHT{
        for x in 0..WIDTH {
            if fold == y { continue; }
            if fold <= y {
                let new_y = (y - 2 * fold).abs();
                if render[idx(x, new_y)] == "#".to_string() && page.contains(&(x,y)) {
                    result[idx(x, new_y)] = ".".to_string();

                } else if render[idx(x, new_y)] == "." && page.contains(&(x,y)) {
                    result[idx(x, new_y)] = "#".to_string();
                } 
                continue;
            }

            if page.contains(&(x,y)) {
                result.push(String::from("#"));

            } else {
                result.push(".".to_string());
            }
        }
    }

    HEIGHT = HEIGHT / 2;
    result
}

fn fold_x(page: &Vec<(i32, i32)>, render: Vec<String>, fold: i32) -> Vec<String> {
    let mut result = Vec::with_capacity(render.len()/2);
    for x in 0..WIDTH{
        for y in 0..HEIGHT {
            if fold == x { continue; }
            if fold <= x {
                let new_x = (y - 2 * fold).abs();
                if render[idx(new_x, y)] == "#".to_string() && page.contains(&(x,y)) {
                    result[idx(new_x, y)] = ".".to_string();

                } else if render[idx(new_x, y)] == "." && page.contains(&(x,y)) {
                    result[idx(new_x, y)] = "#".to_string();
                } 
                continue;
            }

            if page.contains(&(x,y)) {
                result.push(String::from("#"));

            } else {
                result.push(".".to_string());
            }
        }
    }

    WIDTH = WIDTH / 2;

    result
}

fn main() {
    let (page, folds) = read_file("input/day13.test");

    println!("{:?}", folds);
    let mut render = Vec::new();

    // init render
    for y in 0..HEIGHT{
        for x in 0..WIDTH {
            let glyph =  {
                if page.contains(&(x,y)) {
                    "#"
                } else {
                    "."
                }
            };
            render.push(glyph.to_string());
        }
    }

    let new_render =  {
        let mut prev = render;
        for fold in folds {
            if let Position::Y(f) = fold {

                prev = fold_y(&page, prev, f);
            } else if let Position::X(f) = fold {
                prev = fold_x(&page, prev, f);
            }
        }

        prev
    };


    for (i, tile) in new_render.iter().enumerate() {
        print!("{}", tile);
        if (i as i32 + 1) % WIDTH == 0 {
            println!("");
        }
    }
    //if let Position::Y(fold) = folds[0] {
    //    let new_render = fold_y(page, render, fold);
    //    for (i, tile) in new_render.iter().enumerate() {
    //        print!("{}", tile);
    //        if (i as i32 + 1) % WIDTH == 0 {
    //            println!("");
    //        }
    //    }
    //}
}
