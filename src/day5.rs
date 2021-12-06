use std::fs;
use regex::Regex;

const WIDTH : usize = 1000;
const HEIGHT : usize = 1000;

fn idx(x: i32, y: i32) -> usize {
    (y as usize * WIDTH as usize) + x as usize
}

fn is_diagonal(p0: (&i32, &i32), p1:(&i32, &i32)) -> bool {
    (p0.0 - p1.0).abs() == (p0.1 - p1.1).abs()
}


fn ordered_pair<'a>(p0 : (&'a i32, &'a i32), p1 : (&'a i32, &'a i32)) -> ((&'a i32, &'a i32), (&'a i32, &'a i32)) {

    if p0.0 == p1.0 {
        if p0.1 <= p1.1 {
            return (p0, p1);
        } else {
            return (p1, p0);
        }
    }

    if p0.0 <= p1.0 { 
        return (p0, p1);
    }

    (p1, p0)
}

fn main() {
    let contents = fs::read_to_string("input/day5.txt")
        .expect("Something went wrong reading the file");

    let mut map = Vec::with_capacity(WIDTH * HEIGHT);
    for _i in 0..WIDTH*HEIGHT {
        map.push(0);
    }

    //let points = Vec::new();
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    for line in contents.split("\n") {
        if !re.is_match(line) {continue;}
        for cap in re.captures_iter(line) {
            let p1 = (&cap[1].parse::<i32>().unwrap(), &cap[2].parse::<i32>().unwrap());
            let p2 = (&cap[3].parse::<i32>().unwrap(), &cap[4].parse::<i32>().unwrap());

            let nod = p1.0 == p2.0 || p1.1 == p2.1;
            if nod {
                let pair = ordered_pair(p1, p2);
                println!("{:?}", pair);
                {
                    let p1 = pair.0;
                    let p2 = pair.1;

                    if p1.0 == p2.0 {
                        for i in *p1.1..=*p2.1 {
                            let p = (p1.0, i);
                            map[idx(*p.0, p.1)] += 1;
                        }
                    }
                    if p1.1 == p2.1 {
                        for i in *p1.0..=*p2.0 {
                            let p = (i, p1.1);
                            map[idx(p.0, *p.1)] += 1;
                        }
                    }
                }
            }

            if is_diagonal (p1, p2) {
                println!("{:?}", (p1,p2));
                let mut x_iter = 1;
                let mut y_iter = 1;

                if p1.0 > p2.0 {
                    x_iter = -1;
                }
                if p1.1 > p2.1 {
                    y_iter = -1;
                }

                let mut x = *p1.0;
                let mut y = *p1.1;
                while x != *p2.0 && y != *p2.1 {
                    map[idx(x, y)] += 1;
                    x += 1 * x_iter;
                    y += 1 * y_iter;
                }
                map[idx(x, y)] += 1;
            }
        }
    }

    let mut i = 0;
    let mut cnt = 0;
    for tile in map {
        cnt += (tile > 1) as i32;
        if tile == 0 {
            print!(".");
        } else {
            print!("{}", tile);
        }
        i+=1;
        if i % WIDTH == 0 {
            print!("\n");
        }
    }

    println!("p1: {}", cnt);
}
