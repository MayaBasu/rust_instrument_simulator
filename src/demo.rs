use std::fs::read;


use crate::effects::{DARK_CURRENT, POINT_SPREAD_FUNCTION, QUANTUM_EFFICIENCY, READ_NOISE, REFLECTANCE, VINIETTING};
use crate::instrument::Instrument;
use crate::objects::TelescopeObject;
use crate::uvex::{initialize_fuv_channel, initialize_nuv_channel, initialize_spectrograph, initialize_tma};
use crate::uvex_details::{Use, UVEX_Details};
use serde::{Serialize, Deserialize};
use crate::effects::{Effect};
use std::io::{Read, Write};
use std::fs;






pub fn initialize_demo(demo_details: Demo_Details,path:&str) -> Instrument{


    let psf = POINT_SPREAD_FUNCTION.new("fuv_psf", demo_details.psf_directory.1);
    let qe = QUANTUM_EFFICIENCY.new("fuv_qe",demo_details.qe.1);
    let read_noise = READ_NOISE.new("fuv_read_noise",demo_details.read_noise.1);

    let detector = TelescopeObject::new("demo_detector",
                                        vec![qe,read_noise,psf],
                                        vec![]);


    let mut demo = Instrument{
        fov_x: 3.52,
        fov_y: 3.52,
        instrument_label: "demo".to_string(),
        entry_point: detector.unique_label.to_string(),
        measurement_points: vec![detector.unique_label.clone()],
        telescope_objects: vec![detector],

    };
    demo.write_to_yaml(path);
    demo

}


#[derive(Serialize,Debug,Deserialize)]
pub struct Demo_Details{
    pub psf_directory: (Use,String), //directory of FITS files
    pub qe: (Use,String),
    pub read_noise: (Use,String)

}

impl Demo_Details {
    pub fn default(path:&str) -> Demo_Details{

        let demo_details = Demo_Details{
            psf_directory: (Use::Off, "data/demo/demo_psf".to_string()),
            qe: (Use::Off, "data/demo/qe".to_string()),
            read_noise: (Use::Off, "data/demo/read_noise".to_string()),
        };
        demo_details.write_to_yaml(path);
        demo_details
    }

    pub fn write_to_yaml(&self, file_name:&str,) {
        println!("Writing uvex details to {:?}", file_name);
        let serialized_self = serde_yaml::to_string(&self).expect("Failed to YAMLify the object");
        let mut file = fs::File::create(file_name).expect("Couldn't create the config file");
        write!(file, "{}", serialized_self).expect("Failed to write YAML to config file");
    }

    pub fn read_from_yaml(file_name:&str)-> Demo_Details{
        let details: String = fs::read_to_string(file_name).expect("couldn't read from details file");
        let details: Demo_Details = serde_yaml::from_str(details.as_str()).expect("invalid details data");
        details
    }

}


