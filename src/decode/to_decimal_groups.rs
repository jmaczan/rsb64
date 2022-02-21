use crate::common::constants;

pub fn to_decimal_groups(string_groups: Vec<String>) -> Vec<usize> {
    string_groups
        .into_iter()
        .filter(|character| character != &constants::SINGLE_EQUALS_SIGN)
        .map(to_decimal_group)
        .collect()
}

fn to_decimal_group(string: String) -> usize {
    constants::ALLOWED_ASCII_CHARACTERS
        .chars()
        .position(|allowed_character| allowed_character == string.chars().nth(0).unwrap())
        .unwrap()
}
