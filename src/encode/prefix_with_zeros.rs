use super::constants::ZERO_STRING;

pub fn prefix_all_with_zeros(strings_to_prefix: Vec<String>, desired_length: usize) -> Vec<String> {
    strings_to_prefix
        .into_iter()
        .map(|string| prefix_with_zeros(string, desired_length))
        .collect::<Vec<String>>()
}

pub fn prefix_with_zeros(string_to_prefix: String, desired_length: usize) -> String {
    let length = string_to_prefix.len();

    if length != desired_length {
        let mut zeros = "".to_string();

        for _ in 0..(desired_length - length) {
            zeros = format!("{}{}", &zeros, ZERO_STRING);
        }

        format!("{}{}", zeros, string_to_prefix)
    } else {
        string_to_prefix
    }
}
