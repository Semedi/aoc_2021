use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_input(day: i64) -> Option<Vec<i32>>{

    let mut data = Vec::new();

    if let Ok(lines) = read_lines(format!("input/day{}.txt", day)) {
        for line in lines {
            if let Ok(value) = line {
                data.push(value.parse::<i32>().unwrap());
            }
            else {
                println!("fail read lines!");
                return None;
            }
        }
    } else {
        return None;
    }

    Some(data)
}

pub fn read_input_str_i(day: i64) -> Option<Vec<(String, i32)>>{

    let mut data = Vec::new();

    if let Ok(lines) = read_lines(format!("input/day{}.txt", day)) {
        for line in lines {
            if let Ok(value) = line {
                let split = value.split(" ").collect::<Vec<&str>>();
                data.push((String::from(split[0]), split[1].parse::<i32>().unwrap()));
            }
            else {
                println!("fail read lines!");
                return None;
            }
        }
    } else {
        return None;
    }

    Some(data)
}
