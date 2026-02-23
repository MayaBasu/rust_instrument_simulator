use std::fs::File;
use crate::uvex_details::UVEX_Details;

use crate::hallucinate::{hallucinate_bytes, name_gen};
use crate::sources::source_list;


mod objects;
mod uvex;
mod hallucinate;
mod effects;
mod instrument;
mod sources;
mod fits2;
mod fits_readers;
mod uvex_details;

fn main() {
    let details = UVEX_Details::blank();
    details.write_to_yaml("configuration/details");
    let details = UVEX_Details::read_from_yaml("configuration/details");
    println!("{:?}",details);
    let uvex = uvex::initialize_uvex(details,"configuration/uvex");




/*


    let spectrum = [1.0;spectral_resolution];
    let mut source_list = source_list::new_random_point_source_field(3, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, spectrum);
    uvex.run(&mut source_list);
    source_list.write_to_yaml("sources");

 */

   // println!("{:?}", source_list)
}


