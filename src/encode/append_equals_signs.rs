pub fn append_equals_signs(ascii_string: String, last_byte_length: usize) -> String {
    if last_byte_length == 0 {
        ascii_string
    } else if last_byte_length == 4 {
        format!("{}{}", &ascii_string, "=").to_string()
    } else {
        format!("{}{}", &ascii_string, "==").to_string()
    }
}