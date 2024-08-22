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

}


#[cfg(test)]
pub mod tests {
    use crate::array::{Array, ListError};

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
    fn eigen_method_arr_2d_1() -> Result<(), ListError>{
        let ma = vec![
            1.23, 1.25,
            0.01, 5.09,
        ];

        let dim: (usize, usize) = (2, 2);
        let e_vals: Vec<f64> = Array::eigen_values(&ma, dim, true, None, None)?;
        println!("{:?}", e_vals);

        Ok(())
    }

    #[test]
    fn eigen_method_arr_2d_2() -> Result<(), ListError>{
        let ma = vec![
            2.0, 0.0, 0.0,
            7.0, 11.0, 0.0,
            17.0, 19.0, 23.0
        ];

        let dim: (usize, usize) = (3, 3);

        let e_vals: Vec<f64> = Array::eigen_values(&ma, dim, true, None, None)?;
        println!("{:?}", e_vals);

        Ok(())
    }
}
