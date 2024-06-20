use crate::Base64Format;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD,URL_SAFE_NO_PAD},
    Engine as _,
};

use std::io::Read;

pub fn process_endcode(reader:&mut dyn Read,format:Base64Format) -> Result<String>{

    let mut buf:Vec<u8> = Vec::new();

    reader.read_to_end(&mut buf)?;
    
}

