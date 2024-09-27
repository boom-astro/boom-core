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