use chrono::{Datelike, Utc};
use std::cmp::Ordering;
use std::str::FromStr;

fn unix_time(date: &Date) -> u64 {
    todo!()
}

#[derive(Debug)]
pub enum DateScope {
    Day,   // dd/mm/yyyy
    Month, // mm/yyyy
    Year,  // yyyy
}

#[derive(Debug)]
pub struct Date {
    scope: DateScope,
    year: u32,
    month: Option<u8>,
    day: Option<u8>,
}

impl Date {
    pub fn new(date_str: String) -> Result<Date, String> {
        // delimiter is either / or -

        let mut delim = 'n'; // n for none

        if date_str.contains("-") && date_str.contains("/") {
            return Err(format!(
                "Ambiguous delimiter in date {}, must be / or -",
                date_str
            ));
        } else if date_str.contains("-") {
            delim = '-';
        } else if date_str.contains("/") {
            delim = '/';
        }

        let mut year: u32 = 0;
        let mut month: Option<u8> = None;
        let mut day: Option<u8> = None;
        let mut scope: DateScope = DateScope::Year;

        if delim == 'n' {
            match u32::from_str(&date_str) {
                Ok(res) => year = res,
                Err(err) => {
                    return Err(format!("Could not parse {} as a year. ({})", date_str, err));
                }
            }
            scope = DateScope::Year
        } else {
            let comps = date_str.split(delim).collect::<Vec<&str>>();

            if comps.len() == 2 {
                scope = DateScope::Month;

                match u8::from_str(comps[0]) {
                    Ok(res) => month = Some(res),
                    Err(err) => {
                        return Err(format!(
                            "Could not parse first part of {} as a month. ({})",
                            date_str, err
                        ));
                    }
                }

                match u32::from_str(comps[1]) {
                    Ok(res) => year = res,
                    Err(err) => {
                        return Err(format!(
                            "Could not parse second part of {} as a year. ({})",
                            date_str, err
                        ));
                    }
                }
            } else if comps.len() == 3 {
                scope = DateScope::Day;

                match u8::from_str(comps[0]) {
                    Ok(res) => day = Some(res),
                    Err(err) => {
                        return Err(format!(
                            "Could not parse first part of {} as a day. ({})",
                            date_str, err
                        ));
                    }
                }

                match u8::from_str(comps[1]) {
                    Ok(res) => month = Some(res),
                    Err(err) => {
                        return Err(format!(
                            "Could not parse second part of {} as a month. ({})",
                            date_str, err
                        ));
                    }
                }

                match u32::from_str(comps[2]) {
                    Ok(res) => year = res,
                    Err(err) => {
                        return Err(format!(
                            "Could not parse third part of {} as a year. ({})",
                            date_str, err
                        ));
                    }
                }
            } else {
                return Err(format!(
                    "Could not split date {} into parts correctly using delimiter {}",
                    date_str, delim
                ));
            }
        }

        if year > 100 {
            if Utc::now().year() as u32 > (2000 + year) {
                year += 1900;
            } else {
                year += 2000;
            }
        }

        Ok(Date {
            scope,
            year,
            month,
            day,
        }
        .validate()?)
    }

    pub fn validate(self) -> Result<Self, String> {
        let _31_days: Vec<u8> = vec![1, 3, 5, 6, 7, 10, 12];
        let _30_days: Vec<u8> = vec![4, 6, 9, 11];

        if self.month.is_some() {
            let _month = self.month.unwrap();

            if _month < 1 || _month > 12 {
                return Err(format!("{} is not a valid month number.", _month));
            }
        }

        if self.day.is_some() {
            let _days = self.day.unwrap();
            let _month = self.month.unwrap();

            if (_31_days.contains(&_month) && _days > 31)
                || (_30_days.contains(&_month) && _days > 30)
                || (_month == 2 && (_days != 28 || _days != 29))
            {
                return Err(format!(
                    "{} is not a valid number of days for month {}.",
                    self.format(),
                    self.month.unwrap()
                ));
            }
        }

        Ok(self)
    }

    fn format(&self) -> String {
        let mut buf: String = String::new();

        if self.day.is_some() {
            buf += (self.day.unwrap().to_string() + "/").as_str();
        }

        if self.month.is_some() {
            buf += (self.month.unwrap().to_string() + "/").as_str();
        }

        buf += self.year.to_string().as_str();

        buf
    }

    fn has_equal_scope(&self, other: &Self) -> bool {
        let mut eq = true;

        if (self.month.is_some() || other.month.is_some())
            && (self.month.is_none() || other.month.is_none())
        {
            eq = false;
        }

        if (self.day.is_some() || other.day.is_some()) && (self.day.is_none() || other.day.is_none())
        {
            eq = false;
        }

        eq
    }
}

impl PartialEq<Self> for Date {
    fn eq(&self, other: &Self) -> bool {
        let mut eq = true;

        if self.has_equal_scope(other) {
            if self.year != other.year {
                eq = false;
            }

            if self.month.is_some() {
                if self.month.unwrap() != other.month.unwrap() {
                    eq = false;
                }
            }

            if self.day.is_some() {
                if self.day.unwrap() != other.day.unwrap() {
                    eq = false;
                }
            }

            eq
        } else {
            false
        }
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self > other {
            Some(Ordering::Greater)
        } else if self < other {
            Some(Ordering::Less)
        } else {
            None
        }
    }
    /*
    if the full scope is not provided the earliest date is assumed,
    e.g. given just 2000, for comparison purposes it is assumed that is it 01-01-2000
    so 2000 < 01-02-2000
    */
    fn lt(&self, other: &Self) -> bool {
        if unix_time(self) < unix_time(other) {
            true
        } else {
            false
        }
    }

    fn le(&self, other: &Self) -> bool {
        if unix_time(self) <= unix_time(other) {
            true
        } else {
            false
        }
    }

    fn gt(&self, other: &Self) -> bool {
        if unix_time(self) < unix_time(other) {
            true
        } else {
            false
        }
    }

    fn ge(&self, other: &Self) -> bool {
        if unix_time(self) <= unix_time(other) {
            true
        } else {
            false
        }
    }
}
