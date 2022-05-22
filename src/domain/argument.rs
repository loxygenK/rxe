use crate::domain::Constraints;

#[derive(Debug, PartialEq, Eq)]
pub struct Argument {
    pub name: String,
    pub short_hand: Option<String>,
    pub constraint: Constraints,
    pub multi: bool,
}
