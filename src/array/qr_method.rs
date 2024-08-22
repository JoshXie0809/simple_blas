use super::{Array, ListError, idxr, idxc};
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

    pub fn mqr_householder(&self) -> Result<(Vec<Vec<T>>, Array<T>), ListError>{
        match self {
            Array::Array2D { arr, nr, nc, put_val_by_row } 
            => {
                let dim: (usize, usize) = (*nr, *nc);
                let by_row: bool = *put_val_by_row;
                if *nr < *nc {return Err(ListError::MatrixQRHouseHolderDimError);}
                let (q_factor, r) = Array::qr_householder(&arr, dim, by_row);
               
                let r = Array::new_array_2d(
                    r.into_boxed_slice(), 
                    (*nr as isize, *nc as isize), 
                    by_row
                )?;

                Ok((q_factor, r))
            },

            _ => return Err(ListError::MismatchedTypes),
        }
    }

    pub fn mq_factor_mult_mat_a(q_factor: &Vec<Vec<T>>, ma: &mut Self) -> Result<(), ListError>{
        match ma {
            Array::Array2D { arr, nr, nc, put_val_by_row } 
            => {
                let dim: (usize, usize) = (*nr, *nc);
                let idx: fn(usize, usize, (usize, usize)) -> usize = if *put_val_by_row {idxr} else {idxc};
                Array::q_factor_dot_ma(q_factor,arr, dim, idx, false);
            }

            _ => return Err(ListError::MismatchedTypes)
        }        

        Ok(())
    }

    pub fn mmat_a_mult_q_factor(ma: &mut Self, q_factor: &Vec<Vec<T>>) -> Result<(), ListError>{
        match ma {
            Array::Array2D { arr, nr, nc, put_val_by_row } 
            => {
                let dim: (usize, usize) = (*nr, *nc);
                Array::ma_dot_q_factor(arr, dim, *put_val_by_row, q_factor);
            }

            _ => return Err(ListError::MismatchedTypes)
        }        

        Ok(())
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
        let idx: fn(usize, usize, (usize, usize)) -> usize = idxr;

        let (q_factor, mut r) = Array::qr_householder(&a_mat, dim, true);

        for q in q_factor.iter() {
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
        let idx: fn(usize, usize, (usize, usize)) -> usize = idxr;

        let (q_factor, mut r) = Array::qr_householder(&a_mat, dim, true);

        for q in q_factor.iter() {
            Array::reflector_mat_dot_mat(q, &mut r, dim, idx);
        }

        // a_mat = q*r
        let d = Array::dist_n2_vec_v1_v2(&a_mat, &r)?;
        println!("{:e}", d);
        assert!(d < 1e-10);
        Ok(())
    }
}