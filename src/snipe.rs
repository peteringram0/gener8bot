use std::{thread::sleep};
use std::time::Duration;

use colored::*;
use crate::product::{self};
use crate::{settings::Settings, me::Me};

pub async fn snipe(settings: &Settings, me: &Me) {

  // Check the products last price before sniping
  let product_latest = product::get(settings).await;
  if product_latest.is_err() {
    panic!("Could not update the product before sniping")
  }
  let product_unwrapped = product_latest.unwrap();

  let snipe_time = product_unwrapped.seconds_until_finishes();

  println!("{} {} {}", "Will post bid in".red(), snipe_time.to_string().red().bold(), "seconds".red());
  sleep(Duration::from_secs(snipe_time));

  // Run the snipe
  let bid = product::post_bid(&product_unwrapped, settings).await;
  if bid.is_err() {
    panic!("Could not snipe")
  }

}

#[cfg(test)]
mod tests {
  use chrono::{Utc};
  use httpmock::prelude::*;
  use serde_json::json;
  use super::*;

  #[tokio::test]
  async fn snipe_full_test() {

    let server = MockServer::start();

    let re_get_product = server.mock(|when, then| {
      when.method(GET);
      then.status(200)
        .header("content-type", "application/json")
          .json_body(json!({
            "data": {
              "attributes": {
                "active-users": 0,
                "bids-made": 0,
                "current-price": 170,
                "ends-at": Utc::now().to_rfc3339(),
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

    let settings = Settings::new(server.base_url(), "token".to_string(), "product".to_string());
    let me = Me {balance: 200};

    snipe(&settings, &me).await;

    re_get_product.assert();
    post_bid.assert();

    // TODO: bid should be 172 .. how to check this in the payload?

  }


}