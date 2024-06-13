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