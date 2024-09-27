/// Calculate the refraction correction for a given true altitude.
/// 
/// # Arguments
/// 
/// * `h` - True altitude in degrees.
/// 
/// # Returns
/// 
/// * The refraction correction in degrees.
/// 
/// # Example
/// 
/// ```
/// use flare::corrections::refraction;
/// 
/// let h = 0.5541;
/// let r = refraction(h);
/// println!("Refraction correction: {:.4} degrees", r);
/// assert!((r - 0.410302).abs() < 1.0e-6);
/// ```
/// 
/// # References
/// formula 16.4 of "Astronomical Algorithms" 2nd edition by Jean Meeus (Willmann-Bell, Richmond) 1998.
/// 1.02 / tan(h + 10.3 / (h + 5.11)) h in degrees, result in arc minutes
pub fn refraction(h: f64) -> f64 {
    if h < 0.0 {
        return 0.0;
    }

    (1.02 / (h + (10.3 / (h + 5.11))).to_radians().tan()) / 60.0
}