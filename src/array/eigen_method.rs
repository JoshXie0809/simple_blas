use super::Array;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};
use crate::array::matrix_operations::Sqrt;

impl<T> Array<T> 
where T: Sqrt + 
Add<Output=T> + Mul<Output=T> + 
Div<Output=T> + Sub<Output=T>
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + PartialOrd + From<f32>
{
    pub fn meig() {
        
    }
}


#[cfg(test)]
pub mod tests {
    use crate::array::{idxr, Array, ListError};

    #[test]
    fn hessenberg_arr_2d_1() -> Result<(), ListError> {
        let mut ma = vec![
            4.0, 1.0, -2.0, 2.0,
            1.0, 2.0, 0.0, 1.0,
            -2.0, 0.0, 3.0, -2.0,
            2.0, 1.0, -2.0, 1.0
        ];

        let dim: (usize, usize) = (4, 4);

        let qf = Array::hessenberg(&mut ma, dim, true)?;
        
        println!("{:?}", ma);
        println!("{:?}", qf);

        Ok(())
    }

    #[test]
    fn hessenberg_arr_2d_2() -> Result<(), ListError> {
        let mut ma = vec![
            1.0, 2.0, 3.0, 5.0,
            4.0, 6.0, 11.0, 7.0,
            5.0, 9.0, 13.0, 4.0,
            11.0, 9.0, 5.0, 19.0
        ];

        let dim: (usize, usize) = (4, 4);

        let qf = Array::hessenberg(&mut ma, dim, true)?;
        
        println!("{:?}", ma);
        println!("{:?}", qf);

        Ok(())
    }

    #[test]
    fn hessenberg_arr_2d_3() -> Result<(), ListError> {
        let mut ma: Vec<f64> = vec![
            -149.0, -50.0, -154.0,
            537.0, 180.0, 546.0, 
            -27.0, -9.0, -25.0,
        ];

        let dim: (usize, usize) = (3, 3);

        let qf = Array::hessenberg(&mut ma, dim, true)?;
        
        println!("{:?}", ma);
        println!("{:?}", qf);

        Ok(())
    }

    #[test]
    fn hessenberg_arr_2d_4() -> Result<(), ListError> {
        let mut ma: Vec<f64> = vec![
            1.0, 3.0, 4.0,
            4.0, 5.0, 0.0, 
            1.0, 2.0, 1.0,
        ];

        let dim: (usize, usize) = (3, 3);

        let qf = Array::hessenberg(&mut ma, dim, true)?;
        
        println!("{:?}", ma);
        println!("{:?}", qf);

        Ok(())
    }

    #[test]
    fn ma_dot_q_factor_arr_2d_1() -> Result<(), ListError>{
        let arr: Vec<f64> = vec![
            1.0, 2.0, 3.0,
            4.0, 9.0, 11.0,
            17.0, 18.0, 29.0,
        ];

        let dim: (usize, usize) = (3, 3);

        let (qf, mut r) = Array::qr_householder(&arr, dim, true)?;
        let qm: Vec<f64> = Array::get_qm(&qf, 3);

        let mut res: Vec<f64> = vec![0.0; 9];
        Array::mat_m1_mat_mult_mat_m2(&mut res, &r, &qm, (3, 3), 3, true, true);

        println!("{:?}", res);

        let idx: fn(usize, usize, (usize, usize)) -> usize = idxr;
        Array::ma_dot_q_factor(&mut r, dim, idx, &qf);
        println!("{:?}", r);

        let d: f64 = Array::dist_n2_vec_v1_v2(&r, &res)?;
        assert!(d < 1e-10);
        
        Ok(())
    }

    #[test]
    fn eigen_method_arr_2d_1() -> Result<(), ListError>{
        let ma = vec![
            1.23, 0.05,
            0.05, 5.09,
        ];

        let dim: (usize, usize) = (2, 2);
        let e_vals: Vec<f64> = Array::eigen_values(&ma, dim, true, None, None)?;
        println!("{:?}", e_vals);

        Ok(())
    } 

    #[test]
    fn eigen_method_arr_2d_2() -> Result<(), ListError> {
        let ma: Vec<f64> = vec![
             1.01, 0.0, 0.0,
             5.07, 6.07, 0.0,
             7.21, 3.12, 5.20
        ];

        let dim: (usize, usize) = (3, 3);

        let e_vals: Vec<f64> = Array::eigen_values(&ma, dim, true, None, None)?;
        println!("{:?}", e_vals);

        let d: f64 = Array::dist_n2_vec_v1_v2(&e_vals, &vec![6.07, 5.20, 1.01])?;

        assert!(d < 1e-10);

        Ok(())
    }

    #[test]
    fn eigen_method_arr_2d_3() -> Result<(), ListError> {
        let ma: Vec<f64> = vec![
             1.01, 2.02, 3.79, 0.05,
             5.07, 6.07, 11.95, 1.23,
             7.21, 3.12, 5.20, 1.17,
             7.719, 5.508, 9.919, 11.21,
        ];

        let dim: (usize, usize) = (4, 4);

        let e_vals: Vec<f64> = Array::eigen_values(&ma, dim, true, None, None)?;
        println!("{:?}", e_vals);

        let e_vecs: Vec<f64> = Array::eigen_vectors(&ma, dim, true, &e_vals)?;
        println!("{:?}", e_vecs);

        Ok(())
    }

    #[test]
    fn eigen_method_arr_2d_4() -> Result<(), ListError> {
        let ma: Vec<f64> = vec![
             1.5, 2.19, 0.27,
             0.00, 1.01, 0.9,
             0.00, 0.09, 3.45,
        ];

        let dim: (usize, usize) = (3, 3);

        let e_vals: Vec<f64> = Array::eigen_values(&ma, dim, true, None, None)?;
        println!("{:?}", e_vals);

        let e_vecs: Vec<f64> = Array::eigen_vectors(&ma, dim, true, &e_vals)?;
        println!("{:?}", e_vecs);

        Ok(())
    }
}
