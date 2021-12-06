use std::fs;

const DAYS: usize = 256;
const CYCLE: usize = 9;

fn main() {
    let content : Vec<i64> = fs::read_to_string("input/day6.txt")
        .expect("Something went wrong reading the file")
        .split(",")
        .map(|s| s.trim().to_string().parse::<i64>().unwrap())
        .collect();

    // initizalize fishes mapping
    let mut fishes = vec![0; CYCLE];
    for _i in 0..CYCLE {
        fishes.push(0);
    }
    for data in &content {
        fishes[*data as usize] += 1;
    }

    for _day in 1..=DAYS {
        let mut next = vec![0; CYCLE];
        next[6] = fishes[0];
        next[8] = fishes[0];
        for i in 1..CYCLE {
            next[i-1] += fishes[i];
        }
        fishes = next;
    }

    println!("{:?}", fishes.iter().sum::<i64>());
}

