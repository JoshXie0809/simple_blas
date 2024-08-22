use super::{Array, ListError, idxr, idxc};
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
    fn eigen_method_arr_2d_1() -> Result<(), ListError>{
        let ma = vec![
            1.23, 0.2,
            0.1, 5.09,
        ];

        let dim: (usize, usize) = (2, 2);

        let e_vals: Vec<f64> = Array::eigen_values(&ma, dim, true)?;

        println!("{:?}", e_vals);

        Ok(())
    }
}
