// Video tags: rust, model, computational, modeling, modelling, simulator, simulation, atom, atomic, modelucle, molecular, learning, code 

/// TODO LIST
/// | | Create molecules based on a selection of each molecule type, 
///     so there is only one molecule propriety declaration
///
/// | | Create InverseState for printing molecular information using
///     the same standards as pandas.DataFrame.from_dict() 
/// 
/// | | Create method to save the created/processed data
///     | | History definition (Vec!<State>) - State(time)
///     | | Save files in an organized and generalized way
/// 
/// | | Create the system iteraction methods and funcitons
///     | | System itself (dynamic velocity/position variations)
///     | | Force/iteractions calculations



// Packages
use std::collections::HashMap;
// use polars;


// Constant dfinitions
pub const PI:f64 = 3.14159;
pub const C0:f64 = 299792458.; // (m/s)
pub const AN:f64 = 6.02214076e+023; // (m/s)


// Processing defs
pub const DT:f64 = 1./100.;
pub const MOLECULE_NUMBER:    u64 = 5;
pub const POSITION_DOMAIN:    [[f64; 2]; 3] = [[ 0.,  1.], [ 0.,  2.], [ 5.,  6.]]; //[[x_min, x_max], [y_min, y_max]...]
pub const VELOCITY_INIT:      [[f64; 2]; 3] = [[-1.,  1.], [-2.,  2.], [ 2.,  6.]]; //[[x/s_min, x_max], [y/s_min, y_max]...]
// Theta contained in [0, 1] == [0, 2*pi]rad;
// Phy is contained in [0, 1] == [0, pi]rad
pub const ORIENTATION_INIT:   [[f64; 2]; 2] = [[ 0.,  1.], [ 0.,  1.]]; //[[theta_min, theta_max], [phi_min, phi_max]]
pub const ANGULAR_VEL_INIT:   [[f64; 2]; 2] = [[ 0.,  1.], [ 0.,  1.]]; //[[theta/s_min, theta/s_max], [phi/s_min, phi/s_max]]

// -------------------------------------------------------------------------------
// Types of molecules and its proprieties
#[derive(Clone, Debug, PartialEq)]
pub enum MolecularType{H2, H2O, CO, CO2, O2, N2}
impl MolecularType {
    fn get_mass(&self) -> f64 {
        return match MOLECULE_DOMAIN.iter().find(
            |m| m.molecular_type == *self
        ) {
            Some(molecule) => molecule.mass,
            None => panic!("Molecule not find!"), 
        }
    }
    fn get_polarity(&self) -> f64 {
        return match MOLECULE_DOMAIN.iter().find(
            |m| m.molecular_type == *self
        ) {
            Some(molecule) => molecule.polarity,
            None => panic!("Molecule not find!"), // Default value if Alice is not found
        }
    }
}

// List of molecule proprieties
pub struct Molecule {
    pub molecular_type: MolecularType,
    pub mass:           f64,
    pub polarity:       f64
}
pub const MOLECULE_DOMAIN: [Molecule; 2] = {
    [
        Molecule {
            molecular_type: MolecularType::H2,
            mass:           (1.008*2./1000.)/AN,
            polarity:       0.
        },
        Molecule {
            molecular_type: MolecularType::H2O,
            mass:           ((1.008*2. + 16.)/1000.)/AN,
            polarity:       1.85*(3.33564e-30)
        }
    ]
};

// -------------------------------------------------------------------------------
// Simulation conditions
pub struct Conditions {
    pub mn: u64,    // number of molecules - system proprierty
    pub molecule_type: MolecularType, // Molecule type identifiers

    // Molecular variables
    pub pos_dom: [[f64; 2]; 3], // position domain (m)
    pub vel_ini: [[f64; 2]; 3], // velocity initializer (m/s)
    pub ori_ini: [[f64; 2]; 2], // orientation initializer (degrees)    
    pub ave_ini: [[f64; 2]; 2], // angular velocity initializer (degrees/s)
}

// -------------------------------------------------------------------------------
// State 
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum StateVariables {Position, Velocity, Orientation, AngularVelocity}
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum StateIntrinsic {Mass, Polarity}
    // State could be either a current state or a history of states,
    // hence the generic types
pub struct State {
    pub var: HashMap<StateVariables,  Vec<Vec<f64>>>, // Change with t
    pub pro: HashMap<StateIntrinsic,  Vec<f64>>,      // Dosen't change with t
    pub ids: Vec<MolecularType>,                      // Dosen't change with t
}
pub struct History {
    pub var: HashMap<StateVariables, Vec<Vec<Vec<f64>>>>,
    pub pro: HashMap<StateIntrinsic, Vec<f64>>,
    pub ids: Vec<MolecularType>,
}
// impl History {
//     fn update(&self, state:State) {
//         for (k, v) in self.var {
//             v.push(
//                 match state.var.get(&k) {
//                     Some(cur) => *cur, 
//                     None => panic!()
//                 }
//             );
//         }
//     }
// }

fn create_molecules(cs:Conditions) -> State {
    let (mn, pd, vi, oi, ai) = (cs.mn, cs.pos_dom, cs.vel_ini, cs.ori_ini, cs.ave_ini);
    return State {
        var: HashMap::from([
            (
                StateVariables::Position, (0..mn).map(|_| vec![
                    pd[0][0] + (pd[0][1] - pd[0][0])*(rand::random::<f64>()), // x
                    pd[1][0] + (pd[1][1] - pd[1][0])*(rand::random::<f64>()), // y
                    pd[2][0] + (pd[2][1] - pd[2][0])*(rand::random::<f64>()) // z
                    ]).collect::<Vec<Vec<f64>>>()
            ),
            (
                StateVariables::Velocity, (0..mn).map(|_| vec![
                    vi[0][0] + (vi[0][1] - vi[0][0])*(rand::random::<f64>()), // x
                    vi[1][0] + (vi[1][1] - vi[1][0])*(rand::random::<f64>()), // y
                    vi[2][0] + (vi[2][1] - vi[2][0])*(rand::random::<f64>()) // z
                    ]).collect::<Vec<Vec<f64>>>()
            ),
            (
                StateVariables::Orientation, (0..mn).map(|_| vec![
                    oi[0][0] + (oi[0][1] - oi[0][0])*(rand::random::<f64>()), // theta
                    oi[1][0] + (oi[1][1] - oi[1][0])*(rand::random::<f64>()), // phi
                    ]).collect::<Vec<Vec<f64>>>()
            ),
            (
                StateVariables::AngularVelocity, (0..mn).map(|_| vec![
                    ai[0][0] + (ai[0][1] - ai[0][0])*(rand::random::<f64>()), // theta
                    ai[1][0] + (ai[1][1] - ai[1][0])*(rand::random::<f64>()), // phi
                    ]).collect::<Vec<Vec<f64>>>()
            )
        ]),
        pro: HashMap::from([
            (
                StateIntrinsic::Mass, (0..mn).map(
                    |_| cs.molecule_type.get_mass()
                ).collect::<Vec<f64>>()
            ),
            (
                StateIntrinsic::Polarity, (0..mn).map(
                    |_| cs.molecule_type.get_polarity()
                ).collect::<Vec<f64>>()
            )
        ]),
        ids: (0..mn).map(|_| cs.molecule_type.clone()).collect::<Vec<MolecularType>>()
    }
}

fn main() {
    let state_h2 = create_molecules(
        Conditions{
            mn:      MOLECULE_NUMBER,
            molecule_type: MolecularType::H2,
            pos_dom: POSITION_DOMAIN,
            vel_ini: VELOCITY_INIT,
            ori_ini: ORIENTATION_INIT,
            ave_ini: ANGULAR_VEL_INIT
        }
    );

    let state_h2o = create_molecules(
        Conditions{
            mn:      MOLECULE_NUMBER,
            molecule_type: MolecularType::H2O,
            pos_dom: POSITION_DOMAIN,
            vel_ini: VELOCITY_INIT,
            ori_ini: ORIENTATION_INIT,
            ave_ini: ANGULAR_VEL_INIT
        }
    );

    for state in [state_h2, state_h2o] {
        println!("Molecules:");
        println!("{:?}", state.ids);
        for (k, v) in state.pro.iter() {
            println!("KEY: {:?}", k);
            println!("{:?}", v)
            // println!("{:.2?}", v)
        };
        for (k, v) in state.var.iter() {
            println!("KEY: {:?}", k);
            println!("{:.2?}", v)
            // println!("{:.#2?}", v)
        };
    };
    // // assert!(state.get_contents() == state.get_keys(), "Test is NOT WORKING");
}