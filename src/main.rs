mod constants;
mod read_arguments;

pub mod decode;
pub mod encode;

fn main() {
    let (file_content, action) = read_arguments::read_arguments();

    if action == constants::Action::Decode {
        println!("Decoded text: {}", decode::decode::decode(file_content));
    } else {
        println!("Encoded text: {}", encode::encode::encode(file_content));
    }
}
