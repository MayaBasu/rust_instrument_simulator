use std::fs::File;
use serde::{Deserialize, Serialize};
use crate::objects::TelescopeObject;
use std::io::Write;
use memmap2::Mmap;
pub const spectral_resolution:usize  = 1000;

pub struct point_source{
    pub source_x:f64, //floats between 0 and 4000
    pub source_y:f64,
    pub luminosity:f64,
}

pub struct source_list{
    pub sources: Vec<spectrum_group>
}

pub struct spectrum_group{
    //this is a group of point source which share a given spectra
    //It could be that there is one spectrum per point source, or that many sources share the same spectrum
    pub spectrum: [f64;spectral_resolution],
    pub point_sources: Vec<point_source>,
}




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
    pub fn make_lightflow_matrix(&self){

    }
    pub fn write_to_yaml(&self, file_name:&str,) {
        println!("Writing configuration data for {:?} to {:?}", self.instrument_label, file_name);
        let serialized_self = serde_yaml::to_string(&self).expect("Failed to YAMLify the instrument");
        let mut file = File::create(file_name).expect("Couldn't create the config file");
        write!(file, "{}", serialized_self).expect("Failed to write YAML to config file");
    }

    pub fn run(&self, source_list: source_list) {

        /*
        We want to take in a list of spectra, each of which has an associated set of locations at which it is at.
        The source_list is a vector of spectrum groups
        A spectrum group is a group of stars which all share a given spectrum
        Each spectrum group contains an array which is the spectrum shared by all the point sources in the group
        and it contains a list of point source locations of these point sources.
        */

        let initial_object = self.rummage(self.entry_point.clone());
        println!("{:?}", initial_object)


    }
    pub fn rummage(&self, object_name:String) -> &TelescopeObject{

        /* rummage around in the telescope_objects list and look for an object called
        object_name. If not exactly one object has this name, the function panics, because
        this means the instrument was not set up correctly
         */

        let mut found_objects:Vec<&TelescopeObject> = Vec::new();

        self.telescope_objects.iter().for_each(|object:&TelescopeObject|{
            if object.unique_label == object_name{
                found_objects.push(object)
            }
        });
        if found_objects.len() ==0{
            panic!("We rummaged for an object called {:?} but came up empty handed :( \
            \n please make sure the instrument is set up correctly", object_name)
        }
        if found_objects.len() >1{
            panic!("Oh no! There are more than one objects called {:?}! \
            \n please make sure the instrument is set up correctly", object_name)
        }
        else{
            return found_objects[0]
        }
    }

}

/*

       let input_data = File::open(input_data_path).expect("Could not open the input data file");
       let input_data = unsafe { Mmap::map(&input_data)}.expect("failed to memory map the input data");
       let input_data = &input_data[..];
       let input_data:&[f64] = bytemuck::cast_slice(input_data);

        */


