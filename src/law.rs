use crate::{Current, Resistance, Voltage};
use core::ops;

impl ops::Div<Resistance> for Voltage {
    type Output = Current;

    /// Calculates the current through a resistive load given the voltage across it.
    ///
    /// Will be rounded down to the nearest whole microamp (μA).
    /// Panics if the resistance is zero.
    fn div(self, resistance: Resistance) -> Self::Output {
        if resistance.is_zero() {
            panic!("Resistance cannot be zero, infinite current would result");
        }

        let micro_volts = self.micro_volts().unsigned_abs();

        let nano_volts = (micro_volts as u64)
            .checked_mul(1000u64)
            .expect("Voltage would overflow");

        let micro_amps = nano_volts
            .checked_div(resistance.milli_ohms() as u64)
            .expect("Current would overflow");

        Current::from_micro_amps(micro_amps as u32)
    }
}

impl ops::Mul<Resistance> for Current {
    type Output = Voltage;

    /// Calculates the voltage across a resistive load given the current through it.
    ///
    /// Will be rounded down to the nearest whole microvolt (μV).
    fn mul(self, resistance: Resistance) -> Self::Output {
        let micro_amps = self.micro_amps();
        let nano_volts = micro_amps
            .checked_mul(resistance.milli_ohms())
            .expect("Voltage would overflow");

        let micro_volts = nano_volts / 1000u32;
        Voltage::from_micro_volts(micro_volts as i32)
    }
}

impl ops::Mul<Current> for Resistance {
    type Output = Voltage;

    /// Calculates the voltage across a resistive load given the current through it.
    ///
    /// Will be rounded down to the nearest whole microvolt (μV).
    fn mul(self, current: Current) -> Self::Output {
        current * self
    }
}

impl ops::Div<Current> for Voltage {
    type Output = Resistance;

    /// Calculates the resistance of a resistive load given the voltage across it and the current.
    ///
    /// Will be rounded down to the nearest whole milliohm (mΩ).
    /// Panics if the current is zero.
    fn div(self, current: Current) -> Self::Output {
        if current.is_zero() {
            panic!("Current cannot be zero, infinite resistance would result");
        }

        let micro_volts = self.abs().micro_volts() as u32;
        let micro_ohms = micro_volts
            .checked_div(current.micro_amps())
            .expect("Resistance would overflow");

        let milli_ohms = micro_ohms / 1000u32;
        Resistance::from_milli_ohms(milli_ohms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5_000_000, 100_000, 50_000; "positive 5V, 100Ω equals 50,000μA")]
    #[test_case(-5_000_000, 100_000, 50_000; "negative 5V, 100Ω equals 50,000μA")]
    #[test_case(3_300_000, 4_700_000, 702; "positive 3.3V, 4.7kΩ equals 702μA")]
    fn test_current_equals_voltage_over_resistance(
        micro_volts: i32,
        milli_ohms: u32,
        expected_micro_amps: u32,
    ) {
        let v = Voltage::from_micro_volts(micro_volts);
        let r = Resistance::from_milli_ohms(milli_ohms);
        let current = v / r;

        assert_eq!(current.micro_amps(), expected_micro_amps);
    }
}
