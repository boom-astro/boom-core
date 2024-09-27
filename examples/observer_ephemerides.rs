use flare::{Observer, Time};

fn main() {
    let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    println!("{}", observer);

    let time = Time::new(2024, 9, 10, 3, 0, 0);
    println!("{}", time);

    let (sunrise, sunset) = observer.sun_set_time(Some(&time), None);

    println!("Next sunrise: {}", sunrise);
    println!("Next sunset: {}", sunset);

    // the time is optional, in which case the current time is used
    let (sunrise, sunset) = observer.sun_set_time(None, None);
    println!("Sunrise: {}, Sunset: {}", sunrise, sunset);

    // You can also specify at what altitude the sun should be considered to have risen/set, as an angle in degrees
    let (sunrise, sunset) = observer.sun_set_time(Some(&time), Some(0.0));

    println!("Sunrise: {}, Sunset: {} (at 0.0 deg)", sunrise, sunset);

    // Otherwise, you can get astronomical, nautical, and civil twilight times:
    let (sunrise, sunset) = observer.twilight_astronomical(Some(&time));
    println!("Sunrise: {}, Sunset: {} (astronomical)", sunrise, sunset);

    let (sunrise, sunset) = observer.twilight_nautical(Some(&time));
    println!("Sunrise: {}, Sunset: {} (nautical)", sunrise, sunset);

    let (sunrise, sunset) = observer.twilight_civil(Some(&time));
    println!("Sunrise: {}, Sunset: {} (civil)", sunrise, sunset);
}