pub fn to_ascii_string(ascii_decimals: Vec<String>) -> String {
    let all_ascii_characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut ascii_decimals_copy: Vec<String> = vec!["".to_string()];
    ascii_decimals_copy.extend(ascii_decimals);
    ascii_decimals_copy
        .into_iter()
        .reduce(|ascii_strings, ascii_string| {
            format!(
                "{}{}",
                ascii_strings,
                &all_ascii_characters
                    .chars()
                    .nth(ascii_string.parse::<usize>().unwrap())
                    .unwrap()
                    .to_string()
            )
        })
        .unwrap()
}
