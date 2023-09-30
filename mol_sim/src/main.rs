// Packages
// use rand::Rng::thread_rng;


// Constant dfinitions
pub const PI:f32 = 3.14159;
pub const DT:f32 = 1./100.;
pub const MOLECULE_NUMBER:    u32 = 5;
pub const POSITION_DOMAIN:    [[f32; 2]; 3] = [[ 0.,  1.], [ 0.,  2.], [ 5.,  6.]]; //[[x_min, x_max], [y_min, y_max]...]
pub const VELOCITY_INIT:      [[f32; 2]; 3] = [[-1.,  1.], [-2.,  2.], [ 2.,  6.]]; //[[x/s_min, x_max], [y/s_min, y_max]...]
pub const ORIENTATION_INIT:   [[f32; 2]; 2] = [[ 0.,  1.], [ 0.,  1.]]; //[[theta_min, theta_max], [phi_min, phi_max]]
pub const ANGULAR_VEL_INIT:   [[f32; 2]; 2] = [[ 0.,  1.], [ 0.,  1.]]; //[[theta/s_min, theta/s_max], [phi/s_min, phi/s_max]]

pub trait DataStructure {
    // Returns a list of keys for all the internal data
    fn get_keys(&self) -> Vec<String>;

    // Returns the item which match a key
    fn get_value(&self, k:&String) -> &Vec<Vec<f32>>;

    // Returns the amount of information (len(keys))
    fn get_info_qty(&self) -> usize;

    // Returns a vector of tuples that contins all 
    // the keys and values in the form (key, value).
    fn get_contents(&self) -> Vec<(String, Vec<Vec<f32>>)>;
}

pub struct Conditions {
    pub mn:      u32,           // number of molecules
    pub pos_dom: [[f32; 2]; 3], // position domain (m)
    pub vel_ini: [[f32; 2]; 3], // velocity initializer (m/s)
    pub ori_ini: [[f32; 2]; 2], // orientation initializer (degrees)    
    pub ave_ini: [[f32; 2]; 2], // angular velocity initializer (degrees/s)
}

pub struct State {
    pub pos: Vec<Vec<f32>>, // position (m) - has to be inside the position domain
    pub vel: Vec<Vec<f32>>, // velocity (m/s) - any real value
    pub ori: Vec<Vec<f32>>, // angular orientation (degrees) - limited from 0 to 1
    pub ave: Vec<Vec<f32>>, // angular velocity (degrees/s) - any real value
}

impl DataStructure for State {
    fn get_keys(&self) -> Vec<String> {
        return vec![
            "pos".to_string(), 
            "vel".to_string(), 
            "ori".to_string(), 
            "ave".to_string()
            ]
    }
    fn get_info_qty(&self) -> usize {
        return self.get_keys().len()
    }
    fn get_value(&self, k:&String) -> &Vec<Vec<f32>> {
        return match k.as_str() {
            "pos" => &self.pos,
            "vel" => &self.vel,
            "ori" => &self.ori,
            "ave" => &self.ave,
            &_ => todo!()
        }
    }  
    fn get_contents(&self) -> Vec<(String, Vec<Vec<f32>>)> {
        return self.get_keys().iter().map(
            |k| (k.clone(), self.get_value(k).clone())
        ).collect()
    }
}

fn create_molecules(cs:Conditions) -> State {  //mn:u32, pd:[[f32; 2]; 3], vi:[[f32; 2]; 3]
    let (mn, pd, vi, _oi, _ai) = (cs.mn, cs.pos_dom, cs.vel_ini, cs.ori_ini, cs.ave_ini);
    let pos:Vec<Vec<f32>> = (0..mn).map(|_|
        vec![
            pd[0][0] + (pd[0][1] - pd[0][0])*(rand::random::<f32>()), // x
            pd[1][0] + (pd[1][1] - pd[1][0])*(rand::random::<f32>()), // y
            pd[2][0] + (pd[2][1] - pd[2][0])*(rand::random::<f32>()) // z
        ]
    ).collect();
    let vel:Vec<Vec<f32>> = (0..mn).map(|_|
        vec![
            vi[0][0] + (vi[0][1] - vi[0][0])*(rand::random::<f32>()), // x
            vi[1][0] + (vi[1][1] - vi[1][0])*(rand::random::<f32>()), // y
            vi[2][0] + (vi[2][1] - vi[2][0])*(rand::random::<f32>()) // z
        ]
    ).collect();
    let ori:Vec<Vec<f32>> = (0..mn).map(|_|
        vec![
            vi[0][0] + (vi[0][1] - vi[0][0])*(rand::random::<f32>()), // x
            vi[1][0] + (vi[1][1] - vi[1][0])*(rand::random::<f32>()), // y
        ]
    ).collect();
    let ave:Vec<Vec<f32>> = (0..mn).map(|_|
        vec![
            vi[0][0] + (vi[0][1] - vi[0][0])*(rand::random::<f32>()), // x
            vi[1][0] + (vi[1][1] - vi[1][0])*(rand::random::<f32>()), // y
        ]
    ).collect();
    return State {pos, vel, ori, ave}
}

fn main() {
    let starting_cond = Conditions{
        mn:      MOLECULE_NUMBER,
        pos_dom: POSITION_DOMAIN,
        vel_ini: VELOCITY_INIT,
        ori_ini: ORIENTATION_INIT,
        ave_ini: ANGULAR_VEL_INIT
    };

    let state = create_molecules(starting_cond);

    for (k, v) in state.get_contents() {
        println!("KEY: {}",k);
        println!("{:.2?}", v)
    };
    // assert!(state.get_contents() == state.get_keys(), "Test is NOT WORKING");
}
