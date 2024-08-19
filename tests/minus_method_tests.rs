#[cfg(test)]
pub mod tests {
    
    use simple_blas::array::Array;
    use simple_blas::array::ListError;

    #[test]
    fn minus_method_scalar() -> Result<(), ListError> {
        let mut scalar = Array::new_scalar(1.0);
        scalar.minus(1.0)?;
        assert_eq!(scalar, Array::new_scalar(0.0));
        Ok(())
    }

    #[test]
    fn minus_method_array_1d() -> Result<(), ListError> {
        let a: Box<[i32; 3]> = Box::new([1, 2, 3]);
        let mut arr_1d: Array<i32> = Array::new_array_1d(a);
        let sclar = -1;

        arr_1d.minus(sclar)?;
        assert_eq!(arr_1d, Array::new_array_1d( Box::new([2, 3, 4]) ));

        let b: Box<[i32; 3]> = Box::new([-4, -5, -6]);
        let other_arr_1d: Array<i32> = Array::new_array_1d(b);

        // if add other vector use madd

        arr_1d.mminus(&other_arr_1d)?;
        assert_eq!(arr_1d, Array::new_array_1d(Box::new([6, 8, 10])) );

        let other_arr_1d: Array<i32> = 
            Array::new_array_1d(Box::new([1, 2, 3, 4, 5, 6]));
        if let Err(err) = arr_1d.mminus(&other_arr_1d) {
            assert_eq!(err, ListError::DifferentLength1D)
        };

        Ok(())
    }

    #[test]
    fn minus_method_array_2d() -> Result<(), ListError>{
        
        let mut arr1 = Array::new_array_2d(
            Box::new([1, 2, 3, 4, 5, 6]), 
            (2, 3), 
            false
        )?;

        let arr2 = Array::new_array_2d(
            Box::new([1, 2, 3, 4, 5, 6]), 
            (2, 3), 
            true
        )?;

        arr1.mminus(&arr2)?;

        assert_eq!(arr1,
        Array::new_array_2d(
            Box::new([0, 1, 2, -2, -1, 0]), 
            (2, 3), 
            true)?
        );

        Ok(())
    }

}