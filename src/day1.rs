pub mod helper;

fn main() {
    if let Some(data) = helper::read_input(1) {

        for number in data.iter() {
            println!("{}", number);
        }
    }
}
