mod product;
mod settings;
mod me;
mod snipe;

use std::{thread::sleep, time::Duration};

use clap::Parser;
use settings::Settings;

// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[clap(short, long, value_parser)]
   token: String,

   /// Number of times to greet
   #[clap(short, long, value_parser)]
   product_id: String,
}

fn main() {
  let args = Args::parse();
  let settings = settings::Settings::new("https://apollo.gener8ads.com".to_string(), args.token, args.product_id);
  watch_product(&settings)
}

/**
 * Every 10 seconds we will re get us, and the product to make sure we still have enough points.
 * 10 mins before the auction ends we will setup our snipe
 */
#[tokio::main]
async fn watch_product(settings: &Settings) {

  let me = me::get(&settings).await;
  if me.is_err() {
    panic!("couldnt get me");
  }
  let me = me.unwrap();

  let product = product::get(&settings).await;
  if product.is_err() {
    panic!("couldnt get product");
  }
  let product = product.unwrap();
  // println!("{:?}", product);

  if !product.is_active {
    panic!("Product is not active")
  }

  // Check we can afford the product
  me.can_afford_product(&product);

  if product.seconds_until_finishes() >= 600 { // 10 mins
    // sleep(Duration::from_secs(600));
    return watch_product(settings) // TODO: This errors here !!
  }

  snipe::snipe(&settings, &me).await // TODO: Maybe this will work without putting inside a task .... need a way to check

}

#[cfg(test)]
mod tests {
  use chrono::{Utc, Duration};
  use httpmock::prelude::*;
  use serde_json::json;
  use crate::{settings::Settings, me::Me};
  use super::*;

  #[test]
  fn watch_product_test() {

    let server = MockServer::start();

    let get_product = server.mock(|when, then| {
      when.method(GET)
        .path("/marketplace/auctions/product");
      then.status(200)
        .header("content-type", "application/json")
          .json_body(json!({
            "data": {
              "attributes": {
                "active-users": 0,
                "bids-made": 0,
                "current-price": 170,
                "ends-at": (Utc::now() + Duration::minutes(11)).to_rfc3339(),
                "is-active": true,
                "is-complete": false,
                "starts-at": "2022-09-15T19:51:49+00:00"
              }
            }
          }));
    });

    let get_me = server.mock(|when, then| {
      when.method(GET)
        .path("/tokens/summary");
      then.status(200)
        .header("content-type", "application/json")
          .json_body(json!({
            "data": {
              "attributes": {
                "balance": 200,
              }
            }
          }));
    });

    let post_bid = server.mock(|when, then| {
      when.method(POST)
        .path("/marketplace/auctions/bids");
      then.status(200)
        .header("content-type", "application/json");
    });

    let settings = Settings::new(server.base_url(), "token".to_string(), "product".to_string());

    watch_product(&settings);

    get_me.assert();
    get_product.assert();

  }

}