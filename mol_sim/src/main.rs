// Video tags: rust, model, computational, modeling, modelling, simulator, simulation, atom, atomic, modelucle, molecular, learning, code

// Modules defined on other files
pub mod conditions;
pub mod constants;
pub mod molecules;
pub mod states;


// Packages
use crate::{
        molecules::MolecularType,
        conditions::Conditions,
        states::{
            History, 
            State
        }
    };


// Processing defs
pub const DT:f64 = 1./100.;
pub const MOLECULE_NUMBER:u64 = 100000;
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
    let new_state:states::State = State::from_vec_coupling(vec![
        State::create_randomly_from_intervals(
            Conditions{
                mn:      MOLECULE_NUMBER,
                molecule_type: MolecularType::CO,
                pos_dom: POSITION_DOMAIN,
                vel_ini: VELOCITY_INIT,
                ori_ini: ORIENTATION_INIT,
                ave_ini: ANGULAR_VEL_INIT
            }
        ),
        State::create_randomly_from_intervals(
            Conditions{
                mn:      MOLECULE_NUMBER,
                molecule_type: MolecularType::CO2,
                pos_dom: POSITION_DOMAIN,
                vel_ini: VELOCITY_INIT,
                ori_ini: ORIENTATION_INIT,
                ave_ini: ANGULAR_VEL_INIT
            }
        ),
        State::create_randomly_from_intervals(
            Conditions{
                mn:      MOLECULE_NUMBER,
                molecule_type: MolecularType::H2,
                pos_dom: POSITION_DOMAIN,
                vel_ini: VELOCITY_INIT,
                ori_ini: ORIENTATION_INIT,
                ave_ini: ANGULAR_VEL_INIT
            }
        ),
        State::create_randomly_from_intervals(
            Conditions{
                mn:      MOLECULE_NUMBER,
                molecule_type: MolecularType::H2O,
                pos_dom: POSITION_DOMAIN,
                vel_ini: VELOCITY_INIT,
                ori_ini: ORIENTATION_INIT,
                ave_ini: ANGULAR_VEL_INIT
            }
        ),
        State::create_randomly_from_intervals(
            Conditions{
                mn:      MOLECULE_NUMBER,
                molecule_type: MolecularType::N2,
                pos_dom: POSITION_DOMAIN,
                vel_ini: VELOCITY_INIT,
                ori_ini: ORIENTATION_INIT,
                ave_ini: ANGULAR_VEL_INIT
            }
        ),
        State::create_randomly_from_intervals(
            Conditions{
                mn:      MOLECULE_NUMBER,
                molecule_type: MolecularType::O2,
                pos_dom: POSITION_DOMAIN,
                vel_ini: VELOCITY_INIT,
                ori_ini: ORIENTATION_INIT,
                ave_ini: ANGULAR_VEL_INIT
            }
        )
    ]);


    let mut new_history = History::create(&new_state);
    new_history.update(&new_state, &1.);


    new_history.to_json(&"/home/luks/Projects/mol_sim/data/test.json");
    new_history.save_as_polars_df(&"/home/luks/Projects/mol_sim/data/test.parquet");
    
    MolecularType::save_standard_data("/home/luks/Projects/mol_sim/data/std_mol_data.json")

    // assert!(state.get_contents() == state.get_keys(), "Test is NOT WORKING");
}