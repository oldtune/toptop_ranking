extern crate tokio;
use core::panic;
mod app_err;
mod profile_fetcher;
use app_err::Result;

use reqwest::{Request, StatusCode};

#[tokio::main]
async fn main() -> Result<()> {
    let mut counter: u64 = 0;
    let builder = reqwest::ClientBuilder::new();
    let client = builder.build().unwrap();
    loop {
        let res = client.get("https://tiktok.com/@taylorswift").send().await;
        match res {
            Ok(res) => {
                if res.status().is_client_error() || res.status().is_server_error() {
                    println!("{:?}", res);
                    panic!();
                }
                counter += 1;
                println!("{}", counter);
            }
            Err(_) => {
                println!("failed after {} times", counter);
                panic!();
            }
        }
    }
}
