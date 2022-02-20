use super::constants::ASCII_DECIMAL_RADIX;

pub fn to_ascii_decimal(binary_groups: Vec<String>) -> Vec<String> {
    binary_groups
        .into_iter()
        .map(|decimal| u8::from_str_radix(&decimal, ASCII_DECIMAL_RADIX).unwrap().to_string())
        .collect::<Vec<String>>()
}
