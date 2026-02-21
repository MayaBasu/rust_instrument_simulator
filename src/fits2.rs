
use uvex_fitrs::{Fits, FitsData, FitsDataArray};


pub const fits_path: &str = "/Users/mayabasu/RustroverProjects/image_simulator_outline/python_plotting/UVEX_FUV_PSF_1um_F001 1.fits";

pub fn open_fits(path:&str)  -> u64{
    println!("OPENING GGG");
    let fits = Fits::open(path).expect("Failed to open");
    // Iterate over HDUs


    let Some(hdu) = fits.iter().next() else { panic!("alsiejof")};

    //println!("header length is {:?}", hdu.header.len());
    let data = match hdu.read_data() {
        FitsData::FloatingPoint32(FitsDataArray { shape, data }) => {
            //println!("{:?}", shape);
            //println!("{:?}", data);
            data
        }
        _ => { panic!("couldnt get data")}
    };
    let start = hdu.data_start;
    println!("data start is at {start}");
    //println!("{:?}", data);

    start




}


