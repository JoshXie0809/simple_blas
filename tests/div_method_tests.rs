#[cfg(test)]
mod tests {
    use simple_blas::Array;
    // use simple_blas::ListError;

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


}
