use super::Array;
use super::ListError;
use std::ops::Sub;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

use crate::array::{idxr, idxc};

impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> + Sub<Output=T>
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + From<f32> + PartialOrd
{
    pub fn mdet(&self) -> Result<T, ListError> {
        match self {
            Array::Array2D { arr, nr, nc, put_val_by_row} => {
                if *nr != *nc {return Err(ListError::MatrixDetDimError);}

                let mut arr1 = arr.clone();
                let mut p = Vec::new();
                let det = Array::gaussian_det(&mut arr1, &mut p, *nr, *put_val_by_row);
                
                return Ok(det);
            },
            _ => return Err(ListError::MismatchedTypes),
        }
    }

    pub(crate) fn gaussian_det(arr: &mut Box<[T]>, p: &mut Vec<(usize, usize)>, n: usize, by_row: bool) -> T {
        
        // if SQUARE matrix
        // nr = nc = n
        Array::gaussian_eliminate(arr, by_row, (n, n), p);

        // compute sign and UPPER TRIANGULAR matrix's determinant
        let mut det = T::from( 1.0_f32 );
        let num_swp = (p.len() % 2) as i32;
        let sign = T::from( (1.0_f32).powi(num_swp) );

        for i in 0..n {
            det *= arr[i * n + i];
        }

        sign * det
    }

    pub(crate) fn gaussian_eliminate(arr: &mut Box<[T]>, by_row: bool, dim: (usize, usize), p: &mut Vec<(usize, usize)>) {

        let idx: fn(usize, usize, (usize, usize)) -> usize = if by_row {idxr} else {idxc};
        let (nr, nc) = dim;
        let z: T = T::default();

        let n = if nr < nc {nr} else {nc};
        
        for r in 0..n {
            Array::permute_r(arr, r, dim, z, p, idx);

            for r2 in (r+1)..nr {

                let factor: T = arr[idx(r2, r, dim)] / arr[idx(r, r, dim)];
                arr[idx(r2, r, dim)] = T::default();

                for c2 in (r+1)..nc {
                    arr[idx(r2, c2, dim)] -= factor * arr[idx(r, c2, dim)];
                }
            }
        }

    }
}

impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> + Sub<Output=T>
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + PartialOrd
{
    pub fn permute(&mut self) -> Result< Vec<(usize, usize)>, ListError > {
        match self {
            Array::Array2D { arr, nr, nc, put_val_by_row }
            => {

                // index function
                let idx: fn(usize, usize, (usize, usize)) -> usize = if *put_val_by_row {idxr} else {idxc};
                // record swap rows
                let mut p: Vec<(usize, usize)> = vec![];
                // dim for index function
                let dim: (usize, usize) = (*nr, *nc);
                // for abs function
                let z = T::default();
                
                for r in 0..(*nr-1) {
                    Array::permute_r(arr, r, dim, z, &mut p, idx);
                }

                return Ok(p);
            },
            
            _ => Err(ListError::MismatchedTypes),

        }
    }

    pub(crate) fn permute_r(
        arr: &mut Box<[T]>, 
        r:usize, dim: (usize, usize), z: T, 
        p: &mut Vec<(usize, usize)>,
        idx: fn(usize, usize, (usize, usize)) -> usize,
    ) {
        let (nr, nc) = dim;
        // now is rth row
        // assume max element on this row
        let mut maxr = r;
        let mut maxv = Array::abs(arr[idx(r, r, dim)], z);

        // find max in row under r
        for i in 1..(nr - r) {
            let val1 = Array::abs(arr[idx(r + i, r, dim)], z);
            if maxv < val1 {
                maxr = r + i;
                maxv = val1;
            }
        }
        
        // we find the maxr,
        // swap rth and maxr
        if r != maxr {
            Array::swap_r(arr, r, maxr, 0, nc, idx, dim);
            p.push((r, maxr));
        }
    }

    pub(crate) fn swap_r(arr: &mut Box<[T]>, 
                  i: usize, j: usize, 
                  begin_col: usize, 
                  end_col: usize, 
                  idx: fn(usize, usize, (usize, usize)) -> usize,
                  dim: (usize, usize)) 
    {
            
            for c in begin_col..end_col {
                (arr[idx(i, c, dim)], arr[idx(j, c, dim)]) = 
                (arr[idx(j, c, dim)], arr[idx(i, c, dim)]);
        }
    }

    pub(crate) fn abs(val: T, z: T) -> T {
        if val < z {return z-val};
        val
    }
}



#[cfg(test)]
pub mod tests {
    use crate::array::Array;

    #[test]
    fn gaussian_eliminate_1() {

        let arr: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let mut arr: Box<[f64]> = arr.into_boxed_slice();
        let mut p: Vec<(usize, usize)> = vec![];

        Array::gaussian_eliminate(&mut arr, false, (4, 2), &mut p);
        let arr2: Vec<f64> = Vec::from([4.0, 0.0, 0.0, 0.0, 8.0, 3.0, 0.0, 0.0]);
        let arr2: Box<[f64]> = arr2.into_boxed_slice();
        assert_eq!(arr, arr2);
    }

    #[test]
    fn gaussian_eliminate_2() {

        let arr: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let mut arr: Box<[f64]> = arr.into_boxed_slice();
        let mut p: Vec<(usize, usize)> = vec![];

        Array::gaussian_eliminate(&mut arr, false, (3, 3), &mut p);
        let arr2: Vec<f64> = Vec::from([3.0, 0.0, 0.0, 6.0, 2.0, 0.0, 9.0, 4.0, 0.0]);
        let arr2: Box<[f64]> = arr2.into_boxed_slice();
        assert_eq!(arr, arr2);
    }

    #[test]
    fn gaussian_eliminate_3() {

        let arr: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let mut arr: Box<[f64]> = arr.into_boxed_slice();
        let mut p: Vec<(usize, usize)> = vec![];

        Array::gaussian_eliminate(&mut arr, false, (2, 4), &mut p);
        let arr2: Vec<f64> = Vec::from([2.0, 0.0, 4.0, 1.0, 6.0, 2.0, 8.0, 3.0]);
        let arr2: Box<[f64]> = arr2.into_boxed_slice();
        assert_eq!(arr, arr2);
    }
}

