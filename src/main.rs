mod me;
mod auction_product;

use auction_product::AuctionProduct;

fn main() {

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
