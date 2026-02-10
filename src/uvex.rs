use crate::effects::*;
use crate::objects::{TelescopeObject};
use crate::instrument::Instrument;






//generate the mirror objects for the initial optical train of the UVEX instrument
pub fn initialize_tma(contamination_data_path1:&str, reflectance_data_path1:&str,
                      contamination_data_path2:&str, reflectance_data_path2:&str,
                      contamination_data_path3:&str, reflectance_data_path3:&str
                        ) -> (TelescopeObject, TelescopeObject, TelescopeObject){

    //contamination for each mirror in the tma (this is a single number)
    let m1_contamination = CONTAMINATION.new("m1_contamination", contamination_data_path1);
    let m2_contamination = CONTAMINATION.new("m2_contamination", contamination_data_path2);
    let m3_contamination = CONTAMINATION.new("m3_contamination", contamination_data_path3);

    //reflectivity as a function of wavelength for each mirror in the tma
    let m1_reflectivity = REFLECTANCE.new("m1_reflectivity", reflectance_data_path1);
    let m2_reflectivity = REFLECTANCE.new("m2_reflectivity", reflectance_data_path2);
    let m3_reflectivity = REFLECTANCE.new("m3_reflectivity", reflectance_data_path3);


    //now we initialize the mirror objects with these effects
    let m1 = TelescopeObject::new("m1",
                                  vec![m1_reflectivity,m1_contamination],
                                  vec!["m2"]);
    let m2 = TelescopeObject::new("m2",
                                  vec![m2_reflectivity,m2_contamination],
                                  vec!["m3"]);
    let m3 = TelescopeObject::new("m3",
                                  vec![m3_reflectivity,m3_contamination],
                                  vec![]);
    (m1,m2,m3)

}

pub fn initialize_uvex(){
    let contamination_data_path = "contamination_data";
    let reflectance_data_path = "reflectance_data";
    let qe_data_path = "qe_data";
    let read_noise_data_path = "read_noise_data";

    //for simplicity, we assume all three mirrors have the same contamination and reflectance
    let (m1,m2, mut m3) = initialize_tma(
        contamination_data_path, reflectance_data_path,
        contamination_data_path, reflectance_data_path,
        contamination_data_path, reflectance_data_path);
    //initialize a detector with a quantum efficiency and a read noise
    let detector_qe = QUANTUM_EFFICIENCY.new("detector_qe", qe_data_path);
    let detector_read_noise = READ_NOISE.new("detector_read_noise", read_noise_data_path);
    let detector = TelescopeObject::new("detector",
                                        vec![detector_qe,detector_read_noise],vec![]);

    //we have to add the detector as a recipient of mirror 3
    m3.add_recipient("detector");
    //make a new instrument called "uvex" with the three mirrors and the detector, with entry point m1
    let mut uvex = Instrument::new("uvex", "m1");
    uvex.add_object(m1);
    uvex.add_object(m2);
    uvex.add_object(m3);
    uvex.add_object(detector);
    //we want to "measure" the instrument after the detector effects are applied:
    uvex.add_measurement_point("detector");
    //serialize the whole instrument to a YAML
    uvex.write_to_yaml("uvex.yaml")


}