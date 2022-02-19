pub fn prefix_with_zeros(binary_groups: Vec<String>) -> Vec<String> {
    let mut prefixed_binary_groups: Vec<String> = Vec::new();

    for character in binary_groups {
        println!(
            "prefixed with zeros: {}",
            (&format!("00{}", character)).to_string()
        );
        prefixed_binary_groups.push((&format!("00{}", character)).to_string());
    }

    prefixed_binary_groups
}
