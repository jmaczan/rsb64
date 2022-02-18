use std::env;
use std::fs;

pub mod encode;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        println!("Please run rsb64 in following way:\n");
        println!("To encode: ./rsb64 file_name");
        println!("To decode: ./rsb64 -decode file_name");
        println!("\nIn place of file_name put a name of file containing the text to encode/decode");
        return;
    }

    let decoding = args.len() == 3;

    let contents = fs::read_to_string(if decoding {
        args[2].to_string()
    } else {
        args[1].to_string()
    })
    .expect("Something went wrong reading the file");

    let text_to_encode = contents.trim().to_string();

    println!("Clear text: {}", text_to_encode);
    println!("Encoded text: {}", encode::encode::encode(text_to_encode));
}
