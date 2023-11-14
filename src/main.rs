mod atoms;

fn main() {
    let mut _atoms = vec![atoms::HYDROGEN, atoms::HYDROGEN, atoms::OXYGEN];
    
    println!("\n\n\n\nWater: \n{:#?}", atoms::create_molecule(_atoms));
}
