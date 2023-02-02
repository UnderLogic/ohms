use core::{cmp, ops};

/// Represents a resistance value, stored as whole milliohms (mΩ).
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
///    println!("{} is greater than {}", r1, r2);
/// } else {
///   println!("{} is less than or equal to {}", r1, r2);
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
    pub fn is_zero(&self) -> bool {
        self.milliohms == 0
    }

    /// Returns a `Resistance` value of zero ohms (0Ω).
    #[inline]
    pub fn zero() -> Self {
        Resistance::from_milli_ohms(0)
    }
}

impl PartialEq for Resistance {
    fn eq(&self, other: &Resistance) -> bool {
        self.milliohms == other.milliohms
    }
}

impl Eq for Resistance {}

impl PartialOrd for Resistance {
    fn partial_cmp(&self, other: &Resistance) -> Option<cmp::Ordering> {
        self.milliohms.partial_cmp(&other.milliohms)
    }
}

impl Ord for Resistance {
    fn cmp(&self, other: &Resistance) -> cmp::Ordering {
        self.milliohms.cmp(&other.milliohms)
    }
}

impl ops::Add for Resistance {
    type Output = Resistance;

    fn add(self, other: Resistance) -> Resistance {
        self.milliohms
            .checked_add(other.milliohms)
            .map(Resistance::from_milli_ohms)
            .expect("Overflow when adding resistance values")
    }
}

impl ops::Sub for Resistance {
    type Output = Resistance;

    fn sub(self, other: Resistance) -> Resistance {
        self.milliohms
            .checked_sub(other.milliohms)
            .map(Resistance::from_milli_ohms)
            .expect("Overflow when subtracting resistance values")
    }
}

impl ops::Mul<u32> for Resistance {
    type Output = Resistance;

    fn mul(self, other: u32) -> Resistance {
        self.milliohms
            .checked_mul(other)
            .map(Resistance::from_milli_ohms)
            .expect("Overflow when multiplying resistance value")
    }
}

impl ops::Mul<f32> for Resistance {
    type Output = Resistance;

    fn mul(self, other: f32) -> Resistance {
        let milliohms = self.milliohms as f32 * other;
        match milliohms {
            milliohms if milliohms.is_infinite() => {
                panic!("Infinity when multiplying resistance value")
            }
            milliohms if milliohms.is_nan() => panic!("NaN when multiplying resistance value"),
            milliohms if milliohms.is_sign_negative() => {
                panic!("Negative value when multiplying resistance value")
            }
            _ => Resistance::from_milli_ohms(milliohms as u32),
        }
    }
}

impl ops::Div<u32> for Resistance {
    type Output = Resistance;

    fn div(self, other: u32) -> Resistance {
        self.milliohms
            .checked_div(other)
            .map(Resistance::from_milli_ohms)
            .expect("Overflow when dividing resistance value")
    }
}

impl ops::Div<f32> for Resistance {
    type Output = Resistance;

    fn div(self, other: f32) -> Resistance {
        let milliohms = self.milliohms as f32 / other;
        match milliohms {
            milliohms if milliohms.is_infinite() => {
                panic!("Infinity when dividing resistance value")
            }
            milliohms if milliohms.is_nan() => panic!("NaN when dividing resistance value"),
            milliohms if milliohms.is_sign_negative() => {
                panic!("Negative value when dividing resistance value")
            }
            _ => Resistance::from_milli_ohms(milliohms as u32),
        }
    }
}

/// Extension trait for simple short-hands for creating `Resistance` values from `u32` values.
pub trait ExtU32 {
    fn milli_ohms(self) -> Resistance;
    fn ohms(self) -> Resistance;
    fn kilo_ohms(self) -> Resistance;
    fn mega_ohms(self) -> Resistance;
}

impl ExtU32 for u32 {
    /// Creates a new `Resistance` from a number of whole milliohms (mΩ).
    #[inline]
    fn milli_ohms(self) -> Resistance {
        Resistance::from_milli_ohms(self)
    }

    /// Creates a new `Resistance` from a number of whole ohms (Ω).
    #[inline]
    fn ohms(self) -> Resistance {
        Resistance::from_milli_ohms(self * 1_000)
    }

    /// Creates a new `Resistance` from a number of whole kilohms (kΩ).
    #[inline]
    fn kilo_ohms(self) -> Resistance {
        Resistance::from_milli_ohms(self * 1_000_000)
    }

    /// Creates a new `Resistance` from a number of whole megaohms (MΩ).
    #[inline]
    fn mega_ohms(self) -> Resistance {
        Resistance::from_milli_ohms(self * 1_000_000_000)
    }
}

/// Extension trait for simple short-hands for creating `Resistance` values from `f32` values.
pub trait ExtF32 {
    fn milli_ohms(self) -> Resistance;
    fn ohms(self) -> Resistance;
    fn kilo_ohms(self) -> Resistance;
    fn mega_ohms(self) -> Resistance;
}

impl ExtF32 for f32 {
    /// Creates a new `Resistance` from a number of fractional milliohms (mΩ).
    ///
    /// The fractional part is rounded down to the nearest whole milliohm (mΩ).
    #[inline]
    fn milli_ohms(self) -> Resistance {
        Resistance::from_milli_ohms(self as u32)
    }

    /// Creates a new `Resistance` from a number of fractional ohms (Ω).
    ///
    /// The fractional part is rounded down to the nearest whole milliohm (mΩ).
    #[inline]
    fn ohms(self) -> Resistance {
        Resistance::from_milli_ohms((self * 1_000f32) as u32)
    }

    /// Creates a new `Resistance` from a number of fractional kilohms (kΩ).
    ///
    /// The fractional part is rounded down to the nearest whole milliohm (mΩ).
    #[inline]
    fn kilo_ohms(self) -> Resistance {
        Resistance::from_milli_ohms((self * 1_000_000f32) as u32)
    }

    /// Creates a new `Resistance` from a number of fractional megaohms (MΩ).
    ///
    /// The fractional part is rounded down to the nearest whole milliohm (mΩ).
    #[inline]
    fn mega_ohms(self) -> Resistance {
        Resistance::from_milli_ohms((self * 1_000_000_000f32) as u32)
    }
}
