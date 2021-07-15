mod qrcode;
mod solomon_reed;

use qrcode::*;
use solomon_reed::*;

fn main() {
    // let qrcode_ = QRCode::new(String::from("Hello, World!"), ErrorCorrectionLevel::L).unwrap();
    // println!("{:?}", qrcode_.encode());

    pretty_print_polynomial(get_generator_polynomial(253));
}
