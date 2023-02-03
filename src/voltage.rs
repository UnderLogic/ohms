use crate::{assert, helpers};
use core::{cmp, ops};

/// Represents a voltage value, stored as whole microvolts (μV).
/// This can be positive or negative.
///
/// **Reminder:** `1000 μV = 1 mV, 1000 mV = 1 V, 1000 V = 1k V`
///
/// This is an immutable type. Any math operators return a new `Voltage` value.
///
/// # Creating a Voltage value
/// You can create a `Voltage` value using the `from_micro_volts` method, or using one of the
/// extension methods on `i32` and `f32`:
///
/// ```rust
/// let v1 = Voltage::from_micro_volts(325); // 325μV
///
/// // More ergonomic:
/// let v2 = 900.milli_volts(); // 900mV
/// let v3 = 12.volts(); // 12V
/// let v4 = 3.3f32.volts(); // 3.3V
/// ```
///
/// # Comparing Voltage values
/// You can compare two `Voltage` values using the `==`, `!=`, `<`, `>`, `<=` and `>=` operators.
///
/// ```rust
/// let v1 = 3.3f32.volts(); // 3.3V
/// let v2 = 5.2f32.volts(); // 5.2V
///
/// if v1 > v2 {
///     println!("{} is greater than {}", v1, v2);
/// } else {
///     println!("{} is less than or equal to {}", v1, v2);
/// }
/// ```
///
/// # Combining Voltage values
/// You can use the `+` and `-` operators to add and subtract `Voltage` values from each other.
/// The result is a new `Voltage` value, rounded down to the nearest whole microvolt (μV).
///
/// If the result of the operation would overflow or underflow the `i32` value, the operation will panic.
///
/// ```rust
/// let v1 = 3.7f32.volts(); // 3.7V
/// let v2 = 9.volts(); // 9V
///
/// let v3 = v1 + v2; // 12.7V
/// let v4 = v2 - 6.volts(); // 3V
/// ```
///
/// # Scaling Voltage values
/// You can use the `*` and `/` operators to scale `Voltage` values by a scalar `i32` or `f32` value.
/// The result is a new `Voltage` value, rounded down to the nearest whole microvolt (μV).
///
/// If the result of operation would overflow or underflow the `i32` value, the operation will panic.
///
/// If the result of the operation would be infinite or NaN, the operation will panic.
///
/// ```rust
/// let v1 = 6u32.volts(); // 6V
/// let v2 = v1 * 2; // 12V
///
/// let v3 = 250u32.micro_volts(); // 250μV
/// let v4 = v3 / 2f32; // 125μV
/// ```
///
/// # Converting to other denominations
/// You can use the `micro_volts`, `milli_volts`, `volts`, and `kilo_volts` methods to convert a `Voltage`
/// value to a `i32` or `f32` value in the specified denomination.
///
/// ```rust
/// let v1 = 3.3f32.volts(); // 3.3V
///
/// println!("{:.2}V is {:.1}mV", v1.volts(), v1.milli_volts());
/// ```
///
#[derive(Clone, Copy, Debug)]
pub struct Voltage {
    microvolts: i32,
}

impl Voltage {
    /// Creates a new `Voltage` from a number of whole microvolts (μV).
    ///
    /// It is recommended to use the `micro_volts`, `milli_volts`, `volts`, and `kilo_volts` extension
    /// methods on `i32` and `f32` instead of this method for ergonomics.
    #[inline]
    pub const fn from_micro_volts(microvolts: i32) -> Voltage {
        Voltage { microvolts }
    }

    /// Returns the voltage value in fractional millivolts (mV).
    #[inline]
    pub const fn milli_volts(&self) -> f32 {
        self.microvolts as f32 / 1_000f32
    }

    /// Returns the voltage value in fractional volts (V).
    #[inline]
    pub fn volts(&self) -> f32 {
        self.microvolts as f32 / 1_000_000f32
    }

    /// Returns the voltage value in fractional kilovolts (kV).
    #[inline]
    pub fn kilo_volts(&self) -> f32 {
        self.microvolts as f32 / 1_000_000_000f32
    }

    /// Returns whether the voltage value is zero volts (0V).
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.microvolts == 0
    }

    /// Returns whether the voltage value is positive.
    ///
    /// This returns `true` if the voltage value is greater than or equal to zero volts (0V).
    #[inline]
    pub const fn is_positive(&self) -> bool {
        self.microvolts >= 0
    }

    /// Returns whether the voltage value is negative.
    ///
    /// This returns `true` if the voltage value is less than zero volts (0V).
    #[inline]
    pub const fn is_negative(&self) -> bool {
        self.microvolts < 0
    }

    /// Returns the absolute value of the voltage value.
    #[inline]
    pub const fn abs(&self) -> Voltage {
        Voltage::from_micro_volts(self.microvolts.abs())
    }

    /// Inverts the voltage value from positive to negative or negative to positive.
    #[inline]
    pub const fn invert(&self) -> Voltage {
        Voltage::from_micro_volts(-self.microvolts)
    }

    /// Returns a `Voltage` value of zero volts (0V).
    #[inline]
    pub const fn zero() -> Self {
        Voltage::from_micro_volts(0)
    }
}

// Equality traits
impl PartialEq for Voltage {
    #[inline]
    fn eq(&self, other: &Voltage) -> bool {
        self.microvolts == other.microvolts
    }
}

impl Eq for Voltage {}

// Comparison traits
impl PartialOrd for Voltage {
    #[inline]
    fn partial_cmp(&self, other: &Voltage) -> Option<cmp::Ordering> {
        self.microvolts.partial_cmp(&other.microvolts)
    }
}

impl Ord for Voltage {
    #[inline]
    fn cmp(&self, other: &Voltage) -> cmp::Ordering {
        self.microvolts.cmp(&other.microvolts)
    }
}

// Math operators
impl ops::Add for Voltage {
    type Output = Voltage;

    #[inline]
    fn add(self, other: Voltage) -> Voltage {
        self.microvolts
            .checked_add(other.microvolts)
            .map(Voltage::from_micro_volts)
            .expect("Overflow when adding voltage values")
    }
}

impl ops::Sub for Voltage {
    type Output = Voltage;

    #[inline]
    fn sub(self, other: Voltage) -> Voltage {
        self.microvolts
            .checked_sub(other.microvolts)
            .map(Voltage::from_micro_volts)
            .expect("Overflow when subtracting voltage values")
    }
}

impl ops::Mul<i32> for Voltage {
    type Output = Voltage;

    #[inline]
    fn mul(self, other: i32) -> Voltage {
        self.microvolts
            .checked_mul(other)
            .map(Voltage::from_micro_volts)
            .expect("Overflow when multiplying voltage value")
    }
}

impl ops::Mul<f32> for Voltage {
    type Output = Voltage;

    #[inline]
    fn mul(self, other: f32) -> Voltage {
        let result = match other {
            _ if other.is_infinite() => {
                panic!("Cannot multiply voltage value by infinity")
            }
            _ if other.is_nan() => panic!("Cannot multiply voltage value by NaN"),
            _ if other == 0f32 => Some(0),
            _ => helpers::checked_mul_signed_f32(self.microvolts, other),
        };

        match result {
            Some(microvolts) => Voltage::from_micro_volts(microvolts),
            _ => panic!("Overflow when multiplying voltage value"),
        }
    }
}

impl ops::Div<i32> for Voltage {
    type Output = Voltage;

    #[inline]
    fn div(self, other: i32) -> Voltage {
        self.microvolts
            .checked_div(other)
            .map(Voltage::from_micro_volts)
            .expect("Overflow when dividing voltage value")
    }
}

impl ops::Div<f32> for Voltage {
    type Output = Voltage;

    #[inline]
    fn div(self, other: f32) -> Voltage {
        let result = match other {
            _ if other.is_infinite() => {
                panic!("Cannot divide voltage value by infinity")
            }
            _ if other.is_nan() => panic!("Cannot divide voltage value by NaN"),
            _ if other == 0f32 => panic!("Cannot divide voltage value by zero"),
            _ => helpers::checked_div_signed_f32(self.microvolts, other),
        };

        match result {
            Some(microvolts) => Voltage::from_micro_volts(microvolts),
            _ => panic!("Overflow when dividing voltage value"),
        }
    }
}

/// Extension trait for simple short-hands for creating `Voltage` values from `u32` values.
pub trait ExtI32 {
    /// Creates a new `Voltage` from a number of whole microvolts (μV).
    fn micro_volts(self) -> Voltage;

    /// Creates a new `Voltage` from a number of whole millivolts (mV).
    fn milli_volts(self) -> Voltage;

    /// Creates a new `Voltage` from a number of whole volts (V).
    fn volts(self) -> Voltage;

    /// Creates a new `Voltage` from a number of whole kilovolts (kV).
    fn kilo_volts(self) -> Voltage;
}

impl ExtI32 for i32 {
    #[inline]
    fn micro_volts(self) -> Voltage {
        Voltage::from_micro_volts(self)
    }

    #[inline]
    fn milli_volts(self) -> Voltage {
        let microvolts = self
            .checked_mul(1_000)
            .expect("Overflow when converting millivolts to microvolts");
        Voltage::from_micro_volts(microvolts)
    }

    #[inline]
    fn volts(self) -> Voltage {
        let microvolts = self
            .checked_mul(1_000_000)
            .expect("Overflow when converting volts to microvolts");
        Voltage::from_micro_volts(microvolts)
    }

    #[inline]
    fn kilo_volts(self) -> Voltage {
        let microvolts = self
            .checked_mul(1_000_000_000)
            .expect("Overflow when converting kilovolts to microvolts");
        Voltage::from_micro_volts(microvolts)
    }
}

/// Extension trait for simple short-hands for creating `Voltage` values from `f32` values.
pub trait ExtF32 {
    /// Creates a new `Voltage` from a number of fractional microvolts (μV).
    ///
    /// The fractional part is rounded down to the nearest whole microvolt (μV).
    fn micro_volts(self) -> Voltage;

    /// Creates a new `Voltage` from a number of fractional millivolts (mV).
    ///
    /// The fractional part is rounded down to the nearest whole microvolt (μV).
    fn milli_volts(self) -> Voltage;

    /// Creates a new `Voltage` from a number of fractional volts (V).
    ///
    /// The fractional part is rounded down to the nearest whole microvolt (μV).
    fn volts(self) -> Voltage;

    /// Creates a new `Voltage` from a number of fractional kilovolts (kV).
    ///
    /// The fractional part is rounded down to the nearest whole microvolt (μV).
    fn kilo_volts(self) -> Voltage;
}

impl ExtF32 for f32 {
    #[inline]
    fn micro_volts(self) -> Voltage {
        Voltage::from_micro_volts(self as i32)
    }

    #[inline]
    fn milli_volts(self) -> Voltage {
        let millivolts = helpers::checked_mul_signed_f32(1_000, self)
            .expect("Overflow when converting millivolts to microvolts");
        Voltage::from_micro_volts(millivolts)
    }

    #[inline]
    fn volts(self) -> Voltage {
        let millivolts = helpers::checked_mul_signed_f32(1_000_000, self)
            .expect("Overflow when converting volts to microvolts");
        Voltage::from_micro_volts(millivolts)
    }

    #[inline]
    fn kilo_volts(self) -> Voltage {
        let millivolts = helpers::checked_mul_signed_f32(1_000_000_000, self)
            .expect("Overflow when converting kilovolts to microvolts");
        Voltage::from_micro_volts(millivolts)
    }
}
