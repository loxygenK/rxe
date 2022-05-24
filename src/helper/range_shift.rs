use std::ops::{Add, Range};

pub trait RangeShift<T>
    where T: Add<T, Output = T> + Copy
{
    fn shift(&self, n: T) -> Range<T>;
}
impl<T> RangeShift<T> for Range<T>
    where T: Add<T, Output = T> + Copy
{
    fn shift(&self, n: T) -> Range<T> {
        Range { start: self.start + n, end: self.end + n }
    }
}