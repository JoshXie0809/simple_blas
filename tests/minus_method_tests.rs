#[cfg(test)]
pub mod tests {
    
    use simple_blas::Array;
    use simple_blas::ListError;

    #[test]
    fn minus_method_scalar() -> Result<(), ListError> {
        let mut scalar = Array::new_scalar(1.0);
        let other_scalar = Array::new_scalar(1.0);
        scalar.minus(&other_scalar)?;
        assert_eq!(scalar, Array::new_scalar(0.0));
        Ok(())
    }

    #[test]
    fn minus_method_array_1d() -> Result<(), ListError> {
        let a: Box<[i32; 3]> = Box::new([1, 2, 3]);
        let b: Box<[i32; 3]> = Box::new([4, 5, 6]);

        let mut arr_1d: Array<i32> = Array::new_array_1d(a);
        let other_arr_1d: Array<i32> = Array::new_array_1d(b);
        
        arr_1d.minus(&other_arr_1d)?;
        assert_eq!(arr_1d, Array::new_array_1d(Box::new([-3, -3, -3])) );
        
        let sclar: Array<i32> = Array::new_scalar(1);
        
        arr_1d.minus(&sclar)?;
        assert_eq!(arr_1d, Array::new_array_1d( Box::new([-4, -4, -4]) ));

        let other_arr_1d: Array<i32> = 
            Array::new_array_1d(Box::new([1, 2, 3, 4, 5, 6]));
        if let Err(err) = arr_1d.minus(&other_arr_1d) {
            assert_eq!(err, ListError::DifferentLength1D)
        };

        Ok(())
    }

}