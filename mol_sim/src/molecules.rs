use {
    crate::{
        constants::Constants::AN,
        states::StateVariables
    },
    serde::{
        Deserialize,
        Serialize
    },
    std::{
        collections::HashMap,
        ops::Deref
    },
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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



pub trait MoleculeData {
    fn print(&self) {}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MoleculeState {
    pub id:         u64,
    pub mol_type:   MolecularType,
    pub var:        HashMap<StateVariables, Vec<f64>>
}
impl MoleculeData for MoleculeState {
    fn print(&self) {
        println!();
        println!("MOLECULE ID: {:?}",        self.id);
        println!("MOLECULE TYPE: {:?}",      self.mol_type);
        println!("POSITION: {:.2?}",         self.var[&StateVariables::Position]);
        println!("VELOCITY: {:.2?}",         self.var[&StateVariables::Velocity]);
        println!("ORIENTATION: {:.2?}",      self.var[&StateVariables::Orientation]);
        println!("ANGULAR_VELOCITY: {:.2?}", self.var[&StateVariables::AngularVelocity]);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MoleculeDynamicState {
    pub parent: MoleculeState,
    pub t:  f64,
}
impl Deref for MoleculeDynamicState {
    type Target = MoleculeState;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
impl MoleculeData for MoleculeDynamicState {
    fn print(&self) {
        println!();
        println!("TIME {:?}", self.t);
        println!("MOLECULE ID: {:?}",        self.id);
        println!("MOLECULE TYPE: {:?}",      self.mol_type);
        println!("POSITION: {:.2?}",         self.var[&StateVariables::Position]);
        println!("VELOCITY: {:.2?}",         self.var[&StateVariables::Velocity]);
        println!("ORIENTATION: {:.2?}",      self.var[&StateVariables::Orientation]);
        println!("ANGULAR_VELOCITY: {:.2?}", self.var[&StateVariables::AngularVelocity]);
    }
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
