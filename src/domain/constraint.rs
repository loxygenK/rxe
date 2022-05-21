#[derive(Debug, PartialEq, Eq)]
pub enum Constraints {
    Text,
    Flag,
    Number,
    Choice(Vec<String>),
}
