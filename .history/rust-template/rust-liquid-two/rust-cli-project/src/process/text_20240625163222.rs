use crate::{process_genpass,process_text_sign};
use anyhow::Result;
use ed25519_dalek::{Singature,Singer,SigningKey,Verifier,VerifyingKey};
