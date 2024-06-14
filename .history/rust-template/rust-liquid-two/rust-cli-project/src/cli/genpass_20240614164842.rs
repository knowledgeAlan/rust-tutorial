use crate::CmdExector;
use clap::Parser;
use zxcvbn::zxcvbn;

#[derive(Debug,Parser)]

pub struct GenPassOpts {

    #[arg(short,long,default_value_t=16)]
    pub length: u8,

    #[arg(long,default_value_t=true)]
    pub uppercase: bool,

    #[arg(long,default_value_t=true)]
    pub lowercase: bool,


    #[arg(long,default_value_t=true)]
    pub number: bool,

    #[arg(long,default_value_t=true)]
    pub symbol: bool,
}


impl CmdExector for GenPassOpts {

    async fn execute(self) -> anyhow::Result<()>{

    }
}
