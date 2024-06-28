use crate::{dot_product,Vector};
use anyhow::{anyhow,Result};
use std::{
    fmt,
    ops::{Add,AddAssign,Mul},
    sync::mpsc,
    thread,
};


const NUM_THREADS:usize = 4;