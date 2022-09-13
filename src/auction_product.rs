use serde::{Serialize, Deserialize};
use chrono::prelude::*;

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
pub struct AuctionProduct {
  data: AuctionProductData
}

impl AuctionProduct {

  #[tokio::main]
  pub async fn get_product_info(product_id: &str) -> Result<(), reqwest::Error> {

    let token = "";
    let url = "https://apollo.gener8ads.com/marketplace/auctions/".to_owned() + product_id;

    let product: AuctionProduct = reqwest::Client::new()
      .get(url)
      .bearer_auth(token)
      .send()
      .await?
      .json()
      .await?;

    println!("{:?}", product);

    let end_time = DateTime::parse_from_rfc3339(&product.data.attributes.ends_at);
    println!("{:?}", end_time);

    Ok(())

  }

}