////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// A Verision type that can be easily sorted and persisted in a sortable manner
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
use serde::{Deserialize, Serialize};

use mysql::{FromValueError, Value, prelude::FromValue};

use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Version {
    pub maj: u16,
    pub min: u16,
    pub rel: u16,
    pub bld: u16,
}

impl Version {
    pub fn from_string(instr: &str) -> Result<Version, String> {
        let mut maj: u16 = 0;
        let mut min: u16 = 0;
        let mut rel: u16 = 0;
        let mut bld: u16 = 0;

        let components: Vec<_> = instr.split(".").collect();
        for i in 0..components.len() {
            let next: u16;

            match components[i].parse() {
                Ok(r) => {
                    next = r;
                }
                Err(msg) => {
                    return Err(format!("Error: {msg}"));
                }
            }

            match i {
                0 => maj = next,
                1 => min = next,
                2 => rel = next,
                3 => bld = next,
                _ => {
                    return Err("Too many fields in the string".to_string());
                }
            }
        }

        Ok(Version { maj, min, rel, bld })
    }

    pub fn to_short_string(&self) -> String {
        if self.bld > 0 {
            format!("{}.{}.{}.{}", self.maj, self.min, self.rel, self.bld)
        } else {
            if self.rel > 0 {
                format!("{}.{}.{}", self.maj, self.min, self.rel)
            } else {
                if self.min > 0 {
                    format!("{}.{}", self.maj, self.min)
                } else {
                    format!("{}", self.maj)
                }
            }
        }
    }

    pub fn to_sort_string(&self) -> String {
        let maj = self.maj;
        let min = self.min;
        let rel = self.rel;
        let bld = self.bld;

        format!("{maj:06}.{min:06}.{rel:06}.{bld:06}")
    }
}

impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> Result<Version, Self::Err> {
        Version::from_string(s)
    }
}

impl From<String> for Version {
    fn from(value: String) -> Version {
        Version::from_string(value.as_str())
            .expect(format!("Unable to read Version value from {value}").as_str())
    }
}

impl FromValue for Version {
    type Intermediate = String;

    fn from_value_opt(v: Value) -> Result<Self, FromValueError> {
        match v {
            Value::Bytes(b) => {
                let s: String = String::from_utf8(b).expect("Unable to parse version string");
                Ok(Version::from_string(s.as_str()).expect("Unable to parse version"))
            }
            _ => Err(FromValueError(v)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_good_value() {
        let v1 = Version::from_string("2.3").unwrap();

        assert_eq!(v1.maj, 2);
        assert_eq!(v1.min, 3);
        assert_eq!(v1.rel, 0);
        assert_eq!(v1.bld, 0);
    }

    #[test]
    fn read_bad_value() {
        let v1 = Version::from_string("2.x");

        match v1 {
            Ok(_) => {
                panic!("Shouldn't be able to read a value from \"2.x\"");
            }
            _ => {}
        }
    }

    #[test]
    fn write_short_string() {
        assert_eq!(
            Version::from_string("3.4.0.1").unwrap().to_short_string(),
            "3.4.0.1"
        );
        assert_eq!(
            Version::from_string("3.4.0.0").unwrap().to_short_string(),
            "3.4"
        );
        assert_eq!(
            Version::from_string("0.0.0.1").unwrap().to_short_string(),
            "0.0.0.1"
        );
        assert_eq!(
            Version::from_string("0.4.0.1").unwrap().to_short_string(),
            "0.4.0.1"
        );
    }

    #[test]
    fn write_sort_string() {
        assert_eq!(
            Version::from_string("3.4.0.1").unwrap().to_sort_string(),
            "000003.000004.000000.000001"
        );
        assert_eq!(
            Version::from_string("3.4.0.0").unwrap().to_sort_string(),
            "000003.000004.000000.000000"
        );
        assert_eq!(
            Version::from_string("0.0.0.1").unwrap().to_sort_string(),
            "000000.000000.000000.000001"
        );
        assert_eq!(
            Version::from_string("0.4.0.1").unwrap().to_sort_string(),
            "000000.000004.000000.000001"
        );
    }
}
