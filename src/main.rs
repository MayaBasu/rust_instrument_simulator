use crate::effects::QUANTUM_EFFICIENCY;
use crate::hallucinate::name_gen;
use crate::instrument::{spatial_resolution, spectral_resolution};
use crate::sources::source_list;
use crate::uvex::{initialize_tma, initialize_uvex};

mod objects;
mod uvex;
mod hallucinate;
mod effects;
mod instrument;
mod sources;



fn main() {


    initialize_uvex();

    let (m1,m2,m3) = initialize_tma(
        "contamination_data1", "reflectance_data1",
        "contamination_data2", "reflectance_data2",
        "contamination_data3", "reflectance_data3");

    m1.write_to_yaml("m1.yaml");
    m2.write_to_yaml("m2.yaml");
    m3.write_to_yaml("m3.yaml");






    let qe = QUANTUM_EFFICIENCY.new("test_qe", "test_qe_data.dat");
    qe.write_to_yaml("qe_yaml");




    hallucinate::byte_version(spectral_resolution, spatial_resolution, name_gen(spectral_resolution,spatial_resolution,"test").as_str()).expect("TODO: panic message");
    let uvex = uvex::initialize_uvex();
    let spectrum = [1.0;spectral_resolution];
    let source_list = source_list::new_random_point_source_field(10, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, spectrum);
    println!("{:?}", source_list)
}


