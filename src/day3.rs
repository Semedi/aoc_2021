pub mod helper;
use std::collections::HashMap;


const NBYTES: usize = 12;

fn swap(e : char) -> char {
    if e == '0'{
        '1'

    } else {
        '0'
    }
}
fn most(elements: &Vec<char>, co2: bool) -> char {
    let ones  = elements.iter().filter(|&c| *c == '1').count();
    let zeros = elements.iter().filter(|&c| *c == '0').count();

    if ones > zeros {
       return '1';
    }
    
    if ones == zeros && co2 {
        return '1';

    }

    '0'
}

fn calculate_o2(map : &HashMap<usize, Vec<char>>) -> i32  {
    let mut n3 = Vec::new();
    let mut valid_positions = Vec::new();
    for i in 0..NBYTES {
        let mut ones  = Vec::new();
        let mut zeros = Vec::new();

        let column = map.get(&i).unwrap();
        for (i, e) in column.iter().enumerate() {
            if valid_positions.len() > 0 && !valid_positions.contains(&i) { continue;}
            if *e == '0' { zeros.push(i); }
            if *e == '1' { ones.push(i); }
        }

        // one is most
        if ones.len() >= zeros.len() {
            n3.push('1');
            valid_positions = ones;
        }
        // zero is most
        else if ones.len() < zeros.len() {
            n3.push('0');
            valid_positions = zeros;
        }
    }
    println!("{:?}", n3);

    to_dec(n3)
}

fn calculate_co2(map : &HashMap<usize, Vec<char>>) -> i32  {
    let mut n3 = Vec::new();
    let mut valid_positions = Vec::new();
    for i in 0..NBYTES {
        let mut ones  = Vec::new();
        let mut zeros = Vec::new();

        let column = map.get(&i).unwrap();
        for (idx, e) in column.iter().enumerate() {
            if valid_positions.len() == 0 || valid_positions.contains(&idx) {
                if *e == '1' {
                    ones.push(idx);
                } else {
                    zeros.push(idx);
                }
            }
        }
        // one is most
        if zeros.len() < ones.len() {
            println!("zeros: {}, ones: {}", zeros.len(), ones.len());
            println!("valid positions: {:?}", zeros);
            
            n3.push('0');
            valid_positions = zeros.clone();
        }
        // zero is most
        else if zeros.len() > ones.len() {
            println!("zeros: {}, ones: {}", zeros.len(), ones.len());
            println!("valid positions: {:?}", ones);
            n3.push('1');
            valid_positions = ones.clone();
        // equal
        } else {
            n3.push('0');
            valid_positions = zeros.clone();
        }
    }
    println!("{:?}", n3);

    to_dec(n3)
}

fn to_dec(bits : Vec<char>) -> i32 {
    let base : i32 = 2;
    let mut res = 0;
    for (i, bit) in bits.iter().rev().enumerate() {
        if *bit == '0'{ continue; }

        res += base.pow(i as u32);
    }

    res
}


fn main() {
    if let Some(data) = helper::read_input_raw(3) {

        let mut map = HashMap::new();
        for bits in data.iter() {
            for i in 0..NBYTES {
                if !map.contains_key(&i) {
                    map.insert(i, Vec::new());
                }
                map.get_mut(&i).unwrap().push(bits.chars().nth(i).unwrap());
            }
        }

        let mut n1 = Vec::new();
        for i in 0..NBYTES {
            let column = map.get(&i).unwrap();
            n1.push(most(column, true));
        }
        let n2 = n1.iter().map(|bit| swap(*bit)).collect();
        //.filter(|&(i, _)| i % 7 == 3 )

        println!("part 1: {}", to_dec(n1) * to_dec(n2));
        println!("part 2: {}",calculate_co2(&map) * calculate_o2(&map));
    }
}
