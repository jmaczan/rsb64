pub fn to_ascii_decimal(binary_groups: Vec<String>) -> Vec<String> {
    let mut ascii_decimals: Vec<String> = Vec::new();

    for element in binary_groups {
        let ascii_decimal: String = u8::from_str_radix(&element, 2).unwrap().to_string();
        println!("ascii decimal: {}", ascii_decimal);
        ascii_decimals.push(ascii_decimal);
    }

    ascii_decimals
}