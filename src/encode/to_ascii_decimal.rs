pub fn to_ascii_decimal(binary_groups: Vec<String>) -> Vec<String> {
    binary_groups
        .into_iter()
        .map(|decimal| u8::from_str_radix(&decimal, 2).unwrap().to_string())
        .collect::<Vec<String>>()
}
