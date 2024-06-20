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