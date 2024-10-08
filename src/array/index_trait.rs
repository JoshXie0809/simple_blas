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
            Array::Scalar(val) => {
                if index != 0_usize {
                    panic!("index out of bound");
                }
                val
            },

            Array::Array1D { arr } => {
                &arr[index]
            },

            _ => panic!("Mismatched Index"),
        }
    }
}

impl<T> Index<(usize, usize)> for Array<T> 
where T:
Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        match self {
            Array::Array2D { arr, nr, nc, put_val_by_row } => {
                
                if *put_val_by_row {
                    &arr[(*nc) * index.0 + index.1]
                } else {
                    &arr[index.0 + (*nr) * index.1]
                }
            },

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
            Array::Scalar(val) => {
                if index != 0_usize {
                    panic!("index out of bound");
                }
                val
            },

            Array::Array1D { arr } => {
                &mut arr[index]
            },

            _ => panic!("Mismatched Index"),
        }
    }
}

impl<T> IndexMut<(usize, usize)> for Array<T> 
where T:
Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        match self {
            Array::Array2D { arr, nr, nc, put_val_by_row } => {
                if *put_val_by_row {
                    &mut arr[*nc * index.0 + index.1]
                } else {
                    &mut arr[index.0 + *nr * index.1]
                }
            },

            _ => panic!("Mismatched Index"),
        }
    }
}