use super::constants;

pub fn to_binary_groups(binary_string: String) -> (Vec<String>, usize) {
    let mut binary_string_copy = binary_string.as_str();

    let mut binary_groups: Vec<String> = Vec::new();
    let mut last_byte_length: usize = constants::BINARY_GROUPS_INITIAL_LENGTH;

    while binary_string_copy.chars().count() > 0 {
        let group_length = if binary_string_copy.len() > constants::BINARY_GROUPS_INITIAL_LENGTH {
            constants::BINARY_GROUPS_INITIAL_LENGTH
        } else {
            binary_string_copy.len()
        };
        let group = if group_length != constants::BINARY_GROUPS_INITIAL_LENGTH {
            let mut zeros = "".to_string();
            last_byte_length = group_length;

            for _ in 0..(constants::BINARY_GROUPS_INITIAL_LENGTH - group_length) {
                zeros = format!("{}{}", &zeros, constants::ZERO_STRING);
            }

            format!("{}{}", &binary_string_copy[..group_length], zeros).to_string()
        } else {
            binary_string_copy[..group_length].to_string()
        };
        println!("binary group: {}", group);
        binary_string_copy = &binary_string_copy[group_length..];
        binary_groups.push(group.to_string());
    }

    (binary_groups, last_byte_length)
}
