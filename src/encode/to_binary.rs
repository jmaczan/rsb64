use super::prefix_with_zeros::prefix_with_zeros;
use super::constants;

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

fn to_binary_character(character: u8) -> String {
    let binary_character = format!("0{:b}", character);

    prefix_with_zeros(binary_character, constants::BINARY_GROUPS_DESIRED_LENGTH)
}

fn concatenate_items(name_in_binary: String, binary_character: String) -> String {
    format!("{}{}", name_in_binary, binary_character)
}
