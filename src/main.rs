use std::env;

pub mod encode;

fn main() {
    let text_to_encode: &str = "ABC";
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please run rsb64 in following way:\n");
        println!("To encode: ./rsb64 file_name");
        println!("To decode: ./rsb64 -decode file_name");
        println!("\nIn place of file_name put a name of file containing the text to encode/decode");
        return
    }
    println!("{}", args.len());
    println!("{:?}", args);
    println!("Clear text: {}", text_to_encode);
    println!("Encoded text: {}", encode::encode::encode(text_to_encode));
}
