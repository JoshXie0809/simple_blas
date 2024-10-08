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

                let mut arr1: Box<[T]> = arr.clone();
                let mut pr: Vec<(usize, usize)> = Vec::new();
                let mut pc: Vec<(usize, usize)> = Vec::new();
                let det: T = Array::gaussian_det(&mut arr1, &mut pr, &mut pc, *nr, *put_val_by_row);
                
                return Ok(det);
            },
            _ => return Err(ListError::MismatchedTypes),
        }
    }

    pub(crate) fn gaussian_det(
        arr: &mut Box<[T]>, 
        pr: &mut Vec<(usize, usize)>, 
        pc: &mut Vec<(usize, usize)>, 
        n: usize, by_row: bool
    ) -> T 
    {
        // if SQUARE matrix
        // nr = nc = n
        let dim = (n, n);
        Array::gaussian_eliminate_rc(arr, by_row, dim, pr, pc);

        // compute sign and UPPER TRIANGULAR matrix's determinant
        let mut det = T::from( 1.0_f32 );
        let num_swp = (pr.len() % 2) as i32 + (pc.len() % 2) as i32;
        let sign = T::from( (1.0_f32).powi(num_swp) );

        for i in 0..n {
            det *= arr[i * n + i];
        }

        sign * det
    }

    // extend gaussian_eliminate
    // by full pivoting
    pub(crate) fn gaussian_eliminate_rc(
        arr: &mut Box<[T]>, by_row: bool, dim: (usize, usize), 
        pr: &mut Vec<(usize, usize)>, pc: &mut Vec<(usize, usize)>
    ) {

        let idx: fn(usize, usize, (usize, usize)) -> usize = if by_row {idxr} else {idxc};
        let z: T = T::default();
        let (nr, nc) = dim;
        let n = if nr < nc {nr} else {nc};
        
        for r in 0..n {
            // do permutation if (r, r) is not max
            // if find other do swap_row(r, maxr)
            // find max abs(val) under (r, r) element
            Array::permute_rc(arr, r, dim, z, pr, pc, idx);

            let maxv = arr[idx(r, r, dim)];
            // check if zero is max value
            if maxv == z {continue;}

            // eliminate val under (r, r)
            for r2 in (r+1)..nr {
                let factor: T = arr[idx(r2, r, dim)] / maxv;
                Array::gaussian_eliminate_r(arr, r, r2, dim, factor, idx);
            }
        }
    }
    
    #[allow(dead_code)]
    pub(crate) fn gaussian_eliminate(
        arr: &mut Box<[T]>, by_row: bool, dim: (usize, usize), 
        p: &mut Vec<(usize, usize)>
    ) {

        let idx: fn(usize, usize, (usize, usize)) -> usize = if by_row {idxr} else {idxc};
        let z: T = T::default();
        let (nr, nc) = dim;
        let n = if nr < nc {nr} else {nc};
        
        for r in 0..n {
            // do permutation if (r, r) is not max
            // if find other do swap_row(r, maxr)
            // find max abs(val) under (r, r) element
            Array::permute_r(arr, r, dim, z, p, idx);

            let maxv = arr[idx(r, r, dim)];
            // check if zero is max value
            if maxv == z {continue;}

            // eliminate val under (r, r)
            for r2 in (r+1)..nr {
                let factor: T = arr[idx(r2, r, dim)] / maxv;
                Array::gaussian_eliminate_r(arr, r, r2, dim, factor, idx);
            }
        }
    }

    pub(crate) fn gaussian_eliminate_r(
        arr: &mut Box<[T]>, 
        r: usize, r2: usize, dim: (usize, usize),
        factor: T,
        idx: fn(usize, usize, (usize, usize)) -> usize) 
    {
        
        let (_nr, nc) = dim;

        // eliminate those rows under rth row
        // to get row echelon form

        arr[idx(r2, r, dim)] = T::default();
        let start_col = r+1;
        let end_col = nc;
        
        Array::row_i_minus_frow_j(arr, factor, r2, r, dim, idx, start_col, end_col);

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

    pub fn compute_dist(arr1: &Self, arr2: &Self) -> Result<T, ListError> {
        match (arr1, arr2) {
            (Array::Array2D { arr: arr1, nr, nc, put_val_by_row },
             Array::Array2D { arr: arr2, nr: nr2, nc: nc2, put_val_by_row: put_val_by_row2 })
            => {

                if (*nr, *nc) != (*nr2, *nc2) {return Err(ListError::MismatchedDim);}

                let idx1: fn(usize, usize, (usize, usize)) -> usize = if *put_val_by_row {idxr} else {idxc};
                let idx2: fn(usize, usize, (usize, usize)) -> usize = if *put_val_by_row2 {idxr} else {idxc};
                let dim = (*nr, *nc);

                let z = T::default();
                let mut sum = T::default();

                for r in 0..*nr {
                    for c in 0..*nc {
                        sum += Array::abs(arr1[idx1(r, c, dim)] - arr2[idx2(r, c, dim)], z);
                    }
                }

                return Ok(sum);
            },

            _ => return Err(ListError::MismatchedTypes),
        }
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

