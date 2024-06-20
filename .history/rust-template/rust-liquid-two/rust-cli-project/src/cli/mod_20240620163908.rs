mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path,PathBuf};


pub use self::{base64::*,csv::*, genpass::*,http::*, text::*};

#[derive(Debug,Parser)]]
#[command(name = "rcli", version,author,about,long_about = None)]

pub struct Opts {

    #[command(subcommand)]
    pub cmd:Subcommand,
}

#[derive(Debug,Parser)]
#[enum_dispatch(CmdExector)]

pub enum Subcommand {


    #[command(name ="csv",about "Show CSV,or convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass",about ="Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand,about="Base64 encode/decode")]
    Base64(Base64SubCommand),

    #[command(subcommand,about="Text sign/verify")]
    Text(TextSubCommand),

    #[command(subcommand,about="Http server")]
    Http(HttpSubCommand),

}


