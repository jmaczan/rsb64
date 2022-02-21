use super::constants::ALLOWED_ASCII_CHARACTERS;
use super::constants::SINGLE_EQUALS_SIGN;

pub fn decode(clear_text: String) -> String {
    let characters = clear_text
        .split_terminator("")
        .skip(1)
        .collect::<Vec<&str>>();

    for character in (&characters).into_iter() {
        println!("character: {}", character);
    }

    let decimal_characters = characters
        .into_iter()
        .filter(|character| character != &SINGLE_EQUALS_SIGN)
        .map(|character| {
            ALLOWED_ASCII_CHARACTERS
                .chars()
                .position(|allowed_character| {
                    allowed_character == character.chars().nth(0).unwrap()
                })
                .unwrap()
        });

    for character in decimal_characters.clone().into_iter() {
        println!("decimal character: {}", character);
    }

    for character in decimal_characters.clone().into_iter() {
        println!("binary character: {}", format!("0{:b}", character));
    }

    clear_text
}
