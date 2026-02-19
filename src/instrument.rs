use crate::objects::TelescopeObject;
use crate::effects::{Effect, EffectAction};
use crate::sources::{point_source, source_list};
use std::io::Write;
use memmap2::Mmap;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};
use std::fs::File;
use rand::distr::{Distribution, Uniform};
use serde::{Deserialize, Serialize};
use rayon::prelude::*;
use crate::fits;
use crate::fits::fits_path;

pub const spectral_resolution:usize  = 2;
pub const spatial_resolution:usize  = 4;

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
    pub fn new(instrument_label:&str, entry_point:&str) -> Instrument{
        Instrument{
            instrument_label: instrument_label.to_string(),
            entry_point: entry_point.to_string(),
            telescope_objects: vec![],
            measurement_points: vec![],
        }
    }
    pub fn add_object(&mut self, object:TelescopeObject){
        self.telescope_objects.push(object)
    }
    pub fn add_measurement_point(&mut self, measurement_point:&str){
        self.measurement_points.push(measurement_point.to_string())
    }
    pub fn set_entry_point(&mut self, entry_point:&str){
        self.entry_point = entry_point.to_string()
    }
    pub fn make_lightflow_diagram(&self){

    }
    pub fn write_to_yaml(&self, file_name:&str,) {
        println!("Writing configuration data for {:?} to {:?}", self.instrument_label, file_name);
        let serialized_self = serde_yaml::to_string(&self).expect("Failed to YAMLify the instrument");
        let mut file = File::create(file_name).expect("Couldn't create the config file");
        write!(file, "{}", serialized_self).expect("Failed to write YAML to config file");
    }

    pub fn run(&self, source_list: &mut source_list) {

        /*
        We want to take in a list of spectra, each of which has an associated set of locations at which it is at.
        The source_list is a vector of spectrum groups
        A spectrum group is a group of stars which all share a given spectrum
        Each spectrum group contains an array which is the spectrum shared by all the point sources in the group
        and it contains a list of point source locations of these point sources.
        */

        let mut current_object = self.rummage(self.entry_point.clone());
        let mut terminated:bool = false;

        while terminated ==false{
            let effects:Vec<Effect> = current_object.effects.clone();
            let recipient_names = current_object.recipients.clone();


            for effect in effects{
                let effect_type = effect.effect_type;
                dbg!(effect_type.clone());

                let path = "/Users/mayabasu/RustroverProjects/image_simulator_outline/python_plotting/fits_data";

                let data = File::open(path).expect("Could not open the data file for the effect");
                let data = unsafe { Mmap::map(&data)}.expect("failed to memory map the effect data");
                let data = &data[..];
                let data: &[f32] = bytemuck::cast_slice(data);
                println!("akelfjeo;awihjfw{:?}",data[1955]);

                let effect_action = effect_type.effect_action;
                let effect_spatial = effect_type.spatial_extent;
                let effect_spectral = effect_type.spectral_extent;



                for mut source in source_list.sources{
                    //First, determine the 'bin' of the source according to how finely grained
                    //the data is - if there is no spatial variation then everything is in bin 1

                    let source_bin = source.get_bin(effect_spatial);
                    if effect_spatial == 1 {

                        //if there is no spectral resolution, then each source bin maps exactly to the index of the data
                        let resulting_source_list: source_list = match effect_action {
                            EffectAction::ComponentWiseMultiplicative => {
                                source.luminosity *= data[source_bin];
                                source_list::new_from(
                                    vec![source]
                                )
                            }
                            EffectAction::ComponentWiseAddition => {
                                source.luminosity += data[source_bin];
                                source_list::new_from(
                                    vec![source]
                                )
                            }
                            EffectAction::ConvolutionKernel => {
                                let source_list = source_list::new_empty(effect_spatial);
                                // need to figure out the conversion of pixles in the fits file to my 0-1 x axis
                                let conversion = 1; //TODO
                                for kernel_pixel_x in 0..spatial_resolution{
                                    for kernel_pixel_y in 0..spatial_resolution{
                                       // let kernelified_source =
                                        //source_list.add_source()

                                    }
                                }

                            }
                            EffectAction::Reshape => {}
                        };

                    }

                }






                /*The file format will be a list of lists
                the overlying list will be the pixel bins, "lined up" in order of each row read across
                for each of these points, there will be a list of length spectral extent, which is the application to the spectra *at that point*
                 For example, if the spectral resolution was 3 and the spatial resolution was two, then the data would look like:
                 bin1_spectral1, bin1_spectral2, bin1_spectral3, bin2_spectral1, bin2_spectral2, bin2_spectral3
                we know how to read the file because of the effect type telling us the length of this outer and inner list

                 */
            }
            terminated = true;

        }
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





