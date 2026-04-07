use plotpy::Plot;
use uvex_fitrs::*;
use crate::coordinate_system::CoordinateSystem;
use crate::fits2::{fits_path, fits_path2, open_fits};
use crate::grid::{Grid, PlotPoint};
use crate::instrument::{Instrument, spectral_resolution};


use crate::sources::{PointSource, SourceList};
use crate::uvex_details::UVEX_Details;
mod objects;
mod uvex;

mod effects;
mod instrument;
mod sources;
mod fits2;

mod uvex_details;



mod data_frame;
mod grid;
mod coordinate_system;
mod data;

fn main() {



    //let fuv_path = "/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF";

    let mut grid = uvex::empty_fuv();
    let coords = CoordinateSystem::new((1.0,0.0),(0.0,1.0), (0.0,0.0),"detecro".to_string(),"black".to_string());
    let mut detector = Grid::new_empty((10,10), (0.05,0.05),(0.0,0.0), (0.001),coords);

    let point = detector.random();
    let mut plot = Plot::new();
    grid.plot(&mut plot,PlotPoint::Given(point.0,point.1));
    detector.plot(&mut plot, PlotPoint::Given(point.0,point.1));

    plot.show("figure.svg").expect("lskjef");


    //grid.load_data_frames(fuv_path, ("XPOS", "YPOS"), (64, 64), (6.4, 6.4));
   // grid.validate();



    let coord = CoordinateSystem{
        x_axis: (1.0,0.0),
        y_axis: (0.0,1.0),
        center: (0.0, 0.0),
        color: "red".to_string(),
        label: "regular".to_string(),
    };

    let coord2 = CoordinateSystem{
        x_axis: (1.0,0.3),
        y_axis: (2.0,1.0),
        center: (0.0, 0.0),
        color: "green".to_string(),
        label: "off".to_string(),
    };
   // CoordinateSystem::plot_coordinate_systems(vec![&coord,&coord2])




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



