use {
    crate::{
        constant::AN,
        state::StateVariables
    },
    serde::{
        Deserialize,
        Serialize
    },
    // serde_json,
    std::{
        collections::HashMap,
        fs::write,
        iter::Iterator
    },
    strum::{
        EnumIter,
        IntoEnumIterator,
    },
};

#[derive(Clone, Copy, Debug, Deserialize, EnumIter, PartialEq, Serialize)]
pub enum MolecularType {H2, H2O, CO, CO2, N2, O2}
impl MolecularType {
    pub fn molecule_from_type(&self) -> Molecule {
        // MolecularType::molecule_from_type(MolecularType::H2)
        return match self {
            MolecularType::CO => Molecule {
                molecular_type: *self,
                mass:           ((12.01 + 16.)/1000.)/AN,
                polarity:       0.112*(3.33564e-30)
                },
            MolecularType::CO2 => Molecule {
                molecular_type: *self,
                mass:           ((12.01 + 2.*16.)/1000.)/AN,
                polarity:       0.*(3.33564e-30)
            },
            MolecularType::H2 => Molecule {
                molecular_type: *self,
                mass:           (1.008*2./1000.)/AN,
                polarity:       0.
            },
            MolecularType::H2O =>  Molecule {
                molecular_type: *self,
                mass:           ((1.008*2. + 16.)/1000.)/AN,
                polarity:       1.85*(3.33564e-30)
            },
            MolecularType::N2 => Molecule {
                molecular_type: *self,
                mass:           ((2.*14.01)/1000.)/AN,
                polarity:       0.*(3.33564e-30)
            },
            MolecularType::O2 => Molecule {
                molecular_type: *self,
                mass:           ((2.*16.)/1000.)/AN,
                polarity:       0.*(3.33564e-30)
            }
        };
    }
    pub fn name(&self) -> &str {
        return match self {
            MolecularType::H2   => "H2",
            MolecularType::H2O  => "H2O",
            MolecularType::CO   => "CO",
            MolecularType::CO2  => "CO2",
            MolecularType::N2   => "N2",
            MolecularType::O2   => "O2"
        }
    }
    pub fn get_mass(&self) -> f64 {
        return MolecularType::molecule_from_type(&self).mass; 
    }
    pub fn get_polarity(&self) -> f64 {
        return MolecularType::molecule_from_type(&self).polarity; 
    }
    pub fn save_standard_data(save_path:&str) {
        write(
            save_path, 
            &serde_json::to_string_pretty(
                &MolecularType::iter().map(
                    |mol_typ| MolecularType::molecule_from_type(&mol_typ)
                ).collect::<Vec<Molecule>>()
            ).unwrap()
        ).expect("Unable to write json file");
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
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
    pub t:          f64,
    pub id:         u64,
    pub mol_type:   MolecularType,
    pub var:        HashMap<StateVariables, Vec<f64>>
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



