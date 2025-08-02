use chrono::prelude::*;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if NaiveDate::from_ymd_opt(year as i32, 2, 29).is_some() {
        return None;
    }
    let mid_day_ordinal = 183;
    let date = NaiveDate::from_yo_opt(year as i32, mid_day_ordinal).unwrap();
    Some(date.weekday())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//    use chrono::Weekday;


// #[test]
// fn leap_years() {
//     assert!(middle_day(1892).is_none(), "1892 was a leap year!");
//     assert!(middle_day(1904).is_none(), "1904 was a leap year!");
//     assert!(middle_day(2012).is_none(), "2012 was a leap year!");
// }

// #[test]
// fn weekdays() {
//     assert_eq!(Weekday::Tue, middle_day(2019).unwrap());
//     assert_eq!(Weekday::Wed, middle_day(1997).unwrap());
//     assert_eq!(Weekday::Mon, middle_day(1663).unwrap());
//     assert_eq!(Weekday::Wed, middle_day(1873).unwrap());
//     assert_eq!(Weekday::Thu, middle_day(1953).unwrap());
//     assert_eq!(Weekday::Wed, middle_day(1879).unwrap());
// }
// }
