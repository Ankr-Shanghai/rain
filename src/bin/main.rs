#![allow(unused_imports, unused_variables)]
use axum::{routing::get, routing::post, Router};
use log::{error, info};
use log4rs;
use pkg::endpoints;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    process::exit,
};

#[tokio::main]
async fn main() {
    dotenv::from_filename("env_file").ok();

    // init log system
    log4rs::init_file("config.yaml", Default::default()).unwrap_or_else(|err| {
        println!("init log error {}", err);
        exit(-1)
    });

    let cfg = pkg::config::Config::from_env().expect("parse env failed");
    let io = pkg::handlers::init_iohandlers();

    let app_state = Arc::new(pkg::config::AppState { config: cfg, io });

    // boot routine to do sync remote service info
    let heap_sort: BinaryHeap<endpoints::Node> = BinaryHeap::new();
    let hs = Arc::new(Mutex::new(heap_sort));
    let hsc = hs.clone();
    let uris = app_state.config.uris.clone();

    tokio::spawn(async move {
        pkg::service::remote_info(Box::leak(uris.into_boxed_str()), hsc).await;
    });

    // parse command arguments
    let args = Args::parse();

    // build application with a route
    let app = Router::new()
        .route("/", post(pkg::router::router))
        .route("/status", get(pkg::asist::health))
        .route("/config", get(pkg::asist::config))
        .with_state(app_state);

    let host = args.host.parse::<IpAddr>().unwrap_or_else(|err| {
        error!("host {} error {} ", args.host, err);
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
    });

    let addr = SocketAddr::from((host, args.port));
    info!("listen on {} ...", addr);

    let bindr = axum::Server::try_bind(&addr).unwrap_or_else(|err| {
        error!("bind address error {}", err);
        exit(-1)
    });

    bindr
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|err| {
            error!("service boot error {}", err);
            exit(-1)
        });
}

// define command args
use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}
