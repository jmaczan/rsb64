use super::text_as_binary::text_as_binary;

pub fn encode(clear_text: &str) -> Vec<u32> {
    let clear_text_characters: Vec<char> = string_as_characters(clear_text);
    text_as_binary(clear_text_characters)
}

fn string_as_characters(string: &str) -> Vec<char> {
    string.chars().collect()
}
