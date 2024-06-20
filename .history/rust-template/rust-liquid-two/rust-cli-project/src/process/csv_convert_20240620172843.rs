use anyhow::Result;
use csv::Reader;
use serde::{Deserialize,Serialize};
use std::fs;

use crate::cli::OutputFormat;

