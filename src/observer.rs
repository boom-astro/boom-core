use crate::spatial::DEGRA;
use crate::target::Target;
use crate::time::Time;

/// Observer struct
/// 
/// This struct represents an observer on the Earth's surface.
/// 
/// # Attributes
/// 
/// * `name` - Optional name of the observer
/// * `lat` - Latitude of the observer in degrees
/// * `lon` - Longitude of the observer in degrees
/// * `elevation` - Elevation of the observer in meters
/// 
/// # Methods
/// 
/// * `new` - Create a new Observer
/// * `local_sidereal_time` - Calculate the local sidereal time at a given time
/// * `targets_airmasses` - Calculate the airmasses of a list of targets at a list of times
/// * `to_string` - Convert the Observer to a string
/// 
/// # Examples
/// 
/// ```
/// use flare::{Observer, Target, Time};
/// 
/// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, Some("P48"));
/// let time = Time::new(2024, 8, 24, 6, 35, 34);
/// let lst = observer.local_sidereal_time(&time);
/// println!("Local sidereal time: {}", lst);
/// assert_eq!(lst, 315.09169822871746);
/// ```

pub struct Observer<'a> {
    pub name: Option<&'a str>,
    pub lat: f64,
    pub lon: f64,
    pub elevation: f64, // not used yet, but will be used for refraction correction
}

impl <'a> Observer<'a> {
    /// Create a new Observer
    /// 
    /// # Arguments
    /// 
    /// * `lat` - Latitude of the observer in degrees
    /// * `lon` - Longitude of the observer in degrees
    /// * `elevation` - Elevation of the observer in meters
    /// * `name` - Optional name of the observer
    /// 
    /// # Returns
    /// 
    /// * `Observer` - A new Observer object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::Observer;
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, Some("P48"));
    /// assert_eq!(observer.lat, 33.3633675);
    /// assert_eq!(observer.lon, -116.8361345);
    /// assert_eq!(observer.elevation, 1870.0);
    /// assert_eq!(observer.name, Some("P48"));
    /// println!("{}", observer.to_string());
    /// ```
    /// 
    /// ```
    /// use flare::Observer;
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    /// assert_eq!(observer.lat, 33.3633675);
    /// assert_eq!(observer.lon, -116.8361345);
    /// assert_eq!(observer.elevation, 1870.0);
    /// assert_eq!(observer.name, None);
    /// println!("{}", observer.to_string());
    /// ```
    pub fn new(lat: f64, lon: f64, elevation: f64, name: Option<&str>) -> Observer {
        Observer { name, lat, lon, elevation }
    }

    /// Calculate the local sidereal time at a given time
    /// 
    /// # Arguments
    /// 
    /// * `time` - Time object representing the time at which to calculate the local sidereal time
    /// 
    /// # Returns
    /// 
    /// * `f64` - The local sidereal time in degrees
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::{Observer, Time};
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    /// let time = Time::new(2024, 8, 24, 6, 35, 34);
    /// let lst = observer.local_sidereal_time(&time);
    /// assert_eq!(lst, 315.09169822871746);
    /// println!("Local sidereal time: {}", lst);
    /// ```
    pub fn local_sidereal_time(&self, time: &Time) -> f64 {
        let gst = time.to_gst();
        let lst = gst + self.lon + 360.0;
        lst % 360.0
    }

    /// Calculate the airmasses of a list of targets at a list of times
    /// 
    /// # Arguments
    /// 
    /// * `targets` - A vector of Target objects
    /// * `times` - A vector of Time objects
    /// 
    /// # Returns
    /// 
    /// * `Vec<Vec<f64>>` - A 2D vector of airmasses, with the first dimension being the targets
    /// and the second dimension being the times
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::{Observer, Target, Time};
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    /// let target1 = Target::new(6.374817, 20.242942, None);
    /// let target2 = Target::new(6.374817, 21.242942, None);
    /// let targets = vec![target1, target2];
    /// 
    /// let start_time = Time::new(2024, 8, 24, 0, 0, 0).to_jd();
    /// let end_time = Time::new(2024, 8, 24, 23, 0, 0).to_jd();
    /// 
    /// let nb_samples = 10000;
    /// let delta = (end_time - start_time) / nb_samples as f64;
    /// 
    /// let times = (0..nb_samples).map(|i| Time::from_jd(start_time + i as f64 * delta)).collect::<Vec<Time>>();
    /// 
    /// let airmasses = observer.targets_airmasses(&targets, &times);
    /// 
    /// let count = airmasses.iter().flatten().filter(|&x| *x > 0.0).count();
    /// assert_eq!(count, 11713);
    /// 
    /// let count = airmasses.iter().flatten().filter(|&x| *x > 2.0).count();
    /// assert_eq!(count, 4171);
    /// ```
    /// 
    /// # Notes
    /// 
    /// This airmass calculation is quite simple and does not take into account refraction or other atmospheric effects.
    /// For a more accurate calculation, consider using another dedicated library.
    pub fn targets_airmasses(&self, targets: &Vec<Target>, times: &Vec<Time>) -> Vec<Vec<f64>> {
        let lat = self.lat;
        let lsts = times.iter().map(|time| self.local_sidereal_time(time)).collect::<Vec<f64>>();

        let ra_array = targets.iter().map(|target| target.ra).collect::<Vec<f64>>();
        let dec_array = targets.iter().map(|target| target.dec).collect::<Vec<f64>>();

        let mut airmasses = vec![vec![0.0; times.len()]; targets.len()];

        for (i, _target) in targets.iter().enumerate() {
            for (j, _time) in times.iter().enumerate() {
                let ha = ((lsts[j] - ra_array[i]) % 360.0) * DEGRA;
                let lat = lat * DEGRA;
                let dec = dec_array[i] * DEGRA;
            
                let alt = (dec.sin() * lat.sin() + dec.cos() * lat.cos() * ha.cos()).asin() / DEGRA;
                let alt = alt - 0.0347 * (90.0 - alt).tan().powi(2);
                let sinarg = alt + 244.0 / (165.0 + 47.0 * alt.powf(1.1));
                airmasses[i][j] = 1.0 / (sinarg * DEGRA).sin();
            }
        }
        airmasses
    }

    /// Convert the Observer to a string
    /// 
    /// # Returns
    /// 
    /// * `String` - A string representation of the Observer
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::Observer;
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, Some("P48"));
    /// assert_eq!(observer.to_string(), "Name: P48, Lat: 33.3633675, Lon: -116.8361345, Elevation: 1870");
    /// println!("{}", observer.to_string());
    /// ```
    /// 
    /// ```
    /// use flare::Observer;
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    /// assert_eq!(observer.to_string(), "Lat: 33.3633675, Lon: -116.8361345, Elevation: 1870 (no name)");
    /// println!("{}", observer.to_string());
    /// ```
    pub fn to_string(&self) -> String {
        if let Some(name) = &self.name {
            return format!("Name: {}, Lat: {}, Lon: {}, Elevation: {}", name, self.lat, self.lon, self.elevation)
        }
        format!("Lat: {}, Lon: {}, Elevation: {} (no name)", self.lat, self.lon, self.elevation)
    }

    /// Calculate the time of the next sunrise & sunset (in UTC)
    /// 
    /// # Arguments
    /// 
    /// * `after` - Optional Time object representing the time after which to calculate the next sunrise & sunset
    /// * `solar_alt` - Optional f64 representing the solar altitude at which to calculate the sunrise & sunset
    /// 
    /// # Returns
    /// 
    /// * (`Time`, `Time`) - A tuple of Time objects representing the next sunrise & sunset
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::{Observer, Time};
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    /// let time = Time::new(2024, 9, 10, 3, 0, 0);
    /// let (sunrise, sunset) = observer.sun_set_time(Some(&time), None);
    /// println!("Next sunrise: {}", sunrise.to_string(None));
    /// println!("Next sunset: {}", sunset.to_string(None));
    /// assert_eq!(sunrise.to_string(None), "2024-09-10 13:22:01 UTC");
    /// assert_eq!(sunset.to_string(None), "2024-09-11 02:09:11 UTC");
    /// ```
    /// 
    /// ```
    /// use flare::{Observer, Time};
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    /// let time = Time::new(2024, 9, 10, 3, 0, 0);
    /// let sun_alt_astronomical = -18.0;
    /// let (sunrise, sunset) = observer.sun_set_time(Some(&time), Some(sun_alt_astronomical));
    /// println!("Next sunrise: {}", sunrise.to_string(None));
    /// println!("Next sunset: {}", sunset.to_string(None));
    /// assert_eq!(sunrise.to_string(None), "2024-09-10 11:57:23 UTC");
    /// assert_eq!(sunset.to_string(None), "2024-09-11 03:33:49 UTC");
    /// ```
    /// 
    /// # Notes
    /// 
    /// This calculation is based on the algorithm described at <https://en.wikipedia.org/wiki/Sunrise_equation>
    pub fn sun_set_time(&self, after: Option<&Time>, solar_alt: Option<f64>) -> (Time, Time) {
        // 1. calculate the current julian day
        let after = match after {
            Some(time) => time,
            None => &Time::now(),
        };
        let solar_alt = solar_alt.unwrap_or(-0.833);
        let jd = after.to_jd();

        let n = (jd - (2451545.0 + 0.0009) - 69.184 / 86400.0).ceil();

        // 2. get the mean solar time
        let jstar = n + 0.0009 - self.lon / 360.0;

        // 3. calculate the solar mean anomaly
        let m = (357.5291 + 0.98560028 * jstar) % 360.0;
        let m_rad = m.to_radians();

        // 4. calculate the equation of center
        let c = 1.9148 * m_rad.sin() + 0.0200 * (2.0 * m_rad).sin() + 0.0003 * (3.0 * m_rad).sin();

        // 5. calculate the ecliptic longitude
        let lambda = (m + c + 180.0 + 102.9372) % 360.0;
        let lambda_rad = lambda.to_radians();

        // 6. calculate the solar transit
        let jtransit = 2451545.0 + jstar + 0.0053 * m_rad.sin() - 0.0069 * (2.0 * lambda_rad).sin();

        // 7. calculate the declination of the sun
        let delta_sin = lambda_rad.sin() * 23.4397_f64.to_radians().sin();
        let delta_cos = delta_sin.asin().cos();

        // 8. calculate the hour angle
        let w0_cos = (
            (solar_alt - 2.076 * self.elevation.sqrt() / 60.0).to_radians().sin()
            - self.lat.to_radians().sin() * delta_sin
        ) / (self.lat.to_radians().cos() * delta_cos);
        
        let w0_rad = w0_cos.acos();
        let w0 = w0_rad.to_degrees();

        // 9. calculate the sunrise and sunset times
        let jrise = jtransit - w0 / 360.0;
        let jset = jtransit + w0 / 360.0;

        // 10. convert the sunrise time to a Time object
        let sunrise = Time::from_jd(jrise);
        let sunset = Time::from_jd(jset);

        (sunrise, sunset)
    }

    /// Calculate the time of the next astronomical sunrise & sunset (in UTC)
    /// 
    /// # Arguments
    /// 
    /// * `after` - Optional Time object representing the time after which to calculate the next sunrise & sunset
    /// 
    /// # Returns
    /// 
    /// * (`Time`, `Time`) - A tuple of Time objects representing the next sunrise & sunset
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::{Observer, Time};
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    /// let time = Time::new(2024, 9, 10, 3, 0, 0);
    /// let (sunrise, sunset) = observer.twilight_astronomical(Some(&time));
    /// println!("Next sunrise: {}", sunrise.to_string(None));
    /// println!("Next sunset: {}", sunset.to_string(None));
    /// assert_eq!(sunrise.to_string(None), "2024-09-10 11:57:23 UTC");
    /// assert_eq!(sunset.to_string(None), "2024-09-11 03:33:49 UTC");
    /// ```
    /// 
    /// # Notes
    /// 
    /// Sunrise & sunset astronomical times are defined as the time when the sun is 18 degrees below the horizon.
    /// This is the time when the sky is dark enough for most astronomical observations.
    pub fn twilight_astronomical(&self, after: Option<&Time>) -> (Time, Time) {
        self.sun_set_time(after, Some(-18.0))
    }

    /// Calculate the time of the next nautical sunrise & sunset (in UTC)
    /// 
    /// # Arguments
    /// 
    /// * `after` - Optional Time object representing the time after which to calculate the next sunrise & sunset
    /// 
    /// # Returns
    /// 
    /// * (`Time`, `Time`) - A tuple of Time objects representing the next sunrise & sunset
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::{Observer, Time};
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    /// let time = Time::new(2024, 9, 10, 3, 0, 0);
    /// let (sunrise, sunset) = observer.twilight_nautical(Some(&time));
    /// println!("Next sunrise: {}", sunrise.to_string(None));
    /// println!("Next sunset: {}", sunset.to_string(None));
    /// assert_eq!(sunrise.to_string(None), "2024-09-10 12:27:29 UTC");
    /// assert_eq!(sunset.to_string(None), "2024-09-11 03:03:42 UTC");
    /// ```
    /// 
    /// # Notes
    /// 
    /// Sunrise & sunset nautical times are defined as the time when the sun is 12 degrees below the horizon.
    /// This is the time when the horizon is still visible at sea.
    pub fn twilight_nautical(&self, after: Option<&Time>) -> (Time, Time) {
        self.sun_set_time(after, Some(-12.0))
    }

    /// Calculate the time of the next civil sunrise & sunset (in UTC)
    /// 
    /// # Arguments
    /// 
    /// * `after` - Optional Time object representing the time after which to calculate the next sunrise & sunset
    /// 
    /// # Returns
    /// 
    /// * (`Time`, `Time`) - A tuple of Time objects representing the next sunrise & sunset
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::{Observer, Time};
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    /// let time = Time::new(2024, 9, 10, 3, 0, 0);
    /// let (sunrise, sunset) = observer.twilight_civil(Some(&time));
    /// println!("Next sunrise: {}", sunrise.to_string(None));
    /// println!("Next sunset: {}", sunset.to_string(None));
    /// assert_eq!(sunrise.to_string(None), "2024-09-10 12:56:58 UTC");
    /// assert_eq!(sunset.to_string(None), "2024-09-11 02:34:14 UTC");
    /// ```
    /// 
    /// # Notes
    /// 
    /// Sunrise & sunset civil times are defined as the time when the sun is 6 degrees below the horizon.
    /// This is the time when the sky is light enough for most outdoor activities.
    pub fn twilight_civil(&self, after: Option<&Time>) -> (Time, Time) {
        self.sun_set_time(after, Some(-6.0))
    }
}

impl <'a> std::fmt::Display for Observer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "Name: {}, Lat: {}, Lon: {}, Elevation: {}", name, self.lat, self.lon, self.elevation)
        } else {
            write!(f, "Lat: {}, Lon: {}, Elevation: {} (no name)", self.lat, self.lon, self.elevation)
        }
    }
}
