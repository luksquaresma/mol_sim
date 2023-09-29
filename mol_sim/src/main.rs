// Packages
// use rand::Rng::thread_rng;


// Constant dfinitions
pub const PI:f32 = 3.14159;
pub const DT:f32 = 1./100.;
pub const MOLECULE_NUMBER:    u32 = 30;
pub const POSITION_DOMAIN:    [[f32; 2]; 3] = [[ 0.,  1.], [ 0.,  2.], [ 5.,  6.]]; //[[x_min, x_max], [y_min, y_max]...]
pub const VELOCITY_INIT:      [[f32; 2]; 3] = [[-1.,  1.], [-2.,  2.], [ 2.,  6.]]; //[[x/s_min, x_max], [y/s_min, y_max]...]
pub const ORIENTATION_INIT:   [[f32; 2]; 2] = [[ 0.,  1.], [ 0.,  1.]]; //[[theta_min, theta_max], [phi_min, phi_max]]
pub const ANGULAR_VEL_INIT:   [[f32; 2]; 2] = [[ 0.,  1.], [ 0.,  1.]]; //[[theta/s_min, theta/s_max], [phi/s_min, phi/s_max]]

pub struct Conditions {
    pub mn:      u32,           // number of molecules
    pub pos_dom: [[f32; 2]; 3], // position domain (m)
    pub vel_ini: [[f32; 2]; 3], // velocity initializer (m/s)
    pub ori_ini: [[f32; 2]; 2], // orientation initializer (degrees)    
    pub ave_ini: [[f32; 2]; 2], // angular velocity initializer (degrees/s)
}

pub trait DataStructure {
    fn get_keys(&self) -> Vec<String>;
    // fn get_info_qty(&self) -> usize;
    fn get_value(&self, k:i32) -> Vec<Vec<f32>>;
    // fn get_contents(&self) -> [(String, Vec<Vec<f32>>); 7];
}

pub struct State {
    pub pos: Vec<Vec<f32>>, // position (m) - has to be inside the position domain
    pub vel: Vec<Vec<f32>>, // velocity (m/s) - any real value
    pub ori: Vec<Vec<f32>>, // angular orientation (degrees) - limited from 0 to 1
    pub ave: Vec<Vec<f32>>, // angular velocity (degrees/s) - any real value
}

impl DataStructure for State {
    fn get_keys(&self) -> Vec<String> {
        return vec!["pos".to_string(), "vel".to_string(), "ori".to_string(), "ave".to_string()]
    }
    // fn get_info_qty(&self) -> usize {
    //     return self.get_keys().len()
    // }
    fn get_value(&self, k:i32) -> Vec<Vec<f32>> {
        return match k {
            1 => *(&self.pos),
            2 => *(&self.vel),
            // "ori".to_string() => &self.ori,
            // "ave".to_string() => &self.ave,
        }
    }

    // fn get_keys(&self)     -> Vec<String> {
        
    // }

    // fn get_contents(&self) -> ; {
        
    // }
    
}

fn create_molecules(cs:Conditions) -> State {  //mn:u32, pd:[[f32; 2]; 3], vi:[[f32; 2]; 3]
    let (mn, pd, vi, oi, ai) = (cs.mn, cs.pos_dom, cs.vel_ini, cs.ori_ini, cs.ave_ini);
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
    println!("{:.2?}", state.get_keys());

}



























// //returns [position, orientation, velocity]
// fn create_molecules() -> Vec<[f32; 3]> {
//     //[u32; N as usize] {
//     // [[[i32; 3]; N as usize]; 3] {
//     let _pid:Vec<u32> = (0..N).map(|i| i).collect::<Vec<u32>>(); //p_id: &[u32]
//     let pos:Vec<[f32; 3]> = (0..N).map(|_| 
//         [
//             DOMAIN_SIZE[0][0] + (DOMAIN_SIZE[0][1] - DOMAIN_SIZE[0][0])*(rand::thread_rng().gen::<f32>()),
//             DOMAIN_SIZE[1][0] + (DOMAIN_SIZE[1][1] - DOMAIN_SIZE[1][0])*(rand::thread_rng().gen::<f32>()),
//             DOMAIN_SIZE[2][0] + (DOMAIN_SIZE[2][1] - DOMAIN_SIZE[2][0])*(rand::thread_rng().gen::<f32>()),
//         ]).collect();
    
//     // todo: normalize oringin vector (ori)
//     let ori:Vec<Vec<f32>> = (0..N).map(|_| 
//         vec![
//             (rand::thread_rng().gen::<f32>()),
//             (rand::thread_rng().gen::<f32>()),
//             (rand::thread_rng().gen::<f32>()),
//         ]).collect::<Vec<Vec<f32>>>(); //.map(|x| x.pow(2.)));

    
//     let vel:Vec<[f32; 3]> = (0..N).map(|_| 
//         [
//             VELOCITY_DOMAIN[0] + (VELOCITY_DOMAIN[1] - VELOCITY_DOMAIN[0])*(rand::thread_rng().gen::<f32>()),
//             VELOCITY_DOMAIN[0] + (VELOCITY_DOMAIN[1] - VELOCITY_DOMAIN[0])*(rand::thread_rng().gen::<f32>()),
//             VELOCITY_DOMAIN[0] + (VELOCITY_DOMAIN[1] - VELOCITY_DOMAIN[0])*(rand::thread_rng().gen::<f32>()),
//         ]).collect();
//     return pos;
// }

// fn main() {
//     println!("{:.2?}", create_molecules());
// }
