use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use uvex_fitrs::{Fits, FitsData, FitsDataArray, HeaderValue};
use crate::grid::Grid;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct DataFrame {
    pub x_pixels: usize, //e.g 64 for a psf
    pub y_pixels: usize,
    pub x_pos: f64, //position in FOV in degrees
    pub y_pos: f64,
    pub inverse_scale: f64, //4,000 for qe, dark current, anything samples per pixel but this is 40,000 for the PSF files
    pub data: Vec<f32>,
    pub path: PathBuf,
}
impl DataFrame{
    pub fn frame_psf(
        x_pixels: usize,
        y_pixels: usize,
        inverse_scale: f64, //TODO check the psf scales
        path: PathBuf,
    ) -> DataFrame{

        println!("Loading PSF file {:?} into a DataFrame",path.clone());
        let fits = Fits::open(path.clone()).expect("Failed to open PSF FITS file");
        let primary_hdu= fits.iter().next().expect("Couldn't find primary HDU");

        let (data,shape) = match primary_hdu.read_data() {
            FitsData::FloatingPoint32(FitsDataArray { shape, data }) => (data,shape),
            _ => panic!("Could not unpack PSF data")
        };
        assert_eq!(shape[0], x_pixels,"Diva down!"); //check that the data is the expected size
        assert_eq!(shape[1], y_pixels,"Diva down!");

        let x_pos = *match primary_hdu.value("XFLD").expect("failed to get xpos") {
            HeaderValue::RealFloatingNumber(xpos)=> xpos,
            _ => panic!("could not unpack xpos")
        };
        let y_pos = *match primary_hdu.value("YFLD").expect("failed to get xpos") {
            HeaderValue::RealFloatingNumber(ypos)=> ypos,
            _ => panic!("could not unpack ypos")
        };

        DataFrame{
            x_pixels, //e.g 64 for a psf
            y_pixels,
            x_pos, //position in FOV in degrees
            y_pos,
            inverse_scale, //4,000 for qe, dark current, anything samples per pixel but this is 40,000 for the PSF files
            data,
            path,
        }
    }
    pub fn snap_to_grid(&self, grid: &Grid) -> usize{
        let index = grid.snap(self.x_pos,self.y_pos);
        index
    }
}