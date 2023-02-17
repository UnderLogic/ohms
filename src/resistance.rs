use crate::{assert, helpers};
use core::{cmp, ops};

/// Represents a resistance value, stored as whole milliohms (mΩ).
/// This value can only be positive.
///
/// **Reminder:** `1000 mΩ = 1 Ω, 1000 Ω = 1 kΩ, 1000 kΩ = 1 MΩ`
///
/// This is an immutable type. Any math operators return a new `Resistance` value.
///
/// # Creating a Resistance value
/// You can create a `Resistance` value using the `from_milli_ohms` method, or using one of the
/// extension methods on `u32` and `f32`:
///
/// ```rust
/// use ohms::*;
///
/// let r1 = Resistance::from_milli_ohms(1000); // 1Ω
///
/// // More ergonomic:
/// let r2 = 100.milli_ohms(); // 0.1Ω
/// let r3 = 220u32.ohms(); // 220Ω
/// let r4 = 1.5f32.kilo_ohms(); // 1.5kΩ
/// let r5 = 1.5f32.mega_ohms(); // 1.5MΩ
/// ```
///
/// # Comparing Resistance values
/// You can compare two `Resistance` values using the `==`, `!=`, `<`, `>`, `<=` and `>=` operators.
///
/// ```rust
/// use ohms::*;
///
/// let r1 = 220u32.ohms(); // 220Ω
/// let r2 = 4.7f32.kilo_ohms(); // 4.7kΩ
///
/// if r1 > r2 {
///     println!("{:?} is greater than {:?}", r1, r2);
/// } else {
///     println!("{:?} is less than or equal to {:?}", r1, r2);
/// }
/// ```
///
/// # Combining Resistance values
/// You can use the `+` and `-` operators to add and subtract `Resistance` values from each other.
/// The result is a new `Resistance` value, rounded down to the nearest whole milliohm (mΩ).
///
/// If the result of the operation would overflow or underflow the `u32` value, the operation will panic.
///
/// ```rust
/// use ohms::*;
///
/// let r1 = 220u32.ohms(); // 220Ω
/// let r2 = 4.7f32.kilo_ohms(); // 4.7kΩ
///
/// let r3 = r1 + r2; // 4.92kΩ
/// let r4 = r2 - 2f32.kilo_ohms(); // 2.7kΩ
/// ```
///
/// # Scaling Resistance values
/// You can use the `*` and `/` operators to scale `Resistance` values by a scalar `u32` or `f32` value.
/// The result is a new `Resistance` value, rounded down to the nearest whole milliohm (mΩ).
///
/// If the result of operation would overflow or underflow the `u32` value, the operation will panic.
///
/// If the result of the operation would be infinite or NaN, the operation will panic.
///
/// ```rust
/// use ohms::*;
///
/// let r1 = 220u32.ohms(); // 220Ω
/// let r2 = r1 * 3; // 660Ω
///
/// let r3 = 47u32.kilo_ohms(); // 47kΩ
/// let r4 = r3 / 2.5f32; // 18.8kΩ
/// ```
///
/// # Converting to other denominations
/// You can use the `milli_ohms`, `ohms`, `kilo_ohms` and `mega_ohms` methods to convert a `Resistance`
/// value to a `u32` or `f32` value in the specified denomination.
///
/// ```rust
/// use ohms::*;
///
/// let r1 = 47.5f32.ohms(); // 47.5Ω
///
/// println!("{:.3}kΩ is {:.1}Ω", r1.kilo_ohms(), r1.ohms());
/// ```
///
#[derive(Clone, Copy, Debug)]
pub struct Resistance {
    milliohms: u32,
}

impl Resistance {
    /// Creates a new `Resistance` from a number of whole milliohms (mΩ).
    ///
    /// It is recommended to use the `milli_ohms`, `ohms`, `kilo_ohms` and `mega_ohms` extension
    /// methods on `u32` and `f32` instead of this method for ergonomics.
    #[inline]
    pub const fn from_milli_ohms(milliohms: u32) -> Resistance {
        Resistance { milliohms }
    }

    /// Returns the resistance value in whole milliohms (mΩ).
    #[inline]
    pub const fn milli_ohms(&self) -> u32 {
        self.milliohms
    }

    /// Returns the resistance value in fractional ohms (Ω).
    #[inline]
    pub fn ohms(&self) -> f32 {
        self.milliohms as f32 / 1_000f32
    }

    /// Returns the resistance value in fractional kilohms (kΩ).
    #[inline]
    pub fn kilo_ohms(&self) -> f32 {
        self.milliohms as f32 / 1_000_000f32
    }

    /// Returns the resistance value in fractional megaohms (MΩ).
    #[inline]
    pub fn mega_ohms(&self) -> f32 {
        self.milliohms as f32 / 1_000_000_000f32
    }

    /// Returns whether the resistance value is zero ohms (0Ω).
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.milliohms == 0
    }

    /// Returns a `Resistance` value of zero ohms (0Ω).
    #[inline]
    pub const fn zero() -> Self {
        Resistance::from_milli_ohms(0)
    }
}

// Equality traits
impl PartialEq for Resistance {
    #[inline]
    fn eq(&self, other: &Resistance) -> bool {
        self.milliohms == other.milliohms
    }
}

impl Eq for Resistance {}

// Comparison traits
impl PartialOrd for Resistance {
    #[inline]
    fn partial_cmp(&self, other: &Resistance) -> Option<cmp::Ordering> {
        self.milliohms.partial_cmp(&other.milliohms)
    }
}

impl Ord for Resistance {
    #[inline]
    fn cmp(&self, other: &Resistance) -> cmp::Ordering {
        self.milliohms.cmp(&other.milliohms)
    }
}

// Math operators
impl ops::Add for Resistance {
    type Output = Resistance;

    #[inline]
    fn add(self, other: Resistance) -> Resistance {
        self.milliohms
            .checked_add(other.milliohms)
            .map(Resistance::from_milli_ohms)
            .expect("Overflow when adding resistance values")
    }
}

impl ops::Sub for Resistance {
    type Output = Resistance;

    #[inline]
    fn sub(self, other: Resistance) -> Resistance {
        self.milliohms
            .checked_sub(other.milliohms)
            .map(Resistance::from_milli_ohms)
            .expect("Overflow when subtracting resistance values")
    }
}

impl ops::Mul<u32> for Resistance {
    type Output = Resistance;

    #[inline]
    fn mul(self, other: u32) -> Resistance {
        self.milliohms
            .checked_mul(other)
            .map(Resistance::from_milli_ohms)
            .expect("Overflow when multiplying resistance value")
    }
}

impl ops::Mul<f32> for Resistance {
    type Output = Resistance;

    #[inline]
    fn mul(self, other: f32) -> Resistance {
        let milliohms = match other {
            _ if other.is_infinite() => {
                panic!("Cannot multiply resistance value by infinity")
            }
            _ if other.is_nan() => panic!("Cannot multiply resistance value by NaN"),
            _ if other.is_sign_negative() => {
                panic!("Cannot multiply resistance value by negative value")
            }
            _ if other == 0f32 => Some(0),
            _ => helpers::checked_mul_unsigned_f32(self.milliohms, other),
        };

        Resistance::from_milli_ohms(milliohms.unwrap())
    }
}

impl ops::Div<u32> for Resistance {
    type Output = Resistance;

    #[inline]
    fn div(self, other: u32) -> Resistance {
        if other == 0 {
            panic!("Cannot divide resistance value by zero");
        }
        self.milliohms
            .checked_div(other)
            .map(Resistance::from_milli_ohms)
            .expect("Overflow when dividing resistance value")
    }
}

impl ops::Div<f32> for Resistance {
    type Output = Resistance;

    #[inline]
    fn div(self, other: f32) -> Resistance {
        let milliohms = match other {
            _ if other == 0f32 => panic!("Cannot divide resistance value by zero"),
            _ if other.is_infinite() => {
                panic!("Cannot divide resistance value by infinity")
            }
            _ if other.is_nan() => panic!("Cannot divide resistance value by NaN"),
            _ if other.is_sign_negative() => {
                panic!("Cannot divide resistance value by negative value")
            }
            _ => helpers::checked_div_unsigned_f32(self.milliohms, other),
        };

        Resistance::from_milli_ohms(milliohms.unwrap())
    }
}

/// Extension trait for simple short-hands for creating `Resistance` values from `u32` values.
pub trait ExtU32 {
    /// Creates a new `Resistance` from a number of whole milliohms (mΩ).
    fn milli_ohms(self) -> Resistance;

    /// Creates a new `Resistance` from a number of whole ohms (Ω).
    fn ohms(self) -> Resistance;

    /// Creates a new `Resistance` from a number of whole kilohms (kΩ).
    fn kilo_ohms(self) -> Resistance;

    /// Creates a new `Resistance` from a number of whole megaohms (MΩ).
    fn mega_ohms(self) -> Resistance;
}

impl ExtU32 for u32 {
    #[inline]
    fn milli_ohms(self) -> Resistance {
        Resistance::from_milli_ohms(self)
    }

    #[inline]
    fn ohms(self) -> Resistance {
        let milliohms = self
            .checked_mul(1_000)
            .expect("Overflow when converting ohms to milliohms");
        Resistance::from_milli_ohms(milliohms)
    }

    #[inline]
    fn kilo_ohms(self) -> Resistance {
        let milliohms = self
            .checked_mul(1_000_000)
            .expect("Overflow when converting kilohms to milliohms");
        Resistance::from_milli_ohms(milliohms)
    }

    #[inline]
    fn mega_ohms(self) -> Resistance {
        let milliohms = self
            .checked_mul(1_000_000_000)
            .expect("Overflow when converting megaohms to milliohms");
        Resistance::from_milli_ohms(milliohms)
    }
}

/// Extension trait for simple short-hands for creating `Resistance` values from `f32` values.
pub trait ExtF32 {
    /// Creates a new `Resistance` from a number of fractional milliohms (mΩ).
    ///
    /// The fractional part is rounded down to the nearest whole milliohm (mΩ).
    fn milli_ohms(self) -> Resistance;

    /// Creates a new `Resistance` from a number of fractional ohms (Ω).
    ///
    /// The fractional part is rounded down to the nearest whole milliohm (mΩ).
    fn ohms(self) -> Resistance;

    /// Creates a new `Resistance` from a number of fractional kilohms (kΩ).
    ///
    /// The fractional part is rounded down to the nearest whole milliohm (mΩ).
    fn kilo_ohms(self) -> Resistance;

    /// Creates a new `Resistance` from a number of fractional megaohms (MΩ).
    ///
    /// The fractional part is rounded down to the nearest whole milliohm (mΩ).
    fn mega_ohms(self) -> Resistance;
}

impl ExtF32 for f32 {
    #[inline]
    fn milli_ohms(self) -> Resistance {
        assert::is_positive_value(self);
        Resistance::from_milli_ohms(self as u32)
    }

    #[inline]
    fn ohms(self) -> Resistance {
        assert::is_positive_value(self);
        let milliohms = helpers::checked_mul_unsigned_f32(1_000, self)
            .expect("Overflow when converting ohms to milliohms");
        Resistance::from_milli_ohms(milliohms)
    }

    #[inline]
    fn kilo_ohms(self) -> Resistance {
        assert::is_positive_value(self);
        let milliohms = helpers::checked_mul_unsigned_f32(1_000_000, self)
            .expect("Overflow when converting kilohms to milliohms");
        Resistance::from_milli_ohms(milliohms)
    }

    #[inline]
    fn mega_ohms(self) -> Resistance {
        assert::is_positive_value(self);
        let milliohms = helpers::checked_mul_unsigned_f32(1_000_000_000, self)
            .expect("Overflow when converting megaohms to milliohms");
        Resistance::from_milli_ohms(milliohms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cmp::Ordering;
    use test_case::test_case;

    #[test]
    fn test_from_milli_ohms() {
        let r = Resistance::from_milli_ohms(1_000);
        assert_eq!(r.milli_ohms(), 1_000);
    }

    #[test]
    fn test_milli_ohms() {
        let r = Resistance::from_milli_ohms(1_000);
        assert_eq!(r.milli_ohms(), 1_000);
    }

    #[test]
    fn test_ohms() {
        let r = Resistance::from_milli_ohms(1_000);
        assert_eq!(r.ohms(), 1f32);
    }

    #[test]
    fn test_kilo_ohms() {
        let r = Resistance::from_milli_ohms(1_000_000);
        assert_eq!(r.kilo_ohms(), 1f32);
    }

    #[test]
    fn test_mega_ohms() {
        let r = Resistance::from_milli_ohms(1_000_000_000);
        assert_eq!(r.mega_ohms(), 1f32);
    }

    #[test_case(0, true; "when resistance is zero")]
    #[test_case(1_000, false; "when resistance is not zero")]
    fn test_is_zero(resistance: u32, expected: bool) {
        let r = Resistance::from_milli_ohms(resistance);
        assert_eq!(r.is_zero(), expected);
    }

    #[test]
    fn test_zero() {
        let r = Resistance::zero();
        assert_eq!(r.milli_ohms(), 0);
    }

    #[test_case(0, 0, true; "when both are zero")]
    #[test_case(1_000, 1_000, true; "when lhs equals rhs")]
    #[test_case(1_000, 2_000, false; "when lhs does not equal rhs")]
    fn test_eq(lhs: u32, rhs: u32, expected: bool) {
        let lhs = Resistance::from_milli_ohms(lhs);
        let rhs = Resistance::from_milli_ohms(rhs);
        assert_eq!(lhs == rhs, expected);
    }

    #[test_case(1_000, 1_000, false; "when both are equal")]
    #[test_case(2_000, 1_000, true; "when lhs is greater")]
    #[test_case(1_000, 2_000, false; "when rhs is greater")]
    fn test_gt(lhs: u32, rhs: u32, expected: bool) {
        let lhs = Resistance::from_milli_ohms(lhs);
        let rhs = Resistance::from_milli_ohms(rhs);
        assert_eq!(lhs > rhs, expected);
    }

    #[test_case(1_000, 1_000, true; "when both are equal")]
    #[test_case(2_000, 1_000, true; "when lhs is greater")]
    #[test_case(1_000, 2_000, false; "when rhs is greater")]
    fn test_gte(lhs: u32, rhs: u32, expected: bool) {
        let lhs = Resistance::from_milli_ohms(lhs);
        let rhs = Resistance::from_milli_ohms(rhs);
        assert_eq!(lhs >= rhs, expected);
    }

    #[test_case(1_000, 1_000, false; "when both are equal")]
    #[test_case(1_000, 2_000, true; "when lhs is lesser")]
    #[test_case(2_000, 1_000, false; "when rhs is lesser")]
    fn test_lt(lhs: u32, rhs: u32, expected: bool) {
        let lhs = Resistance::from_milli_ohms(lhs);
        let rhs = Resistance::from_milli_ohms(rhs);
        assert_eq!(lhs < rhs, expected);
    }

    #[test_case(1_000, 1_000, true; "when both are equal")]
    #[test_case(1_000, 2_000, true; "when lhs is lesser")]
    #[test_case(2_000, 1_000, false; "when rhs is lesser")]
    fn test_lte(lhs: u32, rhs: u32, expected: bool) {
        let lhs = Resistance::from_milli_ohms(lhs);
        let rhs = Resistance::from_milli_ohms(rhs);
        assert_eq!(lhs <= rhs, expected);
    }

    #[test_case(1_000, 2_000, Ordering::Less; "when lhs is lesser")]
    #[test_case(1_000, 1_000, Ordering::Equal; "when both are equal")]
    #[test_case(2_000, 1_000, Ordering::Greater; "when lhs is greater")]
    fn test_cmp(lhs: u32, rhs: u32, expected: Ordering) {
        let lhs = Resistance::from_milli_ohms(lhs);
        let rhs = Resistance::from_milli_ohms(rhs);
        assert_eq!(lhs.cmp(&rhs), expected);
    }

    #[test]
    fn test_add_operator() {
        let lhs = Resistance::from_milli_ohms(1_000);
        let rhs = Resistance::from_milli_ohms(2_000);
        let expected = Resistance::from_milli_ohms(3_000);
        assert_eq!(lhs + rhs, expected);
    }

    #[test]
    #[should_panic(expected = "Overflow when adding resistance value")]
    fn test_add_operator_overflow() {
        let _r = Resistance::from_milli_ohms(u32::MAX) + Resistance::from_milli_ohms(1_000);
    }

    #[test]
    fn test_sub_operator() {
        let lhs = Resistance::from_milli_ohms(3_000);
        let rhs = Resistance::from_milli_ohms(1_000);
        let expected = Resistance::from_milli_ohms(2_000);
        assert_eq!(lhs - rhs, expected);
    }

    #[test]
    #[should_panic(expected = "Overflow when subtracting resistance value")]
    fn test_sub_operator_underflow() {
        let _r = Resistance::from_milli_ohms(0) - Resistance::from_milli_ohms(1_000);
    }

    #[test]
    fn test_mul_operator_u32() {
        let r = Resistance::from_milli_ohms(1_000);
        let expected = Resistance::from_milli_ohms(10_000);
        assert_eq!(r * 10u32, expected);
    }

    #[test]
    #[should_panic(expected = "Overflow when multiplying resistance value")]
    fn test_mul_operator_u32_overflow() {
        let _r = Resistance::from_milli_ohms(u32::MAX) * 2u32;
    }

    #[test]
    fn test_mul_operator_f32() {
        let r = Resistance::from_milli_ohms(1_000);
        let expected = Resistance::from_milli_ohms(2_500);
        assert_eq!(r * 2.5f32, expected);
    }

    #[test]
    #[should_panic(expected = "Cannot multiply resistance value by negative value")]
    fn test_mul_operator_f32_negative() {
        let _r = Resistance::from_milli_ohms(1_000) * -1f32;
    }

    #[test_case(f32::INFINITY; "positive infinity")]
    #[test_case(f32::NEG_INFINITY; "negative infinity")]
    #[should_panic(expected = "Cannot multiply resistance value by infinity")]
    fn test_mul_operator_f32_infinity(infinity: f32) {
        let _r = Resistance::from_milli_ohms(1_000) * infinity;
    }

    #[test_case(f32::NAN; "NaN")]
    #[should_panic(expected = "Cannot multiply resistance value by NaN")]
    fn test_mul_operator_f32_nan(nan: f32) {
        let _r = Resistance::from_milli_ohms(1_000) * nan;
    }

    #[test]
    fn test_div_operator_u32() {
        let r = Resistance::from_milli_ohms(10_000);
        let expected = Resistance::from_milli_ohms(1_000);
        assert_eq!(r / 10u32, expected);
    }

    #[test]
    #[should_panic(expected = "Cannot divide resistance value by zero")]
    fn test_div_operator_u32_by_zero() {
        let _r = Resistance::from_milli_ohms(1_000) / 0;
    }

    #[test]
    fn test_div_operator_f32() {
        let r = Resistance::from_milli_ohms(2_500);
        let expected = Resistance::from_milli_ohms(1_000);
        assert_eq!(r / 2.5f32, expected);
    }

    #[test]
    #[should_panic(expected = "Cannot divide resistance value by zero")]
    fn test_div_operator_f32_by_zero() {
        let _r = Resistance::from_milli_ohms(1_000) / 0f32;
    }

    #[test]
    #[should_panic(expected = "Cannot divide resistance value by negative value")]
    fn test_div_operator_f32_negative() {
        let _r = Resistance::from_milli_ohms(1_000) / -1f32;
    }

    #[test_case(f32::INFINITY; "positive infinity")]
    #[test_case(f32::NEG_INFINITY; "negative infinity")]
    #[should_panic(expected = "Cannot divide resistance value by infinity")]
    fn test_div_operator_f32_infinity(infinity: f32) {
        let _r = Resistance::from_milli_ohms(1_000) / infinity;
    }

    #[test_case(f32::NAN; "NaN")]
    #[should_panic(expected = "Cannot divide resistance value by NaN")]
    fn test_div_operator_f32_nan(nan: f32) {
        let _r = Resistance::from_milli_ohms(1_000) / nan;
    }

    #[test]
    fn test_milli_ohms_u32() {
        let r = 1_000u32.milli_ohms();
        assert_eq!(r.milli_ohms(), 1_000u32);
    }

    #[test]
    fn test_ohms_u32() {
        let r = 1_000u32.ohms();
        assert_eq!(r.ohms(), 1_000f32);
    }

    #[test]
    fn test_kilo_ohms_u32() {
        let r = 1_000u32.kilo_ohms();
        assert_eq!(r.kilo_ohms(), 1_000f32);
    }

    #[test]
    fn test_mega_ohms_u32() {
        let r = 2u32.mega_ohms();
        assert_eq!(r.mega_ohms(), 2f32);
    }

    #[test]
    #[should_panic(expected = "Overflow when converting megaohms to milliohms")]
    fn test_mega_ohms_u32_overflow() {
        let _r = 10u32.mega_ohms();
    }

    #[test]
    fn test_milli_ohms_f32() {
        let r = 1_000f32.milli_ohms();
        assert_eq!(r.milli_ohms(), 1_000u32);
    }

    #[test]
    #[should_panic(expected = "Value cannot be negative")]
    fn test_milli_ohms_f32_negative() {
        let _r = (-1f32).milli_ohms();
    }

    #[test]
    fn test_ohms_f32() {
        let r = 1_000f32.ohms();
        assert_eq!(r.ohms(), 1_000f32);
    }

    #[test]
    #[should_panic(expected = "Value cannot be negative")]
    fn test_ohms_f32_negative() {
        let _r = (-1f32).ohms();
    }

    #[test]
    fn test_kilo_ohms_f32() {
        let r = 1_000f32.kilo_ohms();
        assert_eq!(r.kilo_ohms(), 1_000f32);
    }

    #[test]
    #[should_panic(expected = "Value cannot be negative")]
    fn test_kilo_ohms_f32_negative() {
        let _r = (-1f32).kilo_ohms();
    }

    #[test]
    fn test_mega_ohms_f32() {
        let r = 2f32.mega_ohms();
        assert_eq!(r.mega_ohms(), 2f32);
    }

    #[test]
    #[should_panic(expected = "Value cannot be negative")]
    fn test_mega_ohms_f32_negative() {
        let _r = (-1f32).mega_ohms();
    }

    #[test]
    #[should_panic(expected = "Overflow when converting megaohms to milliohms")]
    fn test_mega_ohms_f32_overflow() {
        let _r = 10f32.mega_ohms();
    }
}
