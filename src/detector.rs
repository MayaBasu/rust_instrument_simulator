use plotpy::{Image, Plot, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::coordinate_system::CoordinateSystem;
use crate::grid::Grid;

pub struct detector{
    coordinate_system: CoordinateSystem,
    grid: Grid,
    data: Vec<Vec<f64>>
}

impl detector{

    pub fn show_read_out(){
        let mut data = Vec::new();
        for row in 0..4000{
            let mut row_vec = Vec::new();
            for column in 0..4000{
                row_vec.push(row + column)
            }
            data.push(row_vec);
        }


        let mut img = Image::new();
        img.set_colormap_name("terrain").set_extra("alpha=0.8").draw(&data);
        let mut plot = Plot::new();
        plot.add(&img);
        plot.save( "eeeeh").expect("couldn't save plot!")
    }

}


