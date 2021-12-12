use std::fs;
use std::collections::{HashMap, HashSet};
use regex::Regex;
use std::{thread, time};

const ENTRANCE: usize = 0;
const EXIT: usize = 1;

macro_rules! map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}


fn count_paths(caves: &HashMap<String, HashSet<String>>, current: &str, visited: &mut HashSet<String>) -> usize {
    if current == "end" {
        return 1;
    }

    let mut n = 0;
    let paths = caves.get(current).unwrap();
    for path in paths {
        let is_small = &path.to_ascii_lowercase() == path; 

        if is_small {
            if visited.contains(path) { continue; }
            else {
                visited.insert(path.to_string());
            }
        }


        n += count_paths(caves, path, visited);

        visited.remove(path);
    }

    n
}

fn count_paths2(caves: &HashMap<String, HashSet<String>>, current: &str, visited: &mut HashMap<String, usize>) -> usize {
    if current == "end" {
        return 1;
    }

    let mut n = 0;
    let paths = caves.get(current).unwrap();
    for path in paths {
        let is_small = &path.to_ascii_lowercase() == path; 

        if is_small {

            if visited.contains_key(path) {
                if visited[path] == 2 {
                    continue;
                } else {

                    if path != "start" && path != "end" {
                        visited.insert(path.to_string(), 2); 
                    }
                }

            } else {
                visited.insert(path.to_string(), 1);
            }
        }

        n += count_paths2(caves, path, visited);

        visited.remove(path);
    }

    n
}

fn read_file(filename: &str) -> HashMap<String, HashSet<String>> {
    let re = Regex::new(r"-").unwrap();

    let mut caves = HashMap::new();

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    for line in contents.split("\n") {
        if !re.is_match(line) {continue;}

        let path: Vec<&str> = re.split(line).collect();
        for kind in ENTRANCE..=EXIT {
            let connection = caves
                .entry(path[kind].to_owned())
                .or_insert_with(HashSet::new);

            if kind == ENTRANCE {connection.insert(path[EXIT].to_owned());}
            if kind == EXIT     {connection.insert(path[ENTRANCE].to_owned());}
        }
    }

    caves
}

fn main() {
    let caves = read_file("input/day12.txt");
    let n = count_paths(&caves, "start", &mut HashSet::from(["start".to_string()]));

    //let n2 = count_paths2(&caves, "start", &mut map!["start".to_string() => 2]);

    println!("part 1: {}", n);
    //println!("part 2: {}", n2);

}
