use super::to_decimal_groups::to_decimal_groups;
use super::to_string_groups::to_string_groups;
use crate::decode::binary_to_ascii::binary_to_ascii;
use crate::decode::concatenate_binary_groups::concatenate_binary_groups;
use crate::decode::remove_zeros_prefixes::remove_zeros_prefixes;
use crate::decode::to_binary_groups::split_binary_string_to_binary_groups;
use crate::decode::to_binary_groups::to_binary_groups;

pub fn decode(clear_text: String) -> String {
    let string_groups = to_string_groups(clear_text);
    let decimal_groups = to_decimal_groups(string_groups);
    let binary_groups = to_binary_groups(decimal_groups);
    let unprefixed_binary_groups = remove_zeros_prefixes(binary_groups);
    let concatenated_binary_group = concatenate_binary_groups(unprefixed_binary_groups);
    let binary_groups = split_binary_string_to_binary_groups(concatenated_binary_group);
    let decoded_text = binary_to_ascii(binary_groups);

    decoded_text.to_string()
}
