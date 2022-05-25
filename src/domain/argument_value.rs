#[derive(Debug, PartialEq, Clone)]
pub enum ArgumentValue {
    Text(String),
    Flag(bool),
    Number(f64),
}
