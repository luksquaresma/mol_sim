use crate::constants::Constants::AN;

#[derive(Clone, Debug, PartialEq)]
pub enum MolecularType {H2, H2O, CO, CO2, N2, O2}
impl MolecularType {
    pub fn get_mass(&self) -> f64 {
        return match MOLECULE_DOMAIN.iter().find(
            |m| m.molecular_type == *self
        ) {
            Some(molecule) => molecule.mass,
            None => panic!("Molecule not find!"), 
        }
    }
    pub fn get_polarity(&self) -> f64 {
        return match MOLECULE_DOMAIN.iter().find(
            |m| m.molecular_type == *self
        ) {
            Some(molecule) => molecule.polarity,
            None => panic!("Molecule not find!"), // Default value if Alice is not found
        }
    }
}


pub struct Molecule {
    pub molecular_type: MolecularType,
    pub mass:           f64,
    pub polarity:       f64
}


pub const MOLECULE_DOMAIN:[Molecule; 6] = {
    [
        Molecule {
            molecular_type: MolecularType::CO,
            mass:           ((12.01 + 16.)/1000.)/AN,
            polarity:       0.112*(3.33564e-30)
        },
        Molecule {
            molecular_type: MolecularType::CO2,
            mass:           ((12.01 + 2.*16.)/1000.)/AN,
            polarity:       0.*(3.33564e-30)
        },
        Molecule {
            molecular_type: MolecularType::H2,
            mass:           (1.008*2./1000.)/AN,
            polarity:       0.
        },
        Molecule {
            molecular_type: MolecularType::H2O,
            mass:           ((1.008*2. + 16.)/1000.)/AN,
            polarity:       1.85*(3.33564e-30)
        },
        Molecule {
            molecular_type: MolecularType::N2,
            mass:           ((2.*14.01)/1000.)/AN,
            polarity:       0.*(3.33564e-30)
        },
        Molecule {
            molecular_type: MolecularType::O2,
            mass:           ((2.*16.)/1000.)/AN,
            polarity:       0.*(3.33564e-30)
        }
    ]
};
