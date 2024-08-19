use super::{Array, ListError};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};
use super::{idxr, idxc};

impl<T> Array<T> 
where T: Sqrt + 
Add<Output=T> + Mul<Output=T> + 
Div<Output=T> + Sub<Output=T>
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    
    pub fn qr(&self) -> Result<(Array<T>, Array<T>), ListError> {
        
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
                     
                let (q, r) = Array::qr_dcmp(&arr, dim, *put_val_by_row);
                
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

    pub(crate) fn qr_dcmp(
        arr: &[T],
        dim: (usize, usize),
        by_row: bool,
    ) ->  (Vec<T>, Vec<T>)
    {
        let idx: fn(usize, usize, (usize, usize)) -> usize = if by_row {idxr} else {idxc};

        let (nr, nc) = dim;
        // rank
        let n = if nr < nc {nr} else {nc};
        let dimq: (usize, usize) = (nr, n);
        let dimr: (usize, usize) = (n, nc);

        // qm: Qmatrix, rm: Rmatrix
        let mut qm: Vec<T> = vec![T::default(); nr * n]; 
        let mut rm: Vec<T> = vec![T::default(); n * nc];

        // the 0st column of q is the first column of matrix
        // make orthorgonal vector
        let mut sum: T = T::default();
        for r in 0..dimq.0 {
            let val: T = arr[idx(r, 0, dim)];
            qm[idx(r, 0, dimq)] = val;
            sum += val * val;
        }

        // make unit vector
        let length: T = sum.sqrt();
        
        for r in 0..dimq.0 {
            qm[idx(r, 0, dimq)] = qm[idx(r, 0, dimq)] / length;
        }

        // put val to r
        rm[idx(0, 0, dimr)] = length;
        
        for c in 1..dimq.1 {
            for c2 in 0..c {
                let mut sum: T = T::default();
                // R(r, c) = Qc(r)'A(c)
                // all element in column
                for i in 0..dimq.0 {
                    sum += qm[idx(i, c2, dimq)] * arr[idx(i, c, dim)];
                }
                rm[idx(c2, c, dimr)] = sum;
            }

            for i in 0..dimq.0 {
                qm[idx(i, c, dimq)] = arr[idx(i, c, dim)];
            }

            for c2 in 0..c {
                let factor: T = rm[idx(c2, c, dimr)];
                for i in 0..dimq.0 {
                    let val: T = qm[idx(i, c2, dimq)];
                    qm[idx(i, c, dimq)] -= factor * val;
                }
            }

            let mut sum = T::default();
            for i in 0..dimq.0 {
                let val = qm[idx(i, c, dimq)];
                sum += val * val;
            }
            let length = sum.sqrt();
            for i in 0..dimq.0 {
                qm[idx(i, c, dimq)] = qm[idx(i, c, dimq)] / length;
            }

            rm[idx(c, c, dimr)] = length;
        }

        if nr < nc {
            for c in n..nc {
                let mut b: Vec<T> = vec![T::default(); nr];
                let mut res: Vec<T> = vec![T::default(); nr];
                for i in 0..nr {
                    b[i] = arr[idx(i, c, dim)];
                }
                // q (nr, nr)
                // r (nr, nc)
                // b (nc, 1)
                // q x = b
                
                Array::q_solve(&qm, &b, &mut res, dimq, by_row);

                for i in 0..nr {
                    rm[idx(i, c, dim)] = res[i];
                }
            }
        }

        (qm, rm)
    }

    // Qx = b
    // Q is orthonogal and unit vector
    // x = Q'b
    pub(crate) fn q_solve(
        qm: &[T], b: &[T], res: &mut [T],
        dimq: (usize, usize),
        by_row: bool
    ) {
        // Qt
        // arr
        // a b c d e f
        // dim (3, 2)
        // by_row false
        
        // a d 
        // b e
        // c f

        // arr
        // a b c d e f
        // dim (2, 3)
        // by_row true

        // transpose
        // a b c
        // d e f

        // make q transpose
        let (nr, nc) = dimq;
        let dimqt: (usize, usize) = (nc, nr);
        let by_row = !by_row;

        Array::mat_a_dot_vec_b(qm, b, res, dimqt, by_row);

    }

    pub(crate) fn mat_a_dot_vec_b(
        am: &[T], b: &[T], res: &mut [T],
        dim: (usize, usize), 
        by_row: bool,
    ) {
        
        let idx: fn(usize, usize, (usize, usize)) -> usize = if by_row {idxr} else {idxc};
        // A: m x n
        // b: n x 1
        // res: m x 1
        let (nr, nc) = dim;
        for r in 0..nr {
            let mut sum = T::default();
            for i in 0..nc {
                sum += am[idx(r, i, dim)] * b[i];
            }
            res[r] = sum;
        }
    }

}


#[cfg(test)]
pub mod tests {
    use crate::array::Array;

    #[test]
    fn qr_dcmp_arr_2d() {
        let arr: Box<[f64]> = vec![1.0, 0.0, 3.0, 4.0].into_boxed_slice();
        let dim: (usize, usize) = (2, 2);
        let (q, r) = Array::qr_dcmp(&arr, dim, false);
        
        println!("{:?}", q);
        println!("{:?}", r);
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