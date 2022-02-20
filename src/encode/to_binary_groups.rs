use super::constants;

pub fn to_binary_groups(binary_string: String) -> (Vec<String>, usize) {
    let mut binary_string_copy = binary_string.as_str();

    let mut binary_groups: Vec<String> = Vec::new();
    let mut last_byte_length: usize = constants::BINARY_GROUPS_INITIAL_LENGTH;

    while binary_string_copy.chars().count() > 0 {
        let group_length = get_group_length(binary_string_copy);

        let (group, possible_last_byte_length) =
            create_binary_group(binary_string_copy, group_length);

        if possible_last_byte_length.is_some() {
            last_byte_length = possible_last_byte_length.unwrap()
        }

        binary_string_copy = &binary_string_copy[group_length..];
        binary_groups.push(group.to_string());
    }

    (binary_groups, last_byte_length)
}

fn get_group_length(binary_string: &str) -> usize {
    if binary_string.len() > constants::BINARY_GROUPS_INITIAL_LENGTH {
        constants::BINARY_GROUPS_INITIAL_LENGTH
    } else {
        binary_string.len()
    }
}

fn create_binary_group(binary_string: &str, group_length: usize) -> (String, Option<usize>) {
    if group_length != constants::BINARY_GROUPS_INITIAL_LENGTH {
        let mut zeros = "".to_string();

        for _ in 0..(constants::BINARY_GROUPS_INITIAL_LENGTH - group_length) {
            zeros = format!("{}{}", &zeros, constants::ZERO_STRING);
        }

        (
            format!("{}{}", &binary_string[..group_length], zeros).to_string(),
            Some(group_length),
        )
    } else {
        (binary_string[..group_length].to_string(), None)
    }
}
