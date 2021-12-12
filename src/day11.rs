use std::fs;
use std::collections::VecDeque;

const MAPWIDTH: i32 = 10;
const MAPHEIGHT: i32 = 10;

fn idx(x: i32, y: i32) -> Option<usize> {
    let outer = x < 0 || x >= MAPWIDTH || y < 0 || y >= MAPHEIGHT;

    if outer {
        return None;
    }

    Some((y * MAPWIDTH + x) as usize )
}

fn read_file(file: &str) -> Vec<u32>{
    fs::read_to_string(file)
        .expect("Something went wrong reading the file")
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn neighbours(cur_x: i32, cur_y: i32) -> Vec<usize> {
    let mut adjacents = Vec::new();
    for y in -1..=1 {
        for x in -1..=1 {
            if let Some(i) = idx(cur_x + x, cur_y + y) {
                adjacents.push(i);
            }

        }
    }

    adjacents
}

fn tick(map: &mut Vec<u32>) -> usize {
    let mut queue = VecDeque::from((0..(MAPHEIGHT*MAPWIDTH) as usize).collect::<Vec<usize>>());
    while let Some(i) = queue.pop_front() {
        if map[i] == 10 { continue; }
        map[i] += 1;

        if map[i] == 10 {
            queue.extend(neighbours(i as i32 % MAPWIDTH, i as i32 / MAPWIDTH));
        }
    }

    map
    .iter_mut()
    .filter_map(|o| if *o == 10 { Some(*o = 0) } else { None})
    .count()
}

fn solve1() {
    let mut map   = read_file("input/day11.txt");

    let mut sum = 0;
    for _ in 0..100 {
        sum += tick(&mut map);
    }


    println!("p1: {:?}", sum);
}

fn solve2() {
    let mut map   = read_file("input/day11.txt");

    let mut i = 0;
    loop {
        i += 1;
        tick(&mut map);

        if map.iter().filter(|&o| *o == 0).count() == (MAPHEIGHT*MAPWIDTH) as usize {
            break;
        }
    }

    println!("p2: {:?}", i);
}


fn main() {
    solve1();
    solve2();
}
