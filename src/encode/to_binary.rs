pub fn to_binary(text: String) -> String {
    let name = text;
    let mut name_in_binary = "".to_string();

    for character in name.clone().into_bytes() {
        let mut binary_character = format!("0{:b}", character);

        let binary_length = binary_character.len();

        binary_character = if binary_length != 8 {
            let mut zeros = "".to_string();

            for _ in 0..(8 - binary_length) {
                zeros = format!("{}{}", &zeros, "0");
            }

            format!("{}{}", zeros, binary_character)
        } else {
            binary_character
        };

        println!("binary_character: {}", binary_character);
        name_in_binary += &binary_character;
    }
    println!("to_binary: {}", name_in_binary);
    name_in_binary
}
