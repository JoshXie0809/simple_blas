use super::Array;
use super::ListError;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

// use crate::array::{idxr, idxc};



impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + From<f32>
{
    pub fn mdet(&self) -> Result<T, ListError> {
        match self {
            Array::Array2D { arr, nr, nc, put_val_by_row} => {
                if *nr != *nc {return Err(ListError::MatrixDetDimError);}

                let arr1: Box<[T]> = arr.clone();
                let det: T = Array::gaussian_det(arr1, *put_val_by_row);
                return Ok(det);
            },
            _ => return Err(ListError::MismatchedTypes),
        }
    }

    fn gaussian_det(arr: Box<[T]>, by_row: bool) -> T {
        let mut swap_num: i32 = 0;
        if by_row {
            swap_num += 2;
            arr[0] * T::from((-1.0_f32).powi(swap_num))
        } else {
            arr[0]
        }

    }
}