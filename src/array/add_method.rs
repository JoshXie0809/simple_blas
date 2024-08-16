use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

use super::Array;
use super::ListError;
use crate::array::{idxr, idxc};

impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{

    pub fn add(&mut self, other: &Self) -> Result<(), ListError> {
        
        match (self, other) {
            (Self::Scalar(x), Self::Scalar(y)) 
                => *x += *y,
            
            (Self::Array1D { arr: arr1 }, 
             Self::Array1D { arr: arr2 }) => {
                let len1: usize = arr1.len();
                if len1 != arr2.len() {
                    return Err(ListError::DifferentLength1D);
                }

                for i in 0..(len1) {
                    arr1[i] += arr2[i];
                }
            },

            (Self::Array1D { arr: arr1 }, 
             Self::Scalar(val)) => {
                let len1 = arr1.len();
                for i in 0..(len1) {
                    arr1[i] += *val;
                }
            },

            (Self::Array2D { arr: arr1, nr: nr1, nc: nc1, put_val_by_row: by_row1}, 
             Self::Array2D { arr: arr2, nr: nr2, nc: nc2, put_val_by_row: by_row2}) => {
                
                if (*nr1, *nc1) != (*nr2, *nc2) {
                    return Err(ListError::MismatchedDim);
                }

                Array::arr_2d_add(*by_row1, *by_row2, arr1, arr2, *nr1, *nc1);
            },
            
            _ => {return Err(ListError::MismatchedTypes)},
        }

        Ok(())
    }

    fn arr_2d_add(by_row1: bool, by_row2: bool, arr1: &mut Box<[T]>, arr2: &Box<[T]>, nr1: usize, nc1: usize) {
        let dim: (usize, usize) = (nr1, nc1);

        match (by_row1, by_row2) {
            
            (true, false) => {
                for r in 0..(nr1) {
                    for c in 0..(nc1) {
                        arr1[idxr(r, c, dim)] += arr2[idxc(r, c, dim)];
                    }
                }
            },

            (false, true) => {
                for r in 0..(nr1) {
                    for c in 0..(nc1) {
                        arr1[idxc(r, c, dim)] += arr2[idxr(r, c, dim)];
                    }
                }
            },

            _ => {
                for i in 0..(arr1.len()) {
                    arr1[i] += arr2[i];
                }
            },
        }
    }

}