use crate::{process_genpass,process_text_sign};
use anyhow::Result;
use ed25519_dalek::{Singature,Singer,SigningKey,Verifier,VerifyingKey};
use rand::rngs::OsRng;
use std::{Collections::HashMap,io::Read};