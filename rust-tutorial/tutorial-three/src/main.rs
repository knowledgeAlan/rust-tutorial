fn main() {
    let x = 4;

    println!("x is {}", x);

    {
        let x = x-2;
        println!("x: is {}", x);
    }

    let x = x+1;

    println!("x: is {}", x);

    const TEST_CONST:u32 = 89;

    println!("TEST_CONST: is {}", TEST_CONST);
}
