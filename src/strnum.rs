use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor};


#[derive(Debug, Deserialize)]
pub struct FloatOrString {
    pub val: f64
}

// The `string_or_num` function uses this impl to instantiate a `FloatOrString` if
// the input file contains a string and not a number. 
impl FromStr for FloatOrString {
    type Err = <f64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<FloatOrString, Self::Err> {
        if s=="" {
            Ok(FloatOrString{val: 0.0})
        } else {
            match s.parse::<f64>() {
                Ok(num) => Ok(FloatOrString {val: num}),
                Err(e) => Err(e)
            }
        }
    }
}

impl From<u64> for FloatOrString {
    fn from(val: u64) -> FloatOrString {
        FloatOrString{val: val as f64}
    }
}

impl From<i64> for FloatOrString {
    fn from(val: i64) -> FloatOrString {
        FloatOrString{val: val as f64}
    }
}

impl From<f64> for FloatOrString {
    fn from(val: f64) -> FloatOrString {
        FloatOrString{val: val}
    }
}

pub fn string_or_num<'de, D>(deserializer: D) -> Result<FloatOrString, D::Error>
where
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards number types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrNum<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrNum<T>
    where
        T: Deserialize<'de> + FromStr<Err = <f64 as FromStr>::Err> + From<u64> + From<i64> + From<f64>
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or number")
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
            Ok(From::from(value))
        }

        fn visit_f64<E>(self, value: f64) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(From::from(value))
        }    }

    deserializer.deserialize_any(StringOrNum(PhantomData))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Debug)]
    struct DoubleNum {
        #[serde(deserialize_with = "string_or_num")]
        x: FloatOrString,
        #[serde(deserialize_with = "string_or_num")]
        y: FloatOrString,
        #[serde(deserialize_with = "string_or_num")]
        z: FloatOrString,
    }

    #[test]
    fn convert_str_num() {
        let str_num = "2".to_string();
        let num : f64 = str_num.parse().unwrap();
        assert_eq!(num, 2.0);

        let json: serde_json::Value = serde_json::from_str("{\"x\":\"2.1\",\"y\":3,\"z\":3.4}").unwrap();
        let d_num: DoubleNum  = serde_json::from_value(json).unwrap();
        assert_eq!(d_num.x.val, 2.1);
        assert_eq!(d_num.y.val, 3.0);
        assert_eq!(d_num.z.val, 3.4);
    }
}