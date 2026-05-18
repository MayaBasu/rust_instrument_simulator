use plotpy::{Image, Plot, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::coordinate_system::{CoordinateSystem, Coordinates};
use crate::grid::Grid;

use std::time::{Duration, Instant};
use crate::point::Point;
use crate::psf_grid::PsfGrid;

pub struct detector{
    pub(crate) grid: Grid,
    data: Vec<Vec<f32>>
}

impl detector{

    pub fn new_uvex() -> detector{
        let num_pixels =3096*3;
        let pixel_to_deg_scale = 3.5/num_pixels as f64; //Degrees in FOV to pixels
        let detector_x_axis = (pixel_to_deg_scale,0.0);
        let detector_y_axis = (0.0,pixel_to_deg_scale);
        let detector_center = (-0.5,0.0);
        let coordinate_system = CoordinateSystem::new(detector_x_axis,detector_y_axis,detector_center, "Detector 1".to_string(), "magenta".to_string());
        let grid = Grid::new_empty((num_pixels,num_pixels), (1.0,1.0), (0.0,0.0), 0.01, Coordinates::RELATIVE(coordinate_system));
        let mut data = Vec::with_capacity(num_pixels*num_pixels);
        for _row in 0..num_pixels{
            let mut row_vec = Vec::with_capacity(num_pixels);
            for _column in 0..num_pixels{
                row_vec.push(0.0)
            }
            data.push(row_vec);
        }
        detector{grid, data}
    }
    pub fn show_read_out(&mut self, points:Vec<(Point,f32)>,psf_grid:PsfGrid){

        let start = Instant::now();
        let mut data = &mut self.data;
        /*
        //add background
        for row in 0..4000{
            let mut row_vec = Vec::new();
            for column in 0..4000{
                //if row < matrix_x && column < matrix_y{
                   // row_vec.push(matrix[column][row]);

                //}else{
                //    row_vec.push(0.0);
               // }
                //row_vec.push((((row + column) as f32)/100.0))
                row_vec.push(0.0)
            }
            data.push(row_vec);
        }

         */
       // println!("{}, {}",data.len(),data[0].len());


        for point in points{
           // println!("POINT!!!");
           // println!("adding point {:?}",point);
         //   println!("as absolute: {:?}", point.to_absolute());

            //println!("{:?} adding at {:?}",(point.x,point.y),(new_x,new_y));
            let psf = psf_grid.interpolated_psf(&point.0);
            let ((x_mod,y_mod),binned_psf) = self.grid.bin_up_patch(point.0,&psf,10);
            let binned_matrix_x = binned_psf[0].len();
            let binned_matrix_y = binned_psf.len();

            println!("{:?}",binned_psf);
            /*

            for row in 0..matrix_y{
                for column in 0..matrix_x{

                    //println!("{}{}",column + y, row + y);
                    data[column + new_x][row + new_y] += psf[column][row]
                }
            }

             */

            for row in 0..binned_matrix_y{
                for column in 0..binned_matrix_x{


                    //println!("{}{}",column + y, row + y);
                    if column + y_mod < 4096*3 && row + x_mod < 4096*3{
                        data[column + y_mod][row + x_mod] += binned_psf[column][row]*point.1;
                    }else{
                        println!("dropping pixel");
                    }

                   // println!("modifying pixel {:?} to be {:?}",(row + x_mod,column + y_mod),binned_psf[column][row]);
                }
            }




        }

       // data[0][0]  += 100.0;


        let sum:f32  = data.iter().flatten().sum();
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);

        println!("made array :{}",sum);


        let mut img = Image::new();
        img.set_colormap_name("terrain").set_extra("alpha=0.8").draw(data);
        let mut plot = Plot::new();
        plot.add(&img);

        plot.show( "eeeeh").expect("couldn't save plot!")
    }

}


