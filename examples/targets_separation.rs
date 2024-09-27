use flare::Target;

fn main() {
    let target1 = Target::new(6.374817, 20.242942, Some("A"));
    let target2 = Target::new(6.374817, 21.242942, Some("B"));

    let separation = target1.separation(&target2);

    println!("The angular separation between the two targets is: {}", separation);
}