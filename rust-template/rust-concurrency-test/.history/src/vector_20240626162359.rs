use anyhow::{anyhow,Result};
use core::str;
use std::ops::{Add,AddAssign,Deref,Mul};

pub struct Vector<T>{
    data:Vec<T>,
}
