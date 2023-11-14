use std::fmt;

#[allow(dead_code)]
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

#[derive(Debug)]
pub enum MetallState {
    Metall,
    Metalloid,
    Nonmetall,
}

#[derive(Debug)]
pub struct Atom<'a> {
    pos: Vec2,
    name: &'a str,
    atom: &'a str,
    num: u32,
    electrons: u32,
    protons: u32,
    neutrons: u32,
    matter_stater: MatterState,
    metall_state: MetallState,
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
            "Position: \n\tx: {}\n\ty: {}\nName: {}\nAtom: {}\nAtom number: {}\n\nEletrons: {}\nProtons: {}\nNeutrons: {}\nState of matter in 20°C: {}\n",
            self.pos.x,
            self.pos.y,
            self.name,
            self.atom,
            self.num,
            self.electrons,
            self.protons,
            self.neutrons,
            self.matter_stater,
            self.metall_state
        )
    }
}

#[derive(Debug)]
pub struct Molecule<'a> {
    atoms: Vec<Atom<'a>>,
    name: &'a str,
}

pub const HYDROGEN: Atom = Atom {
    pos: Vec2::new(0, 0),
    name: "Hydrogen",
    atom: "H",
    num: 1,
    electrons: 1,
    protonts: 1,
    neutrons: 0,
    matter_stater: MatterState::Gas,
    metall_state: MetallState::Nonmetall,
};

pub const HELIUM: Atom = Atom {
    pos: Vec2::new(0, 0),
    name: "Helium",
    atom: "He",
    num: 2,
    electrons: 1,
    protonts: 2,
    neutrons: 2,
    matter_stater: MatterState::Gas,
    metall_state: MetallState::Nonmetall,
};

pub const LITHIUM: Atom = Atom {
    pos: Vec2::new(0, 0),
    name: "Helium",
    atom: "He",
    num: 2,
    electrons: 1,
    protonts: 2,
    neutrons: 2,
    matter_stater: MatterState::Gas,
    metall_state: MetallState::Nonmetall,
};
