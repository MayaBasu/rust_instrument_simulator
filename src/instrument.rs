use std::fs::File;
use rand::distr::{Distribution, Uniform};
use serde::{Deserialize, Serialize};
use crate::objects::TelescopeObject;
use std::io::Write;
use memmap2::Mmap;
use crate::effects::Effect;

pub const spectral_resolution:usize  = 1000;
pub const spatial_resolution:usize  = 4000;

#[derive(Debug)]
pub struct point_source{
    pub source_x:f64, //floats between 0 and 1
    pub source_y:f64,
    pub spectrum: [f64;spectral_resolution],
    pub luminosity:f64,
    pub grid_number: usize,
}
impl point_source{
    pub fn new(source_x:f64, source_y:f64, spectrum: [f64;spectral_resolution],luminosity:f64) -> point_source{
        let grid_number = point_source::calculate_grid_number(source_x,source_y);
        point_source{
            source_x,
            source_y,
            spectrum,
            luminosity,
            grid_number,
        }
    }
    fn calculate_grid_number(source_x:f64, source_y:f64)-> usize{
        let column = (source_x*spatial_resolution as f64).floor();
        let row = (source_y*spatial_resolution as f64).floor();
        let grid_number = spatial_resolution as f64*(row-1.0) + column;
        grid_number as usize
    }
}

#[derive(Debug)]
pub struct source_list{
    pub sources: Vec<point_source>,
    sorted: bool,
}
impl source_list{
    pub fn new_from(sources:Vec<point_source>) -> source_list{
        sources.sort_by(|a, b| b.grid_number.cmp(&a.grid_number));
        source_list{
            sources,
            sorted: true,
        }
    }

    pub fn new_random_point_source_field(number_of_point_sources:usize,
                                         min_brightness: f64,
                                         max_brightness: f64,
                                         min_x: f64,
                                         max_x:f64,
                                         min_y: f64,
                                         max_y:f64,
                                         spectrum:[f64;spectral_resolution]) -> source_list{
        //Some checks to make sure that the incoming values are as expected
        for end_point in [min_brightness,max_brightness,min_x,max_x,min_y,max_y]{
            assert!((0.0 <= end_point) || (end_point <= 1.0),"{}: {} must be a float between 0 and 1",stringify!(end_point),end_point );
        }
        assert!(min_brightness <= max_brightness,"min_brightness must be less than or equal to max_brightness");
        assert!(min_x <= max_x,"min_x must be less than or equal to max_x");
        assert!(min_y <= max_y,"min_y must be less than or equal to max_y");

        let luminosities = Uniform::new(min_brightness,max_brightness).expect("Could not generate random luminosities in the given range");
        let x_positions = Uniform::new(min_x,max_x).expect("Could not generate random luminosities in the given range");
        let y_positions = Uniform::new(min_y,max_y).expect("Could not generate random luminosities in the given range");
        let mut rng = rand::rng();
        let sources: Vec<point_source> = (0..number_of_point_sources).map(|_x|{
            let x = x_positions.sample(&mut rng);
            let y = y_positions.sample(&mut rng);
            let luminosity = luminosities.sample(&mut rng);
            point_source::new(x, y, spectrum, luminosity)
        }).collect();
        source_list::new_from(sources)

    }


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
        let effects:Vec<Effect> = &initial_object.effects;
        for effect in effects{
            let effect_type = effect.effect_type;
            println!("Applying effect: {:?}",effect_type);
            let spatial_extent = effect_type.spatial_extent;
            let spectral_extent = effect_type.spectral_extent;
            let effect_action = effect_type.effect_action;

            //Now we have to decide how the data will be processed.
            //case number 1: no spectral resolution, but there is spatial resolution
            if (spatial_extent == spatial_resolution) || (spectral_extent == 1){
                println!("We are applying an effect with spatial variation but no spectral variation");
                for point_source in &source_list.sources {


                }
            }





        }
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




