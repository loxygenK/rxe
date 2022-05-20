use super::Value;

pub struct TextValue(String);
impl Value for TextValue {
    type ValueType = String;

    fn get(&self) -> Self::ValueType {
        self.0.to_owned()
    }
}
impl TextValue {
    pub fn new(value: String) -> TextValue {
        Self(value)
    }
}
