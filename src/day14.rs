use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn read_file(filename: &str) -> (String, HashMap<String,String>) {
    let head = Regex::new(r"^(\w+)$").unwrap();
    let pair = Regex::new(r"^(\w{2}) -> (\w)$").unwrap();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut pairs    = HashMap::new();
    let mut template = String::new();
    for line in contents.split("\n") {
        if head.is_match(line) {
            head.captures_iter(line).for_each(
                |cap| {
                    template = cap[1].to_string();
                }
            );
        }

        if pair.is_match(line) {
            pair.captures_iter(line).for_each(
                |cap| {
                    let couple = &cap[1];
                    let target = &cap[2];

                    pairs.entry(couple.to_string()).or_insert(target.to_string());
                }
            );
        }
    }

    (template, pairs)
}

fn main() {
    let (template, rules) = read_file("input/day14.test");

    let inter = template.chars().collect::<Vec<char>>();
    let mut pairs = inter.windows(2);


    println!("{:?}", pairs.next().unwrap());
    println!("{:?}", pairs.next().unwrap());
    println!("{:?}", pairs.next().unwrap());
}
