mod me;
mod auction_product;

use clap::Parser;
use auction_product::AuctionProduct;

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

    println!("TOKEN .. {}", args.token);
    println!("PRODUCT .. {}", args.product_id);

    // Read from CLI: auction_id and token
    // Store token on the heap (access from each files)

    let me = me::get_me();
    if me.is_err() {
        panic!("couldnt get me");
    }
    println!("{:?}", me);

    let product = AuctionProduct::get_product_info("358c9934-302e-11ed-80c3-b3ff58f38caf");
    if product.is_err() {
        panic!("couldnt get product");
    }
    println!("{:?}", product);

}
