
mod effects;
mod test;

use ndarray::prelude::*;
use ndarray::Array;
use effects::*;
use test::*;


fn main() {

    //let a = Array::<f64, _>::zeros((3, 2, 4).f());

    QuantumEfficiency::hallucinate("test".to_string(),4000,vec![1000,1500,2000,2500]);
  //  println!("serializing!");

}

struct Optic {
    optic_name: String,


}

struct Detector {
    detector_name: String,


}