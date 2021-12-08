use std::fs;
use std::collections::HashMap;

fn sort(chain: &str) -> String {
    let mut l: Vec<char> = chain.chars().collect();
    l.sort_unstable();
    
    l.into_iter().collect()
}

fn new_map(input: &Vec<String>, unique_segments: &Vec<usize>) -> HashMap<usize, String> {
    let mut map = HashMap::new();
    for d in input {
        let n = d.len();
        if unique_segments.contains(&n) {
            let digit = unique_segments.iter().position(|&r| r == n).unwrap();
            map.insert(digit, sort(&d));
        } 
    }
    map
}

fn revert_map(map: HashMap<usize, String>) -> HashMap<String, usize> {

    let mut new = HashMap::new();
    for (key, val) in map.iter() {
        new.insert(val.clone(), *key);
    }

    new
}

fn main() {
    // AWFUL PARSING
    let mut content : Vec<String> = fs::read_to_string("input/day8.txt")
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(|s| s.trim().to_string())
        .collect();

    content.pop();

    let mut segments = Vec::new();
    for line in content {
        let split: Vec<String>= line.split(" | ").map(|s| s.trim().to_string()).collect();

        let input : Vec<String> = split[0].split(" ").map(|s| s.trim().to_string()).collect();
        let output : Vec<String> = split[1].split(" ").map(|s| s.trim().to_string()).collect();

        segments.push((input, output));
    }

    let unique_segments = vec![2,3,4,7];
    let mut counter = vec![0; 10];
    for line in &segments {
        let output = &line.1;

        for d in output {
            let n = d.len();
            if unique_segments.contains(&n) {
                counter[n] +=1;
            }
        }
    }

    // unique_segmets(i) = digit
    let unique_segments = vec![0,2,0,0,4,0,0,3,7];
    println!("p1: {}", counter.iter().sum::<i32>());

    let mut sum = 0;
    for line in segments {
        let input = line.0;
        let output = line.1;

        let mut map = new_map(&input, &unique_segments);
        for d in input {
            let n = d.len();
            let ordered = sort(&d);
            if !unique_segments.contains(&n) {
                match n {
                    5 => {
                        if map[&7].chars().fold(true, |r, c| r & ordered.contains(c)) {
                            map.insert(3, ordered);
                        } else if ordered.chars().fold(0, |sum, c| sum + (map[&4].contains(c)) as i32) == 3 {
                            map.insert(5, ordered);
                        } else {
                            map.insert(2, ordered);
                        }
                    }
                    6 => {
                        if map[&4].chars().fold(true, |r, c| r & ordered.contains(c)) {
                            map.insert(9, ordered);

                        } else if map[&7].chars().fold(true, |r, c| r & ordered.contains(c)) {
                            map.insert(0, ordered);
                        } else {
                            map.insert(6, ordered);
                        }
                    }
                    _ => {panic!("impossible situation")}
                }
            }
        }

        let new_map = revert_map(map);

        for (i, d) in output.iter().rev().enumerate() {
            let ordered = sort(&d);

            sum += new_map[&ordered] as i32 * i32::pow(10, i as u32);
        }
    }
    println!("p2: {}", sum);
}
