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

pub struct Replace<I, T, F>
    where I: Iterator<Item = T>,
          F: Fn() -> T
{
    iter: I,
    from: T,
    to: F
}
impl<I, T, F> Iterator for Replace<I, T, F>
    where I: Iterator<Item = T>,
          T: PartialEq,
          F: Fn() -> T
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.iter.next()?;

        if next_value == self.from {
            Some((self.to)())
        } else {
            Some(next_value)
        }
    }
}

pub trait ReplaceIter<T, F>: Iterator<Item = T>
    where Self: Sized,
          T: PartialEq, Self: Sized,
          F: Fn() -> T
{
    fn replace(self, from: T, to: F) -> Replace<Self, T, F>;
}
impl<I, T, F> ReplaceIter<T, F> for I
    where I: Iterator<Item = T>,
          T: PartialEq,
          F: Fn() -> T
{
    fn replace(self, from: T, to: F) -> Replace<Self, T, F> {
        Replace { iter: self, from, to }
    }
}

