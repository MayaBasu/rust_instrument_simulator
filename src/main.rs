use rand_distr::Distribution;
use std::time::Instant;


use eframe::egui;
use eframe::egui::Response;
use eframe::emath::{vec2, Vec2};

use egui_extras::{Size, StripBuilder};
use egui_plot::{Legend, PlotPoints, Points};
use egui_plot::Line;
use egui_plot::Plot;


use rand::{random, RngExt};
use rand_distr::Poisson;
use crate::binom::test_binom;
use crate::coordinate_system::{CoordinateSystem, Coordinates};
use crate::datafile_reader::{ FrequencyFile};
use crate::flatfieldillumination::{flatfield, load_file, load_flatfield_illumination};
use crate::frequency_response_files::load;
//use crate::dichroic::apply_dichroic;
use crate::grid1d::GRID1D;
use crate::grid2d::{GRID2D, PlotPoint};
use crate::plotting::{display, run};
use crate::geometry::Point;
use crate::psf_grid::PsfGrid;
use crate::point_sources::{PointSource, SourceList};
use crate::units::Units;
use crate::uvex_details::UVEX_Details;
mod objects;
mod uvex;

mod effects;
mod instrument;
mod point_sources;
mod fits2;

mod uvex_details;



mod psf;
mod grid2d;
mod coordinate_system;
mod psf_grid;
mod geometry;
mod detector;
mod datafile_reader;
mod dichroic;
mod grid1d;

mod spectrum_examples;

mod frequency_response_files;
mod plotting;
mod binom;
mod flatfieldillumination;
mod units;

fn main() {

    /*
        let grid = uvex::empty_fuv();
        let sources = point_sources::SourceList::random_bands_point_source_field(1, 0.5, 1.0, &grid);
        flatfieldillumination::apply_flatfield_illumination(sources);
        */








        //display(matrix);
        /*

        for i in 0..10{
            let mut poi = Poisson::new(10.0).unwrap();
            let v: f64 = poi.sample(&mut rand::rng());
            println!("{:?}",v);
        }

         */

        //test_binom();

       // load();







        //egui::go();
        //load();

        //apply_dichroic(SourceList::new_empty(2));
        /*

        let FUV_contamination  = "/Users/mayabasu/Desktop/uvex_psf_files/UVIM_FUV_contamination.dat".to_string();
        let FUV_contamination = FrequencyFile::new_empty(FUV_contamination,890,1.0,110.0,0.01,1.0);
        let FUV_contamination = FUV_contamination.load_data(false);

        let standard_grid = GRID1D::new_empty(1800,0.5,100.0,0.01,1.0);

         */
        //show_interpolation();



       // snap_spectrum_to_grid(FUV_contamination,grid1d)
        /*
        let mut plot = Plot::new();
        let grid1d = GRID1D::new_empty(10,1.0,0.0,0.1,1.0);
        grid1d.plot_points(&mut plot,PlotPoint::Random);
        plot.show("slefje").unwrap();

         */






        let fuv_path = "/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF";
        let mut grid = uvex::empty_fuv();
        let mut plot = plotpy::Plot::new();
        grid.plot_points(&mut plot, PlotPoint::No);
        grid.plot_outline(&mut plot, "yellow");
        let uvex_detector_array = &mut detector::DetectorArray::uvex_detector_array(0.02,0.02);
        let illumination = load_flatfield_illumination(4000*3);
        //println!("illuminatin: {:?}",illumination.grid);
       // illum_grid.plot_outline(&mut plot, "green");
        for detector in &uvex_detector_array.detectors{
                detector.grid.plot_outline(&mut plot, "purple");
        }
        let mut psf_grid = PsfGrid::new(grid);

        psf_grid.grid.plot_outline(&mut plot, "orange");


     //   plot.show("ksenf").expect("hHHHHHH");



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
        let mut source_list = SourceList::random_full_spectrum_point_source_field(1000000,500.0,1000.0,&psf_grid.grid,Units::Flux);
        let duration = start.elapsed();
        println!("Generated {:?} fake point sources in {:?}", source_list.sources.len(), duration.as_millis());


        // let source_list = flatfieldillumination::apply_flatfield_illumination(source_list);
        let flat = load_flatfield_illumination(4096*3);
        source_list.apply_flatfield_illumination(flat);

        //println!("{:?}",source_list);
        //println!("{:?}",source_list.sources[0].spectrum);


        let new_source_list = dichroic::apply_dichroic(source_list);
        /*
        for point in &new_source_list.sources{
                println!("{:?}",point);
                println!("{:?}",detector.grid.inside_or_outside(&point.point))
        }

         */


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




    }
    /*


        let spectrum = [1.0;spectral_resolution];
        let mut source_list = source_list::new_random_point_source_field(3, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, spectrum);
        uvex.run(&mut source_list);
        source_list.write_to_yaml("sources");

     */

       // println!("{:?}", source_list)





