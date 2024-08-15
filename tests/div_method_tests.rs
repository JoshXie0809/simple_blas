#[cfg(test)]
mod tests {
    use simple_blas::array::{Array, ListError};
    // use simple_blas::array::ListError;

    #[test]
    #[should_panic]
    fn div_method_scalar() {
        let mut scalar = Array::new_scalar(1.0);
        let mut other_scalar = Array::new_scalar(0.0);
        
        let _ = other_scalar.div(&scalar);
        assert_eq!(other_scalar, Array::new_scalar(0.0));

        if let Err(error) = scalar.div(&other_scalar) {
            panic!("{:?}", error);
        };
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

        arr_1d.div(&other_arr_1d)?;
        assert_eq!(arr_1d, Array::Array1D { arr: Box::new([0.5, 0.5, 0.5]) });


        // arr_1d devide scalar
        let scalar = Array::Scalar(0.5);
        arr_1d.div(&scalar)?;
        assert_eq!(arr_1d, Array::Array1D { arr: Box::new([1.0, 1.0, 1.0]) });

        // test divide by zero
        let scalar = Array::Scalar(0.0);
        if let Err(err) = arr_1d.div(&scalar) {
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
        

        arr1.div(&arr2)?;

        assert_eq!(arr1,
        Array::new_array_2d(
            Box::new([0, 1, 1, 0, 0, 0]), 
            (2, 3), 
            true)?
        );

        Ok(())


    }

}
