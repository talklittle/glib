// This file was generated by gir (81a781f) from gir-files (469db10)
// DO NOT EDIT

use TimeSpan;
use TimeZone;
use ffi;
use ffi as glib_ffi;
use gobject_ffi;
use std::cmp;
use std::mem;
use std::ptr;
use translate::*;

glib_wrapper! {
    pub struct DateTime(Shared<ffi::GDateTime>);

    match fn {
        ref => |ptr| ffi::g_date_time_ref(ptr),
        unref => |ptr| ffi::g_date_time_unref(ptr),
        get_type => || ffi::g_date_time_get_type(),
    }
}

impl DateTime {
    pub fn new(tz: &TimeZone, year: i32, month: i32, day: i32, hour: i32, minute: i32, seconds: f64) -> DateTime {
        unsafe {
            from_glib_full(ffi::g_date_time_new(tz.to_glib_none().0, year, month, day, hour, minute, seconds))
        }
    }

    //pub fn new_from_timeval_local(tv: /*Ignored*/&TimeVal) -> DateTime {
    //    unsafe { TODO: call ffi::g_date_time_new_from_timeval_local() }
    //}

    //pub fn new_from_timeval_utc(tv: /*Ignored*/&TimeVal) -> DateTime {
    //    unsafe { TODO: call ffi::g_date_time_new_from_timeval_utc() }
    //}

    pub fn new_from_unix_local(t: i64) -> DateTime {
        unsafe {
            from_glib_full(ffi::g_date_time_new_from_unix_local(t))
        }
    }

    pub fn new_from_unix_utc(t: i64) -> DateTime {
        unsafe {
            from_glib_full(ffi::g_date_time_new_from_unix_utc(t))
        }
    }

    pub fn new_local(year: i32, month: i32, day: i32, hour: i32, minute: i32, seconds: f64) -> DateTime {
        unsafe {
            from_glib_full(ffi::g_date_time_new_local(year, month, day, hour, minute, seconds))
        }
    }

    pub fn new_now(tz: &TimeZone) -> DateTime {
        unsafe {
            from_glib_full(ffi::g_date_time_new_now(tz.to_glib_none().0))
        }
    }

    pub fn new_now_local() -> DateTime {
        unsafe {
            from_glib_full(ffi::g_date_time_new_now_local())
        }
    }

    pub fn new_now_utc() -> DateTime {
        unsafe {
            from_glib_full(ffi::g_date_time_new_now_utc())
        }
    }

    pub fn new_utc(year: i32, month: i32, day: i32, hour: i32, minute: i32, seconds: f64) -> DateTime {
        unsafe {
            from_glib_full(ffi::g_date_time_new_utc(year, month, day, hour, minute, seconds))
        }
    }

    pub fn add(&self, timespan: TimeSpan) -> Option<DateTime> {
        unsafe {
            from_glib_full(ffi::g_date_time_add(self.to_glib_none().0, timespan))
        }
    }

    pub fn add_days(&self, days: i32) -> Option<DateTime> {
        unsafe {
            from_glib_full(ffi::g_date_time_add_days(self.to_glib_none().0, days))
        }
    }

    pub fn add_full(&self, years: i32, months: i32, days: i32, hours: i32, minutes: i32, seconds: f64) -> Option<DateTime> {
        unsafe {
            from_glib_full(ffi::g_date_time_add_full(self.to_glib_none().0, years, months, days, hours, minutes, seconds))
        }
    }

    pub fn add_hours(&self, hours: i32) -> Option<DateTime> {
        unsafe {
            from_glib_full(ffi::g_date_time_add_hours(self.to_glib_none().0, hours))
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Option<DateTime> {
        unsafe {
            from_glib_full(ffi::g_date_time_add_minutes(self.to_glib_none().0, minutes))
        }
    }

    pub fn add_months(&self, months: i32) -> Option<DateTime> {
        unsafe {
            from_glib_full(ffi::g_date_time_add_months(self.to_glib_none().0, months))
        }
    }

    pub fn add_seconds(&self, seconds: f64) -> Option<DateTime> {
        unsafe {
            from_glib_full(ffi::g_date_time_add_seconds(self.to_glib_none().0, seconds))
        }
    }

    pub fn add_weeks(&self, weeks: i32) -> Option<DateTime> {
        unsafe {
            from_glib_full(ffi::g_date_time_add_weeks(self.to_glib_none().0, weeks))
        }
    }

    pub fn add_years(&self, years: i32) -> Option<DateTime> {
        unsafe {
            from_glib_full(ffi::g_date_time_add_years(self.to_glib_none().0, years))
        }
    }

    pub fn difference(&self, begin: &DateTime) -> TimeSpan {
        unsafe {
            ffi::g_date_time_difference(self.to_glib_none().0, begin.to_glib_none().0)
        }
    }

    pub fn format(&self, format: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_date_time_format(self.to_glib_none().0, format.to_glib_none().0))
        }
    }

    pub fn get_day_of_month(&self) -> i32 {
        unsafe {
            ffi::g_date_time_get_day_of_month(self.to_glib_none().0)
        }
    }

    pub fn get_day_of_week(&self) -> i32 {
        unsafe {
            ffi::g_date_time_get_day_of_week(self.to_glib_none().0)
        }
    }

    pub fn get_day_of_year(&self) -> i32 {
        unsafe {
            ffi::g_date_time_get_day_of_year(self.to_glib_none().0)
        }
    }

    pub fn get_hour(&self) -> i32 {
        unsafe {
            ffi::g_date_time_get_hour(self.to_glib_none().0)
        }
    }

    pub fn get_microsecond(&self) -> i32 {
        unsafe {
            ffi::g_date_time_get_microsecond(self.to_glib_none().0)
        }
    }

    pub fn get_minute(&self) -> i32 {
        unsafe {
            ffi::g_date_time_get_minute(self.to_glib_none().0)
        }
    }

    pub fn get_month(&self) -> i32 {
        unsafe {
            ffi::g_date_time_get_month(self.to_glib_none().0)
        }
    }

    pub fn get_second(&self) -> i32 {
        unsafe {
            ffi::g_date_time_get_second(self.to_glib_none().0)
        }
    }

    pub fn get_seconds(&self) -> f64 {
        unsafe {
            ffi::g_date_time_get_seconds(self.to_glib_none().0)
        }
    }

    pub fn get_timezone_abbreviation(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_date_time_get_timezone_abbreviation(self.to_glib_none().0))
        }
    }

    pub fn get_utc_offset(&self) -> TimeSpan {
        unsafe {
            ffi::g_date_time_get_utc_offset(self.to_glib_none().0)
        }
    }

    pub fn get_week_numbering_year(&self) -> i32 {
        unsafe {
            ffi::g_date_time_get_week_numbering_year(self.to_glib_none().0)
        }
    }

    pub fn get_week_of_year(&self) -> i32 {
        unsafe {
            ffi::g_date_time_get_week_of_year(self.to_glib_none().0)
        }
    }

    pub fn get_year(&self) -> i32 {
        unsafe {
            ffi::g_date_time_get_year(self.to_glib_none().0)
        }
    }

    pub fn get_ymd(&self) -> (i32, i32, i32) {
        unsafe {
            let mut year = mem::uninitialized();
            let mut month = mem::uninitialized();
            let mut day = mem::uninitialized();
            ffi::g_date_time_get_ymd(self.to_glib_none().0, &mut year, &mut month, &mut day);
            (year, month, day)
        }
    }

    pub fn is_daylight_savings(&self) -> bool {
        unsafe {
            from_glib(ffi::g_date_time_is_daylight_savings(self.to_glib_none().0))
        }
    }

    pub fn to_local(&self) -> Option<DateTime> {
        unsafe {
            from_glib_full(ffi::g_date_time_to_local(self.to_glib_none().0))
        }
    }

    //pub fn to_timeval(&self, tv: /*Ignored*/&mut TimeVal) -> bool {
    //    unsafe { TODO: call ffi::g_date_time_to_timeval() }
    //}

    pub fn to_timezone(&self, tz: &TimeZone) -> Option<DateTime> {
        unsafe {
            from_glib_full(ffi::g_date_time_to_timezone(self.to_glib_none().0, tz.to_glib_none().0))
        }
    }

    pub fn to_unix(&self) -> i64 {
        unsafe {
            ffi::g_date_time_to_unix(self.to_glib_none().0)
        }
    }

    pub fn to_utc(&self) -> Option<DateTime> {
        unsafe {
            from_glib_full(ffi::g_date_time_to_utc(self.to_glib_none().0))
        }
    }

    //pub fn compare(dt1: /*Unimplemented*/Fundamental: Pointer, dt2: /*Unimplemented*/Fundamental: Pointer) -> i32 {
    //    unsafe { TODO: call ffi::g_date_time_compare() }
    //}

    //pub fn equal(dt1: /*Unimplemented*/Fundamental: Pointer, dt2: /*Unimplemented*/Fundamental: Pointer) -> bool {
    //    unsafe { TODO: call ffi::g_date_time_equal() }
    //}

    //pub fn hash(datetime: /*Unimplemented*/Fundamental: Pointer) -> u32 {
    //    unsafe { TODO: call ffi::g_date_time_hash() }
    //}
}

impl PartialOrd for DateTime {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for DateTime {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}

impl PartialEq for DateTime {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for DateTime {}
