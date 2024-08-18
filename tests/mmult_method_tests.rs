// mat_mult

#[cfg(test)]
pub mod tests {
    use simple_blas::array::{Array, ListError};

    #[test]
    fn mmult_method_2d_array_1() -> Result<(), ListError> {
        let arr1 = 
            Array::new_array_2d(
                Box::new([1, 0, 0, 1]), (2, 2), true)?;

        let mut arr2 = 
            Array::new_array_2d(
                Box::new([1, 2, 3, 4]), (2, 2), true)?;
        
        arr2.mmult(&arr1)?;

        assert_eq!(arr2, Array::new_array_2d(
            Box::new([1, 2, 3, 4]), (2, 2), true)?
        );

        Ok(())

    }

    #[test]
    fn mmult_method_2d_array_2() -> Result<(), ListError> {

        let mut arr1 = 
            Array::new_array_2d(
                Box::new([1, 2, 3, 4, 5, 6, 7, 8]), (4, 2), true)?;

        let arr2 = 
            Array::new_array_2d(
                Box::new([1, 2, 3, 4, 5, 6]), (2, 3), false)?;
        
        arr1.mmult(&arr2)?;
        
        assert_eq!(arr1, 
            Array::new_array_2d(Box::new([5, 11, 17, 23, 11, 25, 39, 53, 17, 39, 61, 83]), (4, 3), false)?
        );

        assert_ne!(arr1, 
            Array::new_array_2d(Box::new([5, 11, 17, 23, 11, 25, 349, 53, 17, 39, 61, 83]), (4, 3), false)?
        );

        Ok(())
    }


    #[test]
    fn mmult_method_2d_array_3() -> Result<(), ListError> {

        let mut arr1 = 
            Array::new_array_2d(
                Box::new([1, 2, 3, 4]), (2, 2), true)?;

        let arr2 = 
            Array::new_array_2d(
                Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9]), (3, 3), false)?;
        
        // 2x2 cannot mat_mult to 3x3
        if let Err(err) = arr1.mmult(&arr2) {
            assert_eq!(err, ListError::MatrixMultMismatchedDim);
        };
        

        Ok(())
    }

    // speed up version of mat_mult
    #[test]
    fn mmult_speed_up_method_2d_array_1() -> Result<(), ListError> {
        
        let arr1 = 
            Array::new_array_2d(
                Box::new([1, 0, 0, 1]), (2, 2), true)?;

        let mut arr2 = 
            Array::new_array_2d(
                Box::new([1, 2, 3, 4]), (2, 2), true)?;
        
        arr2.mmult_speed(&arr1)?;

        assert_eq!(arr2, Array::new_array_2d(
            Box::new([1, 2, 3, 4]), (2, 2), true)?
        );

        Ok(())

    }
}