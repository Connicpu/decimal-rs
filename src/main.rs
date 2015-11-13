extern crate decimal;
extern crate num;

fn main() {
    // 1.00
    let dec = decimal::Decimal::from_i32(-115, 2); // 1.00

    println!("let dec = `-1.15` [Decimal::from_i32(-115, 2)]");
    println!("sizeof(dec) => {}", std::mem::size_of::<decimal::Decimal>());
    println!("dec.to_f64() => {}", dec.to_f64());
    println!("dec => {:?}", dec);
}

