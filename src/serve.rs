use clap::Parser;
use std::path::Path;

use crate::{generate::Generate, clean::Clean};

use warp::Filter;
use futures::future;
use tokio::time;
use std::time::Duration;
use local_ip_address::local_ip;

#[derive(Default, Parser)]
pub struct Serve {}

impl Serve {
    pub async fn run(&self) {
        let server_root = Path::new("output");
    
        if Path::exists(server_root) {
            // generate().await;
            Generate::default().run().await
        }
    
        let site = warp::get().and(warp::fs::dir(server_root));
    
        if let Ok(ip_address) = local_ip() {
            println!("website viewable at {}:8080", ip_address);
        }
    
        let (_clean, _server) = future::join(warp::serve(site)
        .run(([0, 0, 0, 0], 8080)), refresh_output_folder()).await;

    }
    
}

async fn refresh_output_folder() {
    let refresh_output = tokio::task::spawn(async {
        let mut interval = time::interval(Duration::from_secs(10));

        loop {
            interval.tick().await;
            Clean::default().run().await;
            Generate::default().run().await;
        }
    });

    let _ = refresh_output.await;
}