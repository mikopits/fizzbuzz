use std::fmt;
use std::ops::Rem;

#[derive(Debug, PartialEq)]
pub enum FizzOrBuzz<T> {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(T),
}

impl<T: fmt::Display> fmt::Display for FizzOrBuzz<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FizzOrBuzz::Fizz => f.pad("Fizz"),
            FizzOrBuzz::Buzz => f.pad("Buzz"),
            FizzOrBuzz::FizzBuzz => f.pad("FizzBuzz"),
            FizzOrBuzz::Number(ref n) => fmt::Display::fmt(n, f),
        }
    }
}

pub trait Inc {
    type Output;

    fn inc(self) -> Self::Output;
}

macro_rules! inc_impl {
    ($($t:ty)*) => ($(
        impl Inc for $t {
            type Output = $t;

            #[inline]
            fn inc(self) -> $t { self + 1 as $t }
        }
    )*)
}

inc_impl! { usize u8 u16 u32 u64 isize i16 i32 i64 f32 f64 }

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! zero_impl {
    ($($t:ty)*) => ($(
        impl Zero for $t {
            #[inline]
            fn zero() -> $t { 0 as $t }
        }
    )*)
}

zero_impl! { usize u8 u16 u32 u64 isize i16 i32 i64 f32 f64 }

pub trait Three {
    fn three() -> Self;
}

macro_rules! three_impl {
    ($($t:ty)*) => ($(
        impl Three for $t {
            #[inline]
            fn three() -> $t { 3 as $t }
        }
    )*)
}

three_impl! { usize u8 u16 u32 u64 isize i16 i32 i64 f32 f64 }

pub trait Five {
    fn five() -> Self;
}

macro_rules! five_impl {
    ($($t:ty)*) => ($(
        impl Five for $t {
            #[inline]
            fn five() -> $t { 5 as $t }
        }
    )*)
}

five_impl! { usize u8 u16 u32 u64 isize i16 i32 i64 f32 f64 }

pub struct FizzBuzz<T> {
    index: T,
    end: T,
}

impl<T: Copy + PartialEq + PartialOrd + Rem<Output = T>
    + Inc<Output = T> + Zero + Three + Five> FizzBuzz<T>
{
    pub fn new(start: T, end: T) -> FizzBuzz<T> {
        FizzBuzz {
            index: start,
            end: end.inc(),
        }
    }

    #[inline]
    pub fn print(&mut self)
        where T: fmt::Display,
    {
        loop {
            match self.next() {
                Some(fb) => println!("{}", fb),
                None => return,
            }
        }
    }

    #[inline]
    pub fn to_vec(&mut self) -> Vec<FizzOrBuzz<T>> {
        let mut v = Vec::new();
        loop {
            match self.next() {
                Some(fb) => v.push(fb),
                None => return v,
            }
        }
    }
}

impl<T: Copy + PartialEq + PartialOrd + Rem<Output = T>
    + Inc<Output = T> + Zero + Three + Five> Iterator for FizzBuzz<T>
{
    type Item = FizzOrBuzz<T>;

    fn next(&mut self) -> Option<FizzOrBuzz<T>> {
        let rem_3 = self.index.rem(T::three());
        let rem_5 = self.index.rem(T::five());
        let result;
        if rem_3 == T::zero() && rem_5 == T::zero() {
            result = FizzOrBuzz::FizzBuzz;
        } else if rem_3 == T::zero() {
            result = FizzOrBuzz::Fizz;
        } else if rem_5 == T::zero() {
            result = FizzOrBuzz::Buzz;
        } else {
            result = FizzOrBuzz::Number(self.index);
        }
        if self.index >= self.end {
            None
        } else {
            self.index = self.index.inc();
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use ::{FizzBuzz, FizzOrBuzz};

    #[test]
    fn test_fizzbuzz() {
        let mut fb = FizzBuzz::new(1, 15);
        let fb15 = vec![FizzOrBuzz::Number(1),
                        FizzOrBuzz::Number(2),
                        FizzOrBuzz::Fizz,
                        FizzOrBuzz::Number(4),
                        FizzOrBuzz::Buzz,
                        FizzOrBuzz::Fizz,
                        FizzOrBuzz::Number(7),
                        FizzOrBuzz::Number(8),
                        FizzOrBuzz::Fizz,
                        FizzOrBuzz::Buzz,
                        FizzOrBuzz::Number(11),
                        FizzOrBuzz::Fizz,
                        FizzOrBuzz::Number(13),
                        FizzOrBuzz::Number(14),
                        FizzOrBuzz::FizzBuzz];
        assert_eq!(fb.to_vec(), fb15);
    }
}
