use std::fs::File;
use std::io::Write;
use rand::distr::{Distribution, Uniform};
use serde::Serialize;
use crate::instrument::{spatial_resolution, spectral_resolution};




#[derive(Debug,Serialize,Clone)]
pub struct point_source{
    pub source_x:f64, //floats between 0 and 1
    pub source_y:f64,
    pub spectrum: [f64;spectral_resolution],
    pub luminosity: f64,
}


#[derive(Debug,Serialize)]
pub struct bin<'a>{
    pub bin_number: usize,
    pub contents: Vec<& 'a point_source>
}

impl point_source{
    pub fn new(source_x:f64, source_y:f64, spectrum: [f64;spectral_resolution],luminosity:f64) -> point_source{
        point_source{
            source_x,
            source_y,
            spectrum,
            luminosity,
        }
    }
    fn get_bin(&self,num_spacial_bins:usize,)-> usize{
        let column = (self.source_x*num_spacial_bins as f64).floor();
       // println!("column is {column}");
        let row = (self.source_y*num_spacial_bins as f64).floor();
       // println!("row is {row}");
        let spatial_grid_number = num_spacial_bins as f64*(row) + column;
       // println!("{}", grid_number);

        spatial_grid_number as usize
    }

}

#[derive(Debug,Serialize)]
pub struct source_list{
    pub sources: Vec<point_source>,
}
impl source_list{
    pub fn new_from(mut sources:Vec<point_source> ) -> source_list{
        //sources.sort_by(|a:&point_source, b:&point_source| b.bin.cmp(&a.bin));
        source_list{
            sources,
        }
    }
    pub fn new_random_point_source_field(number_of_point_sources:usize,
                                         min_brightness: f64,
                                         max_brightness: f64,
                                         min_x: f64,
                                         max_x:f64,
                                         min_y: f64,
                                         max_y:f64,
                                         spectrum:[f64;spectral_resolution],
                                        ) -> source_list{
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
    pub fn write_to_yaml(&self, file_name:&str,) {
        println!("Serializing point sources");
        let serialized_self = serde_yaml::to_string(&self).expect("Failed to YAMLify the sources");
        let mut file = File::create(file_name).expect("Couldn't create the config file");
        write!(file, "{}", serialized_self).expect("Failed to write YAML to config file");
    }
    pub fn bin(&self, num_spatial_bins:usize) -> binned_source_list {
        let mut binned_source_list = binned_source_list::from_point_sources(self.sources.clone(),num_spatial_bins);
        dbg!(&binned_source_list);
        binned_source_list

    }
}


#[derive(Debug,Serialize)]
pub struct binned_source_list<'a>{
    pub bins: Vec<bin<'a>>,
    pub num_spatial_bins: usize,
}

impl binned_source_list<'_>{
    pub fn from_point_sources<'a>( point_sources:Vec<point_source>,num_spatial_bins:usize) -> binned_source_list<'a>{
        let mut bins: Vec<bin> = Vec::new();
        for point_source in point_sources{

            let bin_number = point_source.get_bin(num_spatial_bins);

            match bins.iter().find(|bin| bin.bin_number ==bin_number) {
                Some(bin) => bin.contents.push(&point_source),
                None => {let new_bin = bin{ bin_number, contents: vec![&point_source] };
                    bins.push(new_bin)
                }
            }
        }
        binned_source_list{ bins, num_spatial_bins }
    }


}




