pub fn remove_zeros_prefixes(strings_to_prefix: Vec<String>) -> Vec<String> {
    strings_to_prefix
        .into_iter()
        .map(|string| remove_zeros_prefix(string))
        .collect::<Vec<String>>()
}

fn remove_zeros_prefix(string: String) -> String {
    string[2..8].to_string()
}
