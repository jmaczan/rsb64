pub fn to_string_groups(string: String) -> Vec<String> {
    string
        .split_terminator("")
        .skip(1)
        .map(|group| group.to_string())
        .collect::<Vec<String>>()
}
