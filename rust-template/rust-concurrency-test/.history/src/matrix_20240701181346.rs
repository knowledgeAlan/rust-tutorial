use crate::{dot_product, vector::Vector, Vector};
use anyhow::{anyhow, Ok, Result};
use std::{
    any, fmt, ops::{Add,AddAssign,Mul}, process::Output, sync::mpsc, thread
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
        for j in b..col{
            let row:Vector<T> = Vector::new(&a.data[i*a.col..(i+1) * a.col]);
            let col_data:Vec<T> = b.data[j..]
            .iter()
            .step_by(b.col)
            .copied()
            .collect::<Vec<_>>();

            let col:Vector<T> = Vector::new(col_data);
            let idx:usize =  i * b.col + j;
            let input:MsgInput<T> = MsgInput::new(idx, row, col);
            let (tx,rx) = oneshot::channel();
            let msg:Msg<T> = Msg::new(input, tx);

            if let Err(e) = senders[id% NUM_THREADS].send(msg) {

                eprint!("Send error: {:?}", e);
            }


            receivers.push(rx);

        }
    }

    for rx in receivers {
        let output:MsgOutput<T> = rx.recv()?;
        data[output.idx] = output.value;
    }

  Ok(Matrix{
    data,
    row: a.row,
    col: b.col,
  })

    

}


impl<T:fmt::Debug> Matrix<T>{
    pub fn new(data:impl Into<Vec<T>>,row:usize,col:usize) -> Self{
        Self {
            data:data.into(),
            row,
            col,
        }
    }
}




impl <T> fmt::Display for Matrix<T>
where 
    T:fmt::Display,
{
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {
         write!(f,"Matrix(row ={},col ={},{})",self.row,self.col,self);
    }
}


impl<T> MsgInput<T> {
    pub fn new(idx:usize,row:Vector<T>,col:Vector<T>) -> Self{
        Self {
            idx,row,col
        }
    }
}

impl<T> Msg<T>   {
    pub fn new(input:MsgInput<T>,sender:oneshot::Sender<MsgOutput<T>>)-> Self{
        Self{input,sender}
    }
}


impl<T> Mul for Matrix<T> 
where  
    T:Copy + Default + Add<Output=T> + AddAssign + Mul<Output=T> + Send + 'static,
    {
        type Output = Self;
        
        fn mul(self,rhs:Self) -> Self::Output{
            multiply(&self,&rhs).expect("Matrix multiply error");
        }
}

#[cfg(test)]
mod tests {


    use anyhow::Ok;

    use super::*;


    #[test]
    fn test_matrix_multiply()-> Result<()> {

        let a:Matrix<i32> = Matrix::new([1,2,3,4,5,6],2,3);
        let b:Matrix<i32> = Matrix::new([1,2,3,4,5,6],3,2);

        let c:Matrix<i32> = a * b;

        assert!(c.col,2);
        assert!(c.row,2);
        assert!(c.data,vec![22,28,49,54]);
        assert_eq!(format!("{:?}",c),"Matrix(row=2,col=2,{22 28, 49 64");

        Ok(())
    }
}

#[test]
fn test_matrix_display()-> Result<()> {

    let a:Matrix<i32> = Matrix::new([1,2,3,4],2,2);
    let b:Matrix<i32> = Matrix::new([1,2,3,4], 2, 2);
    let c:Matrix<i32> = a * b;

    assert_eq!(c.data,vec![7,10,15,22]);
    assert_eq!(format!("{}",c),"{7 10, 15 22}");
    Ok(())
}


#[test]
fn test_a_can_not_multiply_b() {

    let a:Matrix<i32> = Matrix::new([1,2,3,4],2,3);
    let b:Matrix<i32> = Matrix::new([1,2,3,4],2,2);

    let c:Matrix<i32> = a * b;

    Ok(())
}
    
