use std::fmt;
use std::str::FromStr;
use std::default;
use std::marker::PhantomData;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct HexNum(String);

impl<'de> Deserialize<'de> for HexNum {
    fn deserialize<D>(deserializer: D) -> Result<HexNum, D::Error>
    where
        D: Deserializer<'de>,
    {
        // This is a Visitor that forwards string types to T's `FromStr` impl and
        // forwards number types to T's `Deserialize` impl. The `PhantomData` is to
        // keep the compiler from complaining about T being an unused generic type
        // parameter. We need T in order to know the Value type for the Visitor
        // impl.
        struct HexNum<T>(PhantomData<fn() -> T>);

        impl<'de, T> Visitor<'de> for HexNum<T>
        where
            T: Deserialize<'de>
                + FromStr<Err = <String as FromStr>::Err>
                + From<u64>
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("hex value as string or number")
            }

            fn visit_str<E>(self, value: &str) -> Result<T, E>
            where
                E: de::Error,
            {
                Ok(FromStr::from_str(value).unwrap())
            }

            fn visit_u64<E>(self, value: u64) -> Result<T, E>
            where
                E: de::Error,
            {
                Ok(From::from(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<T, E>
            where
                E: de::Error,
            {
                Ok(From::from(value as u64))
            }

            fn visit_unit<E>(self) -> Result<T,E>
            where
                E: de::Error,
            {
                Ok(From::from(0))
            }
        }

        deserializer.deserialize_any(HexNum(PhantomData))
    }
}

impl FromStr for HexNum {
    type Err = <String as FromStr>::Err;

    fn from_str(s: &str) -> Result<HexNum, Self::Err> {
        Ok(HexNum { 0: s.to_string() })
    }
}


impl From<u64> for HexNum {
    fn from(val: u64) -> HexNum {
        HexNum { 0: format!("{}", val) }
    }
}

// Manual implementation of Display trait for nice printouts
impl fmt::Display for HexNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Implement default instantiation
impl default::Default for HexNum {
    fn default() -> HexNum {
        HexNum{ 0: "0".to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Debug)]
    struct HexNums {
        x: HexNum,
        y: HexNum,
        z: HexNum,
    }

    #[test]
    fn convert_hex_num() {
        let str_num = "2".to_string();
        let num: f64 = str_num.parse().unwrap();
        assert_eq!(num, 2.0);

        let json: serde_json::Value =
            serde_json::from_str("{\"x\":\"1a\",\"y\":3,\"z\":null}").unwrap();
        let h_num: HexNums = serde_json::from_value(json).unwrap();
        assert_eq!(&h_num.x.0, "1a");
        assert_eq!(&h_num.y.0, "3");
        assert_eq!(&h_num.z.0, "0");
    }
}
