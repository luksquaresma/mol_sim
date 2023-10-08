use {
    std::collections::HashMap,
    crate::{
        molecules::MolecularType,
        conditions::Conditions
    }
};


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StateVariables {Position, Velocity, Orientation, AngularVelocity}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StateIntrinsic {Mass, Polarity}

pub struct MoleculeState {
    pub id:         u64,
    pub mol_type:   MolecularType,
    pub var:        HashMap<StateVariables, Vec<f64>>
}

#[derive(Clone, Debug)]
pub struct State {
    pub var: HashMap<StateVariables,  Vec<Vec<f64>>>, // Change with t
    pub pro: HashMap<StateIntrinsic,  Vec<f64>>,      // Dosen't change with t
    pub ids: Vec<MolecularType>,                      // Dosen't change with t
}
impl State {
    pub fn invert(&self) -> Vec<MoleculeState> {
        return self.ids.iter().enumerate().map(
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
            ids: {
                [self.ids, other.ids].iter().flat_map(
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
            ids: (0..mn).map(|_| cs.molecule_type.clone()).collect::<Vec<MolecularType>>()
        }
    }
}
#[derive(Clone, Debug)]
pub struct History {
    pub var:  HashMap<StateVariables, Vec<Vec<Vec<f64>>>>,
    pub pro:  HashMap<StateIntrinsic, Vec<f64>>,
    pub ids:  Vec<MolecularType>,
    pub time: Vec<f64>,
}
impl History {
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
    pub fn create(state:&State) -> History {
        return History {
            var: state.clone().var.iter().map(
                |(k, val)| (*k, vec![val.clone()])
            ).collect(),
            pro: state.clone().pro.iter().map(
                |(k, val)| (*k, val.clone())
            ).collect(),
            ids: state.ids.clone(),
            time: vec![0.]
        }
    }
}

