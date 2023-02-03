/// Multiplies a u32 by a f32, returning None if the result is not a valid u32.
pub fn checked_mul_f32(value: u32, scale: f32) -> Option<u32> {
    convert_f32_to_u32(value as f32 * scale)
}

/// Divides a u32 by a f32, returning None if the result is not a valid u32.
pub fn checked_div_f32(value: u32, scale: f32) -> Option<u32> {
    convert_f32_to_u32(value as f32 / scale)
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
