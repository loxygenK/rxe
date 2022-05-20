use super::Value;

pub struct FlagValue(bool);
impl Value for FlagValue {
    type ValueType = bool;

    fn get(&self) -> Self::ValueType {
        self.0
    }
}
impl FlagValue {
    pub fn new(value: bool) -> Self {
        FlagValue(value)
    }
}
