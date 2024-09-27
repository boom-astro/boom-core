use chrono::{Utc, TimeZone};
use flare::Time;

fn main() {
    let time = Time::new(2021, 6, 21, 12, 0, 0);

    // You can convert a time (which is timezone-agnostic, in UTC)
    // to a variety of formats:
    let time_jd = time.to_jd();
    println!("The Julian Date of the time is: {}", time_jd);

    // and, you can do the exact opposite, create a `Time` object
    // from a JD, MJD, GST, UTC, or ISO string:
    let time_from_jd = Time::from_jd(2459376.0);
    println!("The time from the Julian Date is: {}", time_from_jd);

    // you can also get the current time:
    let current_time = Time::now();
    println!("{}", current_time);

    // we rely on the fantastic `chrono` library for date-time handling
    // so you can convert a chrono::DateTime<Utc> object to a `Time` object
    let chrono_utc = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let time = Time::from_utc(chrono_utc);

    // you can print a date in any of the supported formats:
    println!("{}", time.to_string(Some("mjd")));
}