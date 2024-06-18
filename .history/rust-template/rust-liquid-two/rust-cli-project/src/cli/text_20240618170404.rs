use crate::{
    get_content,get_reader,process_text_key_generate,process_text_sign,process_text_verify,
    CmdExector,
}

use super::{verify_file,verify_path};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD,Engine};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::{fmt,path::PathBuf,str::FromStr};
use tokio::fs;

#[derive(Debug,Parser)]
#[enum_dispatch(CmdExector)]

pub enum TextSubCommand {

    #[command(about="Sign a text with a private/session key and return a signature")]
    Sign(TextSignOpts),

    #[command(about ="Verify a signature with a public/session key")]
    Verify(TextVerifyOpts),

    #[command(about="Generate a random blake3 key or ed25519 key pair")]
    Generate(KeyGenerateOpts),
}


#[derive(Debug,Parser)]
pub struct TextSignOpts {


    #[arg(short, long,value_parser = verify_file,default_value="-")]
    pub input:String,

    #[arg(short,long,value_parser = verify_file)]
    pub key:String,

    #[arg(long,default_value="blake3",value_parser = parse_text_sign_format)]
    pub format:TextSignFormat,
}


#[derive(Debug,Parser)]
pub struct TextVerifyOpts {


    #[arg(short,long,value_parser=verify_file,default_value="-")]
    pub input:String,

    #[arg(short,long,value_parser=verify_file)]
    pub key:String,

    #[arg(long)]
    pub sig:String,

    #[arg(long,default_value="blake3",value_parser=parse_text_sign_format)]
    pub format:TextSignFormat,
}


#[derive(Debug,Parser)]
pub struct KeyGenerateOpts {

    #[arg(long,default_value="blake3",value_parser=parse_text_sign_format)]
    pub format:TextSignFormat,

    #[arg(short,long,value_parser=verify_path)]
    pub output_path:PathBuf,
}

#[derive(Debug,Clone,Copy)]
pub enum TextSignFormat {

        Blake3,
        Ed25519,
}


fn parse_text_sign_format(format:&str) -> Result<TextSignFormat,anyhow::Error>{
    format.parse();
}