use serde::{Serialize, Deserialize};
use crate::settings::Settings;

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
  balance: u64
}

impl Me {

  #[tokio::main]
  pub async fn get(settings: &Settings) -> Result<Me, reqwest::Error> {
    let client: MeResponse = reqwest::Client::new()
      .get(settings.url.to_owned() + "/tokens/summary")
      .bearer_auth(&settings.token)
      .send()
      .await?
      .json()
      .await?;
    Ok(Me{balance: client.data.attributes.balance})
  }

}