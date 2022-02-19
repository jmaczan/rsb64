pub fn to_ascii_string(ascii_decimals: Vec<String>) -> String {
    let mut ascii_string = "".to_string();

    for decimal in ascii_decimals {
        ascii_string += &"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .chars()
            .nth(decimal.parse::<usize>().unwrap())
            .unwrap()
            .to_string();
    }

    ascii_string.to_string()
}
