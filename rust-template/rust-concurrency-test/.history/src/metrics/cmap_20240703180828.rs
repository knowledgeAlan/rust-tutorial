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
    }
 }