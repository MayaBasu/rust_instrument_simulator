use crate::instrument::{source_list, spectral_resolution};

mod objects;
mod simulator_engine;
mod uvex;
mod hallucinate;
mod effects;
mod instrument;


fn main() {
    //simulator_engine::pipline(true)
    let uvex = uvex::initialize_uvex();
    let spectrum = [1.0;spectral_resolution];
    let source_list = source_list::new_random_point_source_field(10, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, spectrum);
    println!("{:?}", source_list)
}


