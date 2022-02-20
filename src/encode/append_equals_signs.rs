use super::constants;

pub fn append_equals_signs(ascii_string: String, last_byte_length: usize) -> String {
    if last_byte_length == constants::BINARY_GROUPS_INITIAL_LENGTH {
        ascii_string
    } else if last_byte_length == constants::SINGLE_EQUALS_SIGN_LENGTH {
        format!("{}{}", &ascii_string, constants::SINGLE_EQUALS_SIGN).to_string()
    } else {
        format!("{}{}", &ascii_string, constants::DOUBLE_EQUALS_SIGN).to_string()
    }
}
