use serde::{Serialize, Deserialize};
use crate::effects::{Effect};
use std::io::{Read, Write};
use std::fs;
use crate::uvex_details::Use;

#[derive(Serialize,Debug,Deserialize)]
pub struct Demo_Details{
    pub psf_directory: (Use,String), //directory of FITS files
    pub qe: (Use,String),
    pub read_noise: (Use,String)

}

impl Demo_Details {
    pub fn default(path:&str) -> Demo_Details{

        let demo_details = Demo_Details{
            psf_directory: (Use::off, "data/demo/demo_psf".to_string()),
            qe: (Use::off, "data/demo/qe".to_string()),
            read_noise: (Use::off, "data/demo/read_noise".to_string()),
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