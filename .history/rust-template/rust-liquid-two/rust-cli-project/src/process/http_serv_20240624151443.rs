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


    let addr:SocketAddr = SocketAddr::from(([0,0,0,0],port));
    info!("Serving {:?} on {}",path,addr);


    let state:HttpServeState = HttpServeState{path:path.clone()};

    let router:Router = Router::new().nest_service("/tower",ServeDir::new(path))
        .route("/*path",get(file_handler))
        .with_state(AAarc::new(state));


    let listener:tokio::new::TcpListener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener,router).await?;

    Ok(());
}