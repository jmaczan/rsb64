pub fn text_as_binary(text: Vec<char>) -> Vec<u32> {
    let decimals = text_as_decimals(text);
    decimals_as_binary(decimals)
}

fn text_as_decimals(text: Vec<char>) -> Vec<u8> {
    let mut decimals: Vec<u8> = Vec::new();
    for character in text {
        if character.is_ascii() {
            decimals.push(character as u8);
        }
    }
    decimals
}

fn decimals_as_binary(decimals: Vec<u8>) -> Vec<u32> {
    let mut binary: Vec<u32> = Vec::new();

    for decimal in decimals {
        binary.push(decimal_to_binary(decimal))
    }

    binary
}

fn decimal_to_binary(decimal: u8) -> u32 {
    format!("0{:b}", decimal).parse().unwrap()
}
