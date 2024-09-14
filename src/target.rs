use crate::observer::Observer;
use crate::spatial::{DEGRA, great_circle_distance, radec2lb, deg2dms, deg2hms};
use crate::time::Time;

/// Target struct
/// 
/// This struct represents a target in the sky.
/// 
/// # Attributes
/// 
/// * `name` - Optional name of the target
/// * `ra` - Right ascension of the target in degrees
/// * `dec` - Declination of the target in degrees
/// 
/// # Methods
/// 
/// * `new` - Create a new Target
/// * `altitude` - Calculate the altitude of the target at a given time
/// * `airmass` - Calculate the airmass of the target at a given time
/// * `separation` - Calculate the separation to another target
/// * `separations` - Calculate the separations to a list of other targets
/// * `to_string` - Convert the target to a string
/// * `radec2hmsdms` - Convert the target to a tuple of strings with RA and DEC in HMS and DMS format
/// * `radec2lb` - Compute the Galactic coordinates of the target
/// 
/// # Examples
/// 
/// ```
/// use flare::Target;
/// 
/// let target = Target::new(6.374817, 20.242942, Some("Vega".to_string()));
/// println!("{}", target.to_string());
/// ```
pub struct Target {
    pub name: Option<String>,
    pub ra: f64,
    pub dec: f64,
}

impl Target {
    /// Create a new Target
    /// 
    /// # Arguments
    /// 
    /// * `ra` - Right ascension of the target in degrees
    /// * `dec` - Declination of the target in degrees
    /// * `name` - Optional name of the target
    /// 
    /// # Returns
    /// 
    /// * `Target` - A new Target object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::Target;
    /// 
    /// let target = Target::new(6.374817, 20.242942, Some("Vega".to_string()));
    /// assert_eq!(target.ra, 6.374817);
    /// assert_eq!(target.dec, 20.242942);
    /// assert_eq!(target.name, Some("Vega".to_string()));
    /// println!("{}", target.to_string());
    /// ```
    /// 
    /// ```
    /// use flare::Target;
    /// 
    /// let target = Target::new(6.374817, 20.242942, None);
    /// assert_eq!(target.ra, 6.374817);
    /// assert_eq!(target.dec, 20.242942);
    /// assert_eq!(target.name, None);
    /// println!("{}", target.to_string());
    /// ```
    pub fn new(ra: f64, dec: f64, name: Option<String>) -> Target {
        Target { name, ra, dec }
    }

    /// Calculate the altitude of the target at a given time
    /// 
    /// # Arguments
    /// 
    /// * `observer` - Observer object representing the observer
    /// * `time` - Time object representing the time at which to calculate the altitude
    /// 
    /// # Returns
    /// 
    /// * `f64` - The altitude of the target in degrees
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::{Observer, Target, Time};
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    /// let target = Target::new(6.374817, 20.242942, None);
    /// let time = Time::new(2024, 8, 24, 6, 35, 34);
    /// 
    /// let alt = target.altitude(&observer, &time);
    /// assert_eq!(alt, 42.87574211449415);
    /// println!("Altitude: {}", alt);
    /// ```
    /// 
    /// # Notes
    /// 
    /// This altitude calculation is quite simple and does not take into account refraction or other atmospheric effects.
    /// For a more accurate calculation, consider using another dedicated library.
    pub fn altitude(&self, observer: &Observer, time: &Time) -> f64 {
        let (ra, dec) = (self.ra, self.dec);
    
        let ha = ((observer.local_sidereal_time(time) - ra) % 360.0) * DEGRA;
        let lat = observer.lat * DEGRA;
        let dec = dec * DEGRA;
    
        let alt = (dec.sin() * lat.sin() + dec.cos() * lat.cos() * ha.cos()).asin() / DEGRA;
        alt
    }

    /// Calculate the airmass of the target at a given time
    /// 
    /// # Arguments
    /// 
    /// * `observer` - Observer object representing the observer
    /// * `time` - Time object representing the time at which to calculate the airmass
    /// 
    /// # Returns
    /// 
    /// * `f64` - The airmass of the target
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::{Observer, Target, Time};
    /// 
    /// let observer = Observer::new(33.3633675, -116.8361345, 1870.0, None);
    /// let target = Target::new(6.374817, 20.242942, None);
    /// let time = Time::new(2024, 8, 24, 6, 35, 34);
    /// 
    /// let airmass = target.airmass(&observer, &time);
    /// assert_eq!(airmass, 1.467530349026847);
    /// println!("Airmass: {}", airmass);
    /// ```
    /// 
    /// # Notes
    /// 
    /// This airmass calculation is quite simple and does not take into account refraction or other atmospheric effects.
    /// For a more accurate calculation, consider using another dedicated library.
    pub fn airmass(&self, observer: &Observer, time: &Time) -> f64 {
        let alt = self.altitude(observer, time);
        if alt <= 0.0 {
            return std::f64::NEG_INFINITY;
        }
        let sinarg = alt + 244.0 / (165.0 + 47.0 * alt.powf(1.1));
        1.0 / (sinarg * DEGRA).sin()
    }

    /// Calculate the separation to another target
    /// 
    /// # Arguments
    /// 
    /// * `other` - The other target
    /// 
    /// # Returns
    /// 
    /// * `f64` - The separation in degrees
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::Target;
    /// 
    /// let target1 = Target::new(6.374817, 20.242942, None);
    /// let target2 = Target::new(6.374817, 21.242942, None);
    /// assert_eq!((target1.separation(&target2) - 1.0).abs() < 1e-6, true);
    /// ```
    pub fn separation(&self, other: &Target) -> f64 {
        great_circle_distance(self.ra, self.dec, other.ra, other.dec)
    }

    /// Calculate the separations to a list of other targets
    /// 
    /// # Arguments
    /// 
    /// * `others` - A vector of other targets
    /// 
    /// # Returns
    /// 
    /// * `Vec<f64>` - A vector of separations in degrees
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::Target;
    /// 
    /// let target = Target::new(6.374817, 20.242942, None);
    /// let target1 = Target::new(6.374817, 21.242942, None);
    /// let target2 = Target::new(6.374817, 22.242942, None);
    /// let target3 = Target::new(6.374817, 23.242942, None);
    /// let others = vec![target1, target2, target3];
    /// 
    /// let separations = target.separations(&others);
    /// assert_eq!(separations.len(), 3);
    /// assert_eq!((separations[0] - 1.0).abs() < 1e-6, true);
    /// assert_eq!((separations[1] - 2.0).abs() < 1e-6, true);
    /// assert_eq!((separations[2] - 3.0).abs() < 1e-6, true);
    /// ```
    pub fn separations(&self, others: &Vec<Target>) -> Vec<f64> {
        let mut separations = Vec::new();
        for other in others {
            separations.push(self.separation(other));
        }
        separations
    }

    /// Convert the target to a string
    /// 
    /// # Returns
    /// 
    /// * `String` - The target as a string
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::Target;
    /// 
    /// let target = Target::new(6.374817, 20.242942, Some("Vega".to_string()));
    /// assert_eq!(target.to_string(), "Name: Vega, RA: 6.374817, DEC: 20.242942");
    /// println!("{}", target.to_string());
    /// ```
    /// 
    /// ```
    /// use flare::Target;
    /// 
    /// let target = Target::new(6.374817, 20.242942, None);
    /// assert_eq!(target.to_string(), "RA: 6.374817, DEC: 20.242942 (no name)");
    /// println!("{}", target.to_string());
    /// ```
    pub fn to_string(&self) -> String {
        if let Some(name) = &self.name {
            return format!("Name: {}, RA: {}, DEC: {}", name, self.ra, self.dec);
        }
        format!("RA: {}, DEC: {} (no name)", self.ra, self.dec)
    }

    /// Convert the target to a tuple of strings with RA and DEC in HMS and DMS format
    /// 
    /// # Returns
    /// 
    /// * (`String`, `String`) - The target as a string with RA and DEC in HMS and DMS format
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::Target;
    /// 
    /// let target = Target::new(6.374817, 20.242942, Some("Vega".to_string()));
    /// let (hms, dms) = target.radec2hmsdms();
    /// assert_eq!(hms, "00:25:29.9561");
    /// assert_eq!(dms, "20:14:34.591");
    /// println!("RA: {}, DEC: {}", hms, dms);
    /// ```
    pub fn radec2hmsdms(&self) -> (String, String) {
        (deg2hms(self.ra), deg2dms(self.dec))
    }

    /// Compute the Galactic coordinates of the target
    /// 
    /// # Returns
    /// 
    /// * (`f64`, `f64`) - The Galactic longitude and latitude of the target in degrees
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::Target;
    /// 
    /// let target = Target::new(6.374817, 20.242942, Some("Vega".to_string()));
    /// let (l, b) = target.radec2lb();
    /// assert_eq!((l - 114.706509).abs() < 1e-6, true);
    /// assert_eq!((b + 42.214159).abs() < 1e-6, true);
    /// println!("Galactic L: {}, Galactic B: {}", l, b);
    /// ```
    pub fn radec2lb(&self) -> (f64, f64) {
        radec2lb(self.ra, self.dec)
    }

}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "Name: {}, RA: {}, DEC: {}", name, self.ra, self.dec)
        } else {
            write!(f, "RA: {}, DEC: {} (no name)", self.ra, self.dec)
        }
    }
}
