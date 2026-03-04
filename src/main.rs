use uvex_fitrs::*;
use crate::demo::initialize_demo;
use crate::demo_details::Demo_Details;
use crate::fits2::{fits_path, fits_path2, open_fits};
use crate::instrument::{Instrument, spectral_resolution};
use crate::psf_fits_reader::{open_psf_directory, open_psf_fits};
use crate::sources::{point_source, source_list};
use crate::uvex_details::UVEX_Details;
mod objects;
mod uvex;
mod hallucinate;
mod effects;
mod instrument;
mod sources;
mod fits2;

mod uvex_details;
mod psf_fits_reader;
mod demo;
mod demo_details;

fn main() {


    let details = Demo_Details::default("configuration/demo_details");
    let demo = initialize_demo(details,"configuration/demo.yaml");
    open_psf_directory(1,335);
}
/*


    let spectrum = [1.0;spectral_resolution];
    let mut source_list = source_list::new_random_point_source_field(3, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, spectrum);
    uvex.run(&mut source_list);
    source_list.write_to_yaml("sources");

 */

   // println!("{:?}", source_list)



