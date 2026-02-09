use crate::hallucinate::name_gen;
use crate::instrument::{spatial_resolution, spectral_resolution};
use crate::sources::source_list;
mod objects;
mod uvex;
mod hallucinate;
mod effects;
mod instrument;
mod sources;



fn main() {


    hallucinate::byte_version(spectral_resolution, spatial_resolution, name_gen(spectral_resolution,spatial_resolution,"test").as_str()).expect("TODO: panic message");



    let uvex = uvex::initialize_uvex();
    let spectrum = [1.0;spectral_resolution];
    let source_list = source_list::new_random_point_source_field(10, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, spectrum);
    println!("{:?}", source_list)
}


