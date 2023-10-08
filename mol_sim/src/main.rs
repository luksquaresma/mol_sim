// Video tags: rust, model, computational, modeling, modelling, simulator, simulation, atom, atomic, modelucle, molecular, learning, code

/// TODO LIST
/// |X| Create molecules based on a selection of each molecule type, 
///     so there is only one molecule propriety declaration
///
/// 
/// | | Create method to save the created/processed data
///     | | History definition (Vec!<State>) - State(time)
///     | | Save files in an organized and generalized way
/// 
/// | | Create the system iteraction methods and funcitons
///     | | System itself (dynamic velocity/position variations)
///     | | Force/iteractions calculations
/// 

// Modules defined on other files
pub mod conditions;
pub mod constants;
pub mod molecules;
pub mod states;



// Packages
// use json::{JsonValue, object};
use crate::{
    molecules::MolecularType,
    conditions::Conditions,
    states::State,
    states::History
};


// Processing defs
pub const DT:f64 = 1./100.;
pub const MOLECULE_NUMBER:u64 = 1;
pub const POSITION_DOMAIN:[[f64; 2]; 3] = [
    [ 0.,  1.], // [x_min, x_max]
    [ 0.,  2.], // [y_min, y_max]
    [ 5.,  6.]  // [z_min, z_max]
    ]; 
pub const VELOCITY_INIT:[[f64; 2]; 3] = [
    [-1.,  1.], // [x/s_min, x/s_max]
    [-2.,  2.], // [y/s_min, y/s_max]
    [ 2.,  6.]  // [z/s_min, z/s_max]
    ]; 
pub const ORIENTATION_INIT:[[f64; 2]; 2] = [
    [ 0.,  1.], // [theta_min, theta_max] - Theta contained in  [0, 1] == [0, 2*pi]rad;
    [ 0.,  1.]  // [phi_min, phi_max] - Phy is contained in [0, 1] == [0,   pi]rad
    ];
pub const ANGULAR_VEL_INIT:[[f64; 2]; 2] = [
    [ 0.,  1.], 
    [ 0.,  1.]
    ]; 


fn main() {
    let state_h2 = State::create_randomly_from_intervals(
        Conditions{
            mn:      MOLECULE_NUMBER,
            molecule_type: MolecularType::H2,
            pos_dom: POSITION_DOMAIN,
            vel_ini: VELOCITY_INIT,
            ori_ini: ORIENTATION_INIT,
            ave_ini: ANGULAR_VEL_INIT
        }
    );

    let state_h2o = State::create_randomly_from_intervals(
        Conditions{
            mn:      MOLECULE_NUMBER,
            molecule_type: MolecularType::H2O,
            pos_dom: POSITION_DOMAIN,
            vel_ini: VELOCITY_INIT,
            ori_ini: ORIENTATION_INIT,
            ave_ini: ANGULAR_VEL_INIT
        }
    );
    let state_co = State::create_randomly_from_intervals(
        Conditions{
            mn:      MOLECULE_NUMBER,
            molecule_type: MolecularType::CO,
            pos_dom: POSITION_DOMAIN,
            vel_ini: VELOCITY_INIT,
            ori_ini: ORIENTATION_INIT,
            ave_ini: ANGULAR_VEL_INIT
        }
    );
    let state_co2 = State::create_randomly_from_intervals(
        Conditions{
            mn:      MOLECULE_NUMBER,
            molecule_type: MolecularType::CO2,
            pos_dom: POSITION_DOMAIN,
            vel_ini: VELOCITY_INIT,
            ori_ini: ORIENTATION_INIT,
            ave_ini: ANGULAR_VEL_INIT
        }
    );
    // let new_state = state_h2.couple(state_h2o);
    // let new_state = state_h2.from_vec_coupling(
    //     vec![state_h2o, state_co, state_co2]
    // );
    let new_state = State::from_vec_coupling(
        vec![state_h2, state_h2o, state_co, state_co2]
    );
    let inverted_new_state = new_state.invert();

    let mut new_history = History::create(&new_state);
    new_history.update(&new_state, &1.2);

    for hist in [new_history] {
        println!("Molecules:");
        println!("{:?}", hist.ids);
        for (k, v) in hist.pro.iter() {
            println!("KEY: {:?}", k);
            println!("{:?}", v)
            // println!("{:.2?}", v)
        };
        for (k, v) in hist.var.iter() {
            println!("KEY: {:?}", k);
            for (i, vv) in v.iter().enumerate() {
                print!("time == {}", hist.time[i]);
                println!("{:.2?}", vv);
            }
        };
    };

    

    // for i_state in [inverted_new_state] {
    //     for m in i_state.iter() {
    //         println!();
    //         println!("MOLECULE ID: {:?}", m.id);
    //         println!("MOLECULE TYPE: {:?}", m.mol_type);
    //         println!("POSITION: {:.2?}", m.var[&StateVariables::Position]);
    //         println!("VELOCITY: {:.2?}", m.var[&StateVariables::Velocity]);
    //         println!("ORIENTATION: {:.2?}", m.var[&StateVariables::Orientation]);
    //         println!("ANGULAR_VELOCITY: {:.2?}", m.var[&StateVariables::AngularVelocity]);
    //     };
    // };








    // for state in [new_state] {
    //     println!("Molecules:");
    //     println!("{:?}", state.ids);
    //     for (k, v) in state.pro.iter() {
    //         println!("KEY: {:?}", k);
    //         println!("{:?}", v)
    //         // println!("{:.2?}", v)
    //     };
    //     for (k, v) in state.var.iter() {
    //         println!("KEY: {:?}", k);
    //         println!("{:.2?}", v)
    //         // println!("{:.#2?}", v)
    //     };
    // };
    // // assert!(state.get_contents() == state.get_keys(), "Test is NOT WORKING");
}