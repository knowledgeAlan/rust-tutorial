use anyhow::{anyhow, Ok, Result};
use core::str;
use std::ops::{Add,AddAssign,Deref,Mul};

pub struct Vector<T>{
    data:Vec<T>,
}

pub fn dot_product<T>(a:Vector<T>,b:Vector<T>) -> Result<T> where T:Copy+Default + Add<Output = T> + AddAssign + Mul<Output = T>,{
    

    if a.len() != b.len() {
        return  Err(anyhow!("Dot  product error:a.len != b.len"));
    }


    let mut sum:T = T::default();

    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum);
}
