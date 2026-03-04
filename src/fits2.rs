
use uvex_fitrs::{Fits, FitsData, FitsDataArray};


pub const fits_path: &str = "/Users/mayabasu/RustroverProjects/image_simulator_outline/data/demo/demo_psf/FUV PSF/UVEX_FUV_PSF_1um_F001.fits";
pub const fits_path2: &str = "/Users/mayabasu/RustroverProjects/image_simulator_outline/data/demo/demo_psf/FUV PSF/UVEX_FUV_PSF_1um_F002.fits";
    //"data/demo/demo_psf/FUV PSF/UVEX_FUV_PSF_1um_F001.fits";

pub fn open_fits(path:&str)  -> u64{
    println!("OPENING GGG");
    let fits = Fits::open(path).expect("Failed to open");
    // Iterate over HDUs


    let Some(hdu) = fits.iter().next() else { panic!("alsiejof")};


    let data = match hdu.read_data() {
        FitsData::FloatingPoint32(FitsDataArray { shape, data }) => {
            println!("{:?}", shape);
            //println!("{:?}", data);
            data
        }
        _ => { panic!("couldnt get data")}
    };
    let start = hdu.data_start;
    println!("data start is at {:?}",hdu);

    //println!("{:?}", data);

    start




}


