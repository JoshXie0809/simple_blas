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
}