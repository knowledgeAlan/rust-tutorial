use std::{fmt,str::FromStr}
use clap::Parser;

use enum_dispatch::enum_dispatch;


use crate::CmDExcetor;

use super::verify_file;

#[derive(Debug,Parser)]
#[enum_dispatch(CmDExcetor)]
pub enum Name {
    Variant,
}