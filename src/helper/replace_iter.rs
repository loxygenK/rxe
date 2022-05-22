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

