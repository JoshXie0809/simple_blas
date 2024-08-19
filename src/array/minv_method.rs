// matrix inverse

use super::matrix_operations::Sqrt;
use super::Array;
use super::ListError;
use std::mem;
use std::ops::Sub;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

use crate::array::{idxr, idxc};

impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> + Sub<Output=T>
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + From<f32> + PartialOrd + Sqrt
{
    pub fn minv(&mut self) -> Result<(), ListError> {
        match self {
            Array::Array2D { arr, nr, nc, put_val_by_row } 
            => {
                if nr != nc {return Err(ListError::MatrixInvDimError);}

                // replace
                let mut lu: Box<[T]> = mem::replace(arr, arr.clone());

                // p-lu
                // p A = lu
                let mut p : Vec<(usize, usize)> = vec![];
                let dim = (*nr, *nc);
                let idx: fn(usize, usize, (usize, usize)) -> usize = if *put_val_by_row {idxr} else {idxc};

                Array::p_lu(&mut p, &mut lu, dim, idx);

                // check invertibility
                // all diag on U is not zero

                for i in 0..(*nr) {
                    if lu[idx(i, i, dim)] == T::default() {return Err(ListError::SingularMat);}
                }

                // now we have p, l, u
                // and A is invertible
                // to solve Y, we solve every column of y

                let y: Vec<T> = vec![T::default(); (*nr) * (*nc)];
                let mut y: Box<[T]> = y.into_boxed_slice();

                for c in 0..(*nc) {
                    // ei: ith column on Identity matrix
                    let mut ei = vec![T::default(); *nr];
                    ei[c] = T::from(1.0_f32);
                    
                    // b = p * ei
                    for &(i, j) in p.iter() {
                        (ei[i], ei[j]) = (ei[j], ei[i]);
                    }

                    let is_lu: bool = true;
                    // Y's column
                    // assumne put val by col
                    // 0st column idx is 0..(nr)
                    // 1nd colunm idx is nr..(2nr)
                    let start: usize = c * (*nr);
                    let end: usize = start + *nr;
                    let yc = &mut y[start..end];
                    Array::l_tri_solve(&lu, &ei, yc, dim, idx, is_lu);
                }
                
                for c in 0..(*nc) {
                    let start: usize = c * (*nr);
                    let end: usize = start + *nr;
                    let yc = &y[start..end];
                    let mut xc = vec![T::default(); *nr];

                    // solve Uxc = yc
                    Array::u_tri_solve(&lu, yc, xc.as_mut_slice(), dim, idx);
                    
                    // write result
                    for (i, xc_i) in xc.into_iter().enumerate() {
                        arr[idx(i, c, dim)] = xc_i;
                    }
                }
            },

            _ => return Err(ListError::MismatchedTypes),
        }
        Ok(())
    }

}


#[cfg(test)] 
pub mod tests {
    use crate::array::{idxr, Array};

    #[test]
    fn plu_2d_arr() {
        let arr: Vec<f64> = vec![
            2.0, 4.0, 7.0, 
            2.0, 5.0, 8.0, 
            3.0, 6.0, 9.0,
        ];

        let mut arr: Box<[f64]> = arr.into_boxed_slice();
        let mut p: Vec<(usize, usize)> = vec![];
        let dim: (usize, usize) = (3, 3);

        let idx: fn(usize, usize, (usize, usize)) -> usize = idxr;

        Array::p_lu(&mut p, &mut arr, dim, idx);

        println!("{:?}", p);
        println!("{:?}", arr);

    }

    #[test]
    fn plu_2d_arr_2() {
        let arr: Vec<f64> = vec![
            1.0, 0.0,
            3.0, 4.0,
        ];

        let mut arr: Box<[f64]> = arr.into_boxed_slice();
        let mut p: Vec<(usize, usize)> = vec![];
        let dim: (usize, usize) = (2, 2);
        let idx: fn(usize, usize, (usize, usize)) -> usize = idxr;
        Array::p_lu(&mut p, &mut arr, dim, idx);

        println!("{:?}", p);
        println!("{:?}", arr);

    }

    #[test]
    fn plu_solve_test() {
        let arr: Vec<f64> = vec![
            1.0, 0.0,
            3.0, 4.0,
        ];

        let mut arr: Box<[f64]> = arr.into_boxed_slice();
        let mut p: Vec<(usize, usize)> = vec![];
        let dim: (usize, usize) = (2, 2);
        let idx: fn(usize, usize, (usize, usize)) -> usize = idxr;

        Array::p_lu(&mut p, &mut arr, dim, idx);
        
        // now arr store lu
        let lu: Box<[f64]> = arr;
        let mut b: Vec<f64> = vec![1.0, -1.0];
        let mut x: Vec<f64> = vec![0.0, 0.0];

        Array::p_lu_solve(&lu, &p, &mut b, x.as_mut_slice(), dim, idx);

        let mut dist: f64 = 0.0;
        let true_vec: Vec<f64> = vec![1.0, -1.0];
        for i in 0..2 {
            dist += (x[i] - true_vec[i]).powi(2);
        }

        let dist = dist.powf(0.5);
        assert!(dist < 1e-10);

        // check b is not change
       assert_eq!(b, vec![1.0, -1.0]);

    }
}