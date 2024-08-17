#[cfg(test)]
pub mod tests {
    use simple_blas::array::{Array, ListError};


    #[test]
    fn inv_arr_2d_1() -> Result<(), ListError>{
        let mut arr = Array::new_array_2d(
            Box::new([1.0, 2.0,
                           3.0, 4.0]), 
            (2, 2), 
            true
        )?;

        arr.inv()?;

        let real_inv = Array::new_array_2d(
            Box::new([-2.0, 1.0, 1.5, -0.5]), 
            (2, 2), 
            true
        )?;

        let tol: f64 = 1e-10;

        let diff: f64 = Array::compute_dist(&arr, &real_inv)?;

        assert!(diff < tol);

        Ok(())
    }

    #[test]
    fn inv_arr_2d_2() -> Result<(), ListError>{
        let mut arr = Array::new_array_2d(
            Box::new([1.0, 22.0, 5.0,
                           3.0, 4.0, 11.0,
                           12.0, 14.0, 19.0]), 
            (3, 3), 
            true
        )?;

        let arr2 = Array::new_array_2d(
            Box::new([1.0, 22.0, 5.0,
                           3.0, 4.0, 11.0,
                           12.0, 14.0, 19.0]), 
            (3, 3), 
            true
        )?;

        arr.inv()?;
        arr.mmult(&arr2)?;

        println!("{:?}", arr);

        let real_inv = Array::new_array_2d(
            Box::new([1.0, 0.0, 0.0,
                           0.0, 1.0, 0.0,
                           0.0, 0.0, 1.0]), 
            (3, 3), 
            true
        )?;

        let tol: f64 = 1e-10;

        let diff: f64 = Array::compute_dist(&arr, &real_inv)?;

        assert!(diff < tol);

        Ok(())
    }
}