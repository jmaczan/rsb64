use crate::common::to_binary_character::to_binary_character;

pub fn to_binary_groups(decimal_groups: Vec<usize>) -> Vec<String> {
    decimal_groups
        .into_iter()
        .map(|group| to_binary_character(group as u8))
        .collect()
}
