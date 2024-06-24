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


    
}
