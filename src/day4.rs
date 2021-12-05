use std::fs;


const WIDTH: usize = 5;
const HEIGHT: usize = 5;

fn main() {

    // Read file
    let contents = fs::read_to_string("input/day4.txt")
        .expect("Something went wrong reading the file");

    // Parsing
    let mut split = contents.split("\n");
    let mut numbers = Vec::new();
    for i in split.next().unwrap().split(",") {
        numbers.push(i);
    }

    let mut boards = Vec::new();
    let mut idx = 0;
    let mut board = Vec::new();
    for line in split {
        if line == ""{ continue; }

        for i in line.split(" ") {
            if i == "" { continue; }

            if idx % (WIDTH * HEIGHT) == 0  {
                if !board.is_empty() {
                    boards.push(board.clone());
                    board.clear()
                }
            }
            board.push(i);
            idx += 1;
        }
    }
    boards.push(board);

    let world_width = WIDTH;
    let world_height = boards.len() * HEIGHT;
    let size : usize= world_width * world_height;

    // Initialize marks
    let mut marks = Vec::with_capacity(world_width * world_height);
    for selected_board in &boards {
        for selection in selected_board {
            marks.push(false);
        }
    }

    let mut winners =  Vec::new();
    for pick in numbers {
        idx = 0;
        for selected_board in &boards {
            for selection in selected_board {
                //mark
                if pick == *selection {
                    if !marks[idx] {
                        marks[idx] = true;
                    }
                }

                idx += 1;
            }
        }


        let mut matched : bool;
        for nboard in 0..boards.len() {
            if winners.contains(&nboard) { continue; }
            for y in 0..HEIGHT {
                matched = true;
                for x in 0..WIDTH {
                    let real = ((y * WIDTH) + x) + (nboard * WIDTH * HEIGHT);
                    matched = matched && marks[real];
                    println!("{}", real);
                }

                if matched {
                    winners.push(nboard);
                    println!("tengo fila");
                    println!("{}", pick);
                    println!("tablon: {}", nboard);

                    idx = 0;
                    let mut sum = 0;
                    for winner in &boards[nboard]{
                        let real = idx + (nboard * (WIDTH * HEIGHT));

                        if !marks[real] {
                            sum += winner.parse::<i32>().unwrap();
                        }

                        idx += 1;

                    }
                    println!("p1: {}", sum * pick.parse::<i32>().unwrap());
                }
            }
        }

        for nboard in 0..boards.len() {
            if winners.contains(&nboard) { continue; }
            for x in 0..WIDTH {
                matched = true;
                for y in 0..HEIGHT {
                    let real = ((y * WIDTH) + x) + (nboard * WIDTH * HEIGHT);
                    matched &= marks[real];
                }

                if matched {
                    winners.push(nboard);
                    println!("tengo columna");
                    println!("{}", pick);
                    println!("tablon: {}", nboard);

                    idx = 0;
                    let mut sum = 0;
                    for winner in &boards[nboard]{
                        let real = idx + (nboard * (WIDTH * HEIGHT));

                        if !marks[real] {
                            sum += winner.parse::<i32>().unwrap();
                        }

                        idx += 1;

                    }
                    println!("p1: {}", sum * pick.parse::<i32>().unwrap());
                }
            }
        }
    }
}
