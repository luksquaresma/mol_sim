// Packages
use rand::prelude::*;


// Constant dfinitions
pub const PI:f32 = 3.14159;
pub const N:u32 = 30;
pub const DOMAIN_SIZE:[[f32; 2]; 3] = [[0., 1.], [0., 2.], [5., 6.]]; //[[x_min, x_max], [y_min, y_max]...]
pub const VELOCITY_DOMAIN:[f32; 2] = [5., 10.]; //[[x_min, x_max], [y_min, y_max]...]


//returns [position, orientation, velocity]
fn create_particles() -> Vec<[f32; 3]> {
    //[u32; N as usize] {
    // [[[i32; 3]; N as usize]; 3] {
    let _pid:Vec<u32> = (0..N).map(|i| i).collect::<Vec<u32>>(); //p_id: &[u32]
    let pos:Vec<[f32; 3]> = (0..N).map(|_| 
        [
            DOMAIN_SIZE[0][0] + (DOMAIN_SIZE[0][1] - DOMAIN_SIZE[0][0])*(rand::thread_rng().gen::<f32>()),
            DOMAIN_SIZE[1][0] + (DOMAIN_SIZE[1][1] - DOMAIN_SIZE[1][0])*(rand::thread_rng().gen::<f32>()),
            DOMAIN_SIZE[2][0] + (DOMAIN_SIZE[2][1] - DOMAIN_SIZE[2][0])*(rand::thread_rng().gen::<f32>()),
        ]).collect();
    
    // todo: normalize oringin vector (ori)
    let ori:Vec<Vec<f32>> = (0..N).map(|_| 
        vec![
            (rand::thread_rng().gen::<f32>()),
            (rand::thread_rng().gen::<f32>()),
            (rand::thread_rng().gen::<f32>()),
        ]).collect::<Vec<Vec<f32>>>(); //.map(|x| x.pow(2.)));

    
    let vel:Vec<[f32; 3]> = (0..N).map(|_| 
        [
            VELOCITY_DOMAIN[0] + (VELOCITY_DOMAIN[1] - VELOCITY_DOMAIN[0])*(rand::thread_rng().gen::<f32>()),
            VELOCITY_DOMAIN[0] + (VELOCITY_DOMAIN[1] - VELOCITY_DOMAIN[0])*(rand::thread_rng().gen::<f32>()),
            VELOCITY_DOMAIN[0] + (VELOCITY_DOMAIN[1] - VELOCITY_DOMAIN[0])*(rand::thread_rng().gen::<f32>()),
        ]).collect();
    return pos;
}

fn main() {
    println!("{:.2?}", create_particles());
}
