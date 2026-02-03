use crate::telescope_objects::{Effect, TelescopeObject};
use crate::telescope_objects::EffectName::{Contamination, Reflectance};
use crate::telescope_objects::TelescopeObjectName::Mirror;

//generate the mirror objects for the initial optical train of the UVEX instrument
pub fn tma() -> (TelescopeObject, TelescopeObject, TelescopeObject){
    //contamination for each mirror in the tma (this is a single number)
    let fuv_leakage_current = Effect{
        effect_name: Contamination,
        unique_label: "m1_contamination".to_string(),
        active:true,
        file_path: "contamination".to_string(),
    };
    let m2_contamination = Effect{
        effect_name: Contamination,
        unique_label: "m2_contamination".to_string(),
        active:true,
        file_path: "contamination".to_string(),
    };
    let m3_contamination = Effect{
        effect_name: Contamination,
        unique_label: "m3_contamination".to_string(),
        active:true,
        file_path: "contamination".to_string(),
    };
    //reflectivity as a function of wavelength for each mirror in the tma
    let m1_reflectivity = Effect{
        effect_name: Reflectance,
        unique_label: "m1_reflectivity".to_string(),
        active:true,
        file_path: "tma_reflectivity".to_string(),
    };
    let m2_reflectivity = Effect{
        effect_name: Reflectance,
        unique_label: "m2_reflectivity".to_string(),
        active:true,
        file_path: "tma_reflectivity".to_string(),
    };
    let m3_reflectivity = Effect{
        effect_name: Reflectance,
        unique_label: "m3_reflectivity".to_string(),
        active:true,
        file_path: "tma_reflectivity".to_string(),
    };
    //now we define the mirror objects with the above effects
    let m3 = TelescopeObject{
        object_name: Mirror,
        unique_label: "m1".to_string(),
        effects: vec![m1_contamination,m1_reflectivity],
        recipients: vec![],
    };
    let m2 = TelescopeObject{
        object_name: Mirror,
        unique_label: "m2".to_string(),
        effects: vec![m2_contamination,m2_reflectivity],
        recipients: vec!["m3".to_string()],
    };
    let m1 = TelescopeObject{
        object_name: Mirror,
        unique_label: "m3".to_string(),
        effects: vec![m3_contamination,m3_reflectivity],
        recipients: vec!["m2".to_string()],
    };

    (m1,m2,m3)
}