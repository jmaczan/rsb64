use crate::common::to_binary_character::to_binary_character;

pub fn to_binary_groups(decimal_groups: Vec<usize>) -> Vec<String> {
    decimal_groups
        .into_iter()
        .map(|group| to_binary_character(group as u8))
        .collect()
}

pub fn split_binary_string_to_binary_groups(binary_string: String) -> Vec<String> {
    let mut binary_string_copy = binary_string.as_str();

    let mut binary_groups: Vec<String> = Vec::new();

    while binary_string_copy.chars().count() > 0 {
        let group_length = get_group_length(binary_string_copy);

        let group = binary_string_copy[..group_length].to_string();

        binary_string_copy = &binary_string_copy[group_length..];
        binary_groups.push(group.to_string());
    }

    binary_groups
        .into_iter()
        .filter(|group| group.len() == 8)
        .collect()
}

fn get_group_length(binary_string: &str) -> usize {
    if binary_string.len() > 8 {
        8
    } else {
        binary_string.len()
    }
}
