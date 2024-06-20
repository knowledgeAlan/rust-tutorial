use anyhow::Result;
use csv::Reader;
use serde::{Deserialize,Serialize};
use std::fs;

use crate::cli::OutputFormat;

#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all="PascalCase")]

struct Player {
    name: String,
    position:String,
    #[serde(rename="DOB")]
    dob:String,
    nationality:String,
    #[serde(rename="Kit Number")]
    kit:u8,
}


