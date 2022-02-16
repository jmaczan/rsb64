mod encode;

use encode::encode;

fn main() {
    let hello: &str = "Hello, world!";
    println!("{}", hello);
    encode(hello);
}
