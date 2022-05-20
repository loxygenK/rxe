use super::Value;

pub struct NumberValue(f64);
impl Value for NumberValue {
    type ValueType = f64;

    fn get(&self) -> Self::ValueType {
        self.0
    }
}
impl NumberValue {
    pub fn new(value: f64) -> Self {
        NumberValue(value)
    }
}
