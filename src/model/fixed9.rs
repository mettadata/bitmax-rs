use failure::Fallible;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

// Fixed9 represents a fixed-point number with precision 10^-9
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Debug)]
pub struct Fixed9(pub i64);

pub const FIXED9_DECIMALS: i64 = 1_000_000_000; // 10^9
pub const MULT_TABLE: &[i64] = &[
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
];

impl Fixed9 {
    pub fn decimal(self) -> i64 {
        self.0 / FIXED9_DECIMALS
    }
}

impl fmt::Display for Fixed9 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}",
            self.0 / FIXED9_DECIMALS,
            self.0 % FIXED9_DECIMALS
        )
    }
}

impl Add for Fixed9 {
    type Output = Fixed9;

    fn add(self, rhs: Self) -> Self::Output {
        Fixed9(self.0 + rhs.0)
    }
}

impl AddAssign for Fixed9 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Fixed9 {
    type Output = Fixed9;

    fn sub(self, rhs: Self) -> Self::Output {
        Fixed9(self.0 - rhs.0)
    }
}

impl SubAssign for Fixed9 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul<i64> for Fixed9 {
    type Output = Fixed9;

    fn mul(self, rhs: i64) -> Self::Output {
        Fixed9(self.0 * rhs)
    }
}

impl MulAssign<i64> for Fixed9 {
    fn mul_assign(&mut self, rhs: i64) {
        self.0 *= rhs
    }
}

impl Serialize for Fixed9 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let abs_val = self.0.abs();
        let decimals = abs_val / FIXED9_DECIMALS;
        let residual = abs_val % FIXED9_DECIMALS;

        let sign = if self.0 < 0 { "-" } else { "" };

        let v = format!("{}{}.{:09}", sign, decimals, residual);

        serializer.serialize_str(&v)
    }
}

pub struct Fixed9Visitor {}

impl<'de> Visitor<'de> for Fixed9Visitor {
    type Value = Fixed9;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string with a fixed-point float with 10^-9 precision")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(|e| E::custom(e))
    }
}

impl<'de> Deserialize<'de> for Fixed9 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Fixed9Visitor {})
    }
}

impl FromStr for Fixed9 {
    type Err = failure::Error;

    fn from_str(v: &str) -> Fallible<Self> {
        let (v, sign): (&str, i64) = if v.starts_with('-') {
            (&v[1..], -1)
        } else {
            (v, 1)
        };

        let mut parts = v.split('.');

        // Unwrap is safe here bc split always produces at least one value
        let decimals: i64 = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|_| failure::format_err!("couldn't parse decimal part of fixed9 value"))?;
        let residual: i64 = match parts.next() {
            Some(value) => {
                if value == "" {
                    0
                } else {
                    let value = if value.len() > 9 { &value[..9] } else { value };

                    value.parse::<i64>().map_err(|_| {
                        failure::format_err!("couldn't parse float part of fixed9 value")
                    })? * MULT_TABLE[9 - value.len()]
                }
            }
            None => 0,
        };

        Ok(Fixed9(sign * (decimals * FIXED9_DECIMALS + residual)))
    }
}
