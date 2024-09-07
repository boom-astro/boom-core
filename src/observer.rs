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
/// * `_elevation` - Elevation of the observer in meters
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
/// use boom_core::{Observer, Target, Time};
/// 
/// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, Some("P48".to_string()));
/// let time = Time::new(2024, 8, 24, 6, 35, 34);
/// let lst = observer.local_sidereal_time(&time);
/// println!("Local sidereal time: {}", lst);
/// assert_eq!(lst, 315.09169822871746);
/// ```

pub struct Observer {
    pub name: Option<String>,
    pub lat: f64,
    pub lon: f64,
    pub _elevation: f64, // not used yet, but will be used for refraction correction
}

impl Observer {
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
    /// use boom_core::Observer;
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, Some("P48".to_string()));
    /// assert_eq!(observer.lat, 33.3633675);
    /// assert_eq!(observer.lon, -116.8361345);
    /// assert_eq!(observer._elevation, 1870.0);
    /// assert_eq!(observer.name, Some("P48".to_string()));
    /// println!("{}", observer.to_string());
    /// ```
    /// 
    /// ```
    /// use boom_core::Observer;
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    /// assert_eq!(observer.lat, 33.3633675);
    /// assert_eq!(observer.lon, -116.8361345);
    /// assert_eq!(observer._elevation, 1870.0);
    /// assert_eq!(observer.name, None);
    /// println!("{}", observer.to_string());
    /// ```
    pub fn new(lat: f64, lon: f64, elevation: f64, name: Option<String>) -> Observer {
        Observer { name, lat, lon, _elevation: elevation }
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
    /// use boom_core::{Observer, Time};
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
    /// use boom_core::{Observer, Target, Time};
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
    /// assert_eq!(count, 11665);
    /// 
    /// let count = airmasses.iter().flatten().filter(|&x| *x > 2.0).count();
    /// assert_eq!(count, 4179);
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
    /// use boom_core::Observer;
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, Some("P48".to_string()));
    /// assert_eq!(observer.to_string(), "Name: P48, Lat: 33.3633675, Lon: -116.8361345, Elevation: 1870");
    /// println!("{}", observer.to_string());
    /// ```
    /// 
    /// ```
    /// use boom_core::Observer;
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    /// assert_eq!(observer.to_string(), "Lat: 33.3633675, Lon: -116.8361345, Elevation: 1870 (no name)");
    /// println!("{}", observer.to_string());
    /// ```
    pub fn to_string(&self) -> String {
        if let Some(name) = &self.name {
            return format!("Name: {}, Lat: {}, Lon: {}, Elevation: {}", name, self.lat, self.lon, self._elevation)
        }
        format!("Lat: {}, Lon: {}, Elevation: {} (no name)", self.lat, self.lon, self._elevation)
    }
}
