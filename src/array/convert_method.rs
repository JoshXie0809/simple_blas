use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

use super::{Array, ListError};

impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    pub fn convert_to_scalar(&mut self, val: T) -> Result<(), ListError> {
        match self {
            Array::Null => {
                *self = Array::new_scalar(val);
            },

            _ => return Err(ListError::MismatchedTypes)
        };

        Ok(())
    }

    pub fn convert_to_arr_1d (&mut self, arr: Box<[T]>) -> Result<(), ListError> {
        match self {
            Array::Null => *self = Array::new_array_1d(arr),

            _ => return Err(ListError::MismatchedTypes),
        }

        Ok(())
    }

}
