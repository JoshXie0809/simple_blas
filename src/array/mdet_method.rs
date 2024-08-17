use super::Array;
use super::ListError;
use std::ops::Sub;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

use crate::array::{idxr, idxc};

impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + From<f32>
{
    pub fn mdet(&self) -> Result<T, ListError> {
        match self {
            Array::Array2D { arr, nr, nc, put_val_by_row} => {
                if *nr != *nc {return Err(ListError::MatrixDetDimError);}

                let det: T = Array::gaussian_det(arr.clone(), *put_val_by_row);
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


impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> + Sub<Output=T>
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + PartialOrd
{
    pub fn permute(&mut self) -> Result< Vec<(usize, usize)>, ListError > {
        match self {
            Array::Array2D { arr, nr, nc, put_val_by_row }
            => {
                let idx: fn(usize, usize, (usize, usize)) -> usize = if *put_val_by_row {idxr} else {idxc};
                let nr = *nr;
                let nc = *nc;

                let mut p: Vec<(usize, usize)> = vec![];

                let dim: (usize, usize) = (nr, nc);
                let z = T::default();
                
                for r in 0..(nr-1) {
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
                        Array::swap_r(arr, r, maxr, 0, nc, *put_val_by_row, dim);
                        p.push((r, maxr));
                    }
                }

                return Ok(p);
            },
            
            _ => Err(ListError::MismatchedTypes),

        }
    }

    pub(crate) fn swap_r(arr: &mut Box<[T]>, 
                  i: usize, j: usize, 
                  begin_col: usize, 
                  end_col: usize, 
                  by_row: bool, 
                  dim: (usize, usize)) 
        {

            let idx: fn(usize, usize, (usize, usize)) -> usize = if by_row {idxr} else {idxc};
            
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
