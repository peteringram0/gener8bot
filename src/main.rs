mod product;
mod settings;
mod me;
mod snipe;

use tokio::{task};
use std::{time::Duration};
use colored::*;
use clap::Parser;
use settings::Settings;
use tokio::time::sleep;

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

#[derive(PartialEq)]
enum WatchResult {
  LOOP,
  SNIPE
}

fn main() {
  let args = Args::parse();
  let settings = settings::Settings::new("https://apollo.gener8ads.com".to_string(), args.token, args.product_id);
  main_inner(settings)
}

#[tokio::main]
async fn main_inner(settings: Settings) {
  let forever = task::spawn({
    async move {
      loop {
        let res = watch_product(&settings).await;
        if res == WatchResult::SNIPE {
          break;
        }
        sleep(Duration::from_secs(60)).await;
      }
    }
  });
  forever.await.unwrap();
  println!("{}", "Auction finished. I hope you won the product".to_string().red().bold());
}

/**
 * Every 10 seconds we will re get us, and the product to make sure we still have enough points.
 * 10 mins before the auction ends we will setup our snipe
 */
async fn watch_product(settings: &Settings) -> WatchResult {

  let me = me::get(&settings).await;
  let product = product::get(&settings).await;

  product.is_not_active(); // make sure the product is active
  product.has_finished(); // make sure the action has not changed
  me.can_afford_product(&product); // make sure we can still afford the product

  if product.seconds_until_finishes() > 60 { // 1 min
    println!("{}", "Over 60 seconds until auction finishes ... will keep watching the product! ...".to_string().red());
    return WatchResult::LOOP
  }

  snipe::snipe(&settings).await;
  WatchResult::SNIPE

}

#[cfg(test)]
mod tests {
  use chrono::{Utc, Duration};
  use httpmock::prelude::*;
  use serde_json::json;
  use crate::{settings::Settings};
  use super::*;

  #[tokio::test]
  async fn watch_product_test_over_2_mins() {

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
                "ends-at": (Utc::now() + Duration::minutes(2)).to_rfc3339(),
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

    watch_product(&settings).await;

    get_me.assert();
    get_product.assert();
    post_bid.assert_hits(0)

  }

  #[test]
  fn main_test() {
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
                "ends-at": (Utc::now() + Duration::minutes(2)).to_rfc3339(),
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
    main_inner(settings);

    get_product.assert_hits(3);
    get_me.assert_hits(2);
    post_bid.assert_hits(1);

  }

}