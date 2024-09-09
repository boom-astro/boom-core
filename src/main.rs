fn main() {
    println!("Hello, world!");
    let z = 0.0246;
    let cosmology = boom_core::cosmo::Plank18::new();
    let lumdist = cosmology.luminosity_distance(z);
    println!("Luminosity distance: {:.2} Mpc", lumdist);
}