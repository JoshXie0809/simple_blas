use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

use super::Array;
use super::ListError;
use crate::array::{idxr, idxc};

impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    
    pub fn add(&mut self, val: T) -> Result<(), ListError> {
        match self {
            Array::Scalar(s) => {
                *s += val;
            }
            
            _ => return Err(ListError::MismatchedTypes)
        }

        Ok(())
    }

}