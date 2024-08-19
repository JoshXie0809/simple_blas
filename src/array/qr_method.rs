use super::{Array, ListError};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};
use super::{idxr, idxc};

impl<T> Array<T> 
where T: Sqrt + Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    
    fn qr(&self) -> Result<( Vec<T>, Vec<T> ), ListError> {
        
        let (q, r) = 
        match self {
            Array::Array2D { arr, nr, nc, put_val_by_row }
            => {
                
                let idx: fn(usize, usize, (usize, usize)) -> usize = if *put_val_by_row {idxr} else {idxc};
                // matrix is n x m
                // q is n x n, orthorgonal matrix
                // m is n x m, u-tri matrix
                let dim = (*nr, *nc);
                let (nr, nc) = dim;
                let dimq: (usize, usize) = (nr, nr);
                let dimr: (usize, usize) = (nr, nc);
                
                     
                let (q, r) = Array::qr_dcmp(&arr, dim, idx);
                (q, r)
            },

            _ => return Err(ListError::MismatchedTypes),
        };

        Ok((q, r))
    }

    pub(crate) fn qr_dcmp(
        arr: &[T],
        dim: (usize, usize),
        idx: fn(usize, usize, (usize, usize)) -> usize
    ) ->  (Vec<T>, Vec<T>)
    {
        let (nr, nc) = dim;
        let dimq: (usize, usize) = (nr, nr);
        let dimr: (usize, usize) = (nr, nc);

        let mut q = vec![T::default(); (nr) * (nr)]; 
        let mut r = vec![T::default(); (nr) * (nc)];

        
        // the 0st column of q is the first column of matrix
        // make orthorgonal vector
        let mut sum: T = T::default();
        for r in 0..dimq.0 {
            let val: T = arr[idx(r, 0, dim)];
            q[idx(r, 0, dimq)] = val;
            sum += val * val;
        }

        // make unit vector
        let length: T = sum.sqrt();
        for r in 0..dimq.0 {
            q[idx(r, 0, dimq)] = q[idx(r, 0, dimq)] / length;
        }
        
        for c in 1..dimq.1 {
            for ci in 0..c {

            }
        }

        (q, r)
    }
}


#[cfg(test)]
pub mod tests {
    use crate::array::{Array, ListError};

    #[test]
    fn qr_arr_2d() {
        let arr = Array::Scalar(1.0);
        if let Err(error) = arr.qr() {
            assert_eq!(error, ListError::MismatchedTypes);
        };
    }
}


pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}