use std::cmp::Ordering;

pub struct Date {
    year: Option<u32>,
    month: Option<u8>,
    day: Option<u8>
}

impl Date {
    fn new(date_str: String) -> Result<Date, String> {
        Ok(Date{year: None, month: None, day: None})
    }

    fn has_equal_scope(&self, other: &Self) -> bool {
        let mut eq = true;

        if (self.year.is_some() || self.year.is_some()) && (self.year.is_none() || self.year.is_none()) {
            eq = false;
        }

        if (self.month.is_some() || self.month.is_some()) && (self.month.is_none() || self.month.is_none()) {
            eq = false;
        }

        if (self.day.is_some() || self.day.is_some()) && (self.day.is_none() || self.day.is_none()) {
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