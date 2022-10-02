use serde::{Serialize, Deserialize};
use crate::{settings::Settings, product::Product};
use colored::*;

#[derive(Debug, Serialize, Deserialize)]
struct MeAttributes {
  balance: u64
}

#[derive(Debug, Serialize, Deserialize)]
struct MeInner {
  attributes: MeAttributes
}

#[derive(Debug, Serialize, Deserialize)]
struct MeResponse {
  data: MeInner
}

#[derive(Debug)]
pub struct Me {
  pub balance: u64
}

impl Me {
  pub fn can_afford_product(&self, product: &Product) {
    if product.current_price >= self.balance {
      panic!("Sorry you didn't have enough points to snipe");
    }
  }
}

pub async fn get(settings: &Settings) -> Me {
  let response = reqwest::Client::new()
    .get(settings.url.to_owned() + "/tokens/summary")
    .bearer_auth(&settings.token)
    .send()
    .await
    .unwrap();

    match response.status() {
      reqwest::StatusCode::OK => {
        match response.json::<MeResponse>().await {
          Ok(parsed) => {
            println!("{} {} {}", "You currently have".to_string().purple(), parsed.data.attributes.balance.to_string().purple().bold(), "points".to_string().purple());
            Me{balance: parsed.data.attributes.balance}
          },
          Err(_) => {
            println!("Hm, the response didn't match the shape we expected.");
            panic!("couldnt get me");
          },
        }
      }
      other => {
        panic!("Uh oh! Something unexpected happened: {:?}", other);
      }
    }
}