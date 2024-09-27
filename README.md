# Flare

`flare` is a lightweight library designed to perform basic astronomical calculations, inspired by Python's Astropy syntax.

## Table of Contents

- [Installation](#installation)
- [Usage & Features](#features--usage)
- [Contributing](#contributing)
- [License](#license)

## Installation

To include flare in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
flare = "0.1.0"
```

## Features & Usage

You can do a couple of different things with `flare`. We recommend reading the documentation that you can find [here](https://boom-astro.github.io/flare/index.html).

Here are some examples of what you can do with `flare`:

- Handle dates in multiple standard & astronomical formats:
    
    ```rust
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
    ```

- Calculate the angular separation between two targets/objects in the sky:
    
    ```rust
    use flare::Target;

    fn main() {
        let target1 = Target::new(6.374817, 20.242942, Some("A"));
        let target2 = Target::new(6.374817, 21.242942, Some("B"));

        let separation = target1.separation(&target2);

        println!("The angular separation between the two targets is: {}", separation);
    }
    ```

- Given an observer on earth, find the airmass of a target (at a given time):

    ```rust
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
    ```

- For an observer, find the next sunrise & sunset times (after a given time):

    ```rust
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
    ```

- Work with photometry, in mag and flux space:

    ```rust
    use flare::phot::{mag_to_flux, flux_to_mag, limmag_to_fluxerr, fluxerr_to_limmag, ZP};

    fn main() {
        let mag = 20.0;
        let magerr = 0.1;
        let limmag = 21.0;

        // mag to flux
        let (flux, fluxerr) = mag_to_flux(mag, magerr, ZP);
        println!("flux: {}, fluxerr: {}", flux, fluxerr);

        // flux to mag
        let (mag, magerr) = flux_to_mag(flux, fluxerr, ZP);
        println!("mag: {}, mag: {}", mag, magerr);

        // limiting mag to fluxerr, at 5-sigma
        let fluxerr = limmag_to_fluxerr(limmag, ZP, 5.0);
        println!("fluxerr (from limmag): {}", fluxerr);

        // fluxerr to limiting mag at 5-sigma
        let limmag = fluxerr_to_limmag(fluxerr, ZP, 5.0);
        println!("limmag (from fluxerr, at 5-sigma): {}", limmag);

        // ZP = 23.9, but you can specify your own zero point for any of the above,
        // just like you can specify a different sigma value
        let limmag = limmag_to_fluxerr(limmag, 25.0, 3.0);
        println!("fluxerr (from limmag, 3-sigma & ZP=25.0): {}", limmag);
    }
    ```

- Use a cosmology (custom, or one of the built-in ones) to compute distances:
    
    ```rust
    use flare::Cosmo;

    fn main() {
        let z = 0.1;

        // you can use one of the built-in cosmologies
        let cosmo = Cosmo::planck18();

        let luminosity_distance = cosmo.luminosity_distance(z);
        println!("Luminosity distance: {}", luminosity_distance);

        let dm = cosmo.dm(z);
        println!("Distance modulus: {}", dm);

        let angular_diameter_distance = cosmo.angular_diameter_distance(z);
        println!("Angular diameter distance: {}", angular_diameter_distance);

        // for example, you could this to get the absolute magnitude of a target
        let apparent_mag = 20.0;
        let abs_mag = apparent_mag - dm;
        println!("{} mag at z={} is {}", apparent_mag, z, abs_mag);

        // You can also create your own cosmology
        let cosmology = Cosmo::new(67.66, 0.3103, 0.6897, Some("mycosmo"));

        let z = 0.0246;
        let dm = cosmology.dm(z);
        println!("Distance modulus at z={} (using custom cosmology {}) is {}", z, cosmology.name.unwrap(), dm);
    }
    ```

## Contributing

We welcome contributions! No specific guidelines yet, but feel free to open an issue or a PR. Keep in mind that the goal is to keep this library lightweight and focused on basic astronomical calculations. We are not trying to replicate the functionality of Astropy in Rust.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
