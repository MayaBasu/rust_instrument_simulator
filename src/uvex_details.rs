use serde::{Serialize, Deserialize};
use crate::effects::{Effect};
use std::io::{Read, Write};
use std::fs;

#[derive(Serialize, Debug, Deserialize, Clone, Copy)]
pub enum Use{
    on,
    off,
}

#[derive(Serialize,Debug,Deserialize)]
pub struct TMA_Details { //struct which contains the details for a uvex instance
    pub spacecraft_pointing: (Use, f64),  //width of the gaussian to be convolved with the image

    pub tma_m1_reflectance: (Use, String),
    pub tma_m1_contamination: (Use, f64),

    pub tma_m2_reflectance: (Use, String),
    pub tma_m2_contamination: (Use, f64),

    pub tma_m3_reflectance: (Use, String),
    pub tma_m3_contamination: (Use, f64),

}
#[derive(Serialize,Debug,Deserialize)]
pub struct FUV_DETAILS {
    pub dichroic_fuv_transmission: (Use, String),

    pub fuv_psf_directory: (Use, String), //directory of FITS files

    pub fuv_qe: (Use, String),
    pub fuv_read_noise: (Use, String),
    pub fuv_dark_current: (Use, String),
    pub fuv_vinietting: (Use,String), //path to FITS

    pub fuv_dead_pixels: (Use, String),

}
#[derive(Serialize,Debug,Deserialize)]
pub struct NUV_DETAILS {
    pub dichroic_nuv_transmission: (Use, String),

    pub nuv_psf_directory: (Use, String), //directory of FITS files

    pub nuv_qe: (Use, String),
    pub nuv_read_noise: (Use, String),
    pub nuv_dark_current: (Use, String),
    pub nuv_vinietting: (Use,String), //path to FITS

    pub nuv_dead_pixels: (Use, String),
}

#[derive(Serialize,Debug,Deserialize)]
pub struct SPECTROGRAPH_DETAILS{

    pub slit_psf_directory: (Use,String), //directory of FITS files
    pub slit_mask: (Use,String),

    pub spectrograph_m1_reflectance: (Use,String),
    pub spectrograph_m1_contamination: (Use,f64),

    pub spectrograph_grating_reflectance: (Use,String),
    pub spectrograph_grating_contamination: (Use,f64),

    pub spectrograph_m3_reflectance: (Use,String),
    pub spectrograph_m3_contamination: (Use,f64),

    pub image_plane_qe: (Use,String),
    pub image_plane_dead_pixels: (Use,String),
    pub image_plane_read_noise: (Use,String),
    pub image_plane_dark_current: (Use,String),

}
#[derive(Serialize,Debug,Deserialize)]
pub struct UVEX_Details{
    pub tma_details: TMA_Details,
    pub fuv_details: FUV_DETAILS,
    pub nuv_details: NUV_DETAILS,
    pub spectrograph_details: SPECTROGRAPH_DETAILS,
}

impl UVEX_Details {
    pub fn default(path:&str) -> UVEX_Details {

        let d = Use::off;

        let tma_details = TMA_Details {
            spacecraft_pointing: (d, 1.0),  //width of the gaussian to be convolved with the image
            tma_m1_reflectance: (d, "data/tma/tma_m1_reflectance".to_string()),
            tma_m1_contamination: (d, 2.0),

            tma_m2_reflectance: (d, "data/tma/tma_m1_reflectance".to_string()),
            tma_m2_contamination: (d, 2.0),

            tma_m3_reflectance: (d, "data/tma/tma_m1_reflectance".to_string()),
            tma_m3_contamination: (d, 2.0),
        };

        let fuv_details = FUV_DETAILS {
            fuv_psf_directory: (d, "data/fuv/fuv_psf_directory".to_string()), //directory of FITS files
            fuv_qe: (d, "data/fuv/fuv_qe".to_string()),
            fuv_dead_pixels: (d, "data/fuv/fuv_dead_pixels".to_string()),
            fuv_read_noise: (d, "data/fuv/fuv_read_noise".to_string()),
            fuv_dark_current: (d, "data/fuv/fuv_dark_current".to_string()),
            dichroic_fuv_transmission: (d, "data/fuv/dichroic_fuv_transmission".to_string()),
            fuv_vinietting: (d, "data/fuv/path_to_fuv_and_nuv_vinietting".to_string()),
        };

        let nuv_details = NUV_DETAILS {
            nuv_psf_directory: (d, "data/nuv/nuv_psf_directory".to_string()), //directory of FITS files
            nuv_qe: (d, "data/nuv/nuv_qe".to_string()),
            nuv_dead_pixels: (d, "data/nuv/nuv_dead_pixels".to_string()),
            nuv_read_noise: (d, "data/nuv/nuv_read_noise".to_string()),
            nuv_dark_current: (d, "data/nuv/nuv_dark_current".to_string()),
            dichroic_nuv_transmission: (d, "data/nuv/dichroic_fuv_transmission".to_string()),
            nuv_vinietting: (d, "data/nuv/path_to_fuv_and_nuv_vinietting".to_string())
        };

        let spectrograph_details = SPECTROGRAPH_DETAILS{

            slit_psf_directory: (d,"data/spectrograph/path_to_slit_psf_directory".to_string()), //directory of FITS files
            slit_mask: (d, "data/spectrograph/slit_mask".to_string()),

            spectrograph_m1_reflectance: (d,"data/spectrograph/spectrograph_m1_reflectance".to_string()),
            spectrograph_m1_contamination: (d,2.0),


            spectrograph_grating_reflectance: (d,"data/spectrograph/spectrograph_grating_reflectance".to_string()),
            spectrograph_grating_contamination: (d,2.0),

            spectrograph_m3_reflectance: (d,"data/spectrograph/spectrograph_m3_reflectance".to_string()),
            spectrograph_m3_contamination: (d,2.0),

            image_plane_qe: (d,"data/spectrograph/image_plane_qe".to_string()),
            image_plane_dead_pixels: (d,"data/spectrograph/image_plane_dead_pixels".to_string()),
            image_plane_read_noise: (d,"data/spectrograph/image_plane_read_noise".to_string()),
            image_plane_dark_current: (d,"data/spectrograph/image_plane_dark_current".to_string()),

        };

        let uvex = UVEX_Details{
            tma_details,
            fuv_details,
            nuv_details,
            spectrograph_details,
        };

        uvex.write_to_yaml(path);
        uvex
    }

    pub fn write_to_yaml(&self, file_name:&str,) {
        println!("Writing uvex details to {:?}", file_name);
        let serialized_self = serde_yaml::to_string(&self).expect("Failed to YAMLify the object");
        let mut file = fs::File::create(file_name).expect("Couldn't create the config file");
        write!(file, "{}", serialized_self).expect("Failed to write YAML to config file");
    }

    pub fn read_from_yaml(file_name:&str)-> UVEX_Details {
        let details: String = fs::read_to_string(file_name).expect("couldn't read from details file");
        let details: UVEX_Details = serde_yaml::from_str(details.as_str()).expect("invalid details data");
        details
    }

}