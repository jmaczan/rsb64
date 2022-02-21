use super::constants;
use super::prefix_with_zeros::prefix_with_zeros;

pub fn to_binary_character(character: u8) -> String {
    let binary_character = format!("0{:b}", character);

    prefix_with_zeros(binary_character, constants::BINARY_GROUPS_DESIRED_LENGTH)
}
