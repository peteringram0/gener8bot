mod product;
mod settings;
mod me;
mod snipe;

use clap::Parser;

// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[clap(short, long, value_parser)]
   token: String,

   /// Number of times to greet
   #[clap(short, long, value_parser)]
   product_id: String,
}

#[tokio::main]
async fn main() {

  let args = Args::parse();

  let settings = settings::Settings::new("https://apollo.gener8ads.com".to_string(), args.token, args.product_id);

  let me = me::get(&settings).await;
  if me.is_err() {
    panic!("couldnt get me");
  }
  // println!("{:?}", me);

  let product = product::get(&settings).await;
  if product.is_err() {
    panic!("couldnt get product");
  }
  // println!("{:?}", product);

  match product {
    Ok(product) => {
      if !product.is_active {
        panic!("Product is not active")
      }
      snipe::snipe(&settings, &me.unwrap()).await // TODO: Maybe this will work without putting inside a task .... need a way to check
    },
    Err(error) => println!("error {}", error)
  }

}