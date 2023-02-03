use crate::{assert, helpers};
use core::{cmp, ops};

/// Represents a resistance value, stored as whole milliohms (mΩ).
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
/// let r1 = Resistance::from_milli_ohms(1000); // 1Ω
///
/// // More ergonomic:
/// let r2 = 100u32.milli_ohms(); // 0.1Ω
/// let r3 = 220u32.ohms(); // 220Ω
/// let r4 = 1.5f32.kilo_ohms(); // 1.5kΩ
/// let r5 = 1.5f32.mega_ohms(); // 1.5MΩ
/// ```
///
/// # Comparing Resistance values
/// You can compare two `Resistance` values using the `==`, `!=`, `<`, `>`, `<=` and `>=` operators.
///
/// ```rust
/// let r1 = 220u32.ohms(); // 220Ω
/// let r2 = 4.7f32.kilo_ohms(); // 4.7kΩ
///
/// if r1 > r2 {
///     println!("{} is greater than {}", r1, r2);
/// } else {
///     println!("{} is less than or equal to {}", r1, r2);
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
        let result = match other {
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

        match result {
            Some(milliohms) => Resistance::from_milli_ohms(milliohms),
            _ => panic!("Overflow when multiplying resistance value"),
        }
    }
}

impl ops::Div<u32> for Resistance {
    type Output = Resistance;

    #[inline]
    fn div(self, other: u32) -> Resistance {
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
        let result = match other {
            _ if other.is_infinite() => {
                panic!("Cannot divide resistance value by infinity")
            }
            _ if other.is_nan() => panic!("Cannot divide resistance value by NaN"),
            _ if other.is_sign_negative() => {
                panic!("Cannot divide resistance value by negative value")
            }
            _ if other == 0f32 => panic!("Cannot divide resistance value by zero"),
            _ => helpers::checked_div_unsigned_f32(self.milliohms, other),
        };

        match result {
            Some(milliohms) => Resistance::from_milli_ohms(milliohms),
            _ => panic!("Overflow when dividing resistance value"),
        }
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
