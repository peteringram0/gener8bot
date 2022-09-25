use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use crate::settings::Settings;
// use async_trait::async_trait;

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

  // #[serde(rename = "is-complete")]
  // is_complete: bool,

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
}

impl Product {
  pub fn seconds_until_finishes(&self) -> u64 {
    (self.ends_at.time() - Utc::now().time())
      .num_seconds() as u64
  }
}

/**
 * Get product
 */
pub async fn get(settings: &Settings) -> Result<Product, reqwest::Error> {
  let product: AuctionProductResponse = reqwest::Client::new()
    .get(settings.url.to_owned() + "/marketplace/auctions/" + &settings.product_id)
    .bearer_auth(&settings.token)
    .send()
    .await?
    .json()
    .await?;
  Ok(Product {
    current_price: product.data.attributes.current_price,
    ends_at: DateTime::parse_from_rfc3339(&product.data.attributes.ends_at).unwrap().with_timezone(&Utc),
    is_active: product.data.attributes.is_active,
  })
}

/**
 * Post bid
 */
pub async fn post_bid(product: &Product, settings: &Settings) -> Result<(), reqwest::Error> {
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
    .await?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use serde_json::json;
  use httpmock::prelude::*;
  use super::*;

  #[tokio::test]
  async fn get_info() {

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
                  "current-price": 200,
                  "ends-at": "2022-09-15T19:51:49+00:00",
                  "is-active": true
                }
              }
            }));
    });

    let settings = Settings::new(server.base_url(), "token".to_string(), "product".to_string());

    let a = get(&settings).await;

    mock.assert();

  }

}