use core::{cmp, ops};

/// Represents a voltage value, stored as whole microvolts (μV) stored in an `i64` value.
/// This value can be positive or negative.
///
/// **Reminder:** `1000 μV = 1 mV, 1000 mV = 1 V, 1000 V = 1k V`
///
/// This is an immutable type. Any math operators return a new `Voltage` value.
///
/// # Creating a Voltage value
/// You can create a `Voltage` value using the `from_micro_volts` method, or using one of the
/// extension methods on integer and floating-point types.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let v1 = Voltage::from_micro_volts(325); // 325μV
///
/// // More ergonomic:
/// let v2 = 900.milli_volts(); // 900mV
/// let v3 = 12.volts(); // 12V
/// let v4 = 3.3.volts(); // 3.3V
/// ```
///
/// # Comparing Voltage values
/// You can compare two `Voltage` values using the `==`, `!=`, `<`, `>`, `<=` and `>=` operators.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let v1 = 3.3.volts(); // 3.3V
/// let v2 = 5.2.volts(); // 5.2V
///
/// if v1 > v2 {
///     println!("{:?} is greater than {:?}", v1, v2);
/// } else {
///     println!("{:?} is less than or equal to {:?}", v1, v2);
/// }
/// ```
///
/// # Combining Voltage values
/// You can use the `+` and `-` operators to add and subtract `Voltage` values from each other.
/// The result is a new `Voltage` value, rounded down to the nearest whole microvolt (μV).
///
/// If the result of the operation would overflow or underflow, the operation will panic.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let v1 = 3.7.volts(); // 3.7V
/// let v2 = 9.volts(); // 9V
///
/// let v3 = v1 + v2; // 12.7V
/// let v4 = v2 - 6.volts(); // 3V
/// ```
///
/// # Scaling Voltage values
/// You can use the `*` and `/` operators to scale `Voltage` values by an integer or floating-point value.
/// The result is a new `Voltage` value, rounded down to the nearest whole microvolt (μV).
///
/// If the result of operation would overflow or underflow, the operation will panic.
///
/// If the result of the operation would be infinite or NaN, the operation will panic.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let v1 = 6.volts(); // 6V
/// let v2 = v1 * 2; // 12V
///
/// let v3 = 250.micro_volts(); // 250μV
/// let v4 = v3 / 2.0; // 125μV
/// ```
///
/// # Converting to other denominations
/// You can use the `micro_volts`, `milli_volts`, `volts`, and `kilo_volts` methods to convert a `Voltage`
/// value to a numeric value in the specified denomination.
///
/// ```rust
/// use ohms::prelude::*;
///
/// let v1 = 3.3.volts(); // 3.3V
///
/// println!("{:.2}V is {:.1}mV", v1.volts(), v1.milli_volts());
/// ```
///
#[derive(Clone, Copy, Debug)]
pub struct Voltage {
    raw: i64,
}

impl Voltage {
    /// Creates a new `Voltage` from a number of whole microvolts (μV).
    ///
    /// It is recommended to use the `micro_volts`, `milli_volts`, `volts`, and `kilo_volts` extension
    /// methods on integer and floating-point types instead.
    #[inline]
    pub const fn from_micro_volts(value: i64) -> Voltage {
        Voltage { raw: value }
    }

    /// Returns the voltage value in whole microvolts (μV).
    #[inline]
    pub fn micro_volts(&self) -> i64 {
        self.raw
    }

    /// Returns the voltage value in fractional millivolts (mV).
    #[inline]
    pub fn milli_volts(&self) -> f64 {
        self.raw as f64 / 1_000_f64
    }

    /// Returns the voltage value in fractional volts (V).
    #[inline]
    pub fn volts(&self) -> f64 {
        self.raw as f64 / 1_000_000_f64
    }

    /// Returns the voltage value in fractional kilovolts (kV).
    #[inline]
    pub fn kilo_volts(&self) -> f64 {
        self.raw as f64 / 1_000_000_000_f64
    }

    /// Returns whether the voltage value is zero volts (0V).
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.raw == 0
    }

    /// Returns whether the voltage value is positive.
    ///
    /// This returns `true` if the voltage value is greater than or equal to zero volts (0V).
    #[inline]
    pub const fn is_positive(&self) -> bool {
        self.raw >= 0
    }

    /// Returns whether the voltage value is negative.
    ///
    /// This returns `true` if the voltage value is less than zero volts (0V).
    #[inline]
    pub const fn is_negative(&self) -> bool {
        self.raw < 0
    }

    /// Returns the absolute value of the voltage value.
    #[inline]
    pub const fn abs(&self) -> Voltage {
        Voltage::from_micro_volts(self.raw.abs())
    }

    /// Inverts the voltage value from positive to negative or negative to positive.
    #[inline]
    pub const fn invert(&self) -> Voltage {
        Voltage::from_micro_volts(self.raw * -1)
    }

    /// Returns a `Voltage` value of zero volts (0V).
    #[inline]
    pub const fn zero() -> Self {
        Voltage::from_micro_volts(0)
    }
}

impl PartialEq for Voltage {
    #[inline]
    fn eq(&self, other: &Voltage) -> bool {
        self.raw == other.raw
    }
}

impl Eq for Voltage {}

impl PartialOrd for Voltage {
    #[inline]
    fn partial_cmp(&self, other: &Voltage) -> Option<cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}

impl Ord for Voltage {
    #[inline]
    fn cmp(&self, other: &Voltage) -> cmp::Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl ops::Add for Voltage {
    type Output = Voltage;

    /// Adds two `Voltage` values together, returning a new `Voltage` value.
    #[inline]
    fn add(self, other: Voltage) -> Voltage {
        self.raw
            .checked_add(other.raw)
            .map(Voltage::from_micro_volts)
            .expect("Overflow when adding voltage values")
    }
}

impl ops::Sub for Voltage {
    type Output = Voltage;

    /// Subtracts the `Voltage` value from another, returning a new `Voltage` value.
    #[inline]
    fn sub(self, other: Voltage) -> Voltage {
        self.raw
            .checked_sub(other.raw)
            .map(Voltage::from_micro_volts)
            .expect("Overflow when subtracting voltage values")
    }
}

macro_rules! impl_mul_for_integer {
    ($i: ty) => {
        impl ops::Mul<$i> for Voltage {
            type Output = Voltage;

            /// Multiplies the `Voltage` value by an integer value, returning a new `Voltage` value.
            #[inline]
            fn mul(self, other: $i) -> Voltage {
                self.raw
                    .checked_mul(other as i64)
                    .map(Voltage::from_micro_volts)
                    .expect("Overflow when multiplying voltage value")
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

impl ops::Mul<f32> for Voltage {
    type Output = Voltage;

    /// Multiplies the `Voltage` value by a floating-point value, returning a new `Voltage` value.
    #[inline]
    fn mul(self, scale_factor: f32) -> Voltage {
        self * scale_factor as f64
    }
}

impl ops::Mul<f64> for Voltage {
    type Output = Voltage;

    /// Multiplies the `Voltage` value by a floating-point value, returning a new `Voltage` value.
    #[inline]
    fn mul(self, scale_factor: f64) -> Voltage {
        let result = match scale_factor {
            _ if scale_factor.is_infinite() => {
                panic!("Cannot multiply voltage value by infinity")
            }
            _ if scale_factor.is_nan() => panic!("Cannot multiply voltage value by NaN"),
            _ => self.raw as f64 * scale_factor,
        };

        Voltage::from_micro_volts(result as i64)
    }
}

macro_rules! impl_div_for_integer {
    ($i: ty) => {
        impl ops::Div<$i> for Voltage {
            type Output = Voltage;

            /// Divides the `Voltage` value by an integer value, returning a new `Voltage` value.
            #[inline]
            fn div(self, divisor: $i) -> Voltage {
                if divisor == 0 {
                    panic!("Cannot divide voltage value by zero");
                }
                self.raw
                    .checked_div(divisor as i64)
                    .map(Voltage::from_micro_volts)
                    .expect("Overflow when dividing voltage value")
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

impl ops::Div<f32> for Voltage {
    type Output = Voltage;

    /// Divides the `Voltage` value by a floating-point value, returning a new `Voltage` value.
    #[inline]
    fn div(self, divisor: f32) -> Voltage {
        self / divisor as f64
    }
}

impl ops::Div<f64> for Voltage {
    type Output = Voltage;

    /// Divides the `Voltage` value by a floating-point value, returning a new `Voltage` value.
    #[inline]
    fn div(self, divisor: f64) -> Voltage {
        let result = match divisor {
            _ if divisor == 0f64 => panic!("Cannot divide voltage value by zero"),
            _ if divisor.is_infinite() => {
                panic!("Cannot divide voltage value by infinity")
            }
            _ if divisor.is_nan() => panic!("Cannot divide voltage value by NaN"),
            _ => (self.raw as f64) / divisor,
        };

        Voltage::from_micro_volts(result as i64)
    }
}

/// Extension trait for simple short-hands for creating `Voltage` values from integer values.
pub trait FromInteger {
    /// Creates a new `Voltage` from a number of whole microvolts (μV).
    fn micro_volts(self) -> Voltage;

    /// Creates a new `Voltage` from a number of whole millivolts (mV).
    fn milli_volts(self) -> Voltage;

    /// Creates a new `Voltage` from a number of whole volts (V).
    fn volts(self) -> Voltage;

    /// Creates a new `Voltage` from a number of whole kilovolts (kV).
    fn kilo_volts(self) -> Voltage;
}

macro_rules! impl_voltage_from_integer {
    ($i: ty) => {
        impl FromInteger for $i {
            #[inline]
            fn micro_volts(self) -> Voltage {
                Voltage::from_micro_volts(self as i64)
            }

            #[inline]
            fn milli_volts(self) -> Voltage {
                let microvolts = (self as i64)
                    .checked_mul(1_000)
                    .expect("Overflow when converting millivolts to microvolts");
                Voltage::from_micro_volts(microvolts)
            }

            #[inline]
            fn volts(self) -> Voltage {
                let microvolts = (self as i64)
                    .checked_mul(1_000_000)
                    .expect("Overflow when converting volts to microvolts");
                Voltage::from_micro_volts(microvolts)
            }

            #[inline]
            fn kilo_volts(self) -> Voltage {
                let microvolts = (self as i64)
                    .checked_mul(1_000_000_000)
                    .expect("Overflow when converting kilovolts to microvolts");
                Voltage::from_micro_volts(microvolts)
            }
        }
    };
}

impl_voltage_from_integer!(u8);
impl_voltage_from_integer!(u16);
impl_voltage_from_integer!(u32);
impl_voltage_from_integer!(u64);
impl_voltage_from_integer!(i8);
impl_voltage_from_integer!(i16);
impl_voltage_from_integer!(i32);
impl_voltage_from_integer!(i64);

/// Extension trait for simple short-hands for creating `Voltage` values from floating-point values.
pub trait FromFloat {
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

macro_rules! impl_voltage_from_float {
    ($f: ty) => {
        impl FromFloat for $f {
            #[inline]
            fn micro_volts(self) -> Voltage {
                Voltage::from_micro_volts(self as i64)
            }

            #[inline]
            fn milli_volts(self) -> Voltage {
                let microvolts = (self as f64) * 1_000f64;
                Voltage::from_micro_volts(microvolts as i64)
            }

            #[inline]
            fn volts(self) -> Voltage {
                let microvolts = (self as f64) * 1_000_000f64;
                Voltage::from_micro_volts(microvolts as i64)
            }

            #[inline]
            fn kilo_volts(self) -> Voltage {
                let microvolts = (self as f64) * 1_000_000_000f64;
                Voltage::from_micro_volts(microvolts as i64)
            }
        }
    };
}

impl_voltage_from_float!(f32);
impl_voltage_from_float!(f64);
