#[macro_export]
macro_rules! assert_positive_float {
    ($value:expr) => {
        if $value.is_nan() {
            panic!("Value is NaN");
        }
        if $value.is_infinite() {
            panic!("Value is infinite");
        }
        if $value < 0.0 {
            panic!("Value is negative");
        }
    };
}
