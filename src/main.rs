mod atoms;

fn main() {
    let mut _atoms = vec![atoms::HYDROGEN, atoms::HYDROGEN, atoms::OXYGEN];
    
    println!("\nWater: \n{}", atoms::create_molecule(_atoms).unwrap());
}
