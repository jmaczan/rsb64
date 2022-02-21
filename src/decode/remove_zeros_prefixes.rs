use crate::common::constants::BINARY_GROUPS_DESIRED_LENGTH;
use crate::common::constants::ZEROS_PREFIX_LENGTH;

pub fn remove_zeros_prefixes(strings_to_prefix: Vec<String>) -> Vec<String> {
    strings_to_prefix
        .into_iter()
        .map(|string| remove_zeros_prefix(string))
        .collect::<Vec<String>>()
}

fn remove_zeros_prefix(string: String) -> String {
    string[ZEROS_PREFIX_LENGTH..BINARY_GROUPS_DESIRED_LENGTH].to_string()
}
