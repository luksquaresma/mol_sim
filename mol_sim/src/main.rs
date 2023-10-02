// Video tags: rust, model, computational, modeling, modelling, simulator, simulation, atom, atomic, modelucle, molecular, learning, code 


/// TODO LIST
/// | | Create molecules based on a selection of each molecule type, 
///     so there is only one molecule propriety declaration
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
// use polars::df;

// Constant dfinitions
pub const PI:f64 = 3.14159;
pub const C0:f64 = 299792458.; // m/s
pub const AN:f64 = 6.02214076e+023; // m/s


// Processing defs
pub const DT:f64 = 1./100.;
pub const MOLECULE_NUMBER:    u64 = 5;
pub const POSITION_DOMAIN:    [[f64; 2]; 3] = [[ 0.,  1.], [ 0.,  2.], [ 5.,  6.]]; //[[x_min, x_max], [y_min, y_max]...]
pub const VELOCITY_INIT:      [[f64; 2]; 3] = [[-1.,  1.], [-2.,  2.], [ 2.,  6.]]; //[[x/s_min, x_max], [y/s_min, y_max]...]
// Theta contained in [0, 1] == [0, 2*pi]rad;
// Phy is contained in [0, 1] == [0, pi]rad
pub const ORIENTATION_INIT:   [[f64; 2]; 2] = [[ 0.,  1.], [ 0.,  1.]]; //[[theta_min, theta_max], [phi_min, phi_max]]
pub const ANGULAR_VEL_INIT:   [[f64; 2]; 2] = [[ 0.,  1.], [ 0.,  1.]]; //[[theta/s_min, theta/s_max], [phi/s_min, phi/s_max]]

// Simulation conditions structure
pub struct MolecularState {
    pub mn: u64,    // number of molecules - system proprierty

    // Molecular identifiers
    pub molecule_type: String, // String with the type

    // Molecular proprieties
    pub mass:     f64,    // (kg)
    pub polarity: f64,    // (C*m)

    // Molecular variables
    pub pos_dom: [[f64; 2]; 3], // position domain (m)
    pub vel_ini: [[f64; 2]; 3], // velocity initializer (m/s)
    pub ori_ini: [[f64; 2]; 2], // orientation initializer (degrees)    
    pub ave_ini: [[f64; 2]; 2], // angular velocity initializer (degrees/s)
}


// // State structure - {T is always a type Vec<something>}
// pub trait DataStructure<T> {    
//     fn get_keys(&self) -> Vec<String>; // Returns a list of keys for all the internal data
//     fn get_value(&self, k:&String) -> &T; // Returns the item which match a key

//     // Returns a vector of tuples that contins all the keys and values in the form (key, value).
//     fn get_contents(&self) -> Vec<(String, T)>;

//     // // Sets the item which match a key
//     // fn set_value(&self, k:&String, v:T);
// }

pub struct State<T,U, V> {
    var: HashMap<String, T>,
    pro: HashMap<String, U>,
    ide: HashMap<String, V>,
}

// impl<T: Clone> State<T> {
//     fn get_keys(&self) -> Vec<String> {
//         return vec![
//             "pos".to_string(), 
//             "vel".to_string(), 
//             "ori".to_string(), 
//             "ave".to_string()
//             ]
//     }
//     fn get_value(&self, k:&String) -> &T {
//         return match k.as_str() {
//             "pos" => &self.pos,
//             "vel" => &self.vel,
//             "ori" => &self.ori,
//             "ave" => &self.ave,
//             &_ => todo!()
//         }
//     }  
//     fn get_contents(&self) -> Vec<(String, T)> {
//         return self.get_keys().iter().map(
//             |k| (k.clone(), (*self.get_value(k)).clone())
//         ).collect()
//     }
// }


fn create_molecules(cs:MolecularState) -> State<Vec<Vec<f64>>, Vec<f64>, Vec<String>> {
    let (mn, pd, vi, oi, ai) = (cs.mn, cs.pos_dom, cs.vel_ini, cs.ori_ini, cs.ave_ini);
    return State {
        var: HashMap::from([
            (
                "pos".to_string(), (0..mn).map(|_| vec![
                    pd[0][0] + (pd[0][1] - pd[0][0])*(rand::random::<f64>()), // x
                    pd[1][0] + (pd[1][1] - pd[1][0])*(rand::random::<f64>()), // y
                    pd[2][0] + (pd[2][1] - pd[2][0])*(rand::random::<f64>()) // z
                    ]).collect::<Vec<Vec<f64>>>()
            ),
            (
                "vel".to_string(), (0..mn).map(|_| vec![
                    vi[0][0] + (vi[0][1] - vi[0][0])*(rand::random::<f64>()), // x
                    vi[1][0] + (vi[1][1] - vi[1][0])*(rand::random::<f64>()), // y
                    vi[2][0] + (vi[2][1] - vi[2][0])*(rand::random::<f64>()) // z
                    ]).collect::<Vec<Vec<f64>>>()
            ),
            (
                "ori".to_string(), (0..mn).map(|_| vec![
                    oi[0][0] + (oi[0][1] - oi[0][0])*(rand::random::<f64>()), // theta
                    oi[1][0] + (oi[1][1] - oi[1][0])*(rand::random::<f64>()), // phi
                    ]).collect::<Vec<Vec<f64>>>()
            ),
            (
                "ave".to_string(), (0..mn).map(|_| vec![
                    ai[0][0] + (ai[0][1] - ai[0][0])*(rand::random::<f64>()), // theta
                    ai[1][0] + (ai[1][1] - ai[1][0])*(rand::random::<f64>()), // phi
                    ]).collect::<Vec<Vec<f64>>>()
            )
        ]),
        pro: HashMap::from([
            (
                "pol".to_string(), (0..mn).map(|_| cs.polarity).collect::<Vec<f64>>()
            ),
            (
                "mas".to_string(), (0..mn).map(|_| cs.mass).collect::<Vec<f64>>()
            )
        ]),
        ide: HashMap::from([
            (
                "id".to_string(), (0..mn).map(|_| cs.molecule_type.clone()).collect::<Vec<String>>()
            )
        ])
    }
}

fn main() {
    let starting_cond_h2 = MolecularState{
        mn:      MOLECULE_NUMBER,

        // Molecular identifiers
        molecule_type: "H2".to_string(),

        // Molecular proprieties
        mass:          (1.008*2./1000.)/AN,
        polarity:      0.,

        // Molecular variables
        pos_dom: POSITION_DOMAIN,
        vel_ini: VELOCITY_INIT,
        ori_ini: ORIENTATION_INIT,
        ave_ini: ANGULAR_VEL_INIT
    };
    let state_h2 = create_molecules(starting_cond_h2);


    let starting_cond_h2o = MolecularState{
        mn:      MOLECULE_NUMBER,

        // Molecular identifiers
        molecule_type: "H2O".to_string(),

        // Molecular proprieties
        mass:          ((1.008*2. + 16.)/1000.)/AN,
        polarity:      1.85*(3.33564e-30), // C*m
        
        // Molecular variables
        pos_dom: POSITION_DOMAIN,
        vel_ini: VELOCITY_INIT,
        ori_ini: ORIENTATION_INIT,
        ave_ini: ANGULAR_VEL_INIT
    };
    let state_h2o = create_molecules(starting_cond_h2o);


    println!("H2 molecules:");
    for (k, v) in state_h2.ide.iter() {
        println!("KEY: {}",k);
        println!("{:?}", v)
        // println!("{:.2?}", v)
    };

    println!("H2O molecules:");
    for (k, v) in state_h2o.ide.iter() {
        println!("KEY: {}",k);
        println!("{:?}", v)
        // println!("{:.2?}", v)
    };

    // for (k, v) in state.get_contents() {
    //     println!("KEY: {}",k);
    //     println!("{:.2?}", v)
    // };
    // assert!(state.get_contents() == state.get_keys(), "Test is NOT WORKING");
}