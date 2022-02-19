pub fn to_binary_groups(binary: String) -> (Vec<String>, usize) {
    let mut binary_copy = binary.as_str();

    let mut binary_groups: Vec<String> = Vec::new();
    let mut last_byte_length: usize = 0;

    while binary_copy.chars().count() > 0 {
        let group_length = if binary_copy.len() > 6 {
            6
        } else {
            binary_copy.len()
        };
        let group = if group_length != 6 {
            let mut zeros = "".to_string();
            last_byte_length = group_length;

            for _ in 0..(6 - group_length) {
                zeros = format!("{}{}", &zeros, "0");
            }

            format!("{}{}", &binary_copy[..group_length], zeros).to_string()
        } else {
            binary_copy[..group_length].to_string()
        };
        println!("binary group: {}", group);
        binary_copy = &binary_copy[group_length..];
        binary_groups.push(group.to_string());
    }

    (binary_groups, last_byte_length)
}