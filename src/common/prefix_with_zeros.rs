use super::constants;

pub fn prefix_with_zeros(string_to_prefix: String, desired_length: usize) -> String {
    let length = string_to_prefix.len();

    if length != desired_length {
        let mut zeros = "".to_string();

        for _ in 0..(desired_length - length) {
            zeros = format!("{}{}", &zeros, constants::ZERO_STRING);
        }

        format!("{}{}", zeros, string_to_prefix)
    } else {
        string_to_prefix
    }
}
