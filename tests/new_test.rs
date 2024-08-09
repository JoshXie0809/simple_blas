#[cfg(test)]
pub mod tests {
    use simple_blas::Array;

    #[test]
    fn new_a_scalar() {
        let result = Array::new_scalar(123);
        assert_eq!(result, Array::Scalar(123));
    }

    #[test]
    fn new_a_array_1d() {
        let arr1 = Box::new([1, 2, 3, 4, 5]);
        let result = Array::new_array_1d(arr1);
        
        let other_arr = 
            Array::Array1D { arr: Box::new([1, 2, 3, 4, 5]) };
        assert_eq!(result, other_arr);

        let arr1 = Box::new([1.023, 1.1123]);
        let result = Array::new_array_1d(arr1);
        let other_arr = 
            Array::Array1D { arr: Box::new([1.023, 1.1123]) };
        assert_eq!(result, other_arr);
    }

    #[test]
    fn compare_null_and_null() {
        let a: Array<i32> = Array::<i32>::new_null();
        let b: Array<i32> =  Array::<i32>::new_null();
        assert_ne!(a, b);
    }

    #[test]
    fn compare_scalar_and_null() {
        assert_ne!(Array::Null, Array::Scalar(123));
    }

    #[test]
    fn compare_scalar_and_scalar() {
        assert_ne!(Array::Scalar(124), Array::Scalar(123));
    }
}