fn main() {
    println!("Hello, world!");

    let multi_number:i64 = calulate_number(32,45);

    println!("multi_number==={}",multi_number);

    let x = {
        let y = 10;
        y+1
    };

    println!("x==={}",x);
}


fn calulate_number(x:i64, y:i64) -> i64 {
    return x * y;
}