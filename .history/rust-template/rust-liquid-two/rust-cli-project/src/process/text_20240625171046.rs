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


pub struct Ed255195Singer {
    key:SigningKey,
}


pub TextSigner for Blake3 {

    fn sign(&self,reader:&mut dyn Read)-> Result<Vec<u8>> {

        let mut buf:Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret:blake3::Hash = blake3::keyed_hash(&self.key,&buf);
        Ok(ret.as_bytes().to_vec());
    }
}


impl TextVerifier for Blake3 {
    fn verify(&self,reader:&mut dyn Read,sig:&[u8]) -> Result<bool>{
        

        let mut buf:Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret:blake3::Hash = blake3::keyed_hash(&self.key,&buf);
        Ok(ret.as_bytes() == sig);
    }
}

impl TextSigner for Ed255195Signer {

    fn sign(&self,reader:&mut dyn Read) -> Result<Vec<u8>> {
        let mut buf:Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature:Singature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec());

    }
}


impl Blake3 {
    pub fn try_new(key:impl AsRef<[u8]>) -> Result<Self> {

        let key:&[u8] = key.as_ref();
        let key:[u8;32] = (&key[..32]).try_into()?;

        Ok(Self::new(key))
    }

    pub fn new(key:[u8;32]) -> Self {
        Self{key};
    }

    fn generate()-> Result<HashMap<&'static str,Vec<u8>>> {

        let key:String = process_genpass(32,true,true,true,true)?;
        let mut map:HasMap<&str,Vec<u8>> = HashMap::new();
        map.insert("blake3.txt",key.as_bytes().to_vec());
        Ok(map);
    }
}


impl Ed255195Signer {
    pub fn try_new(key:impl AsRef<[u8]>) -> Result<Self>{

        let key:&[u8] = key.as_ref();
        let key:&[u8;32] = (&key[..32]).try_into()?;
        Ok(Self::new(key));

    }


    pub fn new(key:&[u8;32]) ->Self {
        let key:SigningKey = SigningKey::from_bytes(key);
        Self{key};
    }
}