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

    pub(crate) fn self_mult_scalar_s (
        arr: &mut [T], s: T
    ) {
        for i in 0..arr.len() {
            arr[i] *= s;
        }
    }

    pub(crate) fn self_div_scalar_s (
        arr: &mut [T], s: T) -> Result<(), ListError>
    {
        if s == T::default() {return Err(ListError::DivisionByZero);}
        for i in 0..arr.len() {
            arr[i] = arr[i] / s;
        }

        Ok(())
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

    pub(crate) fn self_ele_mult_vec_v2 (
        arr: &mut [T], v2: &[T]
    ) -> Result<(), ListError>
    {
        let length = arr.len();
        if length != v2.len() {return Err(ListError::DifferentLength1D);}

        for i in 0..length {
            arr[i] *= v2[i]
        }

        Ok(())
    }

    pub(crate) fn self_ele_div_vec_v2 (
        arr: &mut [T], v2: &[T]
    ) -> Result<(), ListError>
    {
        let length = arr.len();
        if length != v2.len() {return Err(ListError::DifferentLength1D);}
        let z = T::default();
        for i in 0..length {
            let val = v2[i];
            if val == z {return Err(ListError::DivisionByZero);}
            arr[i] = arr[i] / val;
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

    pub(crate) fn self_ele_mult_mat_m (
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
                arr[idx1(r, c, dim1)] *= other[idx2(r, c, dim2)];
            }
        }

        Ok(())
    }

    pub(crate) fn self_ele_div_mat_m (
        arr: &mut [T], other: &[T],
        dim1: (usize, usize), dim2: (usize, usize),
        idx1: fn(usize, usize, (usize, usize)) -> usize,
        idx2: fn(usize, usize, (usize, usize)) -> usize
    ) -> Result<(), ListError>
    {
        if dim1 != dim2 {return Err(ListError::MismatchedDim);}
        let (nr, nc) = dim1;
        let z = T::default();
        for r in 0..nr {
            for c in 0..nc {
                let val = other[idx2(r, c, dim2)];
                if val == z {return Err(ListError::DivisionByZero);}
                arr[idx1(r, c, dim1)] = arr[idx1(r, c, dim1)] / val;
            }
        }

        Ok(())
    }

    // matrix mult
    
    pub(crate) fn mat_m1_mat_mult_mat_m2(
        res: &mut [T], 
        m1: &[T], m2: &[T], 
        new_dim: (usize, usize), 
        ni: usize, 
        by_row_m1: bool, by_row_m2: bool) 
    {
        let index1: fn(usize, usize, (usize, usize)) -> usize = if by_row_m1 { idxr } else { idxc };
        let index2: fn(usize, usize, (usize, usize)) -> usize = if by_row_m2 { idxr } else { idxc };

        for r in 0..new_dim.0 {
            for c in 0..new_dim.1 {
                let mut sum = T::default();
                for i in 0..ni {
                    sum += 
                    
                    m1[index1(r, i, (new_dim.0, ni))] 
                    * 
                    m2[index2(i, c, (ni, new_dim.1))];
                    
                }

                res[idxr(r, c, new_dim)] = sum;
            }
        }
    }
    
    pub(crate) fn vec_a_dot_vec_b (
        va: &[T], vb: &[T]
    ) -> T 
    {
        let valen: usize = va.len();
        if valen != vb.len() {panic!("cannot dot two different length vector")}
        
        let mut sum = va[0] * vb[0];
        for i in 1..valen {
            sum += va[i] * vb[i]
        }
        sum
    }

    pub(crate) fn mat_a_dot_vec_b (
        am: &[T], b: &[T], res: &mut [T],
        dim: (usize, usize), 
        by_row: bool,
    ) {
        let idx: fn(usize, usize, (usize, usize)) -> usize = if by_row {idxr} else {idxc};
        // A: m x n
        // b: n x 1
        // res: m x 1
        let (nr, nc) = dim;
        if nc != b.len() {panic!("matrix total number of coulmn not matched vector length")}
        
        for r in 0..nr {
            let mut sum = T::default();
            for i in 0..nc {
                sum += am[idx(r, i, dim)] * b[i];
            }
            res[r] = sum;
        }
    }

    pub(crate) fn vec_b_dot_mat_a (
        am: &[T], b: &[T], res: &mut [T],
        dim: (usize, usize), 
        by_row: bool,
    ) {
        let idx: fn(usize, usize, (usize, usize)) -> usize = if by_row {idxr} else {idxc};
        // A: m x n
        // b: n x 1
        // res: m x 1
        let (nr, nc) = dim;
        if nr != b.len() {panic!("matrix total number of row not matched vector length")}

        for c in 0..nc {
            let mut sum = T::default();
            for i in 0..nr {
                sum += am[idx(i, c, dim)] * b[i];
            }
            res[c] = sum;
        }
    }

    pub(crate) fn dist_n1_vec_v1_v2(
        v1: &[T], v2: &[T]
    ) -> Result<T, ListError> {
        let z = T::default();
        let mut sum = z ;
        let length = v1.len();
        if length != v2.len() {return Err(ListError::DifferentLength1D);}
        for i in 0..length {
            sum += Array::abs(v1[i] - v2[i], z)
        }

        Ok(sum)
    }

}



#[allow(dead_code)]
impl<T> Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ Sub<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + PartialOrd + From<f32> + Sqrt
{
    // distance norm 2
    
    pub(crate) fn dist_n2_vec_v1_v2(
        v1: &[T], v2: &[T]
    ) -> Result<T, ListError> {
        let mut sum = T::default();
        let length = v1.len();
        if length != v2.len() {return Err(ListError::DifferentLength1D);}
        for i in 0..length {
            let val = v1[i] - v2[i];
            sum += val.powi(2);
        }

        Ok(sum.sqrt())
    }

    // solve linear algebra

    // LU x = pb
    // give L, U, p, and b to solve x
    pub(crate) fn p_lu_solve(
        lu: &[T], p: &[(usize, usize)], b: &mut [T], x: &mut [T],
        dim: (usize, usize), idx: fn(usize, usize, (usize, usize)) -> usize
    ) -> Result<(), ListError>
    {
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
        Array::l_tri_solve(lu, b, &mut y, dim, idx, is_lu)?;

        // after we solve y
        // solve Ux = y
        // similarly x is easy to solve

        Array::u_tri_solve(lu, &y, x, dim, idx)?;
        
        // turn pb to b
        for &(i, j) in p.iter().rev() {
            b.swap(i, j);
        }

        Ok(())
    }

    // solve Lx = b
    pub(crate) fn l_tri_solve(
        l: &[T], b: &[T],
        x: &mut [T],
        dim: (usize, usize), 
        idx: fn(usize, usize, (usize, usize)) -> usize,
        is_lu: bool
    ) -> Result<(), ListError>
    {
        
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

        let (nr, nc) = dim;
        if nr != nc {return Err(ListError::NotSquareMat);}

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

        Ok(())
        
    }

    // solve Ux = b
    pub(crate) fn u_tri_solve(
        u: &[T], b: &[T],
        x: &mut [T],
        dim: (usize, usize), 
        idx: fn(usize, usize, (usize, usize)) -> usize,
    ) -> Result<(), ListError>
    {
        // similarly, we solve X by column

        // m = n-1
        // u[m, m] * x[m] = yi[m]
        // x[m] = yi[m] / u[m, m]

        // m - 1
        // u[m-1, m-1] * x[m-1] + u[m-1, m] * x[m] = yi[m-1]
        // x[m-1] = (yi[m-1] - u[m-1, m] * x[m]) / u[m-1, m-1]
        
        let (nr, nc) = dim;
        if nr != nc {return Err(ListError::NotSquareMat);}
        for r in (0..nr).rev() {
            // forward solve
            let mut sum = T::default();
            for fi in (r+1)..nc {
                sum += u[idx(r, fi, dim)] * x[fi];
            }
            x[r] = (b[r] - sum) / u[idx(r, r, dim)];
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

    // compute vector norm ||v||_2
    pub(crate) fn norm_2(v1: &[T]) -> T{
        let mut d = v1[0] * v1[0];
        for i in 1..(v1.len()) {
            d += v1[i] * v1[i];
        }
        return d.sqrt();
    }

    // find householder reflector
    pub(crate) fn reflector(y: &[T], reflector: &mut [T]) -> Result<(), ListError> 
    {
        // compute ||w||_2
        // ||w||_2^2  = 2 ||y|| (||y|| + |y_1|)
        let y_norm2: T = Array::norm_2(y);

        let z: T = T::default();
        let sign_y1: T = if y[0] < z {T::from(-1.0_f32)} else {T::from(1.0_f32)};
        let y1_abs: T = sign_y1 * y[0];
        
        let w_norm2: T = T::from(2.0_f32) * y_norm2 * (y_norm2 + y1_abs);
        let w_norm2 = w_norm2.sqrt();

        if w_norm2 < T::from(1e-20_f32) {return Err(ListError::ReflectorZeroLength);}
        
        let ylen = y.len();
        if ylen != reflector.len() {panic!("vec and its reflector must has same length")}
        
        reflector[0] = (y[0] + sign_y1 * y_norm2) / w_norm2;
        for i in 1..y.len() {
            reflector[i] = y[i] / w_norm2;
        }

        Ok(())
    }

    // for reflator vector v
    // H = I - 2 vv'
    // H A
    pub(crate) fn reflector_mat_dot_mat(
        v1: &[T], // refector vector which build H matrix
        ma: &mut [T], // mat_a,
        dim: (usize, usize),
        idx: fn(usize, usize, (usize, usize)) -> usize
    ) {
        let (nr, nc) = dim;
        let k: usize = nr - v1.len(); // k must >= 0

        for c in k..nc {
            // constant
            let mut sum: T = T::default();
            for r in k..nr {
               sum += v1[r-k] * ma[idx(r, c, dim)];
            }
            
            let two = T::from(2.0_f32);
            for r in k..nr {
                ma[idx(r, c, dim)] -= two * sum * v1[r - k];
            }
        }
    }

    // A = q_factor * r
    //  A = H[1] H[2] ... H[n] r
    pub(crate) fn qr_householder(
        ma: &[T],
        dim: (usize, usize),
        by_row: bool
    ) -> Result<(Vec<Vec<T>>, Vec<T>), ListError> {
        let idx: fn(usize, usize, (usize, usize)) -> usize = if by_row {idxr} else {idxc};
        let (nr, nc) = dim;
        
        if nr < nc {panic!("nr must >= nc")}

        let mut a_mat: Vec<T> = ma.to_vec().clone();
        let mut q_factor: Vec<Vec<T>> = vec![];
        let z = T::default();

        for c in 0..nc {
            let mut reflector: Vec<T> = vec![z; nr-c];
            let mut v1: Vec<T> = vec![z; nr-c];
                
            for i in c..nr {
                v1[i-c] = a_mat[idx(i, c, dim)];
            }

            if let Err(error) = Array::reflector(&v1, &mut reflector) {
                if error != ListError::ReflectorZeroLength {return Err(error);}
            }
            Array::reflector_mat_dot_mat(&reflector, &mut a_mat, dim, idx);
            q_factor.push(reflector);
        }

        q_factor.reverse();

        // the a_mat finally become R
        Ok((q_factor, a_mat))
    }

    pub(crate) fn q_factor_dot_ma(
        q_factor: & Vec<Vec<T>>,
        ma: &mut [T],
        dim: (usize, usize),
        idx: fn(usize, usize, (usize, usize)) -> usize,
        transpose: bool
    ) {
        if transpose {
            for q in q_factor.iter().rev() {
                Array::reflector_mat_dot_mat(q, ma, dim, idx);
            }
        } else {
            for q in q_factor.iter() {
                Array::reflector_mat_dot_mat(q, ma, dim, idx);
            }
        }
        
    }

    pub(crate) fn ma_dot_q_factor(
        ma: &mut [T],
        dim: (usize, usize),
        by_row: bool,
        q_factor: & Vec<Vec<T>>,
    ) {
        // A * H[1] H[2] ... H[N]
        // (H[N]H[N-1]... H[1] A')'
        
        // for A'
        let (nr, nc) = dim;
        let dimt: (usize, usize) = (nc, nr);
        let by_row_t: bool = !by_row;
        let idxt: fn(usize, usize, (usize, usize)) -> usize = if by_row_t {idxr} else {idxc};
        Array::q_factor_dot_ma(q_factor, ma, dimt, idxt, true);
    }

    pub(crate) fn eigen_values(
        ma: &[T], dim: (usize, usize), 
        by_row: bool, 
        max_iter: Option<usize>,
        max_tol: Option<f32>
    ) -> Result<Vec<T>, ListError>
    {
        let (nr,nc) = dim;
        if nr != nc {return Err(ListError::EigenMismatchedDim);}
        let mut mat_a: Vec<T> = ma.to_vec();

        let n: usize = nr;
        let idx: fn(usize, usize, (usize, usize)) -> usize = if by_row {idxr} else {idxc};
        let two:  T = T::from(2.0_f32);
        let four: T = T::from(4.0_f32);
        let z:    T = T::default();

        let n_iter: usize = 
        if let Some(ni) = max_iter {ni} else {100_usize};

        let mtol: T =
        if let Some(mt) = max_tol {T::from(mt)} else {T::from(1e-15_f32)};

        for _iter in 0..n_iter {
            // check sub-diagnol whether or not close to zero
            let mut isbreak = true;

            for r in 1..nr {
                for c in 0..r {
                    if Array::abs(mat_a[idx(r, c, dim)], z) > mtol {
                        isbreak = false;
                        break;
                    }
                }
                if !isbreak {break;}
            }

            if isbreak {break;}

            // wilkinson shift
            let b11: T = mat_a[idx(n-2, n-2, dim)];
            let b12: T = mat_a[idx(n-2, n-1, dim)];
            let b21: T = mat_a[idx(n-1, n-2, dim)];
            let b22: T = mat_a[idx(n-1, n-1, dim)];
            let mut lambda1 = z;
            let mut lambda2 = z;
            let p2: T = (b11 - b22).powi(2) + four * b12 * b21; 
            lambda1 +=  (b11 + b22 + p2.sqrt()) / two;
            lambda2 +=  (b11 + b22 - p2.sqrt()) / two;
            let d1: T = Array::abs(lambda1 - b22, z);
            let d2: T = Array::abs(lambda2 - b22, z);
            let s: T = if d1 < d2 {lambda1} else {lambda2};
            
            // make shift
            for i in 0..n {
                mat_a[idx(i, i, dim)] -= s
            }
            
            let (q, mut r) = Array::qr_householder(&mat_a, dim, by_row)?;
            Array::ma_dot_q_factor(&mut r, dim, by_row, &q);
            mat_a = r;

            // recover shift
            for i in 0..n {
                mat_a[idx(i, i, dim)] += s
            }
        }

        let mut eigen_values: Vec<T> = vec![z; n];
        for i in 0..n {
            eigen_values[i] = mat_a[idx(i, i, dim)];
        }
        
        Ok(eigen_values)
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

    // solve QR x = b
    // Q'Q Rx = Q'b
    // Rx = Q'b
    pub(crate) fn qr_solve (
        res: &mut [T],
        qm: &[T], rm: &[T], 
        b: &[T],
        dimq: (usize, usize),
        dimr: (usize, usize),
        by_row_q: bool,
        by_row_r: bool
    ) -> Result<(), ListError>
    {
        // first let y = Rx
        // solve Q y = b
        let (nr_q, nc_q) = dimq;
        if nr_q != nc_q {return Err(ListError::NotSquareMat);}
        let mut y = vec![T::default(); dimr.1];
        Array::q_solve(qm, b, &mut y, dimq, by_row_q);

        //  solve Rx = y
        let idx: fn(usize, usize, (usize, usize)) -> usize = if by_row_r {idxr} else {idxc};
        Array::u_tri_solve(rm, &y, res, dimr, idx)?;

        Ok(())
    }
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
    fn powi(self, n: i32) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
}