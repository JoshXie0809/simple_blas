use std::ops::Sub;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

use super::Array;
use super::ListError;
use crate::array::{idxr, idxc};

impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ Sub<Output=T> + PartialOrd
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    
    pub fn add(&mut self, val: T) -> Result<(), ListError> {
        match self {
            Array::Scalar(s) => {
                *s += val;
            },

            Array::Array1D { arr } => {
                Array::self_add_scalar_s(arr, val);
            },

            Array::Array2D { arr, ..}
            => {
                Array::self_add_scalar_s(arr, val);
            },
            
            _ => return Err(ListError::MismatchedTypes)
        }

        Ok(())
    }

    pub fn madd(&mut self, other: &Self) -> Result<(), ListError> {
        match (self, other) {
            (Array::Array1D {arr: arr1, ..}, 
             Array::Array1D {arr: other, ..}) => {
                Array::self_add_vec_v2(arr1, other)?;
            }, 

            (Array::Array2D { arr: arr1, nr: nr1, nc:nc1, put_val_by_row: by_row1 },
             Array::Array2D { arr: arr2, nr: nr2, nc:nc2, put_val_by_row: by_row2, ..})
            => {
                let dim1: (usize, usize) = (*nr1, *nc1);
                let dim2: (usize, usize) = (*nr2, *nc2);
                
                let idx1: fn(usize, usize, (usize, usize)) -> usize = if *by_row1 {idxr} else {idxc};
                let idx2: fn(usize, usize, (usize, usize)) -> usize = if *by_row2 {idxr} else {idxc};

                Array::self_add_mat_m(arr1, arr2, dim1, dim2, idx1, idx2)?;
            },
            
            _ => return Err(ListError::MismatchedTypes),
        }

        Ok(())
    }

}