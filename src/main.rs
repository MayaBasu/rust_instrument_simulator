use plotpy::Plot;
use rand::RngExt;
use crate::coordinate_system::{CoordinateSystem, Coordinates};
use crate::grid::{Grid, PlotPoint};
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
mod grid;
mod coordinate_system;
mod psf_grid;
mod point;
mod detector;

fn main() {



    let fuv_path = "/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF";
    let mut grid = uvex::empty_fuv();
    let mut plot = Plot::new();
    grid.plot_points(&mut plot, PlotPoint::No);
    grid.plot_outline(&mut plot, "yellow");
    let detector = &mut detector::detector::new_uvex();
    detector.grid.plot_outline(&mut plot, "blue");
    plot.show("ksenf").expect("hHHHHHH");


    let mut psf_grid = PsfGrid::new(grid);
    psf_grid.load_data_frames(fuv_path, ("XFLD", "YFLD"), (64, 64), (6.4, 6.4));
    psf_grid.validate();

    let mut points = Vec::new();
    /*
    for i in 0..psf_grid.grid.num_points{
        let (x,y) = psf_grid.grid.xy_indices(i);
        let x_pos = (3000/psf_grid.grid.x_num)*x + 500;
        let y_pos = (3000/psf_grid.grid.y_num)*y + 500;
        matrices.push(((x_pos,y_pos),psf_grid.grid_psf(i)));
    }

     */
   // println!("AMMMMMMMMM ");

    for i in 1..100000{ //100000

       // println!("AMMMMMMMMM {:?}",psf_grid.grid.random());

        let point = detector.grid.random();
        let mut rng = rand::rng();
        let luminosity:f32 = rng.random();
        points.push((point,luminosity*10000.0));
        println!("Luminosity is {:?}",luminosity);

    }

    let example_psf = psf_grid.grid_psf(1);
    println!("points {:?}",points.len());

    let detector = &mut detector::detector::new_uvex();
    detector.show_read_out(points,psf_grid);





   // let coords = CoordinateSystem::new((1.0,0.2),(-0.2,1.0), (0.0,0.0),"detecro".to_string(),"black".to_string());
    //let mut detector = Grid::new_empty((10,10), (0.05,0.05),(0.0,0.0), (0.001),Coordinates::RELATIVE(coords));

    //let point = detector.random();
    //let mut plot = Plot::new();
    //grid.plot(&mut plot,PlotPoint::Given(point.x,point.y));
    //detector.plot(&mut plot, PlotPoint::Given(point.x,point.y));

    //plot.show("figure.svg").expect("lskjef");




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



