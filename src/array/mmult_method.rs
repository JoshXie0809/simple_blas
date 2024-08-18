// matrix mult

use std::mem;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

use super::{idxc, idxr, Array};
use super::ListError;
use rayon::{prelude::*, ThreadPoolBuilder};

impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default 
{
    pub fn mmult(&mut self, other: &Self) -> Result<(), ListError> {
        match (self, other) {
            (Self::Array2D { arr: arr1, nr: nr1, nc: nc1, put_val_by_row: by_row1}, 
             Self::Array2D { arr: arr2, nr: nr2, nc: nc2, put_val_by_row: by_row2}) => {
                if *nc1 != *nr2 {
                    return Err(ListError::MatrixMultMismatchedDim);
                }

                // dim give to self.dim (nr1, nc2)
                let new_dim: (usize, usize) = (*nr1, *nc2);
                
                // each old self row vector has ni elements
                let ni = *nc1;

                // box give to self.arr
                let new_vec: Vec<T> = vec![T::default(); new_dim.0 * new_dim.1];
                let new_box: Box<[T]> = new_vec.into_boxed_slice();

                // replace arr
                let old_arr: Box<[T]> = mem::replace(arr1, new_box);
                
                // replace nr, nc
                let _ = (
                    mem::replace(nr1, new_dim.0), 
                    mem::replace(nc1, new_dim.1)
                );
                
                // replace by_row
                let old_by_row = mem::replace(by_row1, true);

                Array::matrix_mult(arr1, arr2, old_arr, new_dim, ni, old_by_row, *by_row2);

            },
            
            _ => return Err(ListError::MismatchedTypes),
        };

        Ok(())
    }

    fn matrix_mult(
        arr1: &mut Box<[T]>, arr2: &Box<[T]>, old_arr: Box<[T]>, 
        new_dim: (usize, usize), 
        ni: usize, 
        old_by_row: bool, by_row2: bool) 
    {
        let index_old: fn(usize, usize, (usize, usize)) -> usize =  if old_by_row { idxr } else { idxc };
        let index_other: fn(usize, usize, (usize, usize)) -> usize = if by_row2 { idxr } else { idxc };

        for r in 0..new_dim.0 {
            for c in 0..new_dim.1 {
                let mut sum = T::default();
                for i in 0..ni {
                    sum += 
                    
                    old_arr[index_old(r, i, (new_dim.0, ni))] 
                    * 
                    arr2[index_other(i, c, (ni, new_dim.1))];
                    
                }
                arr1[idxr(r, c, new_dim)] = sum;
            }
        }
    }
}


impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + Sync + Send
{
    pub fn mmult_speed(&mut self, other: &Self) -> Result<(), ListError> {
        match (self, other) {
            (Self::Array2D { arr: arr1, nr: nr1, nc: nc1, put_val_by_row: by_row1}, 
             Self::Array2D { arr: arr2, nr: nr2, nc: nc2, put_val_by_row: by_row2}) => {
                if *nc1 != *nr2 {
                    return Err(ListError::MatrixMultMismatchedDim);
                }

                let new_dim: (usize, usize) = (*nr1, *nc2);
                let ni = *nc1;
                let new_vec: Vec<T> = vec![T::default(); new_dim.0 * new_dim.1];
                let new_box: Box<[T]> = new_vec.into_boxed_slice();
 
                // replace arr
                let old_arr: Box<[T]> = mem::replace(arr1, new_box);                
                // replace nr, nc
                let _ = (
                    mem::replace(nr1, new_dim.0), 
                    mem::replace(nc1, new_dim.1)
                );
                // replace by_row
                let old_by_row = mem::replace(by_row1, true);

                Array::matrix_mult_speed(arr1, arr2, old_arr, new_dim, ni, old_by_row, *by_row2);

            },
            
            _ => return Err(ListError::MismatchedTypes),
        };

        Ok(())
    }

    fn matrix_mult_speed(
        arr1: &mut Box<[T]>, arr2: &Box<[T]>, old_arr: Box<[T]>, 
        new_dim: (usize, usize), 
        ni: usize, 
        old_by_row: bool, by_row2: bool) 
    {
        let pool = ThreadPoolBuilder::new()
            .num_threads(6)
            .build()
            .unwrap();

        let index_old: fn(usize, usize, (usize, usize)) -> usize =  if old_by_row { idxr } else { idxc };
        let index_other: fn(usize, usize, (usize, usize)) -> usize = if by_row2 { idxr } else { idxc };
        
        pool.install(|| {
            arr1.par_chunks_mut(new_dim.1).enumerate().for_each(|(r, row)| {
                for c in 0..new_dim.0 {
                    let mut sum = T::default();
                    for i in 0..ni {
                        sum += 
                        
                        old_arr[index_old(r, i, (new_dim.0, ni))] 
                        * 
                        arr2[index_other(i, c, (ni, new_dim.1))];
                    }

                    row[c] = sum;
                }
            })
        });
    }
}
