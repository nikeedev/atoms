use std::fmt;
use colored::*;

#[allow(dead_code)]
#[allow(unused_variables)]

#[derive(Debug)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub const fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }
}

#[derive(Debug)]
pub enum MatterState {
    Gas,
    Liquid,
    Solid,
}

impl fmt::Display for MatterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatterState::Gas => write!(f, "Gas"),
            MatterState::Liquid => write!(f, "Liquid"),
            MatterState::Solid => write!(f, "Solid"),
        }
    }
}

#[derive(Debug)]
pub enum ElementState {
    Metall,
    Metalloid,
    Nonmetall,
}

impl fmt::Display for ElementState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ElementState::Metall => write!(f, "Metall"),
            ElementState::Metalloid => write!(f, "Metalloid (half-metall)"),
            ElementState::Nonmetall => write!(f, "Nonmetall"),
        }
    }
}

#[derive(Debug)]
pub struct Atom<'a> {
    pos: Vec2,
    name: &'a str,
    atom: &'a str,
    num: u32,
    valence_electrons: u32,
    protons: u32,
    neutrons: u32,
    matter_state: MatterState,
    element_state: ElementState,
}

impl fmt::Display for Atom<'_> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "\n{}\nName: {}\nPosition {{ \n\tx: {}\n\ty: {}\n}}\nAtom name: {}\nAtom number: {}\n\nValence eletrons: {}\nProtons: {}\nNeutrons: {}\nState of matter in 20°C: {}\nElement state/type: {}\n{}\n",
            "----------------------------------------------------------".green(),
            self.name.bright_green().bold(),
            self.pos.x,
            self.pos.y,
            self.atom.bright_purple(),
            self.num,
            self.valence_electrons,
            self.protons,
            self.neutrons,
            self.matter_state,
            self.element_state,
            "----------------------------------------------------------".green()
        )
    }
}

pub struct DisplayableVec<'a>(pub &'a [Atom<'a>]);

impl fmt::Display for DisplayableVec<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for atom in self.0 {
            writeln!(f, "{}", atom)?;
        }
        Ok(())
    }
}

enum NumberOfAtoms {
    Mono,
    Di,
    Tri,
    Tetra,
    Penta,
    Hexa,
    Hepta,
    Octa,
    Nona,
    Deca,
}

fn match_it<'a>(num: usize, atom_name: &str) -> String {
    let _num = match num {
        1 => "mono",
        2 => "di",
        3 => "tri",
        4 => "hexa",
        5 => "tetra",
        6 => "penta",
        7 => "hepta",
        8 => "octa",
        9 => "nona",
        10 => "deca",
        _ => ""
    };

    let _atom_name = match atom_name {
        "H" => "hydrogen",
        "He" => "helium",
        "Li" => "lithium",
        "O" => "oxide",
        &_ => ""
    };

    format!("{}{}", _num, _atom_name)
}

#[derive(Debug)]
pub struct Molecule<'a> {
    atoms: Vec<Atom<'a>>,
    name: String,
    total_electrons: usize,
}

impl fmt::Display for Molecule<'_> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "Molecule :D \n\nMolecule name:{}\nAtoms: \n{}\nTotal valence electrons: {}\n",
            self.name.bold(),
            DisplayableVec(&self.atoms),
            self.total_electrons
        )
    }
}
#[derive(Debug)]
pub struct Trash<'a> {
    atoms: Vec<Atom<'a>>,
    total_electrons: usize,
}

impl fmt::Display for Trash<'_> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "Trash :( \n\nAtoms {}\nTotal valence electrons: {}\n",
            DisplayableVec(&self.atoms),
            self.total_electrons
        )
    }
}

fn calculate_total_electrons(atoms: &[Atom]) -> usize {
    atoms.iter().map(|atom| atom.valence_electrons as usize).sum()
}

pub fn create_molecule(mut atoms: Vec<Atom>) -> Result<Molecule, Trash> {
    let total_electrons = calculate_total_electrons(&atoms);

    // Check if the total electrons fulfill the octet rule
    let can_form_molecule = total_electrons % 8 == 0;

    atoms.sort_by(|a, b| a.atom.cmp(b.atom));

    // If atoms fulfill the octet rule, create the molecule
    if can_form_molecule {
        let mut molecule_name: String = String::new();

        let mut repeating_elements = 0;
        let mut _repeating_atom: String = String::new();
    
        let mut prev_atom: Option<&str> = Some(atoms.first().unwrap().atom);
        
        for (i, atom) in atoms.iter().enumerate() {
            // dbg!(atom);
            // dbg!(prev_atom);
            // dbg!(_repeating_atom.clone());
            // dbg!(repeating_elements);
        
            if let Some(prev) = prev_atom {
                if atom.atom == prev {
                    repeating_elements += 1;
                } else {
                    molecule_name.push_str(format!(" {}", match_it(repeating_elements, prev)).as_str());
                    repeating_elements = 1; // Set to 1 for the current different atom
                }
            } else {
                repeating_elements = 1; // Set to 1 for the first atom
            }
            prev_atom = Some(atom.atom);
        
            // Add handling for the last atom to update molecule_name
            if i == atoms.len() - 1 {
                molecule_name.push_str(format!(" {}", match_it(repeating_elements, prev_atom.unwrap())).as_str());
            }
        }
        // Assuming the molecule name is "MoleculeName" for demonstration purposes
        let molecule = Molecule {
            atoms,
            name: molecule_name,
            total_electrons,
        };
        Ok(molecule)
    } else {
        // Return a Trash struct containing the atoms and total electrons
        let trash = Trash {
            atoms,
            total_electrons,
        };
        Err(trash)
    }
}

pub const HYDROGEN: Atom = Atom {
    pos: Vec2::new(0, 0),
    name: "Hydrogen",
    atom: "H",
    num: 1,
    valence_electrons: 1,
    protons: 1,
    neutrons: 0,
    matter_state: MatterState::Gas,
    element_state: ElementState::Nonmetall,
};

pub const HELIUM: Atom = Atom {
    pos: Vec2::new(0, 0),
    name: "Helium",
    atom: "He",
    num: 2,
    valence_electrons: 2,
    protons: 2,
    neutrons: 2,
    matter_state: MatterState::Gas,
    element_state: ElementState::Nonmetall,
};

pub const LITHIUM: Atom = Atom {
    pos: Vec2::new(0, 0),
    name: "Lithium",
    atom: "Li",
    num: 3,
    valence_electrons: 1,
    protons: 3,
    neutrons: 4,
    matter_state: MatterState::Solid,
    element_state: ElementState::Metall,
};

pub const OXYGEN: Atom = Atom {
    pos: Vec2::new(0, 0),
    name: "Oxygen",
    atom: "O",
    num: 8,
    valence_electrons: 6,
    protons: 8,
    neutrons: 8,
    matter_state: MatterState::Gas,
    element_state: ElementState::Nonmetall,
};
