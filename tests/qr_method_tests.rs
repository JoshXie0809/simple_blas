#[cfg(test)]
pub mod test {
    use simple_blas::array::{Array, ListError};


    #[test]
    fn qr_arr_2d_1() {
        let arr = Array::Scalar(1.0);
        if let Err(error) = arr.qr() {
            assert_eq!(error, ListError::MismatchedTypes);
        };
    }

    #[test]
    fn qr_arr_2d_2() -> Result<(), ListError> {
        let arr = Array::new_array_2d(
            Box::new([
                61.01, 72.02, 
                -94.05, 85.01,
            ]), 
            (2, 2), 
            true
        )?;

        let (mut q, r) = arr.qr()?;

        q.mmult(&r)?;

        let dist = Array::compute_dist(&arr, &q)?;
        assert!(dist < 1e-10);
        
        Ok(())
    }

    #[test]
    fn qr_arr_2d_3() -> Result<(), ListError> {
        let arr = Array::new_array_2d(
            Box::new([
                1.01, 2.02, 3.03, 
                -4.05, 5.01, 6.97
            ]), 
            (2, 3), 
            true
        )?;

        let (mut q, r) = arr.qr()?;

        q.mmult(&r)?;

        let dist = Array::compute_dist(&arr, &q)?;
        println!("{:?}", q);

        assert!(dist < 1e-10);
        
        Ok(())
    }
    
}

