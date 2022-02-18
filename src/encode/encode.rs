pub fn encode(clear_text: String) -> String {
    let binary = to_binary(clear_text);
    let (binary_groups, last_byte_length) = to_binary_groups(binary);
    let prefixed_binary_groups = prefix_with_zeros(binary_groups);
    let ascii_decimals = to_ascii_decimal(prefixed_binary_groups);
    let ascii_string = to_ascii_string(ascii_decimals);

    append_equals_signs(ascii_string, last_byte_length)
}

fn to_binary(text: String) -> String {
    let name = text;
    let mut name_in_binary = "".to_string();

    for character in name.clone().into_bytes() {
        let mut binary_character = format!("0{:b}", character);
        if binary_character == "0100000" {
            binary_character = "00100000".to_string();
        }
        println!("binary_character: {}", binary_character);
        name_in_binary += &binary_character;
    }
    println!("to_binary: {}", name_in_binary);
    name_in_binary
}

fn to_binary_groups(binary: String) -> (Vec<String>, usize) {
    let mut binary_copy = binary.as_str();

    let mut binary_groups: Vec<String> = Vec::new();
    let mut last_byte_length: usize = 0;

    while binary_copy.chars().count() > 0 {
        let group_length = if binary_copy.len() > 6 {
            6
        } else {
            binary_copy.len()
        };
        let group = if group_length != 6 {
            let mut zeros = "".to_string();
            last_byte_length = group_length;

            for _ in 0..(6 - group_length) {
                let a = format!("{}{}", &zeros, "0");
                zeros = a;
            }

            format!("{}{}", &binary_copy[..group_length], zeros).to_string()
        } else {
            binary_copy[..group_length].to_string()
        };
        println!("binary group: {}", group);
        binary_copy = &binary_copy[group_length..];
        binary_groups.push(group.to_string());
    }

    (binary_groups, last_byte_length)
}

fn prefix_with_zeros(binary_groups: Vec<String>) -> Vec<String> {
    let mut prefixed_binary_groups: Vec<String> = Vec::new();

    for character in binary_groups {
        println!("prefixed with zeros: {}", (&format!("00{}", character)).to_string());
        prefixed_binary_groups.push((&format!("00{}", character)).to_string());
    }

    prefixed_binary_groups
}

fn to_ascii_decimal(binary_groups: Vec<String>) -> Vec<String> {
    let mut ascii_decimals: Vec<String> = Vec::new();

    for element in binary_groups {
        let ascii_decimal: String = u8::from_str_radix(&element, 2).unwrap().to_string();
        println!("ascii decimal: {}", ascii_decimal);
        ascii_decimals.push(ascii_decimal);
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

fn append_equals_signs(ascii_string: String, last_byte_length: usize) -> String {
    if last_byte_length == 0 {
        ascii_string
    } else if last_byte_length == 4 {
        format!("{}{}", &ascii_string, "=").to_string()
    } else {
        format!("{}{}", &ascii_string, "==").to_string()
    }
}
