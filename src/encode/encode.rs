pub fn encode(clear_text: &str) {
    let binary = to_binary(clear_text);
    let binary_groups = to_binary_groups(binary);
    let prefixed_binary_groups = prefix_with_zeros(binary_groups);
    
    for element in prefixed_binary_groups {
        println!("{}", element)
    }
}

fn to_binary(text: &str) -> String {
    let name = text.to_string();
    let mut name_in_binary = "".to_string();

    for character in name.clone().into_bytes() {
        name_in_binary += &format!("0{:b}", character);
    }

    name_in_binary
}

fn to_binary_groups<'a>(binary: String) -> Vec<String> {
    let mut binary_copy = binary.as_str();

    let mut binary_groups: Vec<String> = Vec::new();

    while binary_copy.chars().count() > 0 {
        let group = &binary_copy[..6];
        binary_copy = &binary_copy[6..];
        binary_groups.push(group.to_string());
    }

    binary_groups
}

fn prefix_with_zeros(binary_groups: Vec<String>) -> Vec<String> {
    let mut prefixed_binary_groups: Vec<String> = Vec::new();

    for character in binary_groups {
        prefixed_binary_groups.push((&format!("00{}", character)).to_string());
        // println!("{}", &format!("00{}", character))
    }

    prefixed_binary_groups
}
