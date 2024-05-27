use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Expected to read line");

    let int_input:i64 = input.trim().parse().unwrap();

    println!("{}",int_input+2 );

    let x:i32 = 123;
    let y:i32 = 45;

    println!("{}",x+y);
}
