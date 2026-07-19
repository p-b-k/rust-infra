////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Timedata: Time, Day and Timespan structs defined
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
use serde::{Deserialize, Serialize};

use log::error;
use mysql::{FromValueError, Value, prelude::FromValue};
use std::str::FromStr;
use time::Weekday;

// Start with Day
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Day {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Day {
    pub fn to_string(&self) -> String {
        let year = self.year;
        let month = self.month;
        let day = self.day;

        format!("{year:04}{month:02}{day:02}")
    }
}

impl FromStr for Day {
    type Err = String;

    fn from_str(instr: &str) -> Result<Day, Self::Err> {
        if instr.len() != 8 {
            Err(format!("String must be exactly 8 characters long"))
        } else {
            if instr
                .char_indices()
                .fold(true, |acc, (_, c)| acc && c.is_digit(10))
            {
                let year_str = &instr[0..4];
                let month_str = &instr[4..6];
                let day_str = &instr[6..8];

                let year: u16 = year_str.parse().expect("unable to parse year");
                let month: u8 = month_str.parse().expect("unable to parse month");
                let day: u8 = day_str.parse().expect("unable to parse day");

                Ok(Day { year, month, day })
            } else {
                Err(format!("String Non-numeric characters in string"))
            }
        }
    }
}

impl From<String> for Day {
    fn from(value: String) -> Day {
        Day::from_str(value.as_str())
            .expect(format!("Unable to read Day value from {value}").as_str())
    }
}

impl FromValue for Day {
    type Intermediate = String;

    fn from_value_opt(v: Value) -> Result<Self, FromValueError> {
        match v {
            Value::Bytes(b) => {
                let s: String = String::from_utf8(b).expect("Unable to parse time string");
                Ok(Day::from_str(s.as_str()).expect("Unable to parse time"))
            }
            _ => Err(FromValueError(v)),
        }
    }
}

#[cfg(test)]
mod day_tests {
    use super::*;

    #[test]
    fn read_good_value() {
        let v1 = Day::from_str("20260618").unwrap();

        assert_eq!(v1.year, 2026);
        assert_eq!(v1.month, 6);
        assert_eq!(v1.day, 18);
    }

    #[test]
    fn read_bad_value() {
        let v1 = Day::from_str("2x000030");

        match v1 {
            Ok(_) => {
                panic!("Shouldn't be able to read a value from \"2x000030\"");
            }
            _ => {}
        }
    }

    #[test]
    fn write_short_string() {
        assert_eq!(Day::from_str("20260618").unwrap().to_string(), "20260618");
    }
}

// Start with Time
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
}

impl Time {
    pub fn to_string(&self) -> String {
        let hour = self.hour;
        let minute = self.minute;

        format!("{hour:02}{minute:02}")
    }
}

impl FromStr for Time {
    type Err = String;

    fn from_str(instr: &str) -> Result<Time, Self::Err> {
        if instr.len() != 4 {
            Err(format!("String must be exactly 4 characters long"))
        } else {
            if instr
                .char_indices()
                .fold(true, |acc, (_, c)| acc && c.is_digit(10))
            {
                let hour_str = &instr[0..2];
                let minute_str = &instr[2..4];

                let hour: u8 = hour_str.parse().expect("unable to parse hour");
                let minute: u8 = minute_str.parse().expect("unable to parse minute");

                if hour < 24 {
                    if minute < 60 {
                        Ok(Time { hour, minute })
                    } else {
                        Err(format!("Minute value ({minute}) out of range (0 - 59)"))
                    }
                } else {
                    Err(format!("Hour value ({hour}) out of range (0 - 23)"))
                }
            } else {
                Err(format!("String Non-numeric characters in string"))
            }
        }
    }
}

impl From<String> for Time {
    fn from(value: String) -> Time {
        value.parse().unwrap()
    }
}

impl From<u16> for Time {
    fn from(value: u16) -> Time {
        if value < 60 * 24 {
            let hour = (value / 60) as u8;
            let minute = (value % 60) as u8;
            Time { hour, minute }
        } else {
            panic!("time value too large")
        }
    }
}

#[derive(Debug)]
pub struct TimeIr(u16);

impl From<TimeIr> for Time {
    fn from(value: TimeIr) -> Time {
        Time::from(value.0)
    }
}

impl TryFrom<Value> for TimeIr {
    type Error = FromValueError;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::Int(i) => {
                if i < 0 {
                    Err(FromValueError(v))
                } else if i < 24 * 60 {
                    Ok(TimeIr(i as u16))
                } else {
                    Err(FromValueError(v))
                }
            }
            _ => Err(FromValueError(v)),
        }
    }
}

impl From<TimeIr> for Value {
    fn from(ir: TimeIr) -> Self {
        Value::Int(ir.0 as i64)
    }
}

impl FromValue for Time {
    type Intermediate = TimeIr;

    fn from_value(v: Value) -> Self {
        Self::from_value_opt(v).unwrap()
    }

    fn from_value_opt(v: Value) -> Result<Self, FromValueError> {
        let err_cp = v.clone();
        match TimeIr::try_from(v) {
            Ok(t) => {
                let hour = t.0 as u8 / 60;
                let minute = t.0 as u8 % 60;
                Ok(Time { hour, minute })
            }
            Err(e) => {
                error!("MySql Read error: {e}");
                Err(FromValueError(err_cp))
            }
        }
    }
}

#[cfg(test)]
mod time_tests {
    use super::*;

    #[test]
    fn read_good_value_u16() {
        let v1 = Time::from((13 * 60) + 30); // 1:30 PM

        assert_eq!(v1.hour, 13);
        assert_eq!(v1.minute, 30);
    }

    #[test]
    fn read_good_value_str() {
        let v1 = Time::from("1430".to_string()); // 1:30 PM

        assert_eq!(v1.hour, 14);
        assert_eq!(v1.minute, 30);
    }

    #[test]
    fn read_bad_value() {
        let v1 = Time::from_str("2x00");

        match v1 {
            Ok(_) => {
                panic!("Shouldn't be able to read a value from \"2x00\"");
            }
            _ => {}
        }
    }

    #[test]
    fn write_short_string() {
        assert_eq!(Time::from_str("2026").unwrap().to_string(), "2026");
    }
}

// Define for Weekday
struct WeekdayIr(String);

impl From<String> for WeekdayIr {
    fn from(s: String) -> WeekdayIr {
        WeekdayIr { 0: s }
    }
}

impl From<WeekdayIr> for Weekday {
    fn from(value: WeekdayIr) -> Weekday {
        match value.0.as_str() {
            "Sunday" => Weekday::Sunday,
            "Monday" => Weekday::Monday,
            "Tuesday" => Weekday::Tuesday,
            "Wednesday" => Weekday::Wednesday,
            "Thursday" => Weekday::Thursday,
            "Friday" => Weekday::Friday,
            "Saturday" => Weekday::Saturday,
            _ => {
                panic!("Unable to parse weekday {}", value.0);
            }
        }
    }
}

impl FromValue for WeekdayIr {
    type Intermediate = String;

    fn from_value_opt(v: Value) -> Result<Self, FromValueError> {
        match v {
            Value::Bytes(b) => {
                let s: String = String::from_utf8(b).expect("Unable to parse time string");
                Ok(Day::from_str(s.as_str()).expect("Unable to parse time"))
            }
            _ => Err(FromValueError(v)),
        }
    }
}
