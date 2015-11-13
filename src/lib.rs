extern crate num;
use num::BigInt;
use num::bigint::ToBigInt;
use num::bigint::{Sign, BigDigit};
use num::rational::{Ratio, BigRational};
use num::traits::{ToPrimitive};

#[derive(Copy, Clone, Debug)]
pub struct Decimal {
    sign: Sign,
    inv_exp: u8,
    mantissa: [BigDigit; 3],
}

impl Decimal {
    pub fn from_i32(value: i32, inverse_exponent: u8) -> Decimal {
        Decimal {
            sign: match i32::signum(value) {
                1 => Sign::Plus,
                -1 => Sign::Minus,
                _ => Sign::NoSign,
            },
            inv_exp: inverse_exponent,
            mantissa: [value.abs() as BigDigit, 0, 0],
        }
    }

    /// Constructs a number representing `value * 10^(-inv_exp)`
    /// Fails if value is larger than 96 bits
    pub fn from_bigint(value: &BigInt, inverse_exponent: u8) -> Option<Decimal> {
        let (sign, bytes) = value.to_bytes_le();
        if bytes.len() > 12 {
            return None;
        }

        let mut value = Decimal {
            sign: sign,
            inv_exp: inverse_exponent,
            mantissa: [0; 3],
        };

        // Shift all of the bytes into the mantissa
        // to_bytes_le() guarantees them to be in little endian
        for (i, x) in bytes.into_iter().enumerate() {
            let shift = (i as BigDigit % 4) * 8;
            let byte_val = x as BigDigit;
            let shifted = byte_val << shift;
            value.mantissa[i / 4] |= shifted;
        }

        // swap the bytes if we're on a big-endian system
        // this should be optimized to a no-op on little-endian systems
        for val in value.mantissa.iter_mut() {
            *val = u32::from_le(*val);
        }

        Some(value)
    }

    pub fn to_bigrational(&self) -> BigRational {
        // The value is sign*mantissa/(10^inv_exp)
        Ratio::new_raw(
            BigInt::from_slice(self.sign, &self.mantissa),
            num::pow(10u32.to_bigint().unwrap(), self.inv_exp as usize)
        )
    }

    pub fn to_f64(&self) -> f64 {
        let ratio = self.to_bigrational();
        ratio.numer().to_f64().unwrap() / ratio.denom().to_f64().unwrap()
    }
}


