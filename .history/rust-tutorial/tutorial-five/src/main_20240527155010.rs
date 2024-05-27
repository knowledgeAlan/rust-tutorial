use std::io;

fn main() {
    let mut input = String::new();

    io::stdin.read_lines(&mut input).expect("Expected to read line");

    let intInput:i64 = input.trim().parse().unwrap();

    println!("{}",intInput+2 );
}
