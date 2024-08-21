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
    use crate::array::{idxr, Array, ListError};

    // householder reflector
    #[test]
    fn reflector_test_1() -> Result<(), ListError> {
        let a_mat = vec![
           -1.0, -1.0, 1.0,
           1.0, 3.0, 3.0,
           -1.0, -1.0, 5.0,
           1.0, 3.0, 7.0
        ];

        let dim: (usize, usize) = (4_usize, 3_usize);
        let idx = idxr;

        let (q_factor, mut r) = Array::qr_householder(&a_mat, dim, true);

        for q in q_factor.iter().rev() {
            Array::reflector_mat_dot_mat(q, &mut r, dim, idx);
        }

        // a_mat = q*r
        let d = Array::dist_n2_vec_v1_v2(&a_mat, &r)?;
        println!("{:e}", d);
        assert!(d < 1e-10);
        Ok(())
    }

    #[test]
    fn reflector_test_2() -> Result<(), ListError> {
        let a_mat = vec![
           -1.15, -1.210, 1.33,
            1.54,  3.123, 3.34,
           -1.22, -1.076, 5.09,
            1.06,  3.079, 7.06
        ];

        let dim: (usize, usize) = (4_usize, 3_usize);
        let idx = idxr;

        let (q_factor, mut r) = Array::qr_householder(&a_mat, dim, true);

        for q in q_factor.iter().rev() {
            Array::reflector_mat_dot_mat(q, &mut r, dim, idx);
        }

        // a_mat = q*r
        let d = Array::dist_n2_vec_v1_v2(&a_mat, &r)?;
        println!("{:e}", d);
        assert!(d < 1e-10);
        Ok(())
    }
}