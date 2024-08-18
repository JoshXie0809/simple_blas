// matrix inverse

use super::Array;
use super::ListError;
use std::mem;
use std::ops::Sub;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

use crate::array::{idxr, idxc};

impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> + Sub<Output=T>
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + From<f32> + PartialOrd
{
    pub fn inv(&mut self) -> Result<(), ListError> {
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


                    // for i in 0..(*nr) {
                    //     let mut sum = T::default();
                    //     for bi in 0..i {
                    //         sum += lu[idx(i, bi, dim)] * y[idx(bi, c, dim)]
                    //     }
                    //     // yi[i]
                    //     // l[i, i] = 1
                    //     y[idx(i, c, dim)] = ei[i] - sum;
                    // }
                }
                
                for c in 0..(*nc) {

                    let start: usize = c * (*nr);
                    let end: usize = start + *nr;

                    let yc = &y[start..end];
                    let mut xc = vec![T::default(); *nr];

                    // solve Uxc = yc
                    Array::u_tri_solve(&lu, yc, xc.as_mut_slice(), dim, idx);

                    for (i, xc_i) in xc.into_iter().enumerate() {
                        arr[idx(i, c, dim)] = xc_i;
                    }

                    // for m in (0..*nr).rev() {
                    //     let mut sum: T = T::default();
                    //     for fi in (m+1)..(*nc) {
                    //         sum += lu[idx(m, fi, dim)] * arr[idx(fi, c, dim)];
                    //     }        
                    //     arr[idx(m, c, dim)] = (y[idx(m, c, dim)] - sum) / lu[idx(m, m, dim)];
                    // }
                }
            },

            _ => return Err(ListError::MismatchedTypes),
        }
        Ok(())
    }

    // LU x = pb
    // give L, U, p, and b to solve x
    #[allow(dead_code)]
    pub(crate) fn p_lu_solve(
        lu: &[T], p: &[(usize, usize)], b: &mut [T], x: &mut [T],
        dim: (usize, usize), idx: fn(usize, usize, (usize, usize)) -> usize
    ) {
        // L Ux = pb
        // let Ux = y
        // L y = pv
        // y = L^-1 pv 
        // L is lower triangular matrix: easy to solve


        // get pb
        for &(i, j) in p.iter() {
            b.swap(i, j);
        }
        
        let is_lu: bool =  true;
        let (nr, _nc) = dim;
        let mut y: Vec<T> = vec![T::default(); nr];

        // solve y
        Array::l_tri_solve(lu, b, &mut y, dim, idx, is_lu);

        // after we solve y
        // solve Ux = y
        // similarly x is easy to solve

        Array::u_tri_solve(lu, &y, x, dim, idx);
        
        // turn pb to b
        for &(i, j) in p.iter().rev() {
            b.swap(i, j);
        }
    }

    // solve Lx = b
    pub(crate) fn l_tri_solve(
        l: &[T], b: &[T],
        x: &mut [T],
        dim: (usize, usize), 
        idx: fn(usize, usize, (usize, usize)) -> usize,
        is_lu: bool
    ) {
        
        // i = 0
        // l[0, 0] * yi[0] = b[0]
        // yi[0] = b[0] / l[0, 0] = b[0]
        
        // i = 1
        // l[1, 1] * yi[1] + l[0, 1] * yi[0] = b[1]
        // l[1, 1] * yi[1] = b[1] - l[0, 1] * yi[0]
        // yi[1] = (b[1] - l[0, 1] * yi[0]) / l[1, 1]

        // i = 2
        // l[2, 0] * yi[0] + l[2, 1] * yi[1] + l[2, 2] * yi[2] = b[2]
        // yi[2] = (b[2] - l[2, 0] * yi[0] - l[2, 1] * yi[2]) / l[2, 2]

        let (nr, _nc) = dim;

        // foreach row
        for r in 0..nr {                        
            let mut sum = T::default();
            // backward solve
            for bi in 0..r {
                sum += l[idx(r, bi, dim)] * x[bi];
            }

            if is_lu {
                x[r] = b[r] - sum;
            } else {
                x[r] = b[r] - sum / l[idx(r, r, dim)];
            }
        }
    }

    // solve Ux = b
    pub(crate) fn u_tri_solve(
        u: &[T], b: &[T],
        x: &mut [T],
        dim: (usize, usize), 
        idx: fn(usize, usize, (usize, usize)) -> usize,
    ) {
        // similarly, we solve X by column

        // m = n-1
        // u[m, m] * x[m] = yi[m]
        // x[m] = yi[m] / u[m, m]

        // m - 1
        // u[m-1, m-1] * x[m-1] + u[m-1, m] * x[m] = yi[m-1]
        // x[m-1] = (yi[m-1] - u[m-1, m] * x[m]) / u[m-1, m-1]
        
        let (nr, nc) = dim;
        for r in (0..nr).rev() {
            // forward solve
            let mut sum = T::default();
            for fi in (r+1)..nc {
                sum += u[idx(r, fi, dim)] * x[fi];
            }
            x[r] = (b[r] - sum) / u[idx(r, r, dim)];
        }
    }

    // plu is not only for SQUARE
    pub(crate) fn p_lu(
        p: &mut Vec<(usize, usize)>, 
        arr: &mut Box<[T]>,
        dim: (usize, usize),
        idx: fn(usize, usize, (usize, usize)) -> usize
    ) {
        // do procedures like Gaussian eliminate
        // SQUARE MATRIX
        let (nr, nc) = dim;
        // zero val
        let z: T = T::default();
        let n: usize = if nr < nc {nr} else {nc};

        for r in 0..n {
            // p to record row swap
            // P*A = U1
            Array::permute_r(arr, r, dim, z, p, idx);

            // check if max val is zero
            // do not need to do row eliminations
            let maxv: T = arr[idx(r, r, dim)];
            if maxv == z {continue;}

            // do row elimination
            for r2 in (r+1)..nr {
                // Li P*A = U2
                let factor: T = arr[idx(r2, r, dim)] / maxv;
                Array::gaussian_eliminate_r(arr, r, r2, dim, factor, idx);

                // L*Li (P*A) = L (P A)

                // record L in lower triangular
                // we set (r2, r) place as default
                arr[idx(r2, r, dim)] += factor;
            }
        }

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