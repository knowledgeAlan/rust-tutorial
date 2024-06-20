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


fn verify_file(fileName:&str) -> Result<String,&'static str> {


    if fileName == "-" || Path::new(fileName).exists(){

        Ok(fileName.into());
    }else{
        Err("File does not exist");
    }
}


fn verify_path(path: &str) -> Result<PathBuf,&'static str> {

    let p:&Path = Path::new(path);

    if p.exists() && p.is_dir(){

        Ok(path.into();)
    }else{
        Err("Path does not exist or is not a direcotry");
    }
}

#[cfg(test)]

mod tests {
    use super::*

    #[test]
    fn test_verify_input_file(){
        
        assert_eq!(verify_file("-"),Ok("-".into()));
        assert_eq!(verify_file("*"),Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"),Ok("Cargo.html".into()));

    }
}