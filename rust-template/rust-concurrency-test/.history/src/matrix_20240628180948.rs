use crate::{dot_product, vector::Vector, Vector};
use anyhow::{anyhow,Result};
use std::{
    any, fmt, ops::{Add,AddAssign,Mul}, sync::mpsc, thread
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


pub fn multiply<T>(a:&Matrix<T>,b:&Matrix<T>) -> Result<Matrix<T>> 
where T:Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send +'static,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: a.col != b.row"));
    }


    let senders:Vec<mpsc::Sender<Msg<T>>> = (0..NUM_THREADS).map(|_| {
            let (tx,rx) = mpsc::channel::<Msg<T>>();

            thread::spawn(move ||{
                for msg in rx {
                    let  value:T = dot_product(msg.input.row,msg.input.col)?;  
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx:msg.input.idx,
                        value,
                    }){
                        eprint!("Send error:{:?}",e);
                    }
                }
                Ok::<_,anyhow::Error>(())
            });
            tx
    })
    .collect::<Vec<_>>();

    let matrix_len:usize = a.row * b.col;
    let mut data:Vec<T> = vec![T::default();matrix_len];
    let mut receivers:Vec<oneshot::Receiver<MsgOutput<T>>> = Vec::with_capacity(matrix_len);

    for i in 0..a.row{
        for j in b.col{
            let row:Vector<T> = Vector::new(&a.data[i*a.col..(i+1) * a.col]);
            
        }
    }

    for rx in receivers {
        let output:MsgOutput<T> = rx.recv()?;
        data[output.idx] = output.value;
    }



    

}
