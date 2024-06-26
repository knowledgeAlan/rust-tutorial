use anyhow::{anyhow,Result};
use core::str;
use std::ops::{Add,AddAssign,Deref,Mul};

pub struct Vector<T>{
    data:Vec<T>,
}

pub fn dot_product<T>(a:Vector<T>,b:Vector<T>) -> Result<T> where T:Copy+Default + Add<Output = T> + AddAssign + Mul<Output = T>,{
    

    if a.len() != b.len() {
        return  Err(anyhow!("Dot  product error:a.len != b.len"));
    }
}
