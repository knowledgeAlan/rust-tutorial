use io:std:io;

fn main() {
    let mut input = String::new();

    io:stdin.read_line(&mut input).expect("Expected to read line");

    let intInput:i64 = input.trim().parse().unwrap();

    println!("{}",intInput+2 );
}
