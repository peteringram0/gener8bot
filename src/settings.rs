#[derive(Debug)]
pub struct Settings {
  pub token: String,
  pub url: String,
  pub product_id: String
}

impl Settings {
  pub fn new(url: String, token: String, product_id: String) -> Settings {
    Settings {token, url, product_id}
  }
}