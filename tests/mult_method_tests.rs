#[cfg(test)]
pub mod tests {
    
    use simple_blas::array::Array;
    use simple_blas::array::ListError;

    #[test]
    fn mult_method_scalar() -> Result<(), ListError> {
        let mut scalar: Array<f64> = Array::new_scalar(3.0178);
        scalar.mult(3.0178)?;
        assert_eq!(scalar, Array::new_scalar(3.0178 * 3.0178));
        Ok(())
    }

    #[test]
    fn mult_method_array_1d() -> Result<(), ListError> {

        // mult a scalar
        let a: Box<[i32; 3]> = Box::new([1, 2, 3]);
        let mut arr_1d = Array::new_array_1d(a);
        arr_1d.mult(20)?;
        assert_eq!(arr_1d, Array::new_array_1d( Box::new([20, 40, 60]) ));

        // ele_mult
        let other_arr_1d = 
            Array::new_array_1d(Box::new([1, 2, 3]));
        arr_1d.ele_mult(&other_arr_1d)?;
        assert_eq!(arr_1d, Array::new_array_1d( Box::new([20, 80, 180]) ));

        // different length
        let other_arr_1d = 
            Array::new_array_1d(Box::new([1, 2, 3, 4, 5, 6]));
        
        if let Err(err) = arr_1d.ele_mult(&other_arr_1d) {
            assert_eq!(err, ListError::DifferentLength1D)
        };

        Ok(())
    }

    #[test]
    fn mult_method_array_2d_1() -> Result<(), ListError>{
        
        let mut arr1 = Array::new_array_2d(
            Box::new([1, 2, 3, 4, 5, 6]), 
            (2, 3), 
            false
        )?;

        arr1.mult(-1)?;
        assert_eq!(arr1,
            Array::new_array_2d(
                Box::new([-1, -2, -3, -4, -5, -6]), 
                (2, 3), 
                false)?
        );
        
        arr1.mult(-1)?;
        let arr2 = Array::new_array_2d(
            Box::new([1, 2, 3, 4, 5, 6]), 
            (2, 3), 
            true
        )?;
        

        arr1.ele_mult(&arr2)?;

        assert_eq!(arr1,
        Array::new_array_2d(
            Box::new([1, 8, 6, 20, 15, 36]), 
            (2, 3), 
            false)?
        );

        Ok(())

    }

}