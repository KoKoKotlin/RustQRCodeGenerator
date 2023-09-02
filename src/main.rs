mod qrcode;
mod solomon_reed;

use qrcode::*;
use solomon_reed::*;

fn main() {
    let qrcode_ = QRCode::new(String::from("HELLO WORLD"), ErrorCorrectionLevel::M).unwrap();
    println!("{:?}", qrcode_.gen_error_codewords());

    // pretty_print_polynomial(get_generator_polynomial( 20));
}
