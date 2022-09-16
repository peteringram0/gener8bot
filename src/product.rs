use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use std::borrow::Borrow;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use crate::settings::Settings;

#[derive(Debug, Serialize, Deserialize)]
struct AuctionProductAttributes {
  #[serde(rename = "active-users")]
  active_users: u8,

  #[serde(rename = "bids-made")]
  bids_made: u32,

  #[serde(rename = "current-price")]
  current_price: u64,

  #[serde(rename = "ends-at")]
  ends_at: String,

  #[serde(rename = "is-active")]
  is_active: bool,

  #[serde(rename = "is-complete")]
  is_complete: bool,

  #[serde(rename = "starts-at")]
  starts_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuctionProductData {
  attributes: AuctionProductAttributes
}

#[derive(Debug, Serialize, Deserialize)]
struct AuctionProductResponse {
  data: AuctionProductData
}

#[derive(Debug)]
pub struct Product {
  active_users: u8,
  bids_made: u32,
  current_price: u64,
  starts_at: DateTime<Utc>,
  ends_at: DateTime<Utc>,
  is_active: bool,
  is_complete: bool,
}

impl Product {

  #[tokio::main]
  pub async fn get(settings: &Settings) -> Result<Product, reqwest::Error> {
    let product: AuctionProductResponse = reqwest::Client::new()
      .get(settings.url.to_owned() + "/marketplace/auctions/" + &settings.product_id)
      .bearer_auth(&settings.token)
      .send()
      .await?
      .json()
      .await?;
    println!("a: product {:?}", product);
    Ok(Product {
      active_users: product.data.attributes.active_users,
      bids_made: product.data.attributes.bids_made,
      current_price: product.data.attributes.current_price,
      starts_at: DateTime::parse_from_rfc3339(&product.data.attributes.starts_at).unwrap().with_timezone(&Utc),
      ends_at: DateTime::parse_from_rfc3339(&product.data.attributes.ends_at).unwrap().with_timezone(&Utc),
      is_active: product.data.attributes.is_active,
      is_complete: product.data.attributes.is_complete,
    })
  }

  pub fn snipe(&self) {
    let snipe_time = (self.ends_at.time() - Utc::now().time())
      .num_seconds() as u64;
    println!("Will post bid in {} seconds", snipe_time);
    sleep(Duration::from_secs(snipe_time));
    self.bid();
  }

  fn bid(&self) {
    !unimplemented!()
  }

}

#[cfg(test)]
mod tests {
  use chrono::{DateTime, Utc, Duration};
  use serde_json::json;
  use crate::{product::{Product}, settings::Settings};
  use httpmock::prelude::*;

  #[test]
  fn get_info() {

    // Start a lightweight mock server.
    let server = MockServer::start();

    // Create a mock on the server.
    let mock = server.mock(|when, then| {
        when.method(GET);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
              "data": {
                "attributes": {
                  "active-users": 0,
                  "bids-made": 0,
                  "current-price": 200,
                  "ends-at": "2022-09-15T19:51:49+00:00",
                  "is-active": true,
                  "is-complete": false,
                  "starts-at": "2022-09-15T19:51:49+00:00"
                }
              }
            }));
    });

    println!("url, {}", server.url(""));

    let settings = Settings::new(server.base_url(), "token".to_string(), "product".to_string());

    let info = Product::get(&settings);

    mock.assert();

    println!("HERE {:?}", info);

  }

  #[test]
  fn snipe() {
    let d = Product {
      active_users: 0,
      bids_made: 0,
      current_price: 0,
      starts_at: DateTime::parse_from_rfc3339("2022-09-15T19:51:49+00:00").unwrap().with_timezone(&Utc),
      ends_at: Utc::now() + Duration::seconds(3),
      is_active: true,
      is_complete: false
    };
    d.snipe();
    println!("snipe happened {} milliseconds before auction ended", d.ends_at.timestamp_millis() - Utc::now().timestamp_millis());
  }

}