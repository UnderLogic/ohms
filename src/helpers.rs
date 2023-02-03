/// Multiplies a i32 by a f32, returning None if the result is not a valid i32.
pub fn checked_mul_signed_f32(value: i32, scale: f32) -> Option<i32> {
    convert_f32_to_i32(value as f32 * scale)
}

/// Multiplies a u32 by a f32, returning None if the result is not a valid u32.
pub fn checked_mul_unsigned_f32(value: u32, scale: f32) -> Option<u32> {
    convert_f32_to_u32(value as f32 * scale)
}

/// Divides a i32 by a f32, returning None if the result is not a valid i32.
pub fn checked_div_signed_f32(value: i32, scale: f32) -> Option<i32> {
    convert_f32_to_i32(value as f32 / scale)
}

/// Divides a u32 by a f32, returning None if the result is not a valid u32.
pub fn checked_div_unsigned_f32(value: u32, scale: f32) -> Option<u32> {
    convert_f32_to_u32(value as f32 / scale)
}

/// Converts a f32 to a i32, returning None if the result is not a valid i32.
pub fn convert_f32_to_i32(value: f32) -> Option<i32> {
    match value {
        _ if value.is_infinite() => None,
        _ if value.is_nan() => None,
        _ if value < i32::MIN as f32 => None,
        _ if value > i32::MAX as f32 => None,
        _ => Some(value as i32),
    }
}

/// Converts a f32 to a u32, returning None if the result is not a valid u32.
pub fn convert_f32_to_u32(value: f32) -> Option<u32> {
    match value {
        _ if value.is_infinite() => None,
        _ if value.is_nan() => None,
        _ if value.is_sign_negative() => None,
        _ if value < u32::MIN as f32 => None,
        _ if value > u32::MAX as f32 => None,
        _ => Some(value as u32),
    }
}
