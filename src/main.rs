use std::env;

fn print_fizz() {
    print!("fizz")
}
fn print_buzz() {
    print!("buzz")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nbr: i32 = args[1].parse().unwrap();

    if nbr % 3 == 0 {
        print_fizz();
    }

    if nbr % 5 == 0 {
       print_buzz();
    }
}

// write unit test