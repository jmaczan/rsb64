use crate::common::concatenate_items::concatenate_items;

pub fn concatenate_binary_groups(binary_groups: Vec<String>) -> String {
    binary_groups.into_iter().reduce(concatenate_items).unwrap()
}
