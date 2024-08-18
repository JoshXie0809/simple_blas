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
                let mut ul: Box<[T]> = mem::replace(arr, arr.clone());

                // p-lu
                // p A = lu
                let mut p : Vec<(usize, usize)> = vec![];
                let dim = (*nr, *nc);
                let idx: fn(usize, usize, (usize, usize)) -> usize = if *put_val_by_row {idxr} else {idxc};

                Array::p_lu(&mut p, &mut ul, dim, idx);

                // check invertibility
                // all diag on U is not zero

                for i in 0..(*nr) {
                    if ul[idx(i, i, dim)] == T::default() {return Err(ListError::SigularMat);}
                }

                // now we have p, l, u
                // and A is invertible
                // X is A^-1
                // p A X = p I
                // (p A) X = p I
                // lu X = p I
                // first let uX = Y
                // l Y = p I
                // solve Y = [y1, y2, ... yn], 
                // yi is column vector

                // to solve Y, we solve every column of y
                
                let mut y: Box<[T]> = mem::replace(arr, arr.clone());

                for c in 0..(*nc) {
                    // ei: ith column on Identity matrix
                    let mut ei = vec![T::default(); *nr];
                    ei[c] = T::from(1.0_f32);
                    
                    // b = p * ei
                    for &(i, j) in p.iter() {
                        (ei[i], ei[j]) = (ei[j], ei[i]);
                    }

                    // now we get b
                    // solve every element of column vector
                    // our l's elements are all (one)
                    // lower triangular property: 
                    
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

                    for i in 0..(*nr) {
                        let mut sum = T::default();
                        for bi in 0..i {
                            sum += ul[idx(i, bi, dim)] * y[idx(bi, c, dim)]
                        }

                        // yi[i]
                        // l[i, i] = 1
                        y[idx(i, c, dim)] = ei[i] - sum;
                    }
                }
                
                // calc UX = Y
                for c in 0..(*nc) {

                    // similarly, we solve X by column

                    // m = n-1
                    // u[m, m] * x[m] = yi[m]
                    // x[m] = yi[m] / u[m, m]

                    // m - 1
                    // u[m-1, m-1] * x[m-1] + u[m-1, m] * x[m] = yi[m-1]
                    // x[m-1] = (yi[m-1] - u[m-1, m] * x[m]) / u[m-1, m-1] 

                    for m in (0..*nr).rev() {
                        let mut sum: T = T::default();
                        
                        for fi in (m+1)..(*nc) {
                            sum += ul[idx(m, fi, dim)] * arr[idx(fi, c, dim)];
                        }
                        
                        arr[idx(m, c, dim)] = (y[idx(m, c, dim)] - sum) / ul[idx(m, m, dim)];
                    }
                }            
            },

            _ => return Err(ListError::MismatchedTypes),
        }
        Ok(())
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
}