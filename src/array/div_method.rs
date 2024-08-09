use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

use super::Array;
use super::ListError;


impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{

    pub fn div(&mut self, other: &Self) -> Result<(), ListError> {
        match (self, other) {
            (Self::Scalar(x), Self::Scalar(y)) 
                => 
                {
                    if *y == T::default() {
                        return Err(ListError::DivisionByZero);
                    }
                    *x *= *y
                },
            
            _ => return Err(ListError::MismatchedTypes),
        }

        Ok(())
    }

}
