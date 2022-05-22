use std::ops::Deref;

pub trait Identify {
    fn get_identifier(&self) -> String;
}

#[derive(Debug)]
pub struct IdBox<T>(Box<T>) where T: ?Sized + Identify;
impl<T> PartialEq for IdBox<T> where T: ?Sized + Identify {
    fn eq(&self, other: &Self) -> bool {
        *self.get_identifier() == *other.get_identifier()
    }
}
impl<T> Deref for IdBox<T> where T: ?Sized + Identify {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl<T> IdBox<T> where T: ?Sized + Identify {
    pub fn new(value: Box<T>) -> IdBox<T> {
        IdBox(value)
    }
}
