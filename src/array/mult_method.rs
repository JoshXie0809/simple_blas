use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

use super::Array;
use super::ListError;

impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    
    pub fn mult(&mut self, other: &Self) -> Result<(), ListError>{
        match (self, other) {
            (Self::Scalar(x), Self::Scalar(y)) 
                => *x *= *y,
            
            (Self::Array1D { arr: arr1 }, 
             Self::Array1D { arr: arr2 }) => {
                let len1 = arr1.len();
                if len1 != arr2.len() {
                    return Err(ListError::DifferentLength1D);
                }

                for i in 0..(len1) {
                    arr1[i] *= arr2[i];
                }
            },
    
            (Self::Array1D { arr: arr1 }, 
             Self::Scalar(val)) => {
                let len1 = arr1.len();
                for i in 0..(len1) {
                    arr1[i] *= *val;
                }
            },
            
            _ => {return Err(ListError::MismatchedTypes)},
        }

        Ok(())
    }

}