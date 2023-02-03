/// Asserts that a `f32` value is not infinite.
pub fn is_not_infinite(value: f32) {
    if value.is_infinite() {
        panic!("Value cannot be infinite")
    }
}

/// Asserts that a `f32` value is not NaN.
pub fn is_not_nan(value: f32) {
    if value.is_nan() {
        panic!("Value cannot be NaN")
    }
}

/// Asserts that a `f32` value is not negative.
pub fn is_positive(value: f32) {
    if value.is_sign_negative() {
        panic!("Value cannot be negative")
    }
}

/// Asserts that a `f32` value is not infinite, NaN, or negative.
pub fn is_positive_value(value: f32) {
    is_not_infinite(value);
    is_not_nan(value);
    is_positive(value);
}
