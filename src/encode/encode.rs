
pub fn encode(clear_text: &str) -> String {
    let binary = to_binary(clear_text);
    let binary_groups = to_binary_groups(binary);
    let prefixed_binary_groups = prefix_with_zeros(binary_groups);
    let ascii_decimals = to_ascii_decimal(prefixed_binary_groups);
    let encoded_text = to_ascii_string(ascii_decimals);

    encoded_text
}


fn to_binary(text: &str) -> String {
    let name = text.to_string();
    let mut name_in_binary = "".to_string();

    for character in name.clone().into_bytes() {
        name_in_binary += &format!("0{:b}", character);
    }

    name_in_binary
}

fn to_binary_groups<'a>(binary: String) -> Vec<String> {
    let mut binary_copy = binary.as_str();

    let mut binary_groups: Vec<String> = Vec::new();

    while binary_copy.chars().count() > 0 {
        let group = &binary_copy[..6];
        binary_copy = &binary_copy[6..];
        binary_groups.push(group.to_string());
    }

    binary_groups
}

fn prefix_with_zeros(binary_groups: Vec<String>) -> Vec<String> {
    let mut prefixed_binary_groups: Vec<String> = Vec::new();

    for character in binary_groups {
        prefixed_binary_groups.push((&format!("00{}", character)).to_string());
    }

    prefixed_binary_groups
}

fn to_ascii_decimal(binary_groups: Vec<String>) -> Vec<String> {
    let mut ascii_decimals: Vec<String> = Vec::new();

    for element in binary_groups {
        ascii_decimals.push(u8::from_str_radix(&element, 2).unwrap().to_string());
    }

    ascii_decimals
}

fn to_ascii_string(ascii_decimals: Vec<String>) -> String {
    let mut ascii_string = "".to_string();

    for decimal in ascii_decimals {
        ascii_string += &"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars().nth(decimal.parse::<usize>().unwrap()).unwrap().to_string();
    }

    ascii_string.to_string()
}
