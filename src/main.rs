
mod effects;
mod data_cube_management;
mod hallucinations;


use std::fs::File;
use ndarray::prelude::*;

use data_cube_management::SpatialSpectralEffect;
use crate::hallucinations::hallucinate_spatial_spectral;
pub const float_size:usize = 8;  //number of bytes in one float


fn main() {
    let num_pixles:usize = 6000;
    let frequencies = vec![1];


    let input = "/Users/mayabasu/Desktop/data/lite.txt";
    let result = "/Users/mayabasu/Desktop/data/output.txt";
    File::create(input);
    File::create(result);
    hallucinate_spatial_spectral(input,num_pixles,frequencies.clone());

    let qe = SpatialSpectralEffect::initialize("lske".to_string(), true, effects::EffectType::ComponentWiseAddition,num_pixles, frequencies.clone(), input);
    let qe2 = SpatialSpectralEffect::initialize("lske".to_string(), true, effects::EffectType::ComponentWiseAddition,num_pixles, frequencies.clone(), input);
    let qe3 = SpatialSpectralEffect::initialize("lske".to_string(), true, effects::EffectType::ComponentWiseAddition,num_pixles, frequencies.clone(), input);
    let qe4 = SpatialSpectralEffect::initialize("lske".to_string(), true, effects::EffectType::ComponentWiseAddition,num_pixles, frequencies.clone(), input);


    data_cube_management::combine_data_cubes(vec![qe, qe2, qe3, qe4], data_cube_management::ElementWiseCombinationType::ComponentWiseAddition, "result");
    /*
    let fake_light =Array3::random(
        (4000,4000, 10),
        Uniform::new(0., 1.)).to_owned();

     */
    //println!("FAke light is {:?}",fake_light);
    //qe.apply(fake_light)
   // QuantumEfficiency::hallucinate(4000,vec![1000,1500,2000,2500]);
  //  println!("serializing!");

}

//integrate bs checking/data verification and link up gneration/storage


