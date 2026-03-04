use std::fs::read;
use crate::demo_details;
use crate::demo_details::Demo_Details;
use crate::effects::{DARK_CURRENT, POINT_SPREAD_FUNCTION, QUANTUM_EFFICIENCY, READ_NOISE, REFLECTANCE, VINIETTING};
use crate::instrument::Instrument;
use crate::objects::TelescopeObject;
use crate::uvex::{initialize_fuv_channel, initialize_nuv_channel, initialize_spectrograph, initialize_tma};
use crate::uvex_details::{Use, UVEX_Details};





pub fn initialize_demo(demo_details: Demo_Details,path:&str) -> Instrument{


    let psf = POINT_SPREAD_FUNCTION.new("fuv_psf", demo_details.psf_directory.1);
    let qe = QUANTUM_EFFICIENCY.new("fuv_qe",demo_details.qe.1);
    let read_noise = READ_NOISE.new("fuv_read_noise",demo_details.read_noise.1);

    let detector = TelescopeObject::new("demo_detector",
                                        vec![qe,read_noise,psf],
                                        vec![]);


    let mut demo = Instrument{
        instrument_label: "demo".to_string(),
        entry_point: detector.unique_label.to_string(),
        measurement_points: vec![detector.unique_label.clone()],
        telescope_objects: vec![detector],

    };
    demo.write_to_yaml(path);
    demo

}


