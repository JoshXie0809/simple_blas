use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};
use std::usize;
use super::{idxc, idxr, ListError};

use super::Array;

#[allow(dead_code)]
impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ Sub<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + PartialOrd
{
    // basic function
    pub(crate) fn abs(val: T, z: T) -> T {
        if val < z {return z-val};
        val
    }

    // matrix row permutation

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
            Array::swap_r_ij(arr, r, maxr, 0, nc, idx, dim);
            p.push((r, maxr));
        }
    }

    pub(crate) fn permute_rc(
        arr: &mut Box<[T]>, 
        r:usize, dim: (usize, usize), z: T, 
        pr: &mut Vec<(usize, usize)>,
        pc: &mut Vec<(usize, usize)>,
        idx: fn(usize, usize, (usize, usize)) -> usize,
    ) {
        let (nr, nc) = dim;
        // now is rth row
        // assume max element on this row

        let mut maxr = r;
        let mut maxc = r;
        let mut maxv = Array::abs(arr[idx(r, r, dim)], z);

        // find maxv in (maxr, maxc) under (r, r)
        for ri in 0..(nr - r) {
            for ci in 0..(nc - r) {
                let val1 = Array::abs(arr[idx(r + ri, r + ci, dim)], z);
                if maxv < val1 {
                    maxr = r + ri;
                    maxc = r + ci;
                    maxv = val1;
                }
            }
        }
        
        // we find the maxr not equal r,
        // swap rth row and maxr row
        if r != maxr {
            Array::swap_r_ij(arr, r, maxr, 0, nc, idx, dim);
            pr.push((r, maxr));
        }

        // we find the maxc not equal r,
        // swap rth col and maxc col
        if r != maxc {
            Array::swap_c_ij(arr, r, maxc, 0, nr, idx, dim);
            pc.push((r, maxc));
        }

    }

    // swap ith, jth row, in begin..end columns
    pub(crate) fn swap_r_ij(arr: &mut Box<[T]>, 
                  i: usize, j: usize, 
                  begin_col: usize, 
                  end_col: usize, 
                  idx: fn(usize, usize, (usize, usize)) -> usize,
                  dim: (usize, usize)) 
    {
        for c in begin_col..end_col {
            // let idx1: usize = idx(i, c, dim);
            // let idx2: usize = idx(j, c, dim);
            // arr.swap(idx1, idx2);
            arr.swap(idx(i, c, dim), idx(j, c, dim));
        }
    }

    // row operation
    // (row j) minus (factor * row i)
    // from start element to end
    pub(crate) fn row_i_add_frow_j(
        arr: &mut [T], factor: T, i: usize, j: usize,
        dim: (usize, usize), 
        idx: fn(usize, usize, (usize, usize)) -> usize,
        start_col: usize, end_col: usize
    ) {
        for k in start_col..end_col {
            arr[idx(i, k, dim)] += factor * arr[idx(j, k, dim)];
        }
    }

    pub(crate) fn row_i_minus_frow_j(
        arr: &mut [T], factor: T, i: usize, j: usize,
        dim: (usize, usize), 
        idx: fn(usize, usize, (usize, usize)) -> usize,
        start_col: usize, end_col: usize) 
    {
        for k in start_col..end_col {
            arr[idx(i, k, dim)] -= factor * arr[idx(j, k, dim)];
        }
    }

    // swap ith, jth row, in begin..end columns
    pub(crate) fn swap_c_ij(arr: &mut Box<[T]>, 
        i: usize, j: usize, 
        begin_row: usize, 
        end_row: usize, 
        idx: fn(usize, usize, (usize, usize)) -> usize,
        dim: (usize, usize)) 
    {
        for r in begin_row..end_row {
            // let idx1: usize = idx(i, c, dim);
            // let idx2: usize = idx(j, c, dim);
            // arr.swap(idx1, idx2);
            arr.swap(idx(r, i, dim), idx(r, j, dim));
        }
    }

    // matrix add 

    pub(crate) fn self_add_scalar_s (
        arr: &mut [T], s: T
    ) {
        for i in 0..arr.len() {
            arr[i] += s;
        }
    }

    pub(crate) fn self_add_vec_v2 (
        arr: &mut [T], v2: &[T]
    ) -> Result<(), ListError>
    {
        let length = arr.len();
        if length != v2.len() {return Err(ListError::DifferentLength1D);}

        for i in 0..length {
            arr[i] += v2[i]
        }

        Ok(())
    }

    pub(crate) fn self_minus_vec_v2 (
        arr: &mut [T], v2: &[T]
    ) -> Result<(), ListError>
    {
        let length = arr.len();
        if length != v2.len() {return Err(ListError::DifferentLength1D);}

        for i in 0..length {
            arr[i] -= v2[i]
        }

        Ok(())
    }

    pub(crate) fn self_add_mat_m (
        arr: &mut [T], other: &[T],
        dim1: (usize, usize), dim2: (usize, usize),
        idx1: fn(usize, usize, (usize, usize)) -> usize,
        idx2: fn(usize, usize, (usize, usize)) -> usize
    ) -> Result<(), ListError>
    {
        if dim1 != dim2 {return Err(ListError::MismatchedDim);}
        let (nr, nc) = dim1;

        for r in 0..nr {
            for c in 0..nc {
                arr[idx1(r, c, dim1)] += other[idx2(r, c, dim2)];
            }
        }

        Ok(())
    }

    pub(crate) fn self_minus_mat_m (
        arr: &mut [T], other: &[T],
        dim1: (usize, usize), dim2: (usize, usize),
        idx1: fn(usize, usize, (usize, usize)) -> usize,
        idx2: fn(usize, usize, (usize, usize)) -> usize
    ) -> Result<(), ListError>
    {
        if dim1 != dim2 {return Err(ListError::MismatchedDim);}
        let (nr, nc) = dim1;

        for r in 0..nr {
            for c in 0..nc {
                arr[idx1(r, c, dim1)] -= other[idx2(r, c, dim2)];
            }
        }

        Ok(())
    }

    // matrix mult
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



#[allow(dead_code)]
impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ Sub<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + PartialOrd + From<f32> + Sqrt
{
    // solve linear algebra

    // LU x = pb
    // give L, U, p, and b to solve x
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

    pub(crate) fn qr(
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
        // make q transpose
        let (nr, nc) = dimq;
        let dimqt: (usize, usize) = (nc, nr);
        let by_row = !by_row;

        Array::mat_a_dot_vec_b(qm, b, res, dimqt, by_row);
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