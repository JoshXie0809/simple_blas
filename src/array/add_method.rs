use std::ops::Sub;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

use rayon::iter::Zip;

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

            Array::Array2D { arr, nr, nc, put_val_by_row }
            => {
                let _ = (nr, nc, put_val_by_row);
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
            }
            
            _ => return Err(ListError::MismatchedTypes)
        }

        Ok(())
    }

}