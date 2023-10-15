use crate::molecule::MolecularType;

pub struct Conditions {
    pub mn: u64,    // number of molecules - system proprierty
    pub molecule_type: MolecularType, // Molecule type identifiers

    // Molecular variables
    pub pos_dom: [[f64; 2]; 3], // position domain (m)
    pub vel_ini: [[f64; 2]; 3], // velocity initializer (m/s)
    pub ori_ini: [[f64; 2]; 2], // orientation initializer (degrees)    
    pub ave_ini: [[f64; 2]; 2], // angular velocity initializer (degrees/s)
}

