#[cfg(test)]
pub mod tests {
    
    use simple_blas::array::Array;
    use simple_blas::array::ListError;

    #[test]
    fn add_method_scalar() -> Result<(), ListError> {
        let mut scalar = Array::new_scalar(1.0);
        scalar.add(1.0)?;
        assert_eq!(scalar, Array::new_scalar(2.0));
        Ok(())
    }

    #[test]
    fn add_method_array_1d() -> Result<(), ListError> {
        let a: Box<[i32; 3]> = Box::new([1, 2, 3]);
        let mut arr_1d = Array::new_array_1d(a);
        let sclar = 1;

        arr_1d.add(sclar)?;
        assert_eq!(arr_1d, Array::new_array_1d( Box::new([2, 3, 4]) ));

        let b: Box<[i32; 3]> = Box::new([4, 5, 6]);
        let other_arr_1d = Array::new_array_1d(b);

        // if add other vector use madd

        arr_1d.madd(&other_arr_1d)?;
        assert_eq!(arr_1d, Array::new_array_1d(Box::new([6, 8, 10])) );

        let other_arr_1d: Array<i32> = 
            Array::new_array_1d(Box::new([1, 2, 3, 4, 5, 6]));
        if let Err(err) = arr_1d.madd(&other_arr_1d) {
            assert_eq!(err, ListError::DifferentLength1D)
        };

        Ok(())
    }

    #[test]
    fn add_method_array_2d_1() {
        let mut arr1 = Array::Array2D { 
            arr: Box::new([1, 2, 3, 4]), nr: 2, nc: 2, put_val_by_row: true
        };

        let _ = arr1.add(1);

        assert_eq!(
            arr1,
            Array::Array2D { 
                arr: Box::new([2, 4, 3, 5]), nr: 2, nc: 2, put_val_by_row: false
            }   
        )
    }

    // #[test]
    // fn add_method_array_2d_2() {
    //     let mut arr1 = Array::Array2D { 
    //         arr: Box::new([1, 2, 3, 4, 5, 6]), nr: 2, nc: 3, put_val_by_row: true
    //     };

    //     let arr2 = Array::Array2D { 
    //         arr: Box::new([1, 4, 2, 5, 3, 6]), nr: 2, nc: 3, put_val_by_row: false
    //     };

    //     let _ = arr1.add(&arr2);

    //     assert_eq!(
    //         arr1,
    //         Array::Array2D { 
    //             arr: Box::new([2, 4, 6, 8, 10, 12]), nr: 2, nc: 3, put_val_by_row: true
    //         }   
    //     )
    // }

    // #[test]
    // fn add_method_array_2d_3() {
    //     let mut arr1 = Array::Array2D { 
    //         arr: Box::new([1, 2, 3, 4, 5, 6]), nr: 3, nc: 2, put_val_by_row: true
    //     };

    //     let arr2 = Array::Array2D { 
    //         arr: Box::new([1, 2, 3, 4, 5, 6]), nr: 3, nc: 2, put_val_by_row: false
    //     };

    //     let _ = arr1.add(&arr2);

    //     assert_eq!(
    //         arr1,
    //         Array::Array2D { 
    //             arr: Box::new([2, 6, 5, 9, 8, 12]), nr: 3, nc: 2, put_val_by_row: true
    //         }   
    //     )
    // }
}