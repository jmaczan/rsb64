use crate::common::prefix_with_zeros::prefix_with_zeros;

pub fn prefix_all_with_zeros(strings_to_prefix: Vec<String>, desired_length: usize) -> Vec<String> {
    strings_to_prefix
        .into_iter()
        .map(|string| prefix_with_zeros(string, desired_length))
        .collect::<Vec<String>>()
}
