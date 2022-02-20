use super::constants::ALLOWED_ASCII_CHARACTERS;

pub fn to_ascii_string(ascii_decimals: Vec<String>) -> String {
    let mut ascii_decimals_copy: Vec<String> = vec!["".to_string()];
    ascii_decimals_copy.extend(ascii_decimals);
    ascii_decimals_copy
        .into_iter()
        .reduce(|ascii_strings, ascii_string| {
            format!(
                "{}{}",
                ascii_strings,
                ALLOWED_ASCII_CHARACTERS
                    .chars()
                    .nth(ascii_string.parse::<usize>().unwrap())
                    .unwrap()
                    .to_string()
            )
        })
        .unwrap()
}
