use crate::{assert, helpers};
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
/// let c1 = 220u32.milli_amps(); // 220mA
/// let c2 = 1.2f32.amps(); // 1.2A
///
/// if c1 > c2 {
///     println!("{} is greater than {}", c1, c2);
/// } else {
///     println!("{} is less than or equal to {}", c1, c2);
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
        let result = match other {
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

        match result {
            Some(milliamps) => Current::from_micro_amps(milliamps),
            _ => panic!("Overflow when multiplying current value"),
        }
    }
}

impl ops::Div<u32> for Current {
    type Output = Current;

    #[inline]
    fn div(self, other: u32) -> Current {
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
        let result = match other {
            _ if other.is_infinite() => {
                panic!("Cannot divide current value by infinity")
            }
            _ if other.is_nan() => panic!("Cannot divide current value by NaN"),
            _ if other.is_sign_negative() => {
                panic!("Cannot divide current value by negative value")
            }
            _ if other == 0f32 => panic!("Cannot divide current value by zero"),
            _ => helpers::checked_div_unsigned_f32(self.microamps, other),
        };

        match result {
            Some(milliamps) => Current::from_micro_amps(milliamps),
            _ => panic!("Overflow when dividing current value"),
        }
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
