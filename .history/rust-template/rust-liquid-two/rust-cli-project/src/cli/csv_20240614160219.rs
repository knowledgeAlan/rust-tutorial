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

    pub delimiter:char,

    pub header:bool,

}