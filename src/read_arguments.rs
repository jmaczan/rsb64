use std::env;
use std::fs;
use std::process::exit;

use super::constants;

pub fn read_arguments() -> (String, constants::Action) {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 || arguments.len() > 3 {
        println!("Please run rsb64 in following way:\n");
        println!("To encode: ./rsb64 file_name");
        println!("To decode: ./rsb64 -decode file_name");
        println!("\nIn place of file_name put a name of file containing the text to encode/decode");
        exit(1);
    }

    let action = if arguments.len() == 3 {
        constants::Action::Decode
    } else {
        constants::Action::Encode
    };

    let file_content = fs::read_to_string(if action == constants::Action::Decode {
        arguments[2].to_string()
    } else {
        arguments[1].to_string()
    })
    .expect("Something went wrong reading the file");

    (file_content.to_string(), action)
}
