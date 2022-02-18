pub mod encode;

fn main() {
    let text_to_encode: &str = "ABC";
    println!("Clear text: {}", text_to_encode);
    println!("Encoded text: {}", encode::encode::encode(text_to_encode));
}
