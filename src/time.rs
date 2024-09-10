use std::fmt;
use chrono::{DateTime, Datelike, Timelike, Utc, TimeZone};

/// Time struct
/// 
/// This struct represents a time.
/// 
/// # Attributes
/// 
/// * `year` - Year
/// * `month` - Month
/// * `day` - Day
/// * `hour` - Hour
/// * `minute` - Minute
/// * `second` - Second
/// 
/// # Methods
/// 
/// * `new` - Create a new Time
/// * `now` - Get the current time
/// * `from_utc` - Create a new Time from a `DateTime<Utc>`
/// * `from_isot_str` - Create a new Time from an ISO 8601 string
/// * `from_jd` - Create a new Time from a Julian Date
/// * `from_mjd` - Create a new Time from a Modified Julian Date
/// * `to_jd` - Convert the Time to a Julian Date
/// * `to_mjd` - Convert the Time to a Modified Julian Date
/// * `to_gst` - Convert the Time to a Greenwich Sidereal Time
/// * `to_utc` - Convert the Time to a `DateTime<Utc>`
/// * `to_string` - Convert the Time to a string
/// 
/// # Examples
/// 
/// ```
/// use boom_core::Time;
/// 
/// let date = Time::new(2020, 1, 1, 0, 0, 0);
/// assert!(date.year == 2020);
/// assert!(date.month == 1);
/// assert!(date.day == 1);
/// assert!(date.hour == 0);
/// assert!(date.minute == 0);
/// assert!(date.second == 0);
/// ```
#[derive(Debug, Clone)]
pub struct Time {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl Time {
    /// Create a new Time
    /// 
    /// # Arguments
    /// 
    /// * `year` - Year
    /// * `month` - Month
    /// * `day` - Day
    /// * `hour` - Hour
    /// * `minute` - Minute
    /// * `second` - Second
    /// 
    /// # Returns
    /// 
    /// * `Time` - A new Time object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use boom_core::Time;
    /// 
    /// let date = Time::new(2020, 1, 1, 0, 0, 0);
    /// assert!(date.year == 2020);
    /// assert!(date.month == 1);
    /// assert!(date.day == 1);
    /// assert!(date.hour == 0);
    /// assert!(date.minute == 0);
    /// assert!(date.second == 0);
    /// ```
    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Time {
        Time {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    /// Get the current time
    /// 
    /// # Returns
    /// 
    /// * `Time` - A new Time object representing the current time
    /// 
    /// # Examples
    /// 
    /// ```
    /// use boom_core::Time;
    /// 
    /// let date = Time::now();
    /// assert!(date.year > 2023);
    /// ```
    pub fn now() -> Time {
        let utc = Utc::now();
        Time {
            year: utc.year(),
            month: utc.month(),
            day: utc.day(),
            hour: utc.hour(),
            minute: utc.minute(),
            second: utc.second(),
        }
    }

    /// Create a new Time from a `DateTime<Utc>`
    /// 
    /// # Arguments
    /// 
    /// * `utc` - `DateTime<Utc>` object
    /// 
    /// # Returns
    /// 
    /// * `Time` - A new Time object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use chrono::{DateTime, Datelike, Timelike, Utc, TimeZone};
    /// use boom_core::Time;
    /// 
    /// let utc = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    /// let date = Time::from_utc(utc);
    /// assert!(date.year == utc.year());
    /// assert!(date.month == utc.month());
    /// assert!(date.day == utc.day());
    /// assert!(date.hour == utc.hour());
    /// assert!(date.minute == utc.minute());
    /// assert!(date.second == utc.second());
    /// ```
    pub fn from_utc(utc: DateTime<Utc>) -> Time {
        Time {
            year: utc.year(),
            month: utc.month(),
            day: utc.day(),
            hour: utc.hour(),
            minute: utc.minute(),
            second: utc.second(),
        }
    }

    /// Create a new Time from an ISO 8601 string
    /// 
    /// # Arguments
    /// 
    /// * `isot` - ISO 8601 string
    /// 
    /// # Returns
    /// 
    /// * `Time` - A new Time object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use boom_core::Time;
    /// 
    /// let isot = "2020-01-01T00:00:00Z";
    /// let date = Time::from_isot_str(isot);
    /// assert!(date.year == 2020);
    /// assert!(date.month == 1);
    /// assert!(date.day == 1);
    /// assert!(date.hour == 0);
    /// assert!(date.minute == 0);
    /// assert!(date.second == 0);
    /// ```
    pub fn from_isot_str(isot: &str) -> Time {
        let utc = DateTime::parse_from_rfc3339(isot).unwrap();
        Time {
            year: utc.year(),
            month: utc.month(),
            day: utc.day(),
            hour: utc.hour(),
            minute: utc.minute(),
            second: utc.second(),
        }
    }

    /// Create a new Time from a Julian Date
    /// 
    /// # Arguments
    /// 
    /// * `jd` - Julian Date
    /// 
    /// # Returns
    /// 
    /// * `Time` - A new Time object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use boom_core::Time;
    /// 
    /// let jd = 2460564.0569609753;
    /// let date = Time::from_jd(jd);
    /// println!("{:?}", date);
    /// assert!(date.year == 2024);
    /// assert!(date.month == 9);
    /// assert!(date.day == 10);
    /// assert!(date.hour == 13);
    /// assert!(date.minute == 22);
    /// assert!(date.second == 1);
    /// ```
    pub fn from_jd(jd: f64) -> Time {
        let temp = jd + 0.5;
        let z = temp.floor() as i32;
        let mut f = temp - z as f64;
        let mut a = z;
        if z > 2299161 {
            let alpha = ((z as f64 - 1867216.25) / 36524.25).floor() as i32;
            a = z + 1 + alpha - (alpha / 4);
        }
        let b = a + 1524;
        let c = ((b as f64 - 122.1) / 365.25).floor();
        let d = (365.25 * c) as i32;
        let e = ((b as f64 - d as f64) / 30.6001).floor() as i32;

        let day = b - d - ((30.6001 * e as f64) as i32) + f as i32;
        let month = if e < 14 { e - 1 } else { e - 13 };
        let year = if month > 2 { c - 4716.0 } else { c - 4715.0 };

        let hour = ((f * 24.0) as i32).abs();
        f = f - (hour as f64 / 24.0);
        let minute = ((f * 1440.0) as i32).abs();
        f = f - (minute as f64 / 1440.0);
        let second = ((f * 86400.0) as i32).abs();

        Time {
            year: year as i32,
            month: month as u32,
            day: day as u32,
            hour: hour as u32,
            minute: minute as u32,
            second: second as u32,
        }
    }

    /// Create a new Time from a Modified Julian Date
    /// 
    /// # Arguments
    /// 
    /// * `mjd` - Modified Julian Date
    /// 
    /// # Returns
    /// 
    /// * `Time` - A new Time object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use boom_core::Time;
    /// 
    /// let mjd = 58849.0;
    /// let date = Time::from_mjd(mjd);
    /// assert!(date.year == 2020);
    /// assert!(date.month == 1);
    /// assert!(date.day == 1);
    /// assert!(date.hour == 0);
    /// assert!(date.minute == 0);
    /// assert!(date.second == 0);
    /// ```
    pub fn from_mjd(mjd: f64) -> Time {
        Time::from_jd(mjd + 2400000.5)
    }

    /// Convert the Time to a Julian Date
    /// 
    /// # Returns
    /// 
    /// * `f64` - Julian Date
    /// 
    /// # Examples
    /// 
    /// ```
    /// use boom_core::Time;
    /// 
    /// let date = Time::new(2024, 8, 24, 6, 35, 34);
    /// let jd = date.to_jd();
    /// assert_eq!(jd, 2460546.774699074);
    /// ```
    pub fn to_jd(&self) -> f64 {
        let year = self.year as f64;
        let month = self.month as f64;
        let day = self.day as f64;
        let hour = self.hour as f64;
        let minute = self.minute as f64;
        let second = self.second as f64;

        let jd = 367.0 * year - ((year + ((month + 9.0) / 12.0)).floor() * 7.0 / 4.0).floor()
            + ((275.0 * month) / 9.0).floor() + day + 1721013.5
            + ((hour + (minute / 60.0) + (second / 3600.0)) / 24.0);
        jd
    }

    /// Convert the Time to a Modified Julian Date
    /// 
    /// # Returns
    /// 
    /// * `f64` - Modified Julian Date
    /// 
    /// # Examples
    /// 
    /// ```
    /// use boom_core::Time;
    /// 
    /// let date = Time::new(2024, 8, 24, 6, 35, 34);
    /// let mjd = date.to_mjd();
    /// assert_eq!(mjd, 60546.274699074216);
    /// ```
    pub fn to_mjd(&self) -> f64 {
        self.to_jd() - 2400000.5
    }

    /// Convert the Time to a Greenwich Sidereal Time
    /// 
    /// # Returns
    /// 
    /// * `f64` - Greenwich Sidereal Time in degrees
    /// 
    /// # Examples
    /// 
    /// ```
    /// use boom_core::Time;
    /// 
    /// let date = Time::new(2024, 8, 24, 6, 35, 34);
    /// let gst = date.to_gst();
    /// assert_eq!(gst, 71.92783272871748);
    /// ```
    pub fn to_gst(&self) -> f64 {
        let jd = self.to_jd();
        let t = (jd - 2451545.0) / 36525.0;
        let gst = 280.46061837 + 360.98564736629 * (jd - 2451545.0)
            + 0.000387933 * t * t
            - (t * t * t) / 38710000.0;
        gst % 360.0
    }

    /// Convert the Time to a `DateTime<Utc>`
    /// 
    /// # Returns
    /// 
    /// * `DateTime<Utc>` - `DateTime<Utc>` object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use boom_core::Time;
    /// use chrono::{DateTime, Utc};
    /// 
    /// let date = Time::new(2024, 8, 24, 6, 35, 34);
    /// let utc = date.to_utc();
    /// let utc_str = utc.to_string();
    /// assert_eq!(utc_str, "2024-08-24 06:35:34 UTC");
    /// ```
    pub fn to_utc(&self) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
        ).unwrap()
    }

    /// Convert the Time to a string
    /// 
    /// # Arguments
    /// 
    /// * `format` - Format of the string
    /// 
    /// # Returns
    /// 
    /// * `String` - String representation of the Time
    /// 
    /// # Examples
    /// 
    /// ```
    /// use boom_core::Time;
    /// 
    /// let date = Time::new(2020, 1, 1, 0, 0, 0);
    /// let jd_str = date.to_string(Some("jd"));
    /// assert_eq!(jd_str, "2458849.5");
    /// 
    /// let mjd_str = date.to_string(Some("mjd"));
    /// assert_eq!(mjd_str, "58849");
    /// 
    /// let utc_str = date.to_string(Some("utc"));
    /// assert_eq!(utc_str, "2020-01-01 00:00:00 UTC");
    /// 
    /// let isot_str = date.to_string(Some("isot"));
    /// assert_eq!(isot_str, "2020-01-01T00:00:00+00:00");
    /// ```
    /// 
    /// ```
    /// use boom_core::Time;
    /// 
    /// let date = Time::new(2020, 1, 1, 0, 0, 0);
    /// let str = date.to_string(None);
    /// assert_eq!(str, "2020-01-01 00:00:00 UTC");
    /// ```
    pub fn to_string(&self, format: Option<&str>) -> String {
        if let Some(format) = format {
            if format == "jd" {
                return self.to_jd().to_string();
            } else if format == "mjd" {
                return self.to_mjd().to_string();
            } else if format == "utc" {
                return self.to_utc().to_string();
            } else if format == "isot" {
                return self.to_utc().to_rfc3339();
            } else {
                return "Invalid format".to_string();
            }
        } else {
            return self.to_utc().to_string();
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{:02}-{:02} {:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)
    }
}