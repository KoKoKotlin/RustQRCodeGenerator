mod qrcode;
mod solomon_reed;

use qrcode::*;
use solomon_reed::*;

fn main() {
    let qrcode_ = QRCode::new(String::from("HELLO WORLD"), ErrorCorrectionLevel::M).unwrap();
    println!("{:?}", qrcode_.get_coeffs());

    // pretty_print_polynomial(get_generator_polynomial( 20));
}
