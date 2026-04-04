use uvex_fitrs::*;


use crate::fits2::{fits_path, fits_path2, open_fits};
use crate::grid::Grid;
use crate::instrument::{Instrument, spectral_resolution};


use crate::sources::{PointSource, SourceList};
use crate::uvex_details::UVEX_Details;
mod objects;
mod uvex;
mod hallucinate;
mod effects;
mod instrument;
mod sources;
mod fits2;

mod uvex_details;



mod data_frame;
mod grid;
mod field_of_view;

fn main() {

    let fuv_path = "/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF";
    let mut grid = uvex::empty_fuv();
    grid.load_data_frames(fuv_path, ("XPOS", "YPOS"), (64, 64), (6.4, 6.4));
    grid.validate();
  


   // let details = uvex_details::UVEX_Details::default("details.yaml");

    //let demo = initialize_demo(details,"configuration/demo.yaml");
    //let mut grid = Grid::load_fuv(fuv_path);
    //grid.validate();
    //println!("{:?}",grid.corner());
    //println!("{:?}",grid.size());
    //grid.pretty_print()
}
/*


    let spectrum = [1.0;spectral_resolution];
    let mut source_list = source_list::new_random_point_source_field(3, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, spectrum);
    uvex.run(&mut source_list);
    source_list.write_to_yaml("sources");

 */

   // println!("{:?}", source_list)



