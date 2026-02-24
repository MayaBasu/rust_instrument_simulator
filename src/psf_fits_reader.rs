use uvex_fitrs::*;
use crate::fits2::fits_path;
use ndarray::{array, Array2, ArrayBase, Ix2, OwnedRepr};
use ndarray::{Axis};
use ndarray::prelude::*;
use ndarray::Array;
use std::f64::INFINITY as inf;

pub fn open_psf_fits(path:&str){
    let fits = uvex_fitrs::Fits::open(path).expect("Failed to open PSF FITS file");
    let primary_hdu= fits.iter().next().expect("Couldn't find primary HDU");

    let primary_hdu_data = match primary_hdu.read_data() {
        FitsData::FloatingPoint32(FitsDataArray { shape, data }) => {
            //println!("{:?}", shape);
            //println!("{:?}", data);
            //println!("{:?}",data.len());
            (data,shape)
        }
        _ => {panic!("Could not unpack PSF data") }
    };

    let (data_chunks, []) = primary_hdu_data.0.as_chunks::<64>() else {
        panic!("data is the wrong size have even length")
    };



    let data = array![data_chunks];


}


