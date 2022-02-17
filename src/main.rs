pub mod encode;

fn main() {
    let text_to_encode: &str = "ABC";
    println!("{}", text_to_encode);
    encode::encode::encode(text_to_encode);
}
