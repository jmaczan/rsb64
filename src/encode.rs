pub fn encode(clear_text: &str) {
    let clear_text_characters: Vec<char> = clear_text.chars().collect();
    for character in clear_text_characters {
        println!("{}", character)
    }
}