use std::iter::FlatMap;

use polars::prelude::DataFrame;

use {
    crate::{
        molecules::{
            MoleculeData,
            MoleculeState,
            MoleculeDynamicState,
            MolecularType
        },
        conditions::Conditions
    },
    polars::prelude::*,
    serde::{
        Deserialize,
        Serialize
    },
    serde_json,
    std::{
        collections::HashMap,
        fs::write
    },
};

pub trait Data<T: MoleculeData> {
    fn invert(&self) -> Vec<T>;
    fn invert_and_print(&self) {
        for ms in self.invert().iter() {
            ms.print()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum StateVariables {Position, Velocity, Orientation, AngularVelocity}
impl StateVariables {
    pub fn name(&self) -> &str {
        return match self {
            StateVariables::Position        => "Position",
            StateVariables::Velocity        => "Velocity",
            StateVariables::Orientation     => "Orientation",
            StateVariables::AngularVelocity => "AngularVelocity"
        }
    }
    pub fn coordinate_names(&self) -> Vec<&str> {
        return match self {
            StateVariables::Position        => vec!["x", "y", "z"],
            StateVariables::Velocity        => vec!["x", "y", "z"],
            StateVariables::Orientation     => vec!["theta", "phi"],
            StateVariables::AngularVelocity => vec!["theta", "phi"]
        }
    }
    pub fn units(&self) -> &str {
        return match self {
            StateVariables::Position        => "m",
            StateVariables::Velocity        => "m/s",
            StateVariables::Orientation     => "degree",
            StateVariables::AngularVelocity => "degree/s"
        }
    }
    
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum StateIntrinsic {Mass, Polarity}

#[derive(Clone, Debug)]
pub struct State {
    pub var: HashMap<StateVariables,  Vec<Vec<f64>>>, // Change with t
    pub pro: HashMap<StateIntrinsic,  Vec<f64>>,      // Dosen't change with t
    pub typ: Vec<MolecularType>,                      // Dosen't change with t
}
impl Data <MoleculeState> for State {
    fn invert(&self) -> Vec<MoleculeState> {
        return self.typ.iter().enumerate().map(
            |(i, val)| 
            MoleculeState {
                id: i as u64,
                mol_type: val.clone(),
                var: self.var.iter().map(
                    |(k, val_h)|
                    (*k, val_h[i].clone())
                ).collect::<HashMap<StateVariables, Vec<f64>>>()
            }
        ).collect::<Vec<MoleculeState>>()
    }
}
impl State {
    fn couple(self, other:State) -> State {
        return State {
            var: {
        let mut result:HashMap<StateVariables,  Vec<Vec<f64>>> = HashMap::new();
        for k in self.var.keys().clone() {
            result.insert(*k, {
                [self.var[k].clone(), other.var[k].clone()].iter().flat_map(
                    |vec| vec.iter().cloned()
                ).collect::<Vec<Vec<f64>>>()
            });
        }
        result
            },
            pro: {
                let mut result:HashMap<StateIntrinsic,  Vec<f64>> = HashMap::new();
                for k in self.pro.keys().clone() {
                    result.insert(*k, {
                        [self.pro[k].clone(), other.pro[k].clone()].iter().flat_map(
                            |vec| vec.iter().cloned()
                        ).collect::<Vec<f64>>()
                    });
                }
                result
            },
            typ: {
                [self.typ, other.typ].iter().flat_map(
                    |vec| vec.iter().cloned()
                ).collect::<Vec<MolecularType>>()
            }
        }
    }
    pub fn from_vec_coupling(state_list:Vec<State>) -> State {
        let mut state = state_list[0].clone();
        for (i, s) in state_list.iter().enumerate() {
            if i > 0 {
                state = state.couple(
                    s.clone()
                );
            }
        }
        return state
    }
    pub fn create_randomly_from_intervals(cs:Conditions) -> State {
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
            typ: (0..mn).map(|_| cs.molecule_type.clone()).collect::<Vec<MolecularType>>()
        }
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct History {
    pub var:  HashMap<StateVariables, Vec<Vec<Vec<f64>>>>,
    pub pro:  HashMap<StateIntrinsic, Vec<f64>>,
    pub typ:  Vec<MolecularType>,
    pub time: Vec<f64>,
}
impl History {
    pub fn create(state:&State) -> History {
        return History {
            var: state.clone().var.iter().map(
                |(k, val)| (*k, vec![val.clone()])
            ).collect(),
            pro: state.clone().pro.iter().map(
                |(k, val)| (*k, val.clone())
            ).collect(),
            typ: state.typ.clone(),
            time: vec![0.]
        }
    }
    pub fn update(&mut self, state:&State, t:&f64) {
        for k in self.clone().var.keys() {
            self.var.insert(
                *k,
                [self.var[k].clone(), Vec::from([state.var[k].clone()])].iter().flat_map(
                    |vec| vec.iter().cloned()).collect::<Vec<Vec<Vec<f64>>>>()
                );
        };
        self.time.append(&mut vec![*t]);
    }
    pub fn to_json(&self, save_path:&str) {
        write(
            save_path, 
            &serde_json::to_vec_pretty(&self.invert()).unwrap() //::to_string_pretty
        ).expect("Unable to write json file");
    }
    pub fn save_as_polars_df(&self, save_path:&str) {
        // Create the dataframe from History
        let var_vecs: Vec<Series> = self.var.iter().flat_map(
            |(k, var_vec)| 
            {
                let name_prefix = k.name();

                let data = var_vec.iter().flat_map(
                    |var_mol| var_mol.clone()
                    ).collect::<Vec<Vec<f64>>>();
                
                k.coordinate_names().iter().enumerate().map(
                    |(c_number, c_name)|
                    Series::new(
                        &([name_prefix.clone(), "_", c_name].concat()), 
                        data.iter().map(|v| v[c_number]).collect::<Vec<f64>>()
                    )
                ).collect::<Vec<Series>>()
            }
        ).collect::<Vec<Series>>();


        let typs: Series = Series::new(
            "mol_type",
            self.time.iter().flat_map(
                |_| 
                self.typ.iter().map(
                    |mt|
                    mt.name()
                ).collect::<Vec<&str>>()
            ).collect::<Vec<&str>>()
        );

        let ts: Series = Series::new(
            "t",
            self.time.iter().flat_map(
                |t| 
                self.typ.iter().map(
                    |_|
                    t.clone()
                ).collect::<Vec<f64>>()
            ).collect::<Vec<f64>>()
        );
        
        let mut data = DataFrame::new(
            [vec![ts, typs], var_vecs].concat()
        ).unwrap();

        // Save the data to a file
        ParquetWriter::new(
            &mut std::fs::File::create(
                save_path
            ).expect("Could not create parquet file!")
        ).finish(&mut data).unwrap();
    }
}
impl Data <MoleculeDynamicState> for History {
    fn invert(&self) -> Vec<MoleculeDynamicState> {
    return self.time.iter().enumerate().flat_map(
        |(it, t)|
        self.typ.iter().enumerate().map(
            |(im, typ)|
            MoleculeDynamicState {
                t:*t,
                id: im as u64,
                mol_type: typ.clone(),
                var: self.var.iter().map(
                    |(k, val_h)|
                    (*k, val_h[it][im].clone())
                ).collect::<HashMap<StateVariables, Vec<f64>>>()
            }
        ).collect::<Vec<MoleculeDynamicState>>()
    ).collect::<Vec<MoleculeDynamicState>>();
    }
}