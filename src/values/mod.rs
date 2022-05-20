pub mod number;
pub mod text;
pub mod flag;

pub trait Value {
    type ValueType;

    fn get(&self) -> Self::ValueType;
}
