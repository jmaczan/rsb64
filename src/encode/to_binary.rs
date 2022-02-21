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

fn concatenate_items(name_in_binary: String, binary_character: String) -> String {
    format!("{}{}", name_in_binary, binary_character)
}
