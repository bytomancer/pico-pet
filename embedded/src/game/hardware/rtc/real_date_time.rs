use core::cmp;
use core::ops::Sub;

use super::interval_date_time::IntervalDateTime;
use super::month::Month;
use super::real_date::RealDate;
use super::real_time::RealTime;

#[allow(unused)]
#[derive(Copy, Clone)]
pub struct RealDateTime {
    pub time: RealTime,
    pub date: RealDate,
}
impl RealDateTime {
    #[allow(unused)]
    pub fn new(time: RealTime, date: RealDate) -> Self {
        Self { time, date }
    }

    pub fn to_y2k_epoch(&self) -> u32 {
        let mut seconds_from_previous_months = 0;
        for month in 1..self.date.month {
            let days_in_month = Month::from(month).days_in_month(self.date.year_since_2k);
            seconds_from_previous_months += days_in_month as u32 * (60 * 60 * 24);
        }
        self.time.sec as u32
            + self.time.min as u32 * 60
            + self.time.hr as u32 * (60 * 60)
            + self.date.day_of_month as u32 * (60 * 60 * 24)
            + seconds_from_previous_months
    }

    pub fn dec_by_1_hour(&mut self) {
        if self.time.hr > 0 {
            self.time.hr -= 1;
        } else {
            self.time.hr = 23;
            self.date.dec_by_1_day();
        }
    }

    // pub fn inc_by_1_day(&mut self) {
    //     self.date.inc_by_1_day();
    // }

    pub fn to_fixed_str(&self) -> fixedstr::str32 {
        fixedstr::str_format!(
            fixedstr::str32,
            "{} {}",
            self.date.yyyy_mm_dd_str(),
            self.time.hh_mm_ss_str()
        )
    }
}
impl PartialEq for RealDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.date == other.date
    }
}
impl PartialOrd for RealDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let my_epoch = self.to_y2k_epoch();
        let other_epoch = other.to_y2k_epoch();
        Some(my_epoch.cmp(&other_epoch))
        // if self.date == other.date {
        //     if self.time == other.time {
        //         Some(cmp::Ordering::Equal)
        //     } else if self.time > other.time {
        //         Some(cmp::Ordering::Greater)
        //     } else {
        //         Some(cmp::Ordering::Less)
        //     }
        // } else if self.date > other.date {
        //     Some(cmp::Ordering::Greater)
        // } else {
        //     Some(cmp::Ordering::Less)
        // }
    }
}
// impl Sub for RealDateTime {
//     type Output = i32;
//     fn sub(self, other: Self) -> i32 {
//         let my_epoch = self.to_y2k_epoch() as i32;
//         let other_epoch = other.to_y2k_epoch() as i32;
//         my_epoch - other_epoch
//     }
// }
impl Sub for RealDateTime {
    type Output = IntervalDateTime;
    fn sub(self, other: Self) -> IntervalDateTime {
        let my_epoch = self.to_y2k_epoch() as i32;
        let other_epoch = other.to_y2k_epoch() as i32;
        IntervalDateTime::from_epoch(my_epoch - other_epoch)
    }
}
