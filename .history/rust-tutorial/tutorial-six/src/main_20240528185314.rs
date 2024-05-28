fn main() {
    println!("Hello, world!");

    let multiNumber:i64 = calulate_number(32,45);

    println!("multiNumber==={}",multiNumber);
}


fn calulate_number(x:i64, y:i64) -> i64 {
    return x * y;
}