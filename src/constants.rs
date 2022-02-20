#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Encode,
    Decode,
}

pub const MIN_ARGUMENTS_LENGTH: usize = 2;
pub const MAX_ARGUMENTS_LENGTH: usize = 3;