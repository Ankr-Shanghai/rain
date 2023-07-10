#![allow(unused_imports, unused_variables)]
use axum::{routing::get, routing::post, Router};
use log::{error, info};
use log4rs;
use pkg::endpoints;
use pkg::ethdb::cache;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex, RwLock};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    process::exit,
    thread,
};
use tokio::task;

#[tokio::main]
async fn main() {
    // init log system
    log4rs::init_file("log_config.yaml", Default::default()).unwrap_or_else(|err| {
        println!("init log error {}", err);
        exit(-1)
    });

    let cfg = pkg::config::Config::from_env().expect("parse env failed");
    let db = Arc::new(RwLock::new(pkg::ethdb::store::DB::new(
        cfg.database.path.clone(),
    )));
    let cache: Arc<cache::MemStore> = Arc::new(cache::MemStore::new());
    let io: jsonrpc_core::IoHandler = pkg::handlers::init_iohandlers(db.clone(), cache.clone());

    let app_state: Arc<pkg::config::AppState> = Arc::new(pkg::config::AppState { config: cfg, io });

    // boot routine to do sync remote service info
    let heap_sort: BinaryHeap<endpoints::Node> = BinaryHeap::new();
    let hs: Arc<Mutex<BinaryHeap<endpoints::Node>>> = Arc::new(Mutex::new(heap_sort));
    let hsc: Arc<Mutex<BinaryHeap<endpoints::Node>>> = hs.clone();
    let uris: Vec<String> = app_state.config.uris.clone();

    task::spawn(async move {
        pkg::service::remote_info(uris, hsc).await;
    });

    // init database and boot sync service
    let mut service = pkg::service::Service::new(db);
    task::spawn(async move {
        service.sync(hs, cache).await;
    });

    info!("sync service started ...");

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
