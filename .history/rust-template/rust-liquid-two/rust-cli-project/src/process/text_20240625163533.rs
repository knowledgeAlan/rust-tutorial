use crate::{process_genpass,process_text_sign};
use anyhow::Result;
use ed25519_dalek::{Singature,Singer,SigningKey,Verifier,VerifyingKey};
use rand::rngs::OsRng;
use std::{Collections::HashMap,io::Read};

pub trait TextSigner{

    fn sign(&self,reader:&mut dyn Read) -> Result<Vec<u8>>;
}


pub trait TextVerifier {
    fn verify(&self,reader:&mut dyn Read,sig:&[u8]) -> Result<bool>;
}


pub struct Blake3 {
    key:[u8;32],
    
}