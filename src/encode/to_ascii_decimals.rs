use crate::common::constants::ASCII_DECIMAL_RADIX;

pub fn to_ascii_decimals(binary_groups: Vec<String>) -> Vec<String> {
    binary_groups
        .into_iter()
        .map(to_ascii_decimal)
        .collect::<Vec<String>>()
}

fn to_ascii_decimal(binary_group: String) -> String {
    u8::from_str_radix(&binary_group, ASCII_DECIMAL_RADIX)
        .unwrap()
        .to_string()
}
