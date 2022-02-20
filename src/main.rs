mod constants;
mod read_arguments;

pub mod decode;
pub mod encode;

fn main() {
    let (file_content, action) = read_arguments::read_arguments();

    if action == constants::Action::Decode {
        println!("{}", decode::decode::decode(file_content));
    } else {
        println!("{}", encode::encode::encode(file_content));
    }
}
