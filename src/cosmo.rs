const C: f64 = 299792.458;

fn integrate<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / n as f64;
    let s = (1..n)
        .map(|i| f(a + i as f64 * h))
        .sum::<f64>();
    h / 2.0 * (f(a) + f(b) + 2.0 * s)
}

/// Cosmology
/// 
/// # Examples
/// 
/// ```
/// use flare::cosmo::Cosmo;
/// 
/// let cosmology = Cosmo::planck18();
/// 
/// let z = 0.0246;
/// let lumdist = cosmology.luminosity_distance(z);
/// assert_eq!((lumdist - 111.038270).abs() < 1e-6, true);
/// println!("Luminosity distance: {:.2} Mpc", lumdist);
/// 
/// let dm = cosmology.dm(z);
/// assert_eq!((dm - 35.227363).abs() < 1e-6, true);
/// println!("Distance modulus: {:.4}", dm);
/// 
/// let d_a = cosmology.angular_diameter_distance(z);
/// assert_eq!((d_a - 105.770361).abs() < 1e-6, true);
/// println!("Angular diameter distance: {:.4} Mpc", d_a);
/// ```
pub struct Cosmo<'a> {
    // Constants
    pub h0: f64,
    pub omega_m: f64,
    pub omega_lambda: f64,
    pub omega_k: f64,
    pub name: Option<&'a str>,
}

impl <'a> Cosmo<'a> {
    /// Create a new cosmology
    /// 
    /// # Returns
    /// 
    /// * `Cosmo` - Cosmology
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::cosmo::Cosmo;
    /// 
    /// let cosmology = Cosmo::new(67.66, 0.3103, 0.6897, Some("Test"));
    /// assert_eq!(cosmology.h0, 67.66);
    /// ```
    pub fn new(h0: f64, omega_m: f64, omega_lambda: f64, name: Option<&'a str>) -> Self {
        let omega_k = 1.0 - omega_m - omega_lambda;
        Self { h0, omega_m, omega_lambda, omega_k, name }
    }

    /// Create a new cosmology with the Planck 2018 parameters
    /// 
    /// # Returns
    /// 
    /// * `Cosmo` - Cosmology
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::cosmo::Cosmo;
    /// 
    /// let cosmology = Cosmo::planck18();
    /// assert_eq!(cosmology.h0, 67.66);
    /// ```
    pub fn planck18() -> Self {
        let h0 = 67.66;
        let omega_m = 0.3103;
        let omega_lambda = 0.6897;
        let omega_k = 1.0 - omega_m - omega_lambda;
        Self { h0, omega_m, omega_lambda, omega_k, name: Some("Planck18") }
    }
    
    /// Calculate the luminosity distance from the redshift
    /// 
    /// # Arguments
    /// 
    /// * `redshift` - Redshift
    /// 
    /// # Returns
    /// 
    /// * `f64` - Luminosity distance in Mpc
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::cosmo::Cosmo;
    /// 
    /// let cosmology = Cosmo::new(67.66, 0.3103, 0.6897, None);
    /// 
    /// let z = 0.0246;
    /// let lumdist = cosmology.luminosity_distance(z);
    /// assert_eq!((lumdist - 111.038270).abs() < 1e-6, true);
    /// println!("Luminosity distance: {:.2} Mpc", lumdist);
    /// ```
    pub fn luminosity_distance(&self, redshift: f64) -> f64 {
        let integrand = |z: f64| {
            1.0 / (self.omega_m * (1.0 + z).powi(3) + self.omega_k * (1.0 + z).powi(2) + self.omega_lambda).sqrt()
        };
        let d_h = C / self.h0;
        let d_c = d_h * integrate(&integrand, 0.0, redshift, 1000);
        let d_m = d_c / (1.0 + redshift);
        let d_lum = (1.0 + redshift).powi(2) * d_m;
        d_lum
    }

    /// Calculate the distance modulus from the redshift
    /// 
    /// # Arguments
    /// 
    /// * `z` - Redshift
    /// 
    /// # Returns
    /// 
    /// * `f64` - Distance modulus
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::cosmo::Cosmo;
    /// 
    /// let cosmology = Cosmo::new(67.66, 0.3103, 0.6897, None);
    /// 
    /// let z = 0.0246;
    /// let dm = cosmology.dm(z);
    /// assert_eq!((dm - 35.227363).abs() < 1e-6, true);
    /// println!("Distance modulus: {:.4}", dm);
    /// ```
    pub fn dm(&self, z: f64) -> f64 {
        let lumdist = self.luminosity_distance(z);
        // 5.0 * np.log10((dl * u.Mpc) / (10 * u.pc)).value
        5.0 * ((lumdist * 1.0e6) / 10.0).log10()
    }

    /// Calculate the angular diameter distance from the redshift
    /// 
    /// # Arguments
    /// 
    /// * `z` - Redshift
    /// 
    /// # Returns
    /// 
    /// * `f64` - Angular diameter distance in Mpc
    /// 
    /// # Examples
    /// 
    /// ```
    /// use flare::cosmo::Cosmo;
    /// 
    /// let cosmology = Cosmo::new(67.66, 0.3103, 0.6897, None);
    /// 
    /// let z = 0.0246;
    /// let d_a = cosmology.angular_diameter_distance(z);
    /// assert_eq!((d_a - 105.770361).abs() < 1e-6, true);
    /// println!("Angular diameter distance: {:.4} Mpc", d_a);
    pub fn angular_diameter_distance(&self, z: f64) -> f64 {
        let lumdist = self.luminosity_distance(z);
        if z > 0.01 {
            lumdist / (1.0 + z).powi(2)
        } else {
            lumdist
        }
    }
}