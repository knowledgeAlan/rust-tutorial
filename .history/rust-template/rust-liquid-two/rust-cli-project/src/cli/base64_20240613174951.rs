use std::{fmt,str::FromStr}
use clap::Parser;

use enum_dispatch::enum_dispatch;


use crate::CmDExcetor;

use super::verify_file;

#[derive(Debug,Parser)]
#[enum_dispatch(CmDExcetor)]
pub enum Base64SubCommand {

    #[command(name="encode",about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),

    #[command(name="decode",about = "Decode a base64 string to base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug,Parser)]
pub struct Base64EncodeOpts {
    
    #[arg(short,long,value_parser = verify_file,default_value ="-")]
    pub input: String,
    #[arg(long,value_parser = verify_file,default_value="standard")]
    pub format: Base64Format,
}

#[derive(Debug,Parser)]
pub struct Base64EncodeOpts {

    #[arg(short,long,value_parser = verify_file,default_value = "-")]
    pub input:String,

    #[arg(long,value_parser=parse_base64_format,default_value="standard")]
    pub format:Base64Format,
}

#[derive(Debug,Clone,Copy)]
pub enum Base64Format{

    Standard,
    UrlSafe,
}


fn parse_base64_format(format:&str)-> Result<Base64Format,anyhow::Error> {
    format.parse();
}
