pub fn to_binary(text: String) -> String {
    text.clone()
        .into_bytes()
        .into_iter()
        .map(|character| {
            let binary_character = format!("0{:b}", character);

            let binary_length = binary_character.len();

            if binary_length != 8 {
                let mut zeros = "".to_string();

                for _ in 0..(8 - binary_length) {
                    zeros = format!("{}{}", &zeros, "0");
                }

                format!("{}{}", zeros, binary_character)
            } else {
                binary_character
            }
        })
        .collect::<Vec<String>>()
        .into_iter()
        .reduce(|name_in_binary, binary_character| {
            format!("{}{}", name_in_binary, binary_character)
        })
        .unwrap()
}
