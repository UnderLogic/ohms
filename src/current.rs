use crate::assert_positive_float;
use core::{cmp, fmt, ops};

/// Represents a current value, stored as whole microamps (μA) stored in a `u64` value.
/// This value can only be positive.
///
/// **Reminder:** `1000 μA = 1 mA, 1000 mA = 1 A`
///
/// This is an immutable type. Any math operators return a new `Current` value.
///
/// # Creating a Current value
/// You can create a `Current` value using the `from_micro_amps` method, or using one of the
/// extension methods on integer and floating-point types:
///
/// ```rust
/// use ohms::prelude::*;
///
/// let c1 = Current::from_micro_amps(1000); // 1mA
///
/// // More ergonomic:
/// let c2 = 100.milli_amps(); // 0.1A
/// let c3 = 3.2.amps(); // 3.2A
/// ```
///
/// # Comparing Current values
/// You can compare two `Current` values using the `==`, `!=`, `<`, `>`, `<=` and `>=` operators.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let c1 = 220.milli_amps(); // 220mA
/// let c2 = 1.2.amps(); // 1.2A
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
/// If the result of the operation would overflow or underflow, the operation will panic.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let c1 = 500.amps(); // 0.5A
/// let c2 = 1.1.amps(); // 1.1A
///
/// let c3 = c1 + c2; // 1.6A
/// let c4 = c2 - 300.milli_amps(); // 0.8A
/// ```
///
/// # Scaling Current values
/// You can use the `*` and `/` operators to scale `Current` values by a scalar `u32` or `f32` value.
/// The result is a new `Current` value, rounded down to the nearest whole microamp (μA).
///
/// If the result of operation would overflow or underflow, the operation will panic.
///
/// If the result of the operation would be infinite or NaN, the operation will panic.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let c1 = 200.milli_amps(); // 200mA
/// let c2 = c1 * 3; // 660mA
///
/// let r3 = 1.5.amps(); // 1.5A
/// let r4 = r3 / 2.5; // 0.6A
/// ```
///
/// # Converting to other denominations
/// You can use the `micro_amps`, `milli_amps`, and `amps`, methods to convert a `Current`
/// value to a numeric value in the specified denomination.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let c1 = 1.2.amps(); // 1.2A
///
/// println!("{:.3}A is {:.1}A", c1.amps(), c1.milli_amps());
/// ```
///
#[derive(Clone, Copy, Debug)]
pub struct Current {
    raw: u64,
}

impl Current {
    /// Creates a new `Current` from a number of whole microamps (μA).
    ///
    /// It is recommended to use the `micro_amps`, `milli_amps`, and `amps`, extension
    /// methods on integer and floating-point types instead.
    #[inline]
    pub const fn from_micro_amps(value: u64) -> Current {
        Current { raw: value }
    }

    /// Returns the current value in whole microamps (μA).
    #[inline]
    pub const fn micro_amps(&self) -> u64 {
        self.raw
    }

    /// Returns the current value in fractional milliamps (A).
    #[inline]
    pub fn milli_amps(&self) -> f64 {
        self.raw as f64 / 1_000f64
    }

    /// Returns the current value in fractional amps (A).
    #[inline]
    pub fn amps(&self) -> f64 {
        self.raw as f64 / 1_000_000f64
    }

    /// Returns whether the current value is zero amps (0A).
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.raw == 0
    }

    /// Returns a `Current` value of zero amps (0A).
    #[inline]
    pub const fn zero() -> Self {
        Current::from_micro_amps(0)
    }
}

impl PartialEq for Current {
    #[inline]
    fn eq(&self, other: &Current) -> bool {
        self.raw == other.raw
    }
}

impl Eq for Current {}

impl PartialOrd for Current {
    #[inline]
    fn partial_cmp(&self, other: &Current) -> Option<cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}

impl Ord for Current {
    #[inline]
    fn cmp(&self, other: &Current) -> cmp::Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl ops::Add for Current {
    type Output = Current;

    /// Adds two `Current` values together, returning a new `Current` value.
    #[inline]
    fn add(self, other: Current) -> Current {
        self.raw
            .checked_add(other.raw)
            .map(Current::from_micro_amps)
            .expect("Overflow when adding current values")
    }
}

impl ops::Sub for Current {
    type Output = Current;

    /// Subtracts one `Current` value from another, returning a new `Current` value.
    #[inline]
    fn sub(self, other: Current) -> Current {
        self.raw
            .checked_sub(other.raw)
            .map(Current::from_micro_amps)
            .expect("Overflow when subtracting current values")
    }
}

macro_rules! impl_mul_for_integer {
    ($i:ty) => {
        impl ops::Mul<$i> for Current {
            type Output = Current;

            #[inline]
            #[allow(unused_comparisons)]
            fn mul(self, scale_factor: $i) -> Current {
                if scale_factor < 0 {
                    panic!("Cannot multiply current value by negative value")
                }
                self.raw
                    .checked_mul(scale_factor as u64)
                    .map(Current::from_micro_amps)
                    .expect("Overflow when multiplying current value")
            }
        }
    };
}

impl_mul_for_integer!(u8);
impl_mul_for_integer!(u16);
impl_mul_for_integer!(u32);
impl_mul_for_integer!(u64);
impl_mul_for_integer!(i8);
impl_mul_for_integer!(i16);
impl_mul_for_integer!(i32);
impl_mul_for_integer!(i64);

impl ops::Mul<f32> for Current {
    type Output = Current;

    /// Multiplies a `Current` value by a floating-point value, returning a new `Current` value.
    #[inline]
    fn mul(self, scale_factor: f32) -> Current {
        self * scale_factor as f64
    }
}

impl ops::Mul<f64> for Current {
    type Output = Current;

    /// Multiplies a `Current` value by a floating-point value, returning a new `Current` value.
    #[inline]
    fn mul(self, scale_factor: f64) -> Current {
        let result = match scale_factor {
            _ if scale_factor.is_infinite() => {
                panic!("Cannot multiply current value by infinity")
            }
            _ if scale_factor.is_nan() => panic!("Cannot multiply current value by NaN"),
            _ if scale_factor.is_sign_negative() => {
                panic!("Cannot multiply current value by negative value")
            }
            _ => self.raw as f64 * scale_factor,
        };

        Current::from_micro_amps(result as u64)
    }
}

macro_rules! impl_div_for_integer {
    ($i:ty) => {
        impl ops::Div<$i> for Current {
            type Output = Current;

            /// Divides a `Current` value by an integer value, returning a new `Current` value.
            #[inline]
            #[allow(unused_comparisons)]
            fn div(self, divisor: $i) -> Current {
                if divisor == 0 {
                    panic!("Cannot divide current value by zero");
                } else if divisor < 0 {
                    panic!("Cannot divide current value by negative value");
                }
                self.raw
                    .checked_div(divisor as u64)
                    .map(Current::from_micro_amps)
                    .expect("Overflow when dividing current value")
            }
        }
    };
}

impl_div_for_integer!(u8);
impl_div_for_integer!(u16);
impl_div_for_integer!(u32);
impl_div_for_integer!(u64);
impl_div_for_integer!(i8);
impl_div_for_integer!(i16);
impl_div_for_integer!(i32);
impl_div_for_integer!(i64);

impl ops::Div<f32> for Current {
    type Output = Current;

    /// Divides a `Current` value by a floating-point value, returning a new `Current` value.
    #[inline]
    fn div(self, divisor: f32) -> Current {
        self / divisor as f64
    }
}

impl ops::Div<f64> for Current {
    type Output = Current;

    /// Divides a `Current` value by a floating-point value, returning a new `Current` value.
    #[inline]
    fn div(self, divisor: f64) -> Current {
        let result = match divisor {
            _ if divisor == 0f64 => panic!("Cannot divide current value by zero"),
            _ if divisor.is_infinite() => {
                panic!("Cannot divide current value by infinity")
            }
            _ if divisor.is_nan() => panic!("Cannot divide current value by NaN"),
            _ if divisor.is_sign_negative() => {
                panic!("Cannot divide current value by negative value")
            }
            _ => (self.raw as f64) / divisor,
        };

        Current::from_micro_amps(result as u64)
    }
}

/// Extension trait for simple short-hands for creating `Current` values from `u32` values.
pub trait FromInteger {
    /// Creates a new `Current` from a number of whole microamps (μA).
    fn micro_amps(self) -> Current;

    /// Creates a new `Current` from a number of whole milliamps (mA).
    fn milli_amps(self) -> Current;

    /// Creates a new `Current` from a number of whole amps (A).
    fn amps(self) -> Current;
}

macro_rules! impl_current_from_integer {
    ($i:ty) => {
        impl FromInteger for $i {
            #[inline]
            fn micro_amps(self) -> Current {
                Current::from_micro_amps(self as u64)
            }

            #[inline]
            fn milli_amps(self) -> Current {
                let milliamps = (self as u64)
                    .checked_mul(1_000)
                    .expect("Overflow when converting milliamps to microamps");
                Current::from_micro_amps(milliamps)
            }

            #[inline]
            fn amps(self) -> Current {
                let milliamps = (self as u64)
                    .checked_mul(1_000_000)
                    .expect("Overflow when converting amps to microamps");
                Current::from_micro_amps(milliamps)
            }
        }
    };
}

impl_current_from_integer!(u8);
impl_current_from_integer!(u16);
impl_current_from_integer!(u32);
impl_current_from_integer!(u64);
impl_current_from_integer!(i8);
impl_current_from_integer!(i16);
impl_current_from_integer!(i32);
impl_current_from_integer!(i64);

/// Extension trait for simple short-hands for creating `Current` values from `f32` values.
pub trait FromFloat {
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

macro_rules! impl_current_from_float {
    ($f:ty) => {
        impl FromFloat for $f {
            #[inline]
            fn micro_amps(self) -> Current {
                assert_positive_float!(self);
                Current::from_micro_amps(self as u64)
            }

            #[inline]
            fn milli_amps(self) -> Current {
                assert_positive_float!(self);
                let milliamps = (self as f64) * 1_000f64;
                Current::from_micro_amps(milliamps as u64)
            }

            #[inline]
            fn amps(self) -> Current {
                assert_positive_float!(self);
                let milliamps = (self as f64) * 1_000_000f64;
                Current::from_micro_amps(milliamps as u64)
            }
        }
    };
}

impl_current_from_float!(f32);
impl_current_from_float!(f64);

impl fmt::Display for Current {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (value, unit) = match self.raw {
            0..=999 => (self.raw as f64, "μA"),
            1_000..=999_999 => ((self.raw as f64) / 1_000f64, "mA"),
            _ => ((self.raw as f64) / 1_000_000f64, "A"),
        };

        write!(f, "{value:.2}{unit}")
    }
}
