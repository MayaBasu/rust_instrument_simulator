use crate::effects::*;
use crate::objects::{TelescopeObject};
use crate::instrument::Instrument;






//generate the mirror objects for the initial optical train of the UVEX instrument
pub fn initialize_tma(contamination_data_path:&str, reflectance_data_path:&str) -> (TelescopeObject, TelescopeObject, TelescopeObject){
    //contamination for each mirror in the tma (this is a single number)
    let m1_contamination = CONTAMINATION.new("m1_contamination", contamination_data_path);
    let m2_contamination = CONTAMINATION.new("m2_contamination", contamination_data_path);
    let m3_contamination = CONTAMINATION.new("m3_contamination", contamination_data_path);

    //reflectivity as a function of wavelength for each mirror in the tma
    let m1_reflectivity = REFLECTANCE.new("m1_reflectivity", reflectance_data_path);
    let m2_reflectivity = REFLECTANCE.new("m2_reflectivity", reflectance_data_path);
    let m3_reflectivity = REFLECTANCE.new("m3_reflectivity", reflectance_data_path);


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
    let contamination_data_path = "4000_lines_4000_columns_A_bytes";
    let reflectance_data_path = "4000_lines_4000_columns_C_bytes";
    let (m1,m2,m3) = initialize_tma(contamination_data_path, reflectance_data_path);

    let mut uvex = Instrument::new("uvex".to_string());
    uvex.add_object(m1);
    uvex.add_object(m2);
    uvex.add_object(m3);

    let found = uvex.rummage("m1".to_string());



}