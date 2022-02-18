pub fn encode(clear_text: String) -> String {
    let binary = to_binary(clear_text);
    let binary_groups = to_binary_groups(binary);
    let prefixed_binary_groups = prefix_with_zeros(binary_groups);
    let ascii_decimals = to_ascii_decimal(prefixed_binary_groups);
    let ascii_string = to_ascii_string(ascii_decimals);
    println!("{}", ascii_string.len());
    ascii_string
}

fn to_binary(text: String) -> String {
    let name = text;
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
        let group_length = if binary_copy.len() > 6 { 6 } else { binary_copy.len() };
        let group = if group_length != 6 {
            println!("{}", group_length);
            if group_length == 2 {
                format!("{}{}", &binary_copy[..group_length], "0000").to_string()
            } else {
                format!("{}{}", &binary_copy[..group_length], "00").to_string()
            }
        } else {
            binary_copy[..group_length].to_string()
        };
        binary_copy = &binary_copy[group_length..];
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
        ascii_string += &"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .chars()
            .nth(decimal.parse::<usize>().unwrap())
            .unwrap()
            .to_string();
    }

    ascii_string.to_string()
}
