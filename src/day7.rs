use std::fs;

fn calculate_constant(p1: i32, p2: i32) -> i32 {
    (p1 - p2).abs()
}

fn calculate_increase(p1: i32, p2: i32) -> i32 {
    let fsum =(p1 - p2).abs();

    fsum * (fsum +1)/2
}

fn main() {
    let content : Vec<i32> = fs::read_to_string("input/day7.txt")
        .expect("Something went wrong reading the file")
        .split(",")
        .map(|s| s.trim().to_string().parse::<i32>().unwrap())
        .collect();

    let min = *content.iter().min().unwrap();
    let max = *content.iter().max().unwrap();

    let p1: i32 = (min..max)
        .into_iter()
        .map(|i| content.iter().map(|position| calculate_constant(i, *position)).sum())
        .min()
        .unwrap();

    println!("p1: {}", p1);

    let p2: i32 = (min..max)
        .into_iter()
        .map(|i| content.iter().map(|position| calculate_increase(i, *position)).sum())
        .min()
        .unwrap();

    println!("p2: {}", p2);
}
