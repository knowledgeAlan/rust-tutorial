use crate::{process_http_serve,CmdExector};

use super::verify_path;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[derive(Debug,Parser)]
#[enum_dispatch(CmdExector)]

pub enum HttpSubCommand {

    Serve(HttpServeOpts),
}