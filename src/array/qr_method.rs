use super::{Array, ListError};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};
use crate::array::matrix_operations::Sqrt;

impl<T> Array<T> 
where T: Sqrt + 
Add<Output=T> + Mul<Output=T> + 
Div<Output=T> + Sub<Output=T>
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + PartialOrd + From<f32>
{
    
    pub fn mqr(&self) -> Result<(Array<T>, Array<T>), ListError> {      
        let (q, r) = 
        match self {
            Array::Array2D { arr, nr, nc, put_val_by_row }
            => {
                
                // matrix is n x m
                // q is n x n, orthorgonal matrix
                // m is n x m, u-tri matrix
                let dim: (usize, usize) = (*nr, *nc);
                let (nr, nc) = dim;
                let n = if nr < nc {nr} else {nc};
                let dimq: (isize, isize) = (nr as isize, n as isize);
                let dimr: (isize, isize) = (n as isize, nc as isize);
                     
                let (q, r) = Array::qr(&arr, dim, *put_val_by_row);
                
                let q: Array<T> = Array::new_array_2d(
                    q.into_boxed_slice(),
                    dimq, 
                    *put_val_by_row,
                )?;

                let r: Array<T> = Array::new_array_2d(
                    r.into_boxed_slice(),
                    dimr, 
                    *put_val_by_row,
                )?;

                (q, r)
            },

            _ => return Err(ListError::MismatchedTypes),
        };

        Ok((q, r))
    }
}

