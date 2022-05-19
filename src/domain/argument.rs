use super::constraint::Constraint;

#[derive(Debug, PartialEq, Eq)]
pub struct Argument {
    pub name: String,
    pub short_hand: Option<String>,
    pub constraint: Constraint,
    pub multi: bool
}
