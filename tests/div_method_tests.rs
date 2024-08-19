#[cfg(test)]
mod tests {
    use simple_blas::array::{Array, ListError};
    // use simple_blas::array::ListError;

    #[test]
    fn div_method_scalar() -> Result<(), ListError> {
        let mut scalar: Array<f64> = Array::new_scalar(1.0);
        
        scalar.div(2.0)?;
        assert_eq!(scalar, Array::new_scalar(0.5));

        if let Err(error) = scalar.div(0.0) {
            assert_eq!(error, ListError::DivisionByZero);
        };

        Ok(())
    }

    #[test]
    fn div_method_array_1d() -> Result<(), ListError>{

        // arr_1d devide arr_1d
        let mut arr_1d = Array::new_array_1d(
            Box::new([1.1, 1.2, 1.3])
        );

        let other_arr_1d = Array::new_array_1d(
            Box::new([2.2, 2.4, 2.6])
        );

        arr_1d.ele_div(&other_arr_1d)?;
        assert_eq!(arr_1d, Array::Array1D { arr: Box::new([0.5, 0.5, 0.5]) });


        // arr_1d devide scalar
        arr_1d.div(0.5)?;
        assert_eq!(arr_1d, Array::Array1D { arr: Box::new([1.0, 1.0, 1.0]) });

        // test divide by zero
        if let Err(err) = arr_1d.div(0.0) {
            assert_eq!(err, ListError::DivisionByZero)
        }

        Ok(())
    }

    #[test]
    fn div_method_array_2d_1() -> Result<(), ListError>{
        
        let mut arr1 = Array::new_array_2d(
            Box::new([1, 3, 5, 2, 4, 6]), 
            (2, 3), 
            false
        )?; 

        let arr2 = Array::new_array_2d(
            Box::new([2, 3, 4, 5, 6, 7]), 
            (2, 3), 
            true
        )?;
        

        arr1.ele_div(&arr2)?;

        assert_eq!(arr1,
        Array::new_array_2d(
            Box::new([0, 1, 1, 0, 0, 0]), 
            (2, 3), 
            true)?
        );

        Ok(())

    }

}
