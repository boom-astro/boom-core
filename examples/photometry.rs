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