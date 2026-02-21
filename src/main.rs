use crate::effects::QUANTUM_EFFICIENCY;
use crate::hallucinate::{hallucinate_bytes, name_gen};
use crate::instrument::{spatial_resolution, spectral_resolution};
use crate::sources::source_list;
use crate::uvex::{initialize_tma, initialize_uvex};

mod objects;
mod uvex;
mod hallucinate;
mod effects;
mod instrument;
mod sources;
mod fits2;

mod types;
mod fits;
mod scifmt;

fn main() {


    initialize_uvex();


    let uvex = uvex::initialize_uvex();
    let spectrum = [1.0;spectral_resolution];
    let mut source_list = source_list::new_random_point_source_field(3, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, spectrum);
    uvex.run(&mut source_list);
    source_list.write_to_yaml("sources");

   // println!("{:?}", source_list)
}


