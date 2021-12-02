pub mod helper;

fn main() {

    if let Some(data) = helper::read_input_str_i(2) {
        // Part 1
        let mut horizontal = 0;
        let mut depth      = 0;
        for e in data.iter() {
            match (e.0.as_str(),e.1) {
                ("forward", x) => horizontal += x, 
                ("down", x)    => depth += x,
                ("up", x)      => depth -= x,
                _              => panic!("unparseable"),
            }
        }
        println!("part1: {}", horizontal * depth);

        // Part 2
        let mut aim = 0;
        horizontal  = 0;
        depth       = 0;
        for e in data.iter() {
            match (e.0.as_str(),e.1) {
                ("forward", x) => {
                    horizontal += x; 
                    depth += aim * x;
                },
                ("down", x)    => aim += x,
                ("up", x)      => aim -= x,
                _              => panic!("unparseable"),
            }
        }
        println!("part2: {}", depth * horizontal);

    }

}
