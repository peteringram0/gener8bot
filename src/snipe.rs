use std::thread::sleep;
use std::time::Duration;
use chrono::Utc;

use colored::*;
use crate::{product::Product, settings::Settings, me::Me};

#[tokio::main]
pub async fn snipe(product: Product, settings: &Settings, me: &Me) {

  let snipe_time = (product.ends_at.time() - Utc::now().time())
    .num_seconds() as u64;

  println!("{} {} {}", "Will post bid in".red(), snipe_time.to_string().red().bold(), "seconds".red());
  sleep(Duration::from_secs(snipe_time));

  // Check the products last price before sniping
  let product_latest = product.get_existing(settings);
  if product_latest.is_err() {
    panic!("Could not update the product before sniping")
  }
  let product_unwrapped = product_latest.unwrap();

  // Check we have enough points in our account to snipe the product
  if product_unwrapped.current_price >= me.balance {
    panic!("Sorry you didn't have enough points to snipe");
  }

  // Run the snipe
  let bid = product_unwrapped.post_pid(settings);
  if bid.is_err() {
    panic!("Could not snipe")
  }

}


mod tests {
  use chrono::{Duration};
  use httpmock::prelude::*;
  use serde_json::json;
  use super::*;

  #[test]
  fn snipe_full_test() {

    let product = Product {
      // active_users: 0,
      // bids_made: 0,
      current_price: 150,
      // starts_at: DateTime::parse_from_rfc3339("2022-09-15T19:51:49+00:00").unwrap().with_timezone(&Utc),
      ends_at: Utc::now() + Duration::seconds(3),
      is_active: true,
      // is_complete: false
    };

    let me = Me {
      balance: 200
    };

    // Start a lightweight mock server.
    let server = MockServer::start();

    let settings = Settings::new(server.base_url(), "token".to_string(), "product".to_string());

    let re_get_product = server.mock(|when, then| {
      when.method(GET)
        .path("/marketplace/auctions/");

      then.status(200)
        .header("content-type", "application/json")
          .json_body(json!({
            "data": {
              "attributes": {
                "active-users": 0,
                "bids-made": 0,
                "current-price": 170,
                "ends-at": Utc::now().to_string(),
                "is-active": true,
                "is-complete": false,
                "starts-at": "2022-09-15T19:51:49+00:00"
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

    snipe(product, &settings, &me);

    re_get_product.assert();
    post_bid.assert();

    // TODO: bid should be 172

  }

}