pub mod helper;

fn main() {
    if let Some(data) = helper::read_input(1) {

        let mut prev = -1; 
        let mut sum : i32 = 0;
        let mut cnt = -1;

        // procedural way
        for (i, number) in data.iter().enumerate() {
            if i % 3 == 0 {
                if sum > prev {
                    cnt+=1;
                }

                prev = sum;
                sum = 0;
            }
            sum = sum + number;
        }
        println!("p2: {}", cnt);

        // functional way
        let p2 = data.iter().fold(
            (
                0 as i32,
                (i32::max_value(), i32::max_value(), i32::max_value()),
            ),
            |(cnt, (prev, b, c)), next| 
                (
                    cnt + (prev < *next) as i32 ,(b, c, *next)
                ),
        )
        .0;

        println!("p2: {}", p2);

        let t = [0,2,3,4,5]
        .iter()
        .fold(0, |total, next| total + next);

        println!("test {}", t);
    }
}
