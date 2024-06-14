use crate::CmDExcetor;

use super::verify_file;
use clap::Parser;
use std::{fmt,str::FromStr};

#[derive(Debug,Clone,Copy)]


pub enum OutputFormat {

    Json,
    Yaml,
}

#[derive(Debug,Parser)]

pub struct CsvOps {

    #[arg(short,long,value_parser=verify_file)])]
    pub input:String,

    #[arg(short,long)]
    pub output:Option<String>,

    #[arg(long,value_parser=parse_format,default_value="json")]
    pub format:OutputFormat,

    #[arg(short,long,default_value_t=",")]
    pub delimiter:char,

    #[arg(long,default_value_t=true)]
    pub header:bool,

}