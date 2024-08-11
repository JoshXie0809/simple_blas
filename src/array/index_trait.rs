use std::ops::{Index, IndexMut};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

use super::Array;

impl<T> Index<usize> for Array<T> 
where T:
Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Array::Array1D { arr } => {
                &(**arr)[index]
            }
            _ => panic!("Mismatched Index"),
        }
    }
}

impl<T> IndexMut<usize> for Array<T> 
where T:
Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Array::Array1D { arr } => {
                &mut (**arr)[index]
            }
            _ => panic!("Mismatched Index"),
        }
    }
}