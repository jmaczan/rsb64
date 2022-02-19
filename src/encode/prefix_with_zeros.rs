pub fn prefix_all_with_zeros(strings_to_prefix: Vec<String>, desired_length: usize) -> Vec<String> {
    let mut prefixed_strings: Vec<String> = Vec::new();

    for string_to_prefix in strings_to_prefix {
        println!(
            "prefixed with zeros: {}",
            (&format!("00{}", string_to_prefix)).to_string()
        );
        prefixed_strings.push(prefix_with_zeros(string_to_prefix, desired_length));
    }

    prefixed_strings
}

pub fn prefix_with_zeros(string_to_prefix: String, desired_length: usize) -> String {
    let length = string_to_prefix.len();

    if length != desired_length {
        let mut zeros = "".to_string();

        for _ in 0..(desired_length - length) {
            zeros = format!("{}{}", &zeros, "0");
        }

        format!("{}{}", zeros, string_to_prefix)
    } else {
        string_to_prefix
    }
}