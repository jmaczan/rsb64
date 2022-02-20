use super::append_equals_signs::append_equals_signs;
use super::prefix_with_zeros::prefix_all_with_zeros;
use super::to_ascii_decimals::to_ascii_decimals;
use super::to_ascii_string::to_ascii_string;
use super::to_binary::to_binary;
use super::to_binary_groups::to_binary_groups;
use super::constants::BINARY_GROUPS_DESIRED_LENGTH;

pub fn encode(clear_text: String) -> String {
    let binary = to_binary(clear_text);
    let (binary_groups, last_byte_length) = to_binary_groups(binary);
    let prefixed_binary_groups = prefix_all_with_zeros(binary_groups, BINARY_GROUPS_DESIRED_LENGTH);
    let ascii_decimals = to_ascii_decimals(prefixed_binary_groups);
    let ascii_string = to_ascii_string(ascii_decimals);

    append_equals_signs(ascii_string, last_byte_length)
}
