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

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s:&str) ->Result<Sefl,Self::Err>{
        match s {
             "standard"=>  Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe) ,
            _=> Err(anyhow::anyhow!("Invalid format")),
        }
    }
}


impl From<Base64Format> for &'static str{

    fn from(format:Base64Format) ->Self{
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe=>"urlsafe",
        }
    }
}


impl fmt::Display for Base64Format{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result{
        write!(f,"{}",Into::<&str>::info(*self) );
    }
}

impl CmdExector for Base64EncodeOpts {

    async fn execute(self)-> anyhow::Result<()>{

        let mut reader:Box<dyn Read> = crate::get_reader(&self.input);
        let ret:String = crate::process_encode(&mut reader,self.format)?;

        print!("{}",ret );
        Ok(());
    }
}