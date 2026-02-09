use rand::distr::{Distribution, Uniform};
use crate::instrument::{spatial_resolution, spectral_resolution};

#[derive(Debug)]
pub struct point_source{
    pub source_x:f64, //floats between 0 and 1
    pub source_y:f64,
    pub spectrum: [f64;spectral_resolution],
    pub luminosity: f64,
    pub bin: usize,
}


impl point_source{

    pub fn new(source_x:f64, source_y:f64, spectrum: [f64;spectral_resolution],luminosity:f64) -> point_source{
        let grid_number = point_source::calculate_grid_number(source_x,source_y);
        point_source{
            source_x,
            source_y,
            spectrum,
            luminosity,
            bin: grid_number,
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
    pub fn new_from(mut sources:Vec<point_source>) -> source_list{
        sources.sort_by(|a:&point_source, b:&point_source| b.bin.cmp(&a.bin));
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