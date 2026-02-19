extern crate fitrs;
use fitrs::{Fits, FitsData, FitsDataArray};

pub const fits_path: &str = "/Users/mayabasu/RustroverProjects/image_simulator_outline/python_plotting/UVEX_FUV_PSF_1um_F001 1.fits";
pub fn open_fits(path:&str){
    let fits = Fits::open(path).expect("Failed to open");
    // Iterate over HDUs
    for hdu in fits.iter() {
        println!("{:?}", hdu.value("EXTNAME"));
        println!("{:?}", hdu.read_data());
    }


}
