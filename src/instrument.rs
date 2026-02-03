use std::fs::File;
use serde::{Deserialize, Serialize};
use crate::objects::TelescopeObject;

#[derive(Serialize, Deserialize)]
pub struct Instrument{
    pub instrument_label: String,
    //label incase you want to configure several instruments and then compare them
    pub entry_point: String,
    //the 'unique_label' of the first object to receive starlight
    pub telescope_objects: Vec<TelescopeObject>,
    //A vector, in no particular order, of all the objects in the instrument
    //Objects include detectors, the dichroic (each path is a separate object), each mirror, diffraction grating etc.
    pub measurement_points: Vec<String>,
    //a list of the 'unique_label's which correspond to places where you want to save the data.
    //The data will be saved after the effects of that TelescopeObject are applied
    //If you just want the output at the detectors, then this would be the labels of the detectors.
    //However, if you want to see the intermediate evolution of the data, you can put in any object you want.
}

impl Instrument{
    pub fn new(instrument_label:String) -> Instrument{
        Instrument{
            instrument_label,
            entry_point: "".to_string(),
            telescope_objects: vec![],
            measurement_points: vec![],
        }
    }
    pub fn add_object(&mut self, object:TelescopeObject){
        self.telescope_objects.push(object)
    }
    pub fn add_measurement_point(&mut self, measurement_point:String){
        self.measurement_points.push(measurement_point)
    }
    pub fn set_entry_point(&mut self, entry_point:String){
        self.entry_point = entry_point
    }
    pub fn write_to_YAML(&self, file_name:&str,) {
        println!("Writing configuration data for {:?} to {:?}", self.instrument_label, file_name);
        let serialized_self = serde_yaml::to_string(&self).expect("Failed to YAMLify the instrument");
        let mut file = File::create(file_name).expect("Couldn't create the config file");
        write!(file, "{}", serialized_self).expect("Failed to write YAML to config file");
    }

}