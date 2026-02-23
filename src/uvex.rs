use crate::objects::TelescopeObject;
use crate::instrument::Instrument;
use crate::details::{FUV_DETAILS, NUV_DETAILS, SPECTROGRAPH_DETAILS, TMA_Details, UVEX_Details};
use crate::effects::*;



//generate the mirror objects for the initial optical train of the UVEX instrument
pub fn initialize_tma(tma_details: TMA_Details) -> (TelescopeObject, TelescopeObject, TelescopeObject){

    //contamination for each mirror in the tma (this is a single number)
    let m1_contamination = CONTAMINATION.new("m1_contamination", tma_details.tma_m1_contamination.1.to_string());
    let m2_contamination = CONTAMINATION.new("m2_contamination", tma_details.tma_m2_contamination.1.to_string());
    let m3_contamination = CONTAMINATION.new("m3_contamination", tma_details.tma_m3_contamination.1.to_string());

    //reflectivity as a function of wavelength for each mirror in the tma
    let m1_reflectivity = REFLECTANCE.new("m1_reflectivity", tma_details.tma_m1_reflectance.1);
    let m2_reflectivity = REFLECTANCE.new("m2_reflectivity", tma_details.tma_m2_reflectance.1);
    let m3_reflectivity = REFLECTANCE.new("m3_reflectivity", tma_details.tma_m3_reflectance.1);

    //now we initialize the mirror objects with these effects
    let m1 = TelescopeObject::new("m1", vec![m1_reflectivity,m1_contamination], vec!["m2"]);
    let m2 = TelescopeObject::new("m2", vec![m2_reflectivity,m2_contamination], vec!["m3"]);
    let m3 = TelescopeObject::new("m3", vec![m3_reflectivity,m3_contamination], vec![]);

    (m1,m2,m3)

}


pub fn initialize_fuv_channel(fuv_details: FUV_DETAILS) -> (TelescopeObject, TelescopeObject){

    let fuv_vinietting =VINIETTING.new("fuv_vinietting", fuv_details.fuv_vinietting.1);
    let fuv_psf = POINT_SPREAD_FUNCTION.new("fuv_psf", fuv_details.fuv_psf_directory.1);
    let fuv_qe = QUANTUM_EFFICIENCY.new("fuv_qe",fuv_details.fuv_qe.1);
    let fuv_dead_pixels = QUANTUM_EFFICIENCY.new("fuv_quantum_efficiency",fuv_details.fuv_dead_pixels.1);
    let fuv_dark_current = DARK_CURRENT.new("fuv_dark_current",fuv_details.fuv_dark_current.1);
    let fuv_read_noise = READ_NOISE.new("fuv_read_noise",fuv_details.fuv_read_noise.1);
    let dichroic_fuv_transmission = REFLECTANCE.new("dichroic_fuv_transmission", fuv_details.dichroic_fuv_transmission.1);

    let detector = TelescopeObject::new("fuv_detector",
                                  vec![fuv_psf, fuv_qe,fuv_vinietting,fuv_dark_current,fuv_read_noise,fuv_dead_pixels],
                                  vec![]);


    let dichroic = TelescopeObject::new("dichroic_fuv_path",
                                  vec![dichroic_fuv_transmission],
                                  vec!["fuv_detector"]);
    (dichroic,detector)

}

pub fn initialize_nuv_channel(nuv_details: NUV_DETAILS) -> (TelescopeObject, TelescopeObject){


    let nuv_vinietting =VINIETTING.new("nuv_vinietting", nuv_details.nuv_vinietting.1);
    let nuv_psf = POINT_SPREAD_FUNCTION.new("nuv_psf", nuv_details.nuv_psf_directory.1);
    let nuv_qe = QUANTUM_EFFICIENCY.new("nuv_qe",nuv_details.nuv_qe.1);
    let nuv_dead_pixels = QUANTUM_EFFICIENCY.new("nuv_quantum_efficiency",nuv_details.nuv_dead_pixels.1);
    let nuv_dark_current = DARK_CURRENT.new("nuv_dark_current",nuv_details.nuv_dark_current.1);
    let nuv_read_noise = READ_NOISE.new("nuv_read_noise",nuv_details.nuv_read_noise.1);
    let dichroic_nuv_transmission = REFLECTANCE.new("dichroic_nuv_transmission", nuv_details.dichroic_nuv_transmission.1);

    let detector = TelescopeObject::new("nuv_detector",
                                        vec![nuv_psf, nuv_qe,nuv_vinietting,nuv_dark_current,nuv_read_noise,nuv_dead_pixels],
                                        vec![]);

    let dichroic = TelescopeObject::new("dichroic_nuv_path",
                                        vec![dichroic_nuv_transmission],
                                        vec!["nuv_detector"]);
    (dichroic,detector)

}

pub fn initialize_spectrograph(spectrograph_details: SPECTROGRAPH_DETAILS) -> (TelescopeObject, TelescopeObject, TelescopeObject,TelescopeObject, TelescopeObject){
    let slit_mask = SLIT.new("slit_mask",spectrograph_details.slit_mask.1);
    let slit_psf = POINT_SPREAD_FUNCTION.new("slit_psf",spectrograph_details.slit_psf_directory.1);


    let spectrograph_m1_contamination = CONTAMINATION.new("spectrograph_m1_contamination", spectrograph_details.spectrograph_m1_contamination.1.to_string());
    let spectrograph_grating_contamination = CONTAMINATION.new("spectrograph_grating_contamination", spectrograph_details.spectrograph_grating_contamination.1.to_string());
    let spectrograph_m3_contamination = CONTAMINATION.new("spectrograph_m3_contamination", spectrograph_details.spectrograph_m3_contamination.1.to_string());


    let spectrograph_m1_reflectivity = REFLECTANCE.new("spectrograph_m1_reflectance", spectrograph_details.spectrograph_m1_reflectance.1);
    let spectrograph_grating_reflectivity = REFLECTANCE.new("spectrograph_grating_reflectance", spectrograph_details.spectrograph_grating_reflectance.1);
    let spectrograph_m3_reflectivity = REFLECTANCE.new("spectrograph_m3_reflectance", spectrograph_details.spectrograph_m3_reflectance.1);

    let image_plane_qe = QUANTUM_EFFICIENCY.new("image_plane_qe",spectrograph_details.image_plane_qe.1);
    let image_plane_dead_pixels = QUANTUM_EFFICIENCY.new("image_plane_dead_pixels",spectrograph_details.image_plane_dead_pixels.1);
    let image_plane_read_noise = READ_NOISE.new("image_plane_read_noise",spectrograph_details.image_plane_read_noise.1);
    let image_plane_dark_current = DARK_CURRENT.new("image_plane_dark_current",spectrograph_details.image_plane_dark_current.1);


    let slit = TelescopeObject::new("slit",
                                    vec![slit_psf,slit_mask],
                                    vec!["m1"]);
    let m1 = TelescopeObject::new("spectrograph_m1",
                                  vec![spectrograph_m1_reflectivity,spectrograph_m1_contamination],
                                  vec!["spectrograph_grating"]);
    let grating = TelescopeObject::new("spectrograph_grating",
                                       vec![spectrograph_grating_reflectivity,spectrograph_grating_contamination],
                                       vec!["spectrograph_m3"]);
    let m3 = TelescopeObject::new("spectrograph_m3",
                                  vec![spectrograph_m3_reflectivity,spectrograph_m3_contamination],
                                  vec!["image_plane"]);
    let image_plane = TelescopeObject::new("image_plane",
                                           vec![image_plane_qe,image_plane_read_noise,image_plane_dark_current,image_plane_dead_pixels],
                                           vec!["image_plane"]);


    (slit, m1,grating,m3,image_plane)

}

pub fn initialize_uvex(uvex_details: UVEX_Details) -> Instrument{


    let (m1, m2, mut m3) = initialize_tma(uvex_details.tma_details);
    let (fuv_dichroic, fuv_detector) = initialize_fuv_channel(uvex_details.fuv_details);
    let (nuv_dichroic, nuv_detector) = initialize_nuv_channel(uvex_details.nuv_details);

    let (slit,
        spectrograph_m1,
        grating,
        spectrograph_m3,
        image_plane) = initialize_spectrograph(uvex_details.spectrograph_details);

    //connect the output of m3 to the three channels
    m3.add_recipient(fuv_dichroic.unique_label.as_str());
    m3.add_recipient(nuv_dichroic.unique_label.as_str());
    m3.add_recipient(slit.unique_label.as_str());

    let mut uvex = Instrument{
        instrument_label: "uvex".to_string(),

        entry_point: m1.unique_label.to_string(),

        measurement_points: vec![fuv_detector.unique_label.clone(),nuv_detector.unique_label.clone(),image_plane.unique_label.clone()],

        telescope_objects: vec![m1,m2,m3,
                                fuv_dichroic,fuv_detector,
                                nuv_dichroic,nuv_detector,
                                slit, spectrograph_m1, grating, spectrograph_m3, image_plane],

    };

    uvex.write_to_yaml("uvex.yaml");
    uvex


}


/*
let (m1, m2, mut m3) = initialize_tma(
        ud.tma_m1_contamination.1.to_string(), ud.tma_m1_reflectance.1,
        ud.tma_m2_contamination.1.to_string(), ud.tma_m2_reflectance.1,
        ud.tma_m3_contamination.1.to_string(), ud.tma_m3_reflectance.1);


    let (fuv_dichroic, fuv_detector) = initialize_fuv_channel(ud.fuv_and_nuv_vinietting.1.clone(),
                                                              ud.fuv_psf_directory.1,
                                                              ud.fuv_qe.1,
                                                              ud.fuv_dead_pixels.1,
                                                              ud.fuv_read_noise.1,
                                                              ud.fuv_dark_current.1,
                                                              ud.dichroic_fuv_transmission.1);

    let (nuv_dichroic, nuv_detector) = initialize_nuv_channel(ud.fuv_and_nuv_vinietting.1.clone(),
                                                              ud.nuv_psf_directory.1,
                                                              ud.nuv_qe.1,
                                                              ud.nuv_dead_pixels.1,
                                                              ud.nuv_read_noise.1,
                                                              ud.nuv_dark_current.1,
                                                              ud.dichroic_nuv_transmission.1);

    let (slit, spectrograph_m1, grating, spectrograph_m3, image_plane) = initialize_spectrograph(
        ud.slit_mask.1,
        ud.slit_psf_directory,
        ud.spectrograph_m1_reflectance,
        ud.spectrograph_m1_contamination,
        ud.spectrograph_grating_reflectance,
        ud.spectrograph_grating_contamination,
        ud.spectrograph_m3_reflectance,
        ud.spectrograph_m3_contamination,
        ud.image_plane_qe,
        ud.image_plane_dead_pixels,
        ud.image_plane_read_noise,
        ud.image_plane_dark_current,
    );
 */
