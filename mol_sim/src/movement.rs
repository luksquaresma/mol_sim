use {
    crate::state::{
        History, 
        State,
        StateVariables
    },
    ndarray::{
        ArrayBase,
        arr1,
        arr2,
    },
    ndarray_linalg::solve::Inverse
};

pub enum BoundaryType {Elastic, Infinite, InfiniteRepeating}

pub trait Geometry {
    fn enforce_boundary(&self, state:&State, dim:&Vec<f64>) -> State;
    fn intersection(&self, state:&State, dim:&Vec<f64>) -> Vec<Vec<f64>>;
}

pub mod GeometryConstruct {
    use std::array;

    use ndarray::ArrayBase;
    use ndarray_linalg::{transpose_data, Inverse, Determinant};

    pub enum GeometryConstructType {Line, Lines, Plane, Sphere, Cylinder}
    
    trait Cosntruct {
        fn line_intersect(&self, line:Line) -> Option(Vec<f64>);
    }

    // for single constructs
    struct Line {typ:GeometryConstructType, point:Vec<f64>, vector: Vec<f64>}
    impl Cosntruct for Line {
        fn line_intersect(&self, line:Line) -> Option(Vec<f64>) {
            let mat = arr2(&[self.vector, line.vector]);
            let pts = arr2(&[self.point, point.vector]);
            if let inv = Determinant::det(transpose_data(mat).dot(mat)) != 0 {
                return Some((inv.dot(mat)).dot(pts) as Vec<f64>)
            } else {
                return None()
            }
        }
    }

    struct Plane    {typ:GeometryConstructType, point:Vec<f64>, vector: Vec<f64>}
    impl Cosntruct for Plane {
        fn line_intersect(&self, line:Line) -> Option(Vec<f64>) {
            if let sprod = arr1(&self.vector).dot(arr1(line.vector)) as f64 != 0. {
                return Some(arr1(&line.point) + arr1(&line.vector).dot(
                    (arr1(self.point) - arr1(line.point)).dot(arr1(self.vector))/sprod
                ) as Vec<f64>);
            } else {return None()};
        }
    }
    

    struct Sphere   {typ:GeometryConstructType, center:Vec<f64>, radius: f64}
    impl Cosntruct for Sphere {
        fn line_intersect(&self, line:Line) -> Option(Vec<f64>) {
            let dif = arr1(line.point) - arr1(self.center);
            if let nabla = (
                (arr1(line.vector).dot(dif)).pow(2.)- (dif.dot(dif) - self.radius)
            ) as f64 >= 0. {
                Some(
                     as Vec<f64>
                    )

            } else {return None()};
        }
    }


    
    struct Cylinder {typ:GeometryConstructType, point:Vec<f64>, radius: f64, height: f64}

    // for the whole state
    struct Lines    {typ:GeometryConstructType, points:Vec<Vec<f64>>, vectors:Vec<Vec<f64>>}
    
    trait GeometryConstruction {
        pub fn from_construct_vec<T:Cosntruct>(con_vec:Vec<T>) {

        }
    }
    struct GeometryConstruct


}


pub struct Boundary<T:Geometry> {
    geometry: T,
    b_type: BoundaryType,
    dim: Vec<f64>, // Dimentions vary with T
    ori: Vec<f64>  // Origin
}
impl <T:Geometry> Boundary<T> {
    fn enforce(&self, state:&State) -> State {
        return self.geometry.enforce_boundary(state, &self.dim)
    }
}


pub enum SimpleGeometry {Cuboid, Cylinder, Sphere}
impl Geometry for SimpleGeometry {
    fn intersection(&self, state:&State, dim:&Vec<f64>) -> Vec<Vec<f64>> {
        fn i_cuboid  (geomnetry:SimpleGeometry, state:State) -> State {
            state.var[StateVariables::Position].iter().zip(
                state.var[StateVariables::Velocity].iter()
            ).map(
                |(pos, vel)|
                {

                }
            )
        }
        fn i_cylinder(geomnetry:SimpleGeometry, state:State) -> State {
            todo!();
        }
        fn i_sphere  (geomnetry:SimpleGeometry, state:State) -> State {
            todo!();
        }
        return match self {
            SimpleGeometry::Cuboid   => i_cuboid  (&self, state),
            SimpleGeometry::Cylinder => i_cylinder(&self, state),
            SimpleGeometry::Sphere   => i_sphere  (&self, state)
        }
    }
    
    fn enforce_boundary(&self, state:State, dim:&Vec<f64>) -> State {
        fn e_cuboid  (geomnetry:SimpleGeometry, state:State) -> State {
            todo!();
        }
        fn e_cylinder(geomnetry:SimpleGeometry, state:State) -> State {
            todo!();
        }
        fn e_sphere  (geomnetry:SimpleGeometry, state:State) -> State {
            todo!();
        }
        return match self {
            SimpleGeometry::Cuboid   => e_cuboid  (&self, state),
            SimpleGeometry::Cylinder => e_cylinder(&self, state),
            SimpleGeometry::Sphere   => e_sphere  (&self, state)
        }
    }
}

pub enum CustomGeometry {Custom}
impl Geometry for CustomGeometry {
    fn enforce_boundary(&self, state:State) -> State {
        fn enforce_cuboid  (geomnetry:SimpleGeometry, state:State) -> State {
            todo!();
        }
        fn enforce_cylinder(geomnetry:SimpleGeometry, boundary:Boundary<SimpleGeometry>, state:State) -> State {
            todo!();
        }
        fn enforce_sphere  (geomnetry:SimpleGeometry, boundary:Boundary<SimpleGeometry>, state:State) -> State {
            todo!();
        }
        return match self {
            SimpleGeometry::Cuboid   => enforce_cuboid  (&self, boundary, state),
            SimpleGeometry::Cylinder => enforce_cylinder(&self, boundary, state),
            SimpleGeometry::Sphere   => enforce_sphere  (&self, boundary, state)
        }
    }
    fn enforce_boundary_condition(state:State, b_type: BoundaryType) -> State {
        todo!();
    }
}
impl Intersection for CustomGeometry {
    fn intersection(&self, state:State) -> Vec<Vec<f64>> {
        
    }
}


// pub fn enforce_boundary(checker: fn(Vec<f64>),  condition: BoundaryConditions) {

// }