mod atoms;

fn main() {
    let mut _atoms = vec![
        atoms::HYDROGEN,
        atoms::HYDROGEN,
        atoms::OXYGEN
        ];

    match atoms::create_molecule(_atoms) {
        Ok(molecule) => {
            // Handle successful molecule creation
            println!("\nWater: \n{}", molecule);
        } 
        Err(trash) => {
            // Handle the case where atoms didn't fulfill the octet rule
            println!("\nWater: \n{}", trash);
        }
    }
}
