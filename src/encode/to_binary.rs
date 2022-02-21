use crate::common::concatenate_items::concatenate_items;
use crate::common::to_binary_character::to_binary_character;

pub fn to_binary(text: String) -> String {
    text.clone()
        .into_bytes()
        .into_iter()
        .map(to_binary_character)
        .collect::<Vec<String>>()
        .into_iter()
        .reduce(concatenate_items)
        .unwrap()
}
