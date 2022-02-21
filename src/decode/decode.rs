use super::to_decimal_groups::to_decimal_groups;
use super::to_string_groups::to_string_groups;
use crate::decode::remove_zeros_prefixes::remove_zeros_prefixes;
use crate::decode::to_binary_groups::to_binary_groups;

pub fn decode(clear_text: String) -> String {
    let string_groups = to_string_groups(clear_text);

    for character in (&string_groups).into_iter() {
        println!("character: {}", character);
    }

    let decimal_groups = to_decimal_groups(string_groups);

    for character in decimal_groups.clone().into_iter() {
        println!("decimal character: {}", character);
    }

    let binary_groups = to_binary_groups(decimal_groups);

    for character in binary_groups.clone().into_iter() {
        println!("binary character: {}", character);
    }

    let unprefixed_binary_groups = remove_zeros_prefixes(binary_groups);

    for character in unprefixed_binary_groups.clone().into_iter() {
        println!("unprefixed binary character: {}", character);
    }

    "test".to_string()
}
