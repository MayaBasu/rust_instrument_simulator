
mod effects;

mod hallucinations;

use std::fs::File;
use ndarray::prelude::*;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use effects::SpatialSpectralEffect;
use crate::hallucinations::hallucinate_spatial_spectral;


fn main() {

    //let a = Array::<f64, _>::zeros((3, 2, 4).f());
    let filename = "/Users/mayabasu/Desktop/data/smallfile.txt";

    //let file = hallucinate_spatial_spectral(filename,3,vec![1,2]);
    let qe = SpatialSpectralEffect::initialize("lske".to_string(), true, effects::EffectType::ComponentWiseAddition,10, vec![1000,1500], filename);


    let fake_light =Array3::random(
        (3,3, 2),
        Uniform::new(0., 1.)).to_owned();
    println!("FAke light is {:?}",fake_light);
    qe.apply(fake_light)
   // QuantumEfficiency::hallucinate(4000,vec![1000,1500,2000,2500]);
  //  println!("serializing!");

}

//integrate bs checking/data verification and link up gneration/storage


