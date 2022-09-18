use std::thread::sleep;
use std::time::Duration;
use chrono::Utc;

use crate::{product::Product, settings::Settings, me::Me};

pub fn snipe(product: Product, settings: &Settings, me: &Me) {

  let snipe_time = (product.ends_at.time() - Utc::now().time())
    .num_seconds() as u64;

  println!("Will post bid in {} seconds", snipe_time);
  sleep(Duration::from_secs(snipe_time));

  // Check the products last price before sniping
  let product_latest = product.get_existing(settings);
  if product_latest.is_err() {
    panic!("Could not update the product before sniping")
  }
  let product_unwrapped = product_latest.unwrap();

  // Check we have enough points in our account to snipe the product
  if product_unwrapped.current_price >= me.balance {
    panic!("Sorry you didn't have enough points to snipe");
  }

  // Run the snipe
  let bid = product_unwrapped.post_pid(settings);
  if bid.is_err() {
    panic!("Could not snipe")
  }

}


mod tests {

  #[test]
  fn snipe() {
    // let d = Product {
    //   active_users: 0,
    //   bids_made: 0,
    //   current_price: 0,
    //   starts_at: DateTime::parse_from_rfc3339("2022-09-15T19:51:49+00:00").unwrap().with_timezone(&Utc),
    //   ends_at: Utc::now() + Duration::seconds(3),
    //   is_active: true,
    //   is_complete: false
    // };
    // d.snipe();
    // println!("snipe happened {} milliseconds before auction ended", d.ends_at.timestamp_millis() - Utc::now().timestamp_millis());
  }

}