use anyhow::Result;

use axum::{
    extract::{Path,State},
    http::StatusCode,
    routing::get,
    Router,

};

use std::{net::SocketAddr,path::PathBuf,sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info,warn};


#[derive(Debug)]

struct HttpServeState{
    path: PathBuf,
}


pub async fn process_http_serve(path:PathBuf,port:u16)-> Result<()>{

    
}