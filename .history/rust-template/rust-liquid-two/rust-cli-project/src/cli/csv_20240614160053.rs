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

    pub input:String,

    pub output:Option<String>,

    pub format:OutputFormat,

    pub delimiter:char,

    pub header:bool,

}