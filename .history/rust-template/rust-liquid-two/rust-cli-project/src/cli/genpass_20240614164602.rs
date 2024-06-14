use crate::CmdExector;
use clap::Parser;
use zxcvbn::zxcvbn;

#[derive(Debug,Parser)]

pub struct GenPassOpts {


    pub length: u8,

    pub uppercase: bool,


    pub lowercase: bool,


    pub number: bool,

    pub symbol: bool,
}