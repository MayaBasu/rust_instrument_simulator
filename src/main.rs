use std::time::Instant;
use plotpy::Plot;
use rand::{random, RngExt};
use crate::coordinate_system::{CoordinateSystem, Coordinates};
use crate::dichroic::apply_dichroic;
use crate::grid1d::GRID1D;
use crate::grid2d::{GRID2D, PlotPoint};
use crate::point::Point;
use crate::psf_grid::PsfGrid;
use crate::sources::{PointSource, SourceList};
use crate::uvex_details::UVEX_Details;
mod objects;
mod uvex;

mod effects;
mod instrument;
mod sources;
mod fits2;

mod uvex_details;



mod psf;
mod grid2d;
mod coordinate_system;
mod psf_grid;
mod point;
mod detector;
mod datafile_reader;
mod dichroic;
mod grid1d;

fn main() {

    //apply_dichroic(SourceList::new_empty(2));
    let mut plot = Plot::new();
    let grid1d = GRID1D::new_empty(10,1.0,0.0,0.1,1.0);
    grid1d.plot_points(&mut plot,PlotPoint::Random);
    plot.show("slefje").unwrap();

    /*




    let fuv_path = "/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF";
    let mut grid = uvex::empty_fuv();
    let mut plot = Plot::new();
    grid.plot_points(&mut plot, PlotPoint::No);
    grid.plot_outline(&mut plot, "yellow");
    let uvex_detector_array = &mut detector::DetectorArray::uvex_detector_array(0.02,0.02);
    for detector in &uvex_detector_array.detectors{
        detector.grid.plot_outline(&mut plot, "blue");
    }

   // plot.show("ksenf").expect("hHHHHHH");


    let mut psf_grid = PsfGrid::new(grid);
    psf_grid.load_data_frames(fuv_path, ("XFLD", "YFLD"), (64, 64), (6.4, 6.4));
    psf_grid.validate();


    /*
    for i in 0..psf_grid.grid.num_points{
        let (x,y) = psf_grid.grid.xy_indices(i);
        let x_pos = (3000/psf_grid.grid.x_num)*x + 500;
        let y_pos = (3000/psf_grid.grid.y_num)*y + 500;
        matrices.push(((x_pos,y_pos),psf_grid.grid_psf(i)));
    }

     */
   // println!("AMMMMMMMMM ");
    let mut detector = uvex_detector_array.detectors[0].clone();

    let start = Instant::now();
    let source_list = SourceList::new_random_point_source_field(1000000,0.0,1000.0,&psf_grid.grid);
    let duration = start.elapsed();
    println!("{:?}",source_list.sources[0].spectrum);
    println!("Generated {:?} fake point sources in {:?}", source_list.sources.len(), duration.as_millis());


    let new_source_list = dichroic::apply_dichroic(source_list);

    detector.show_read_out(new_source_list,psf_grid);





   // let coords = CoordinateSystem::new((1.0,0.2),(-0.2,1.0), (0.0,0.0),"detecro".to_string(),"black".to_string());
    //let mut Detector = Grid::new_empty((10,10), (0.05,0.05),(0.0,0.0), (0.001),Coordinates::RELATIVE(coords));

    //let point = Detector.random();
    //let mut plot = Plot::new();
    //grid.plot(&mut plot,PlotPoint::Given(point.x,point.y));
    //Detector.plot(&mut plot, PlotPoint::Given(point.x,point.y));

    //plot.show("figure.svg").expect("lskjef");




   // grid.validate();



   // let details = uvex_details::UVEX_Details::default("details.yaml");

    //let demo = initialize_demo(details,"configuration/demo.yaml");
    //let mut grid = Grid::load_fuv(fuv_path);
    //grid.validate();
    //println!("{:?}",grid.corner());
    //println!("{:?}",grid.size());
    //grid.pretty_print()

     */


}
/*


    let spectrum = [1.0;spectral_resolution];
    let mut source_list = source_list::new_random_point_source_field(3, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, spectrum);
    uvex.run(&mut source_list);
    source_list.write_to_yaml("sources");

 */

   // println!("{:?}", source_list)





