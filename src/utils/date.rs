use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug)]
pub enum DateScope {
    Day, // dd/mm/yyyy
    Month, // mm/yyyy
    Year, // yyyy
}

#[derive(Debug)]
pub struct Date {
    scope: DateScope,
    year: Option<u32>,
    month: Option<u8>,
    day: Option<u8>,
}

impl Date {
    pub(crate) fn new(date_str: String) -> Result<Date, String> {
        // delimiter is either / or -

        let mut delim = 'n'; // n for none

        if date_str.contains("-") && date_str.contains("/") {
            return Err(format!("Ambiguous delimiter in date {}, must be / or -", date_str))
        } else if date_str.contains("-") {
            delim = '-';
        } else if date_str.contains("/") {
            delim = '/';
        }

        let mut year: Option<u32> = None;
        let mut month: Option<u8> = None;
        let mut day: Option<u8> = None;
        let mut scope: DateScope = DateScope::Year;

        if delim == 'n' {
            match u32::from_str(&date_str) {
                Ok(res) => year = Some(res),
                Err(err) => return Err(format!("Could not parse {} as a year. ({})", date_str, err))
            }
            scope = DateScope::Year
        } else {
            let comps = date_str.split(delim).collect::<Vec<&str>>();

            if comps.len() == 2 {
                scope = DateScope::Month;

                match u8::from_str(comps[0]) {
                    Ok(res) => month = Some(res),
                    Err(err) => return Err(format!("Could not parse first part of {} as a month. ({})", date_str, err))
                }

                match u32::from_str(comps[1]) {
                    Ok(res) => year = Some(res),
                    Err(err) => return Err(format!("Could not parse second part of {} as a year. ({})", date_str, err))
                }
            } else if comps.len() == 3 {

                scope = DateScope::Day;

                match u8::from_str(comps[0]) {
                    Ok(res) => day = Some(res),
                    Err(err) => return Err(format!("Could not parse first part of {} as a day. ({})", date_str, err))
                }

                match u8::from_str(comps[1]) {
                    Ok(res) => month = Some(res),
                    Err(err) => return Err(format!("Could not parse second part of {} as a month. ({})", date_str, err))
                }

                match u32::from_str(comps[2]) {
                    Ok(res) => year = Some(res),
                    Err(err) => return Err(format!("Could not parse third part of {} as a year. ({})", date_str, err))
                }

            } else {
                return Err(format!("Could not split date {} into parts correctly using delimiter {}", date_str, delim))
            }
        }
        
        Ok(Date {
            scope,
            year,
            month,
            day
        })
    }

    fn has_equal_scope(&self, other: &Self) -> bool {
        let mut eq = true;

        if (self.year.is_some() || self.year.is_some())
            && (self.year.is_none() || self.year.is_none())
        {
            eq = false;
        }

        if (self.month.is_some() || self.month.is_some())
            && (self.month.is_none() || self.month.is_none())
        {
            eq = false;
        }

        if (self.day.is_some() || self.day.is_some()) && (self.day.is_none() || self.day.is_none())
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
            if self.year.is_some() {
                if self.year.unwrap() != self.year.unwrap() {
                    eq = false;
                }
            }

            if self.year.is_some() {
                if self.month.unwrap() != self.month.unwrap() {
                    eq = false;
                }
            }

            if self.year.is_some() {
                if self.day.unwrap() != self.day.unwrap() {
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
        todo!()
    }

    fn lt(&self, other: &Self) -> bool {
        todo!()
    }

    fn le(&self, other: &Self) -> bool {
        todo!()
    }

    fn gt(&self, other: &Self) -> bool {
        todo!()
    }

    fn ge(&self, other: &Self) -> bool {
        todo!()
    }
}
