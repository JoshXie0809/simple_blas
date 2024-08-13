#[cfg(test)]
pub mod tests {
    
    use simple_blas::array::Array;
    use simple_blas::array::ListError;

    #[test]
    fn add_method_scalar() -> Result<(), ListError> {
        let mut scalar = Array::new_scalar(1.0);
        let other_scalar = Array::new_scalar(1.0);
        scalar.add(&other_scalar)?;
        assert_eq!(scalar, Array::new_scalar(2.0));
        Ok(())
    }

    #[test]
    fn add_method_array_1d() -> Result<(), ListError> {
        let a = Box::new([1, 2, 3]);
        let b = Box::new([4, 5, 6]);

        let mut arr_1d = Array::new_array_1d(a);
        let other_arr_1d = Array::new_array_1d(b);
        
        arr_1d.add(&other_arr_1d)?;
        assert_eq!(arr_1d, Array::new_array_1d(Box::new([5, 7, 9])) );
        
        let sclar = Array::new_scalar(1);
        
        arr_1d.add(&sclar)?;
        assert_eq!(arr_1d, Array::new_array_1d( Box::new([6, 8, 10]) ));

        let other_arr_1d = 
            Array::new_array_1d(Box::new([1, 2, 3, 4, 5, 6]));
        if let Err(err) = arr_1d.add(&other_arr_1d) {
            assert_eq!(err, ListError::DifferentLength1D)
        };

        Ok(())
    }

    #[test]
    fn add_method_array_2d_1() {
        let mut arr1 = Array::Array2D { 
            arr: Box::new([1, 2, 3, 4]), nr: 2, nc: 2, put_val_by_row: true
        };

        let arr2 = Array::Array2D { 
            arr: Box::new([1, 3, 2, 4]), nr: 2, nc: 2, put_val_by_row: false
        };

        let _ = arr1.add(&arr2);

        assert_eq!(
            arr1,
            Array::Array2D { 
                arr: Box::new([2, 4, 6, 8]), nr: 2, nc: 2, put_val_by_row: true
            }   
        )
    }

    #[test]
    fn add_method_array_2d_2() {
        let mut arr1 = Array::Array2D { 
            arr: Box::new([1, 2, 3, 4]), nr: 2, nc: 2, put_val_by_row: false
        };

        let arr2 = Array::Array2D { 
            arr: Box::new([1, 3, 2, 4]), nr: 2, nc: 2, put_val_by_row: true
        };

        let _ = arr1.add(&arr2);

        assert_eq!(
            arr1,
            Array::Array2D { 
                arr: Box::new([2, 6, 4, 8]), nr: 2, nc: 2, put_val_by_row: true
            }   
        )
    }

}