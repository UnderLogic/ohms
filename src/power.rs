use crate::assert_positive_float;
use core::{cmp, fmt, ops};

/// Represents a power value, stored as whole microwatts (μW) as a 64-bit value.
/// This value can only be positive.
///
/// **Reminder:** `1000 μW = 1 mW, 1000 mW = 1 W, 1000W = 1kW`
///
/// This is an immutable type. Any math operators return a new `Power` value.
///
/// # Creating a Power value
/// You can create a `Power` value using the `from_micro_watts` method, or using one of the
/// extension methods on integer and floating-point types:
///
/// ```rust
/// use ohms::prelude::*;
///
/// let p1 = Power::from_micro_watts(800); // 800μW
///
/// // More ergonomic:
/// let p2 = 100.milli_watts(); // 0.1W
/// let p3 = 5.watts(); // 5W
/// ```
///
/// # Comparing Power values
/// You can compare two `Power` values using the `==`, `!=`, `<`, `>`, `<=` and `>=` operators.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let p1 = 200.milli_watts(); // 200mW
/// let p2 = 1.5.watts(); // 1.5W
///
/// if p1 > p2 {
///     println!("{} is greater than {}", p1, p2);
/// } else {
///     println!("{} is less than or equal to {}", p1, p2);
/// }
/// ```
///
/// # Combining Power values
/// You can use the `+` and `-` operators to add and subtract `Power` values from each other.
/// The result is a new `Power` value, rounded down to the nearest whole microwatt (μW).
///
/// If the result of the operation would overflow or underflow, the operation will panic.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let p1 = 500.milli_watts(); // 0.5W
/// let p2 = 1.1.watts(); // 1.1W
///
/// let sum = p1 + p2; // 1.6W
/// let diff = p2 - 300.milli_watts(); // 0.8W
/// ```
///
/// # Scaling Power values
/// You can use the `*` and `/` operators to scale `Power` values by an integer or floating-point value.
/// The result is a new `Power` value, rounded down to the nearest whole microwatt (μW).
///
/// If the result of operation would overflow or underflow, the operation will panic.
///
/// If the result of the operation would be infinite or NaN, the operation will panic.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let p1 = 200.milli_watts(); // 200mW
/// let p2 = p1 * 3; // 600mW
///
/// let p3 = 1.5.watts(); // 1.5W
/// let p4 = p3 / 2.5; // 0.6W
/// ```
///
/// # Converting to other denominations
/// You can use the `micro_watts`, `milli_watts`, `watts`, and `kilo_watts`, methods to convert a `Power`
/// value to a numeric value in the specified denomination.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let p1 = 1.2.watts(); // 1.2W
///
/// println!("{:.3} W is {:.1} mW", p1.watts(), p1.milli_watts());
/// ```
///
#[derive(Clone, Copy, Debug)]
pub struct Power {
    raw: u64,
}

impl Power {
    /// Creates a new `Power` from a number of whole microwatts (μW).
    ///
    /// It is recommended to use the `micro_watts`, `milli_watts`, `watts`, and `kilo_watts`, extension
    /// methods on integer and floating-point types instead.
    #[inline]
    pub const fn from_micro_watts(value: u64) -> Self {
        Self { raw: value }
    }

    /// Returns the power value in whole microwatts (μW).
    #[inline]
    pub const fn micro_watts(&self) -> u64 {
        self.raw
    }

    /// Returns the power value in fractional milliwatts (mW).
    #[inline]
    pub fn milli_watts(&self) -> f64 {
        self.raw as f64 / 1_000f64
    }

    /// Returns the power value in fractional watts (W).
    #[inline]
    pub fn watts(&self) -> f64 {
        self.raw as f64 / 1_000_000f64
    }

    /// Returns the power value in fractional kilowatts (kW).
    #[inline]
    pub fn kilo_watts(&self) -> f64 {
        self.raw as f64 / 1_000_000_000f64
    }

    /// Returns whether the power value is zero watts (0W).
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.raw == 0
    }

    /// Returns a `Power` value of zero watts (0W).
    #[inline]
    pub const fn zero() -> Self {
        Self::from_micro_watts(0)
    }
}

impl PartialEq for Power {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl Eq for Power {}

impl PartialOrd for Power {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}

impl Ord for Power {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl ops::Add for Power {
    type Output = Self;

    /// Adds two `Power` values together, returning a new `Power` value.
    #[inline]
    fn add(self, other: Self) -> Self {
        self.raw
            .checked_add(other.raw)
            .map(Self::from_micro_watts)
            .expect("Overflow when adding power values")
    }
}

impl ops::Sub for Power {
    type Output = Self;

    /// Subtracts one `Power` value from another, returning a new `Power` value.
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.raw
            .checked_sub(other.raw)
            .map(Self::from_micro_watts)
            .expect("Overflow when subtracting power values")
    }
}

macro_rules! impl_mul_for_integer {
    ($i:ty) => {
        impl ops::Mul<$i> for Power {
            type Output = Self;

            #[inline]
            #[allow(unused_comparisons)]
            fn mul(self, scale_factor: $i) -> Self {
                if scale_factor < 0 {
                    panic!("Cannot multiply power value by negative value")
                }
                self.raw
                    .checked_mul(scale_factor as u64)
                    .map(Self::from_micro_watts)
                    .expect("Overflow when multiplying power value")
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

impl ops::Mul<f32> for Power {
    type Output = Self;

    /// Multiplies a `Power` value by a floating-point value, returning a new `Power` value.
    #[inline]
    fn mul(self, scale_factor: f32) -> Self {
        self * scale_factor as f64
    }
}

impl ops::Mul<f64> for Power {
    type Output = Self;

    /// Multiplies a `Power` value by a floating-point value, returning a new `Power` value.
    #[inline]
    fn mul(self, scale_factor: f64) -> Self {
        let result = match scale_factor {
            _ if scale_factor.is_infinite() => {
                panic!("Cannot multiply power value by infinity")
            }
            _ if scale_factor.is_nan() => panic!("Cannot multiply power value by NaN"),
            _ if scale_factor.is_sign_negative() => {
                panic!("Cannot multiply power value by negative value")
            }
            _ => self.raw as f64 * scale_factor,
        };

        Self::from_micro_watts(result as u64)
    }
}

macro_rules! impl_div_for_integer {
    ($i:ty) => {
        impl ops::Div<$i> for Power {
            type Output = Self;

            /// Divides a `Power` value by an integer value, returning a new `Power` value.
            #[inline]
            #[allow(unused_comparisons)]
            fn div(self, divisor: $i) -> Self {
                if divisor == 0 {
                    panic!("Cannot divide power value by zero");
                } else if divisor < 0 {
                    panic!("Cannot divide power value by negative value");
                }
                self.raw
                    .checked_div(divisor as u64)
                    .map(Self::from_micro_watts)
                    .expect("Overflow when dividing power value")
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

impl ops::Div<f32> for Power {
    type Output = Self;

    /// Divides a `Power` value by a floating-point value, returning a new `Power` value.
    #[inline]
    fn div(self, divisor: f32) -> Self {
        self / divisor as f64
    }
}

impl ops::Div<f64> for Power {
    type Output = Self;

    /// Divides a `Power` value by a floating-point value, returning a new `Power` value.
    #[inline]
    fn div(self, divisor: f64) -> Self {
        let result = match divisor {
            _ if divisor == 0f64 => panic!("Cannot divide power value by zero"),
            _ if divisor.is_infinite() => {
                panic!("Cannot divide power value by infinity")
            }
            _ if divisor.is_nan() => panic!("Cannot divide power value by NaN"),
            _ if divisor.is_sign_negative() => {
                panic!("Cannot divide power value by negative value")
            }
            _ => (self.raw as f64) / divisor,
        };

        Self::from_micro_watts(result as u64)
    }
}

/// Extension trait for simple short-hands for creating `Power` values from integer values.
pub trait FromInteger {
    /// Creates a new `Power` from a number of whole microwatts (μW).
    fn micro_watts(self) -> Power;

    /// Creates a new `Power` from a number of whole milliwatts (mW).
    fn milli_watts(self) -> Power;

    /// Creates a new `Power` from a number of whole watts (W).
    fn watts(self) -> Power;

    /// Creates a new `Power` from a number of whole kilowatts (kW).
    fn kilo_watts(self) -> Power;
}

macro_rules! impl_power_from_integer {
    ($i:ty) => {
        impl FromInteger for $i {
            #[inline]
            fn micro_watts(self) -> Power {
                Power::from_micro_watts(self as u64)
            }

            #[inline]
            fn milli_watts(self) -> Power {
                let microwatts = (self as u64)
                    .checked_mul(1_000)
                    .expect("Overflow when converting milliwatts to microwatts");
                Power::from_micro_watts(microwatts)
            }

            #[inline]
            fn watts(self) -> Power {
                let microwatts = (self as u64)
                    .checked_mul(1_000_000)
                    .expect("Overflow when converting watts to microwatts");
                Power::from_micro_watts(microwatts)
            }

            #[inline]
            fn kilo_watts(self) -> Power {
                let microwatts = (self as u64)
                    .checked_mul(1_000_000_000)
                    .expect("Overflow when converting kilowatts to microwatts");
                Power::from_micro_watts(microwatts)
            }
        }
    };
}

impl_power_from_integer!(u8);
impl_power_from_integer!(u16);
impl_power_from_integer!(u32);
impl_power_from_integer!(u64);
impl_power_from_integer!(i8);
impl_power_from_integer!(i16);
impl_power_from_integer!(i32);
impl_power_from_integer!(i64);

/// Extension trait for simple short-hands for creating `Power` values from floating-point values.
pub trait FromFloat {
    /// Creates a new `Power` from a number of fractional microwatts (μW).
    ///
    /// The fractional part is rounded down to the nearest whole microwatt (μW).
    fn micro_watts(self) -> Power;

    /// Creates a new `Power` from a number of fractional milliwatt (mW).
    ///
    /// The fractional part is rounded down to the nearest whole microwatt (μW).
    fn milli_watts(self) -> Power;

    /// Creates a new `Power` from a number of fractional watts (W).
    ///
    /// The fractional part is rounded down to the nearest whole microwatt (μW).
    fn watts(self) -> Power;

    /// Creates a new `Power` from a number of fractional kilowatts (kW).
    ///
    /// The fractional part is rounded down to the nearest whole microwatt (μW).
    fn kilo_watts(self) -> Power;
}

macro_rules! impl_power_from_float {
    ($f:ty) => {
        impl FromFloat for $f {
            #[inline]
            fn micro_watts(self) -> Power {
                assert_positive_float!(self);
                Power::from_micro_watts(self as u64)
            }

            #[inline]
            fn milli_watts(self) -> Power {
                assert_positive_float!(self);
                let microwatts = (self as f64) * 1_000f64;
                Power::from_micro_watts(microwatts as u64)
            }

            #[inline]
            fn watts(self) -> Power {
                assert_positive_float!(self);
                let microwatts = (self as f64) * 1_000_000f64;
                Power::from_micro_watts(microwatts as u64)
            }

            #[inline]
            fn kilo_watts(self) -> Power {
                assert_positive_float!(self);
                let microwatts = (self as f64) * 1_000_000_000f64;
                Power::from_micro_watts(microwatts as u64)
            }
        }
    };
}

impl_power_from_float!(f32);
impl_power_from_float!(f64);

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (value, unit) = match self.raw {
            0..=999 => (self.raw as f64, "μW"),
            1_000..=999_999 => ((self.raw as f64) / 1_000f64, "mW"),
            1_000_000..=999_999_999 => ((self.raw as f64) / 1_000_000f64, "W"),
            _ => ((self.raw as f64) / 1_000_000f64, "kW"),
        };

        write!(f, "{value:.2} {unit}")
    }
}
