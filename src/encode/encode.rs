pub fn encode(clear_text: &str) -> String {
    let binary = to_binary(clear_text);
    let binary_groups = to_binary_groups(&binary);
    
    for character in binary_groups {
        println!("{}", &format!("00{}", character))
    }
    binary
}

fn to_binary(text: &str) -> String {
    let name = text.to_string();
    let mut name_in_binary = "".to_string();

    for character in name.clone().into_bytes() {
        name_in_binary += &format!("0{:b}", character);
    }

    name_in_binary
}

fn to_binary_groups<'a>(binary: &'a String) -> Vec<&'a str> {
    let mut binary_copy = binary.as_str();

    let mut binary_groups: Vec<&str> = Vec::new();

    while binary_copy.chars().count() > 0 {
        let group = &binary_copy[..6];
        binary_copy = &binary_copy[6..];
        println!("{}", group);
        binary_groups.push(group);
    }

    binary_groups
}

fn prefix_with_zeros(binary_groups: Vec<&str>) -> Vec<&str> {
    let mut prefixed_binary_groups: Vec<&str> = Vec::new();

    for character in binary_groups {
        prefixed_binary_groups.push(&format!("00{}", character));
        // println!("{}", &format!("00{}", character))
    }

    prefixed_binary_groups
}
