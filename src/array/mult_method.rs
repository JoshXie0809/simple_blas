use std::ops::{Add, Sub, AddAssign, Div, Mul, MulAssign, SubAssign};

use super::Array;
use super::ListError;
use super::{idxr, idxc};

impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ Sub<Output=T> + PartialOrd
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    
    pub fn mult(&mut self, val: T) -> Result<(), ListError> {
        
        match self {
            Self::Scalar(x) => *x *= val,

            Self::Array1D { arr } => {
                Array::self_mult_scalar_s(arr, val);
            }

            Self::Array2D { arr, ..} => {
                Array::self_mult_scalar_s(arr, val);
            }
            
            _ => {return Err(ListError::MismatchedTypes)},
        }

        Ok(())
    }

    pub fn ele_mult(&mut self, other: &Self) -> Result<(), ListError> {
        
        match (self, other) {
            (Self::Scalar(x), Self::Scalar(y)) => *x *= *y,

            (Self::Array1D { arr: arr1 }, Self::Array1D { arr: arr2 })
            => {
                Array::self_ele_mult_vec_v2(arr1, arr2)?;
            },

            (Array::Array2D { arr: arr1, nr: nr1, nc:nc1, put_val_by_row: by_row1 },
                Array::Array2D { arr: arr2, nr: nr2, nc:nc2, put_val_by_row: by_row2, ..})
               => {
                   let dim1: (usize, usize) = (*nr1, *nc1);
                   let dim2: (usize, usize) = (*nr2, *nc2);
                   
                   let idx1: fn(usize, usize, (usize, usize)) -> usize = if *by_row1 {idxr} else {idxc};
                   let idx2: fn(usize, usize, (usize, usize)) -> usize = if *by_row2 {idxr} else {idxc};
   
                   Array::self_ele_mult_mat_m(arr1, arr2, dim1, dim2, idx1, idx2)?;
               },
            
            _ => {return Err(ListError::MismatchedTypes)},
        }

        Ok(())
    }

}