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

fn main() {

    let args = Args::parse();
    let settings = settings::Settings::new("https://apollo.gener8ads.com".to_string(), args.token, args.product_id);

    let me = me::Me::get(&settings);
    if me.is_err() {
        panic!("couldnt get me");
    }
    println!("{:?}", me);

    let product = product::Product::get(&settings);
    if product.is_err() {
        panic!("couldnt get product");
    }
    println!("{:?}", product);

    match product {
        Ok(product) => snipe::snipe(product, &settings, &me.unwrap()),
        Err(error) => println!("error {}", error)
    }

}
