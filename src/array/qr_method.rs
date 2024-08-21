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

#[cfg(test)]
pub mod tests {
    use crate::array::{idxr, Array};

    // householder reflector
    #[test]
    fn reflector_test_1() {
        let a_mat = vec![
           -1.0, -1.0, 1.0,
           1.0, 1.0, 3.0,
           -1.0, -1.0, 5.0,
           1.0, 3.0, 7.0
        ];

        let dim: (usize, usize) = (4_usize, 3_usize);
        let mut v1: Vec<f64> = vec![];
        let mut reflector: Vec<f64> = vec![0.0; 4];
        let idx: fn(usize, usize, (usize, usize)) -> usize = idxr;
        
        for r in 0..=3_usize {
            v1.push(a_mat[idx(r, 0, dim)]);
        }

        Array::reflector(&v1, &mut reflector);

        println!("{:?}", reflector);

        // let mut res = vec![0.0; 3];
        // Array::vec_b_dot_mat_a(&a_mat, &reflector, &mut res, dim, true);
        // println!("{:?}", res);

    }
}