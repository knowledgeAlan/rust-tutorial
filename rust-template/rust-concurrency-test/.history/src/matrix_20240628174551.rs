use crate::{dot_product,Vector};
use anyhow::{anyhow,Result};
use std::{
    fmt,
    ops::{Add,AddAssign,Mul},
    sync::mpsc,
    thread,
};


const NUM_THREADS:usize = 4;

pub struct Matrix<T> {
    data:Vec<T>,
    row:usize,
    col: usize,
}


pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}


pub struct MsgOutput<T> {

    idx: usize,
    value: T,
}


pub struct Msg<T> {

    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}


pub fn multiply<T>(a:&matrix<T>,b:&Matrix<T>) -> Result<Matrix<T>> 
where T:Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send +'static,
{
    
}
