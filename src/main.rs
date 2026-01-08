
mod effects;
mod test;
mod hallucinations;

use ndarray::prelude::*;
use ndarray::Array;
use effects::*;
use test::*;


fn main() {

    //let a = Array::<f64, _>::zeros((3, 2, 4).f());
    hallucinate("hallucinated_data/ekdkd",4000,vec![1000,1500,2000,2500])

   // QuantumEfficiency::hallucinate(4000,vec![1000,1500,2000,2500]);
  //  println!("serializing!");

}

struct Optic {
    optic_name: String,


}

struct Detector {
    detector_name: String,


}