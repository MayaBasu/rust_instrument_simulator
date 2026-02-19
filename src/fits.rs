extern crate fitrs;
use fitrs::{Fits, FitsData, FitsDataArray};

pub const fits_path: &str = "/Users/mayabasu/RustroverProjects/image_simulator_outline/python_plotting/UVEX_FUV_PSF_1um_F001 1.fits";
/*
ub fn open_fits(path:&str) -> Vec<f32>{
    let fits = Fits::open(path).expect("Failed to open");
    // Iterate over HDUs


    let data = match fits.0.read_data() {
        FitsData::FloatingPoint32(FitsDataArray { shape, data }) => {
            data
        }
        _ => { panic!() }
    };

    data


}

 */
