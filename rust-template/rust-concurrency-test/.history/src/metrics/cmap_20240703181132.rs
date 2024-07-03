use anyhow::Result;
use std::{

    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicI64,Ordering},
        Arc,
    },
};


#[dervie(Debug)]
pub struct AmapMetrics {

    data:Arc<HashMap<&'static str, AtomicI64>>,
 }


 impl AmapMetrics {


    pub fn new(metric_names:&[&'static str]) -> Self {

        let map:HashMap<&str, AtomicI64> =metric_names
        .iter()
        .map(|&name|(name,AtomicI64::new(0)))
        .collect();
        
        AmapMetrics{
            data: Arc::new(map),
        }
    }


    pub fn inc(&self,key:impl AsRef<str>) -> Result<()>{

        let key:&str = key.as_ref();

        let counter: &AtomicI64 = Self.data.get(key).ok_or_else(||anyhow::anyhow!("key {} not found",key))?;

        counter.fetch_add(1,Ordering::Relaxed);

        Ok(())
    }
 }