use uvex_fitrs::*;
use crate::fits2::fits_path;
use crate::instrument::spectral_resolution;
use crate::psf_fits_reader::open_psf_fits;
use crate::sources::{point_source, source_list};
use crate::uvex_details::UVEX_Details;
mod objects;
mod uvex;
mod hallucinate;
mod effects;
mod instrument;
mod sources;
mod fits2;
mod fits_readers;
mod uvex_details;
mod psf_fits_reader;

fn main() {
    let details = UVEX_Details::default("configuration/details");


}
/*


    let spectrum = [1.0;spectral_resolution];
    let mut source_list = source_list::new_random_point_source_field(3, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, spectrum);
    uvex.run(&mut source_list);
    source_list.write_to_yaml("sources");

 */

   // println!("{:?}", source_list)



