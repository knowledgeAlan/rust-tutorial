use crate::{dot_product,Vector};
use anyhow::{anyhow,Result};
use std::{
    fmt,
    ops::{Add,AddAssign,Mul},
    sync::mpsc,
    thread,
};


cont num_threads:usize = 4;