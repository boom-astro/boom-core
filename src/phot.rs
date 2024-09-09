pub const ZP: f64 = 23.9;
const FACTOR: f64 = 1.0857362047581294; // where 1.0857362047581294 = 2.5 / np.log(10)

/// Convert a magnitude to a flux
/// 
/// # Arguments
/// 
/// * `mag` - Magnitude
/// * `zp` - Zero point
/// 
/// # Returns
/// 
/// * `f64` - Flux
/// 
/// # Examples
/// 
/// ```
/// use boom_core::phot::mag_to_flux;
/// 
/// let mag = 20.0;
/// let flux = mag_to_flux(mag, 23.9);
/// println!("{}", flux);
/// assert_eq!((flux - 36.307805).abs() < 1e-6, true);
/// println!("{}", flux);
/// ```
/// 
/// ```
/// use boom_core::phot::{mag_to_flux, ZP};
/// 
/// let mag = 20.0;
/// let flux = mag_to_flux(mag, ZP);
/// assert_eq!((flux - 36.307805).abs() < 1e-6, true);
/// println!("{}", flux);
/// ```
pub fn mag_to_flux(mag: f64, zp: f64) -> f64 {
    10.0_f64.powf(-0.4 * (mag - zp))
}

/// Convert a magnitude error to a flux error
/// 
/// # Arguments
/// 
/// * `magerr` - Magnitude error
/// * `mag` - Magnitude
/// * `zp` - Zero point
/// 
/// # Returns
/// 
/// * `f64` - Flux error
/// 
/// # Examples
/// 
/// ```
/// use boom_core::phot::{magerr_to_fluxerr, ZP};
/// 
/// let magerr = 0.01;
/// let mag = 20.0;
/// let fluxerr = magerr_to_fluxerr(magerr, mag, ZP);
/// assert_eq!((fluxerr - 0.334407).abs() < 1e-6, true);
/// println!("{}", fluxerr);
/// ```
pub fn magerr_to_fluxerr(magerr: f64, mag: f64, zp: f64) -> f64 {
    magerr / FACTOR * mag_to_flux(mag, zp) // where 1.0857362047581294 = 2.5 / np.log(10)
}

/// Convert a flux to a magnitude
/// 
/// # Arguments
/// 
/// * `flux` - Flux
/// * `zp` - Zero point
/// 
/// # Returns
/// 
/// * `f64` - Magnitude
/// 
/// # Examples
/// 
/// ```
/// use boom_core::phot::flux_to_mag;
/// 
/// let flux = 110.0;
/// let mag = flux_to_mag(flux, 23.9);
/// assert_eq!((mag - 18.796518).abs() < 1e-6, true);
/// println!("{}", mag);
/// ```
pub fn flux_to_mag(flux: f64, zp: f64) -> f64 {
    zp - 2.5 * (flux).log10()
}

/// Convert a flux error to a magnitude error
/// 
/// # Arguments
/// 
/// * `fluxerr` - Flux error
/// * `flux` - Flux
/// 
/// # Returns
/// 
/// * `f64` - Magnitude error
/// 
/// # Examples
/// 
/// ```
/// use boom_core::phot::flux_to_magerr;
/// 
/// let fluxerr = 1.0e-10;
/// let flux = 1.0e-9;
/// let magerr = flux_to_magerr(fluxerr, flux);
/// assert_eq!((magerr - 0.108574).abs() < 1e-6, true);
/// println!("{}", magerr);
pub fn flux_to_magerr(fluxerr: f64, flux: f64) -> f64 {
    FACTOR * fluxerr / flux // where 1.0857362047581294 = 2.5 / np.log(10)
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
/// use boom_core::phot::{fluxerr_to_limmag, ZP};
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

/// Calculate the apparent magnitude of a target given its absolute magnitude and distance modulus
/// 
/// # Arguments
/// 
/// * `absmag` - Absolute magnitude
/// * `dm` - Distance modulus
/// 
/// # Returns
/// 
/// * `f64` - Apparent magnitude
/// 
/// # Examples
/// 
/// ```
/// use boom_core::phot::absmag_to_appmag;
/// use boom_core::cosmo::Plank18;
/// 
/// let absmag = 19.292;
/// let z = 0.0246;
/// let cosmology = Plank18::new();
/// let dm = cosmology.dm(z);
/// let appmag = absmag_to_appmag(absmag, dm);
/// assert_eq!((appmag + 15.935363).abs() < 1e-6, true);
/// println!("{}", appmag);
/// ```
pub fn absmag_to_appmag(absmag: f64, dm: f64) -> f64 {
    absmag - dm
}