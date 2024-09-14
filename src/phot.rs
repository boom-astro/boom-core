pub const ZP: f64 = 23.9;
const FACTOR: f64 = 1.0857362047581294; // where 1.0857362047581294 = 2.5 / np.log(10)

/// Convert a magnitude to a flux
/// 
/// # Arguments
/// 
/// * `mag` - Magnitude
/// * `magerr` - Magnitude error
/// * `zp` - Zero point
/// 
/// # Returns
/// 
/// * `f64` - Flux
/// * `f64` - Flux error
/// 
/// # Examples
/// 
/// ```
/// use flare::phot::{mag_to_flux, ZP};
/// 
/// let mag = 20.0;
/// let magerr = 0.1;
/// let (flux, fluxerr) = mag_to_flux(mag, magerr, ZP);
/// println!("flux: {}, fluxerr: {}", flux, fluxerr);
/// assert_eq!((flux - 36.307805).abs() < 1e-6, true);
/// assert_eq!((fluxerr - 3.344072).abs() < 1e-6, true);
/// ```
pub fn mag_to_flux(mag: f64, magerr: f64, zp: f64) -> (f64, f64) {
    let flux = 10.0_f64.powf(-0.4 * (mag - zp));
    let fluxerr = magerr / FACTOR * flux;
    (flux, fluxerr)
}

/// Convert a flux to a magnitude
/// 
/// # Arguments
/// 
/// * `flux` - Flux
/// * `fluxerr` - Flux error
/// * `zp` - Zero point
/// 
/// # Returns
/// 
/// * `f64` - Magnitude
/// * `f64` - Magnitude error
/// 
/// # Examples
/// 
/// ```
/// use flare::phot::flux_to_mag;
/// 
/// let flux = 36.307805;
/// let fluxerr = 3.344072;
/// let (mag, magerr) = flux_to_mag(flux, fluxerr, 23.9);
/// println!("mag: {}, magerr: {}", mag, magerr);
/// assert_eq!((mag - 20.).abs() < 1e-6, true);
/// assert_eq!((magerr - 0.1).abs() < 1e-6, true);
/// ```
pub fn flux_to_mag(flux: f64, fluxerr: f64, zp: f64) -> (f64, f64) {
    let mag = zp - 2.5 * (flux).log10();
    let magerr = FACTOR * fluxerr / flux;
    (mag, magerr)
}


/// Convert a limiting magnitude to a flux error
/// 
/// # Arguments
/// 
/// * `limmag` - Limiting magnitude
/// * `zp` - Zero point
/// * `sigma` - Sigma
/// 
/// # Returns
/// 
/// * `f64` - Flux error
/// 
/// # Examples
/// 
/// ```
/// use flare::phot::{limmag_to_fluxerr, ZP};
/// 
/// let limmag = 19.652575;
/// let sigma = 5.0;
/// let fluxerr = limmag_to_fluxerr(limmag, ZP, sigma);
/// println!("{}", fluxerr);
/// assert_eq!((fluxerr - 10.0).abs() < 1e-6, true);
/// ```
pub fn limmag_to_fluxerr(limmag: f64, zp: f64, sigma: f64) -> f64 {
    10.0_f64.powf((limmag - zp) / -2.5) / sigma
}

/// Convert a flux error to a limiting magnitude
/// 
/// # Arguments
/// 
/// * `fluxerr` - Flux error
/// * `zp` - Zero point
/// * `sigma` - Sigma
/// 
/// # Returns
/// 
/// * `f64` - Limiting magnitude
/// 
/// # Examples
/// 
/// ```
/// use flare::phot::{fluxerr_to_limmag, ZP};
/// 
/// let fluxerr = 10.0;
/// let sigma = 5.0;
/// let limmag = fluxerr_to_limmag(fluxerr, ZP, sigma);
/// assert_eq!((limmag - 19.652575).abs() < 1e-6, true);
/// println!("{}", limmag);
/// ```
pub fn fluxerr_to_limmag(fluxerr: f64, zp: f64, sigma: f64) -> f64 {
    -2.5 * (sigma * fluxerr).log10() + zp
}
