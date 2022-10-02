use std::time::Duration;

use colored::*;
use tokio::time::sleep;
use crate::product::{self};
use crate::{settings::Settings};

pub async fn snipe(settings: &Settings) {

  // Check the products last price before sniping
  let product_latest = product::get(settings).await;

  let snipe_time = product_latest.seconds_until_finishes();

  println!("{} {} {}", "Will post bid in".red(), snipe_time.to_string().red().bold(), "seconds".red());
  sleep(Duration::from_secs(snipe_time)).await;

  // Run the snipe
  product::post_bid(&product_latest, settings).await;

}

#[cfg(test)]
mod tests {
  use chrono::{Utc, Duration};
  use httpmock::prelude::*;
  use serde_json::json;
  use super::*;

  #[tokio::test]
  async fn snipe_full_test() {

    let server = MockServer::start();

    let get_product = server.mock(|when, then| {
      when.method(GET);
      then.status(200)
        .header("content-type", "application/json")
          .json_body(json!({
            "data": {
              "attributes": {
                "active-users": 0,
                "bids-made": 0,
                "current-price": 170,
                "ends-at": (Utc::now() + Duration::seconds(4)).to_rfc3339(),
                "is-active": true,
                "is-complete": false,
                "starts-at": "2022-09-15T19:51:49+00:00"
              }
            }
          }));
    });

    let post_product_bid = server.mock(|when, then| {
      when.method(POST)
        .path("/marketplace/auctions/bids");
      then.status(200)
        .header("content-type", "application/json");
    });

    let settings = Settings::new(server.base_url(), "token".to_string(), "product".to_string());

    snipe(&settings).await;

    get_product.assert();
    post_product_bid.assert();

  }


}