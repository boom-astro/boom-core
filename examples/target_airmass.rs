use flare::{Target, Observer, Time};

fn main() {
    let observer = Observer::new(30.0, 45.0, 1800.0, Some("A"));
    println!("{}", observer);

    let target = Target::new(6.374817, 20.242942, Some("B"));
    println!("{}", target);

    let time = Time::new(2020, 12, 21, 12, 0, 0);
    println!("{}", time);

    let airmass = target.airmass(&observer, &time);

    println!("Airmass at {} is: {}", time, airmass);
}