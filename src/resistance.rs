use crate::assert_positive_float;
use core::{cmp, ops};

/// Represents a resistance value, stored as whole milliohms (mΩ) stored in a `u64` value.
/// This value can only be positive.
///
/// **Reminder:** `1000 mΩ = 1 Ω, 1000 Ω = 1 kΩ, 1000 kΩ = 1 MΩ`
///
/// This is an immutable type. Any math operators return a new `Resistance` value.
///
/// # Creating a Resistance value
/// You can create a `Resistance` value using the `from_milli_ohms` method, or using one of the
/// extension methods on integer and floating-point types:
///
/// ```rust
/// use ohms::prelude::*;
///
/// let r1 = Resistance::from_milli_ohms(1000); // 1Ω
///
/// // More ergonomic:
/// let r2 = 100.milli_ohms(); // 0.1Ω
/// let r3 = 220.ohms(); // 220Ω
/// let r4 = 1.5.kilo_ohms(); // 1.5kΩ
/// let r5 = 1.5.mega_ohms(); // 1.5MΩ
/// ```
///
/// # Comparing Resistance values
/// You can compare two `Resistance` values using the `==`, `!=`, `<`, `>`, `<=` and `>=` operators.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let r1 = 220.ohms(); // 220Ω
/// let r2 = 4.7.kilo_ohms(); // 4.7kΩ
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
/// If the result of the operation would overflow or underflow, the operation will panic.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let r1 = 220.ohms(); // 220Ω
/// let r2 = 4.7.kilo_ohms(); // 4.7kΩ
///
/// let r3 = r1 + r2; // 4.92kΩ
/// let r4 = r2 - 2.kilo_ohms(); // 2.7kΩ
/// ```
///
/// # Scaling Resistance values
/// You can use the `*` and `/` operators to scale `Resistance` values by an integer or floating-point value.
/// The result is a new `Resistance` value, rounded down to the nearest whole milliohm (mΩ).
///
/// If the result of operation would overflow or underflow, the operation will panic.
///
/// If the result of the operation would be infinite or NaN, the operation will panic.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let r1 = 220.ohms(); // 220Ω
/// let r2 = r1 * 3; // 660Ω
///
/// let r3 = 47.kilo_ohms(); // 47kΩ
/// let r4 = r3 / 2.5; // 18.8kΩ
/// ```
///
/// # Converting to other denominations
/// You can use the `milli_ohms`, `ohms`, `kilo_ohms` and `mega_ohms` methods to convert a `Resistance`
/// value to a numeric value in the specified denomination.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let r1 = 47.5.ohms(); // 47.5Ω
///
/// println!("{:.3}kΩ is {:.1}Ω", r1.kilo_ohms(), r1.ohms());
/// ```
///
#[derive(Clone, Copy, Debug)]
pub struct Resistance {
    raw: u64,
}

impl Resistance {
    /// Creates a new `Resistance` from a number of whole milliohms (mΩ).
    ///
    /// It is recommended to use the `milli_ohms`, `ohms`, `kilo_ohms` and `mega_ohms` extension
    /// methods on integer and floating-point values instead.
    #[inline]
    pub const fn from_milli_ohms(value: u64) -> Resistance {
        Resistance { raw: value }
    }

    /// Returns the resistance value in whole milliohms (mΩ).
    #[inline]
    pub const fn milli_ohms(&self) -> u64 {
        self.raw
    }

    /// Returns the resistance value in fractional ohms (Ω).
    #[inline]
    pub fn ohms(&self) -> f64 {
        self.raw as f64 / 1_000f64
    }

    /// Returns the resistance value in fractional kilohms (kΩ).
    #[inline]
    pub fn kilo_ohms(&self) -> f64 {
        self.raw as f64 / 1_000_000f64
    }

    /// Returns the resistance value in fractional megaohms (MΩ).
    #[inline]
    pub fn mega_ohms(&self) -> f64 {
        self.raw as f64 / 1_000_000_000f64
    }

    /// Returns whether the resistance value is zero ohms (0Ω).
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.raw == 0
    }

    /// Returns a `Resistance` value of zero ohms (0Ω).
    #[inline]
    pub const fn zero() -> Self {
        Resistance::from_milli_ohms(0)
    }
}

impl PartialEq for Resistance {
    #[inline]
    fn eq(&self, other: &Resistance) -> bool {
        self.raw == other.raw
    }
}

impl Eq for Resistance {}

impl PartialOrd for Resistance {
    #[inline]
    fn partial_cmp(&self, other: &Resistance) -> Option<cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}

impl Ord for Resistance {
    #[inline]
    fn cmp(&self, other: &Resistance) -> cmp::Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl ops::Add for Resistance {
    type Output = Resistance;

    /// Adds two `Resistance` values together, returning a new `Resistance` value.
    #[inline]
    fn add(self, other: Resistance) -> Resistance {
        self.raw
            .checked_add(other.raw)
            .map(Resistance::from_milli_ohms)
            .expect("Overflow when adding resistance values")
    }
}

impl ops::Sub for Resistance {
    type Output = Resistance;

    /// Subtracts two `Resistance` values from each other, returning a new `Resistance` value.
    #[inline]
    fn sub(self, other: Resistance) -> Resistance {
        self.raw
            .checked_sub(other.raw)
            .map(Resistance::from_milli_ohms)
            .expect("Overflow when subtracting resistance values")
    }
}

macro_rules! impl_mul_for_integer {
    ($i:ty) => {
        impl ops::Mul<$i> for Resistance {
            type Output = Resistance;

            /// Multiplies a `Resistance` value by an integer value, returning a new `Resistance` value.
            #[inline]
            fn mul(self, scale_factor: $i) -> Resistance {
                if scale_factor < 0 {
                    panic!("Cannot multiply resistance value by negative value");
                }
                self.raw
                    .checked_mul(scale_factor as u64)
                    .map(Resistance::from_milli_ohms)
                    .expect("Overflow when multiplying resistance value")
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

impl ops::Mul<f32> for Resistance {
    type Output = Resistance;

    /// Multiplies the `Resistance` value by a floating-point value, returning a new `Resistance` value.
    #[inline]
    fn mul(self, scale_factor: f32) -> Resistance {
        self * scale_factor as f64
    }
}

impl ops::Mul<f64> for Resistance {
    type Output = Resistance;

    /// Multiplies a `Resistance` value by a floating-point value, returning a new `Resistance` value.
    #[inline]
    fn mul(self, scale_factor: f64) -> Resistance {
        let result = match scale_factor {
            _ if scale_factor.is_infinite() => {
                panic!("Cannot multiply resistance value by infinity")
            }
            _ if scale_factor.is_nan() => panic!("Cannot multiply resistance value by NaN"),
            _ if scale_factor.is_sign_negative() => {
                panic!("Cannot multiply resistance value by negative value")
            }
            _ => self.raw as f64 * scale_factor,
        };

        Resistance::from_milli_ohms(result as u64)
    }
}

macro_rules! impl_div_for_integer {
    ($i:ty) => {
        impl ops::Div<$i> for Resistance {
            type Output = Resistance;

            /// Divides a `Resistance` value by an integer value, returning a new `Resistance` value.
            #[inline]
            fn div(self, divisor: $i) -> Resistance {
                if divisor == 0 {
                    panic!("Cannot divide resistance value by zero");
                } else if divisor < 0 {
                    panic!("Cannot divide resistance value by negative value");
                }
                self.raw
                    .checked_div(divisor as u64)
                    .map(Resistance::from_milli_ohms)
                    .expect("Overflow when dividing resistance value")
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

impl ops::Div<f32> for Resistance {
    type Output = Resistance;

    /// Divides a `Resistance` value by a floating-point value, returning a new `Resistance` value.
    #[inline]
    fn div(self, divisor: f32) -> Resistance {
        self / divisor as u64
    }
}

impl ops::Div<f64> for Resistance {
    type Output = Resistance;

    /// Divides a `Resistance` value by a floating-point value, returning a new `Resistance` value.
    #[inline]
    fn div(self, divisor: f64) -> Resistance {
        let result = match divisor {
            _ if divisor == 0f64 => panic!("Cannot divide resistance value by zero"),
            _ if divisor.is_infinite() => {
                panic!("Cannot divide resistance value by infinity")
            }
            _ if divisor.is_nan() => panic!("Cannot divide resistance value by NaN"),
            _ if divisor.is_sign_negative() => {
                panic!("Cannot divide resistance value by negative value")
            }
            _ => (self.raw as f64) / divisor,
        };

        Resistance::from_milli_ohms(result as u64)
    }
}

/// Extension trait for simple short-hands for creating `Resistance` values from integer values.
pub trait FromInteger {
    /// Creates a new `Resistance` from a number of whole milliohms (mΩ).
    fn milli_ohms(self) -> Resistance;

    /// Creates a new `Resistance` from a number of whole ohms (Ω).
    fn ohms(self) -> Resistance;

    /// Creates a new `Resistance` from a number of whole kilohms (kΩ).
    fn kilo_ohms(self) -> Resistance;

    /// Creates a new `Resistance` from a number of whole megaohms (MΩ).
    fn mega_ohms(self) -> Resistance;
}

macro_rules! impl_resistance_from_integer {
    ($i:ty) => {
        impl FromInteger for $i {
            #[inline]
            fn milli_ohms(self) -> Resistance {
                Resistance::from_milli_ohms(self as u64)
            }

            #[inline]
            fn ohms(self) -> Resistance {
                let milliohms = (self as u64)
                    .checked_mul(1_000)
                    .expect("Overflow when converting ohms to milliohms");
                Resistance::from_milli_ohms(milliohms as u64)
            }

            #[inline]
            fn kilo_ohms(self) -> Resistance {
                let milliohms = (self as u64)
                    .checked_mul(1_000_000)
                    .expect("Overflow when converting kilohms to milliohms");
                Resistance::from_milli_ohms(milliohms as u64)
            }

            #[inline]
            fn mega_ohms(self) -> Resistance {
                let milliohms = (self as u64)
                    .checked_mul(1_000_000_000)
                    .expect("Overflow when converting megaohms to milliohms");
                Resistance::from_milli_ohms(milliohms as u64)
            }
        }
    };
}

impl_resistance_from_integer!(u8);
impl_resistance_from_integer!(u16);
impl_resistance_from_integer!(u32);
impl_resistance_from_integer!(u64);
impl_resistance_from_integer!(i8);
impl_resistance_from_integer!(i16);
impl_resistance_from_integer!(i32);
impl_resistance_from_integer!(i64);

/// Extension trait for simple short-hands for creating `Resistance` values from floating-point values.
pub trait FromFloat {
    /// Creates a new `Resistance` from a fractional number of milliohms (mΩ).
    fn milli_ohms(self) -> Resistance;

    /// Creates a new `Resistance` from a fractional number of ohms (Ω).
    fn ohms(self) -> Resistance;

    /// Creates a new `Resistance` from a fractional number of kilohms (kΩ).
    fn kilo_ohms(self) -> Resistance;

    /// Creates a new `Resistance` from a fractional number of megaohms (MΩ).
    fn mega_ohms(self) -> Resistance;
}

macro_rules! impl_resistance_from_float {
    ($f:ty) => {
        impl FromFloat for $f {
            #[inline]
            fn milli_ohms(self) -> Resistance {
                assert_positive_float!(self);
                Resistance::from_milli_ohms(self as u64)
            }

            #[inline]
            fn ohms(self) -> Resistance {
                assert_positive_float!(self);
                let milliohms = (self as f64) * 1_000f64;
                Resistance::from_milli_ohms(milliohms as u64)
            }

            #[inline]
            fn kilo_ohms(self) -> Resistance {
                assert_positive_float!(self);
                let milliohms = (self as f64) * 1_000_000f64;
                Resistance::from_milli_ohms(milliohms as u64)
            }

            #[inline]
            fn mega_ohms(self) -> Resistance {
                assert_positive_float!(self);
                let milliohms = (self as f64) * 1_000_000_000f64;
                Resistance::from_milli_ohms(milliohms as u64)
            }
        }
    };
}

impl_resistance_from_float!(f32);
impl_resistance_from_float!(f64);
