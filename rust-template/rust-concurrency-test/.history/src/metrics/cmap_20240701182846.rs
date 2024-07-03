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