use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use crate::settings::Settings;
use colored::*;

#[derive(Debug, Serialize, Deserialize)]
struct AuctionProductAttributes {
  // #[serde(rename = "active-users")]
  // active_users: u8,

  // #[serde(rename = "bids-made")]
  // bids_made: u32,

  #[serde(rename = "current-price")]
  current_price: u64,

  #[serde(rename = "ends-at")]
  ends_at: String,

  #[serde(rename = "is-active")]
  is_active: bool,

  #[serde(rename = "is-complete")]
  is_complete: bool,

  // #[serde(rename = "starts-at")]
  // starts_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuctionProductData {
  attributes: AuctionProductAttributes
}

#[derive(Debug, Serialize, Deserialize)]
struct AuctionProductResponse {
  data: AuctionProductData
}

#[derive(Debug, Serialize, Deserialize)]
struct BidResponse;

#[derive(Debug)]
pub struct Product {
  pub current_price: u64,
  pub ends_at: DateTime<Utc>,
  pub is_active: bool,
  pub is_complete: bool,
}

impl Product {
  pub fn seconds_until_finishes(&self) -> u64 {
    (self.ends_at.time() - Utc::now().time())
      .num_seconds() as u64
  }
  pub fn has_finished(&self) {
    if self.is_complete {
      panic!("Sorry auction on this product has finished");
    }
  }
  pub fn is_not_active(&self) {
    if !self.is_active {
      panic!("Sorry auction on this product it not active");
    }
  }
}

/**
 * Get product
 */
pub async fn get(settings: &Settings) -> Product {
  let response = reqwest::Client::new()
    .get(settings.url.to_owned() + "/marketplace/auctions/" + &settings.product_id)
    .bearer_auth(&settings.token)
    .send()
    .await
    .unwrap();

    match response.status() {
      reqwest::StatusCode::OK => {
        return match response.json::<AuctionProductResponse>().await {
          Ok(parsed) => {
            let p = Product {
              current_price: parsed.data.attributes.current_price,
              ends_at: DateTime::parse_from_rfc3339(&parsed.data.attributes.ends_at).unwrap().with_timezone(&Utc),
              is_active: parsed.data.attributes.is_active,
              is_complete: parsed.data.attributes.is_complete
            };
            println!("{} {} {}", "Current product price:".to_string().blue(), p.current_price.to_string().blue().bold(), "points".to_string().blue());
            p
          },
          Err(_) => {
            println!("Hm, the response didn't match the shape we expected.");
            panic!("couldnt get product");
          },
        };
      }
      other => {
        panic!("Uh oh! Something unexpected happened: {:?}", other);
      }
    }
}

/**
 * Post bid
 */
pub async fn post_bid(product: &Product, settings: &Settings) {
  reqwest::Client::new()
    .post(settings.url.to_owned() + "/marketplace/auctions/bids")
    .bearer_auth(&settings.token)
    .json(&serde_json::json!({
      "data":{
        "attributes":{
          "amount": product.current_price + 1
        },
        "relationships":{
          "auction":{
            "data":{
              "type": "auctions",
              "id": settings.product_id
            }
          }
        },
        "type": "bid"
      }
    }))
    .send()
    .await
    .unwrap();
  println!("{} {} {}", "Bid made for: ".to_string().blue(), (product.current_price + 1).to_string().blue().bold(), "points".to_string().blue());
}

#[cfg(test)]
mod tests {
  use chrono::{Utc, Duration};
  use serde_json::json;
  use httpmock::prelude::*;
  use super::*;

  #[tokio::test]
  async fn get_info_test() {

    // Start a lightweight mock server.
    let server = MockServer::start();

    // Create a mock on the server.
    let product_response = server.mock(|when, then| {
        when.method(GET)
          .path("/marketplace/auctions/product");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
              "data": {
                "attributes": {
                  "current-price": 200,
                  "ends-at": "2022-09-15T19:51:49+00:00",
                  "is-active": true,
                  "is-complete": false
                }
              }
            }));
    });

    let settings = Settings::new(server.base_url(), "token".to_string(), "product".to_string());

    let _ = get(&settings).await;

    product_response.assert();

  }

  #[tokio::test]
  async fn post_bid_test() {

    // Start a lightweight mock server.
    let server = MockServer::start();

    // Create a mock on the server.
    let bid_response = server.mock(|when, then| {
        when.method(POST)
          .path("/marketplace/auctions/bids");
        then.status(200);
    });

    let settings = Settings::new(server.base_url(), "token".to_string(), "product".to_string());
    let product = Product {
      current_price: 200,
      ends_at: Utc::now() + Duration::minutes(11),
      is_active: true,
      is_complete: false
    };

    let _ = post_bid(&product, &settings).await;

    bid_response.assert();

  }

}