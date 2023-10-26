use std::collections::HashMap;

use crate::state::StateIntrinsic;

use {
    crate::{
        condition::{Conditions, DomainlessConditions},
        state::{History, State, StateVariables},
    },
    ndarray::{ArrayBase, arr1, arr2},
    ndarray_linalg::solve::{Inverse, Determinant}
};


pub enum BoundaryType {Elastic, InfiniteRepeating}



struct IntersectionMap {
    //       [Molecules (max_time, Variable[Coord])]
    pub pos: Vec<Vec<Vec<f64>>>,
    pub vel: Vec<Vec<Vec<f64>>>,
    pub ori: Vec<Vec<Vec<f64>>>,
    pub ave: Vec<Vec<Vec<f64>>> 
}
    

trait Geometry {
    // Remove outside particles
    fn remove_outside(&self, state:&State) -> State;

    // Create molecules within the geometry, if not delete them
    fn create_state_inside_randomly(&self, cs:Conditions) -> State;

    // changes the coordinates general to standard
    fn to_standard(&self, state:&State) -> State;

    // changes the coordinates from standard to general
    fn to_general(&self, state:&State) -> State;
    
    // Calculates the place and time for a number of intersections
    // Hashmap [[intersection, time], [State.var[Molecule[Coordinate]]]]
    fn intersections(&self, state:&State, n_inter:u32) -> IntersectionMap;

    //Having the intersections, calculates the state after possible boundary interactions
    fn enforce_boundary(&self, state:&State, inter_map:IntersectionMap) -> State;
}

impl dyn Geometry {
    fn create_state_inside_randomly_std(&self, cs:DomainlessConditions) -> State {
        let mut conds = cs.to_conditions([[-1., 1.]; 3]);
        let mut state = State::create_randomly_from_intervals(conds);

        conds.mn = self.remove_outside(&state).var[&StateVariables::Position].len() as u64;

        while conds.mn < cs.mn {
            conds.mn = cs.mn - conds.mn;
            state = state.couple(
                self.remove_outside(
                    &State::create_randomly_from_intervals(conds)
                )
            )
        }

        return state;
    }
}

    
// Basic types of geometries proived by mol_sim
pub enum ConstructType {Cuboid, Cylinder, Spheroid}
impl ConstructType {
    fn is_inside_std(&self, point:Vec<f64>) -> bool {
        match &self {
            ConstructType::Cuboid => {
                if point.iter().filter(|p| {**p > 1.} || {**p < -1.}).collect::<Vec<&f64>>().len() > 0 {
                    return false
                } else {return true}
            },
            ConstructType::Cylinder =>{
                if {point[2] > 1.} || {point[2] < -1.} {return false} 
                else if let r2 = point[0].powi(2) + point[2].powi(2) > 1. {return false}
                else {return true}
            }
            ConstructType::Spheroid =>{
                if let r2 = point.iter().map(|p| p.powi(2)).collect::<Vec<f64>>().iter().sum::<f64>() {
                    return false
                } else {return true}
            }
        }
    }

    fn remove_outside_std(&self, state:&State) -> State {
        let mut state = state.clone();
        
        let idx_keep = state.var.get(&StateVariables::Position).unwrap()
        .iter().enumerate().filter(
            |(_, p)| self.is_inside_std(**p))
        .map(|(i, _)| i).collect::<Vec<usize>>();

        for key in state.var.keys() {
            state.var = state.var.iter().enumerate().filter(
                |(i, _)| idx_keep.contains(i)).map(
                    |(_, (k, v))| (*k, *v)
            ).collect::<HashMap<StateVariables,  Vec<Vec<f64>>>>();
        }
        for key in state.pro.keys() {
            state.pro = state.pro.iter().enumerate().filter(
                |(i, _)| idx_keep.contains(i)).map(
                    |(_, (k, v))| (*k, *v)
            ).collect::<HashMap<StateIntrinsic,  Vec<f64>>>();
        }
        state.typ = state.typ.iter().enumerate().filter(
            |(i, _)| idx_keep.contains(i)).map(
                |(_, v)| *v
            ).collect::<Vec<MolecularType>>();
        
        return state.clone();
    } 
}



pub struct Construct {
    typ: ConstructType,
    center_point:  [f64; 3], 
    orientation_z: [f64; 3],
    dimentions:    [f64; 3]
}


impl Geometry for Construct {
    // Remove outside particles
    fn remove_outside(&self, state:&State) -> State {



        
    }

    // Create molecules within the geometry, if not delete them
    fn create_state_inside_randomly(&self, cs:Conditions) -> State {}

    // changes the coordinates general to standard
    fn to_standard(&self, state:&State) -> State {}

    // changes the coordinates from standard to general
    fn to_general(&self, state:&State) -> State {}
    
    // Calculates the place and time for a number of intersections
    // Hashmap [[intersection, time], [State.var[Molecule[Coordinate]]]]
    fn intersections(&self, state:&State, n_inter:u32) -> IntersectionMap;

    //Having the intersections, calculates the state after possible boundary interactions
    fn enforce_boundary(&self, state:&State, inter_map:IntersectionMap) -> State;
}



// pub struct Boundary<T:Geometry> {
//     geometry: T,
//     b_type: BoundaryType,
//     dim: Vec<f64>, // Dimentions vary with T
//     ori: Vec<f64>  // Origin
// }
// impl <T:Geometry> Boundary<T> {
//     fn enforce(&self, state:&State) -> State {
//         return self.geometry.enforce_boundary(state, &self.dim)
//     }
// }


// pub enum SimpleGeometry {Cuboid, Cylinder, Sphere}
// impl Geometry for SimpleGeometry {
//     fn intersection(&self, state:&State, dim:&Vec<f64>) -> Vec<Vec<f64>> {
//         fn i_cuboid  (geomnetry:SimpleGeometry, state:State) -> State {
//             state.var[StateVariables::Position].iter().zip(
//                 state.var[StateVariables::Velocity].iter()
//             ).map(
//                 |(pos, vel)|
//                 {

//                 }
//             )
//         }
//         fn i_cylinder(geomnetry:SimpleGeometry, state:State) -> State {
//             todo!();
//         }
//         fn i_sphere  (geomnetry:SimpleGeometry, state:State) -> State {
//             todo!();
//         }
//         return match self {
//             SimpleGeometry::Cuboid   => i_cuboid  (&self, state),
//             SimpleGeometry::Cylinder => i_cylinder(&self, state),
//             SimpleGeometry::Sphere   => i_sphere  (&self, state)
//         }
//     }
    
//     fn enforce_boundary(&self, state:State, dim:&Vec<f64>) -> State {
//         fn e_cuboid  (geomnetry:SimpleGeometry, state:State) -> State {
//             todo!();
//         }
//         fn e_cylinder(geomnetry:SimpleGeometry, state:State) -> State {
//             todo!();
//         }
//         fn e_sphere  (geomnetry:SimpleGeometry, state:State) -> State {
//             todo!();
//         }
//         return match self {
//             SimpleGeometry::Cuboid   => e_cuboid  (&self, state),
//             SimpleGeometry::Cylinder => e_cylinder(&self, state),
//             SimpleGeometry::Sphere   => e_sphere  (&self, state)
//         }
//     }
// }

// pub enum CustomGeometry {Custom}
// impl Geometry for CustomGeometry {
//     fn enforce_boundary(&self, state:State) -> State {
//         fn enforce_cuboid  (geomnetry:SimpleGeometry, state:State) -> State {
//             todo!();
//         }
//         fn enforce_cylinder(geomnetry:SimpleGeometry, boundary:Boundary<SimpleGeometry>, state:State) -> State {
//             todo!();
//         }
//         fn enforce_sphere  (geomnetry:SimpleGeometry, boundary:Boundary<SimpleGeometry>, state:State) -> State {
//             todo!();
//         }
//         return match self {
//             SimpleGeometry::Cuboid   => enforce_cuboid  (&self, boundary, state),
//             SimpleGeometry::Cylinder => enforce_cylinder(&self, boundary, state),
//             SimpleGeometry::Sphere   => enforce_sphere  (&self, boundary, state)
//         }
//     }
//     fn enforce_boundary_condition(state:State, b_type: BoundaryType) -> State {
//         todo!();
//     }
// }
// impl Intersection for CustomGeometry {
//     fn intersection(&self, state:State) -> Vec<Vec<f64>> {
        
//     }
// }