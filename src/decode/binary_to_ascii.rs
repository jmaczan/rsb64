use crate::common::constants::ASCII_DECIMAL_RADIX;
use crate::decode::concatenate_binary_groups::concatenate_binary_groups;

pub fn binary_to_ascii(binary_groups: Vec<String>) -> String {
    concatenate_binary_groups(
        binary_groups
            .into_iter()
            .map(|group| {
                char::from_u32(u32::from_str_radix(&group, ASCII_DECIMAL_RADIX).unwrap())
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<String>>(),
    )
}
