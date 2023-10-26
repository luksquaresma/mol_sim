use crate::molecule::MolecularType;

#[derive(Clone, Copy)]
pub struct Conditions {
    pub mn: u64,    // number of molecules - system proprierty
    pub molecule_type: MolecularType, // Molecule type identifiers

    // Molecular variables
    pub vel_ini: [[f64; 2]; 3], // velocity initializer (m/s)
    pub ori_ini: [[f64; 2]; 2], // orientation initializer (degrees)    
    pub ave_ini: [[f64; 2]; 2], // angular velocity initializer (degrees/s)

    // Domain variables
    pub pos_dom: [[f64; 2]; 3], // position domain (m)
}

#[derive(Clone, Copy)]
pub struct DomainlessConditions {
    pub mn: u64,    // number of molecules - system proprierty
    pub molecule_type: MolecularType, // Molecule type identifiers

    // Molecular variables
    pub vel_ini: [[f64; 2]; 3], // velocity initializer (m/s)
    pub ori_ini: [[f64; 2]; 2], // orientation initializer (degrees)    
    pub ave_ini: [[f64; 2]; 2], // angular velocity initializer (degrees/s)
}
impl DomainlessConditions {
    pub fn to_conditions(&self, pos_dom: [[f64; 2]; 3]) -> Conditions{
        return Conditions {
            mn:             self.mn,
            molecule_type:  self.molecule_type,
            vel_ini:        self.vel_ini,
            ori_ini:        self.ori_ini,
            ave_ini:        self.ave_ini,
            pos_dom
        }
    }
}



