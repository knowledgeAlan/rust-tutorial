use rand::seq::SliceRandom;


const UPPER:&[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER:&[u8] = b"abcdefghijkmnopqrstuvwxyz"
const NUMBER:&[u8] = b"123456789"
const SYMBOL:&[u8] = b"!@#$%^&*_";


pub fn process_genpass (
    length:u8,
    upper:bool,
    lower:bool,
    number:bool,
    symbol:bool,
)-> anyhow::Result<String>{

    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let mut password:Vec<u8> = Vec::new();
    let mut chars:Vec<u8> = Vec::new();


    if upper {

        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("Upper won't be empty"));
    }



    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER wont be empty"));
    }


    if number {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("NUMBER wont be empty"));
    }


    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL wont be empty"));
    }


    for _ in 0..(length - password.len as u8){
        let c:&u8 = chars.choose(&mut rng).expect("Chars wont be empty in this context");
    }
}
