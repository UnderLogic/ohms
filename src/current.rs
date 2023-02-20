use crate::assert;
use core::{cmp, ops};

/// Represents a current value, stored as whole microamps (μA).
/// This value can only be positive.
///
/// **Reminder:** `1000 μA = 1 mA, 1000 mA = 1 A`
///
/// This is an immutable type. Any math operators return a new `Current` value.
///
/// # Creating a Current value
/// You can create a `Current` value using the `from_micro_amps` method, or using one of the
/// extension methods on `u32` and `f32`:
///
/// ```rust
/// use ohms::prelude::*;
///
/// let c1 = Current::from_micro_amps(1000); // 1mA
///
/// // More ergonomic:
/// let c2 = 100u32.milli_amps(); // 0.1A
/// let c3 = 3.2f32.amps(); // 3.2A
/// ```
///
/// # Comparing Current values
/// You can compare two `Current` values using the `==`, `!=`, `<`, `>`, `<=` and `>=` operators.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let c1 = 220u32.milli_amps(); // 220mA
/// let c2 = 1.2f32.amps(); // 1.2A
///
/// if c1 > c2 {
///     println!("{:?} is greater than {:?}", c1, c2);
/// } else {
///     println!("{:?} is less than or equal to {:?}", c1, c2);
/// }
/// ```
///
/// # Combining Current values
/// You can use the `+` and `-` operators to add and subtract `Current` values from each other.
/// The result is a new `Current` value, rounded down to the nearest whole microamp (μA).
///
/// If the result of the operation would overflow or underflow the `u32` value, the operation will panic.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let c1 = 500u32.amps(); // 0.5A
/// let c2 = 1.1f32.amps(); // 1.1A
///
/// let c3 = c1 + c2; // 1.6A
/// let c4 = c2 - 300u32.milli_amps(); // 0.8A
/// ```
///
/// # Scaling Current values
/// You can use the `*` and `/` operators to scale `Current` values by a scalar `u32` or `f32` value.
/// The result is a new `Current` value, rounded down to the nearest whole microamp (μA).
///
/// If the result of operation would overflow or underflow the `u32` value, the operation will panic.
///
/// If the result of the operation would be infinite or NaN, the operation will panic.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let c1 = 200u32.milli_amps(); // 200mA
/// let c2 = c1 * 3; // 660mA
///
/// let r3 = 1.5f32.amps(); // 1.5A
/// let r4 = r3 / 2.5f32; // 0.6A
/// ```
///
/// # Converting to other denominations
/// You can use the `micro_amps`, `milli_amps`, and `amps`, methods to convert a `Current`
/// value to a `u32` or `f32` value in the specified denomination.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let c1 = 1.2f32.amps(); // 1.2A
///
/// println!("{:.3}A is {:.1}A", c1.amps(), c1.milli_amps());
/// ```
///
#[derive(Clone, Copy, Debug)]
pub struct Current {
    microamps: u32,
}

impl Current {
    /// Creates a new `Current` from a number of whole microamps (μA).
    ///
    /// It is recommended to use the `micro_amps`, `milli_amps`, and `amps`, extension
    /// methods on `u32` and `f32` instead of this method for ergonomics.
    #[inline]
    pub const fn from_micro_amps(microamps: u32) -> Current {
        Current { microamps }
    }

    /// Returns the current value in whole microamps (μA).
    #[inline]
    pub const fn micro_amps(&self) -> u32 {
        self.microamps
    }

    /// Returns the current value in fractional milliamps (A).
    #[inline]
    pub fn milli_amps(&self) -> f32 {
        self.microamps as f32 / 1_000f32
    }

    /// Returns the current value in fractional amps (A).
    #[inline]
    pub fn amps(&self) -> f32 {
        self.microamps as f32 / 1_000_000f32
    }

    /// Returns whether the current value is zero amps (0A).
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.microamps == 0
    }

    /// Returns a `Current` value of zero amps (0A).
    #[inline]
    pub const fn zero() -> Self {
        Current::from_micro_amps(0)
    }
}

// Equality traits
impl PartialEq for Current {
    #[inline]
    fn eq(&self, other: &Current) -> bool {
        self.microamps == other.microamps
    }
}

impl Eq for Current {}

// Comparison traits
impl PartialOrd for Current {
    #[inline]
    fn partial_cmp(&self, other: &Current) -> Option<cmp::Ordering> {
        self.microamps.partial_cmp(&other.microamps)
    }
}

impl Ord for Current {
    #[inline]
    fn cmp(&self, other: &Current) -> cmp::Ordering {
        self.microamps.cmp(&other.microamps)
    }
}

// Math operators
impl ops::Add for Current {
    type Output = Current;

    #[inline]
    fn add(self, other: Current) -> Current {
        self.microamps
            .checked_add(other.microamps)
            .map(Current::from_micro_amps)
            .expect("Overflow when adding current values")
    }
}

impl ops::Sub for Current {
    type Output = Current;

    #[inline]
    fn sub(self, other: Current) -> Current {
        self.microamps
            .checked_sub(other.microamps)
            .map(Current::from_micro_amps)
            .expect("Overflow when subtracting current values")
    }
}

impl ops::Mul<u32> for Current {
    type Output = Current;

    #[inline]
    fn mul(self, other: u32) -> Current {
        self.microamps
            .checked_mul(other)
            .map(Current::from_micro_amps)
            .expect("Overflow when multiplying current value")
    }
}

impl ops::Mul<f32> for Current {
    type Output = Current;

    #[inline]
    fn mul(self, other: f32) -> Current {
        let microamps = match other {
            _ if other.is_infinite() => {
                panic!("Cannot multiply current value by infinity")
            }
            _ if other.is_nan() => panic!("Cannot multiply current value by NaN"),
            _ if other.is_sign_negative() => {
                panic!("Cannot multiply current value by negative value")
            }
            _ if other == 0f32 => Some(0),
            _ => helpers::checked_mul_unsigned_f32(self.microamps, other),
        };

        Current::from_micro_amps(microamps.unwrap())
    }
}

impl ops::Div<u32> for Current {
    type Output = Current;

    #[inline]
    fn div(self, other: u32) -> Current {
        if other == 0 {
            panic!("Cannot divide current value by zero");
        }
        self.microamps
            .checked_div(other)
            .map(Current::from_micro_amps)
            .expect("Overflow when dividing current value")
    }
}

impl ops::Div<f32> for Current {
    type Output = Current;

    #[inline]
    fn div(self, other: f32) -> Current {
        let microamps = match other {
            _ if other == 0f32 => panic!("Cannot divide current value by zero"),
            _ if other.is_infinite() => {
                panic!("Cannot divide current value by infinity")
            }
            _ if other.is_nan() => panic!("Cannot divide current value by NaN"),
            _ if other.is_sign_negative() => {
                panic!("Cannot divide current value by negative value")
            }
            _ => helpers::checked_div_unsigned_f32(self.microamps, other),
        };

        Current::from_micro_amps(microamps.unwrap())
    }
}

/// Extension trait for simple short-hands for creating `Current` values from `u32` values.
pub trait ExtU32 {
    /// Creates a new `Current` from a number of whole microamps (μA).
    fn micro_amps(self) -> Current;

    /// Creates a new `Current` from a number of whole milliamps (mA).
    fn milli_amps(self) -> Current;

    /// Creates a new `Current` from a number of whole amps (A).
    fn amps(self) -> Current;
}

impl ExtU32 for u32 {
    #[inline]
    fn micro_amps(self) -> Current {
        Current::from_micro_amps(self)
    }

    #[inline]
    fn milli_amps(self) -> Current {
        let milliamps = self
            .checked_mul(1_000)
            .expect("Overflow when converting milliamps to microamps");
        Current::from_micro_amps(milliamps)
    }

    #[inline]
    fn amps(self) -> Current {
        let milliamps = self
            .checked_mul(1_000_000)
            .expect("Overflow when converting amps to microamps");
        Current::from_micro_amps(milliamps)
    }
}

/// Extension trait for simple short-hands for creating `Current` values from `f32` values.
pub trait ExtF32 {
    /// Creates a new `Current` from a number of fractional microamps (μA).
    ///
    /// The fractional part is rounded down to the nearest whole microamp (μA).
    fn micro_amps(self) -> Current;

    /// Creates a new `Current` from a number of fractional milliamps (mA).
    ///
    /// The fractional part is rounded down to the nearest whole microamp (μA).
    fn milli_amps(self) -> Current;

    /// Creates a new `Current` from a number of fractional amps (A).
    ///
    /// The fractional part is rounded down to the nearest whole microamp (μA).
    fn amps(self) -> Current;
}

impl ExtF32 for f32 {
    #[inline]
    fn micro_amps(self) -> Current {
        assert::is_positive_value(self);
        Current::from_micro_amps(self as u32)
    }

    #[inline]
    fn milli_amps(self) -> Current {
        assert::is_positive_value(self);
        let milliamps = helpers::checked_mul_unsigned_f32(1_000, self)
            .expect("Overflow when converting milliamps to microamps");
        Current::from_micro_amps(milliamps)
    }

    #[inline]
    fn amps(self) -> Current {
        assert::is_positive_value(self);
        let milliamps = helpers::checked_mul_unsigned_f32(1_000_000, self)
            .expect("Overflow when converting amps to microamps");
        Current::from_micro_amps(milliamps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cmp::Ordering;
    use test_case::test_case;

    #[test]
    fn test_from_micro_amps() {
        let c = Current::from_micro_amps(1_000);
        assert_eq!(c.micro_amps(), 1_000);
    }

    #[test]
    fn test_micro_amps() {
        let c = Current::from_micro_amps(1_000);
        assert_eq!(c.micro_amps(), 1_000);
    }

    #[test]
    fn test_milli_amps() {
        let c = Current::from_micro_amps(1_000);
        assert_eq!(c.milli_amps(), 1f32);
    }

    #[test]
    fn test_amps() {
        let c = Current::from_micro_amps(1_000_000);
        assert_eq!(c.amps(), 1f32);
    }

    #[test_case(0, true; "when current is zero")]
    #[test_case(1_000, false; "when current is not zero")]
    fn test_is_zero(current: u32, expected: bool) {
        let c = Current::from_micro_amps(current);
        assert_eq!(c.is_zero(), expected);
    }

    #[test]
    fn test_zero() {
        let c = Current::zero();
        assert_eq!(c.micro_amps(), 0);
    }

    #[test_case(0, 0, true; "when both are zero")]
    #[test_case(1_000, 1_000, true; "when lhs equals rhs")]
    #[test_case(1_000, 2_000, false; "when lhs does not equal rhs")]
    fn test_eq(lhs: u32, rhs: u32, expected: bool) {
        let lhs = Current::from_micro_amps(lhs);
        let rhs = Current::from_micro_amps(rhs);
        assert_eq!(lhs == rhs, expected);
    }

    #[test_case(1_000, 1_000, false; "when both are equal")]
    #[test_case(2_000, 1_000, true; "when lhs is greater")]
    #[test_case(1_000, 2_000, false; "when rhs is greater")]
    fn test_gt(lhs: u32, rhs: u32, expected: bool) {
        let lhs = Current::from_micro_amps(lhs);
        let rhs = Current::from_micro_amps(rhs);
        assert_eq!(lhs > rhs, expected);
    }

    #[test_case(1_000, 1_000, true; "when both are equal")]
    #[test_case(2_000, 1_000, true; "when lhs is greater")]
    #[test_case(1_000, 2_000, false; "when rhs is greater")]
    fn test_gte(lhs: u32, rhs: u32, expected: bool) {
        let lhs = Current::from_micro_amps(lhs);
        let rhs = Current::from_micro_amps(rhs);
        assert_eq!(lhs >= rhs, expected);
    }

    #[test_case(1_000, 1_000, false; "when both are equal")]
    #[test_case(1_000, 2_000, true; "when lhs is lesser")]
    #[test_case(2_000, 1_000, false; "when rhs is lesser")]
    fn test_lt(lhs: u32, rhs: u32, expected: bool) {
        let lhs = Current::from_micro_amps(lhs);
        let rhs = Current::from_micro_amps(rhs);
        assert_eq!(lhs < rhs, expected);
    }

    #[test_case(1_000, 1_000, true; "when both are equal")]
    #[test_case(1_000, 2_000, true; "when lhs is lesser")]
    #[test_case(2_000, 1_000, false; "when rhs is lesser")]
    fn test_lte(lhs: u32, rhs: u32, expected: bool) {
        let lhs = Current::from_micro_amps(lhs);
        let rhs = Current::from_micro_amps(rhs);
        assert_eq!(lhs <= rhs, expected);
    }

    #[test_case(1_000, 2_000, Ordering::Less; "when lhs is lesser")]
    #[test_case(1_000, 1_000, Ordering::Equal; "when both are equal")]
    #[test_case(2_000, 1_000, Ordering::Greater; "when lhs is greater")]
    fn test_cmp(lhs: u32, rhs: u32, expected: Ordering) {
        let lhs = Current::from_micro_amps(lhs);
        let rhs = Current::from_micro_amps(rhs);
        assert_eq!(lhs.cmp(&rhs), expected);
    }

    #[test]
    fn test_add_operator() {
        let lhs = Current::from_micro_amps(1_000);
        let rhs = Current::from_micro_amps(2_000);
        let expected = Current::from_micro_amps(3_000);
        assert_eq!(lhs + rhs, expected);
    }

    #[test]
    #[should_panic(expected = "Overflow when adding current value")]
    fn test_add_operator_overflow() {
        let _c = Current::from_micro_amps(u32::MAX) + Current::from_micro_amps(1_000);
    }

    #[test]
    fn test_sub_operator() {
        let lhs = Current::from_micro_amps(3_000);
        let rhs = Current::from_micro_amps(1_000);
        let expected = Current::from_micro_amps(2_000);
        assert_eq!(lhs - rhs, expected);
    }

    #[test]
    #[should_panic(expected = "Overflow when subtracting current value")]
    fn test_sub_operator_underflow() {
        let _c = Current::from_micro_amps(0) - Current::from_micro_amps(1_000);
    }

    #[test]
    fn test_mul_operator_u32() {
        let c = Current::from_micro_amps(1_000);
        let expected = Current::from_micro_amps(10_000);
        assert_eq!(c * 10u32, expected);
    }

    #[test]
    #[should_panic(expected = "Overflow when multiplying current value")]
    fn test_mul_operator_u32_overflow() {
        let _c = Current::from_micro_amps(u32::MAX) * 2u32;
    }

    #[test]
    fn test_mul_operator_f32() {
        let c = Current::from_micro_amps(1_000);
        let expected = Current::from_micro_amps(2_500);
        assert_eq!(c * 2.5f32, expected);
    }

    #[test]
    #[should_panic(expected = "Cannot multiply current value by negative value")]
    fn test_mul_operator_f32_negative() {
        let _c = Current::from_micro_amps(1_000) * -1f32;
    }

    #[test_case(f32::INFINITY; "positive infinity")]
    #[test_case(f32::NEG_INFINITY; "negative infinity")]
    #[should_panic(expected = "Cannot multiply current value by infinity")]
    fn test_mul_operator_f32_infinity(infinity: f32) {
        let _c = Current::from_micro_amps(1_000) * infinity;
    }

    #[test_case(f32::NAN; "NaN")]
    #[should_panic(expected = "Cannot multiply current value by NaN")]
    fn test_mul_operator_f32_nan(nan: f32) {
        let _c = Current::from_micro_amps(1_000) * nan;
    }

    #[test]
    fn test_div_operator_u32() {
        let c = Current::from_micro_amps(10_000);
        let expected = Current::from_micro_amps(1_000);
        assert_eq!(c / 10u32, expected);
    }

    #[test]
    #[should_panic(expected = "Cannot divide current value by zero")]
    fn test_div_operator_u32_by_zero() {
        let _c = Current::from_micro_amps(1_000) / 0;
    }

    #[test]
    fn test_div_operator_f32() {
        let c = Current::from_micro_amps(2_500);
        let expected = Current::from_micro_amps(1_000);
        assert_eq!(c / 2.5f32, expected);
    }

    #[test]
    #[should_panic(expected = "Cannot divide current value by zero")]
    fn test_div_operator_f32_by_zero() {
        let _c = Current::from_micro_amps(1_000) / 0f32;
    }

    #[test]
    #[should_panic(expected = "Cannot divide current value by negative value")]
    fn test_div_operator_f32_negative() {
        let _c = Current::from_micro_amps(1_000) / -1f32;
    }

    #[test_case(f32::INFINITY; "positive infinity")]
    #[test_case(f32::NEG_INFINITY; "negative infinity")]
    #[should_panic(expected = "Cannot divide current value by infinity")]
    fn test_div_operator_f32_infinity(infinity: f32) {
        let _c = Current::from_micro_amps(1_000) / infinity;
    }

    #[test_case(f32::NAN; "NaN")]
    #[should_panic(expected = "Cannot divide current value by NaN")]
    fn test_div_operator_f32_nan(nan: f32) {
        let _c = Current::from_micro_amps(1_000) / nan;
    }

    #[test]
    fn test_micro_amps_u32() {
        let c = 1_000u32.micro_amps();
        assert_eq!(c.micro_amps(), 1_000u32);
    }

    #[test]
    fn test_milli_amps_u32() {
        let c = 1_000u32.milli_amps();
        assert_eq!(c.milli_amps(), 1_000f32);
    }

    #[test]
    fn test_amps_u32() {
        let c = 1_000u32.amps();
        assert_eq!(c.amps(), 1_000f32);
    }

    #[test]
    fn test_micro_amps_f32() {
        let c = 1_000f32.micro_amps();
        assert_eq!(c.micro_amps(), 1_000u32);
    }

    #[test]
    #[should_panic(expected = "Value cannot be negative")]
    fn test_micro_amps_f32_negative() {
        let _c = (-1f32).micro_amps();
    }

    #[test]
    fn test_milli_amps_f32() {
        let c = 1_000f32.milli_amps();
        assert_eq!(c.milli_amps(), 1_000f32);
    }

    #[test]
    #[should_panic(expected = "Value cannot be negative")]
    fn test_milli_amps_f32_negative() {
        let _c = (-1f32).milli_amps();
    }

    #[test]
    fn test_amps_f32() {
        let c = 1_000f32.amps();
        assert_eq!(c.amps(), 1_000f32);
    }

    #[test]
    #[should_panic(expected = "Value cannot be negative")]
    fn test_amps_f32_negative() {
        let _c = (-1f32).amps();
    }
}
