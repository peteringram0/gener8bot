use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MeAttributes {
  balance: u64
}

#[derive(Debug, Serialize, Deserialize)]
struct MeInner {
  attributes: MeAttributes
}

#[derive(Debug, Serialize, Deserialize)]
struct MeWrapper {
  data: MeInner
}

#[tokio::main]
pub async fn get_me() -> Result<MeAttributes, reqwest::Error> {

  let token = "";
  // let token = "";

  let client: MeWrapper = reqwest::Client::new()
    .get("https://apollo.gener8ads.com/tokens/summary")
    .bearer_auth(token)
    .send()
    .await?
    .json()
    .await?;

  Ok(client.data.attributes)

}