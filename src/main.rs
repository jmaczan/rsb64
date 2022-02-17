pub mod encode;

fn main() {
    let text_to_encode: &str = "ABC";
    println!("{}", text_to_encode);
    let encoded_text = encode::encode::encode(text_to_encode);
    for item in encoded_text.iter() {
        println!("{}", item);
    }
}
