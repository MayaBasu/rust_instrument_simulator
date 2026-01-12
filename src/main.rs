
mod effects;
mod data_cube_management;
mod hallucinations;
use csv::*;
mod linewisedatagen;

use data_cube_management::ElementWiseCombinationType;


use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;
use ndarray::prelude::*;

use data_cube_management::SpatialSpectralEffect;
use crate::hallucinations::hallucinate_spatial_spectral;
pub const float_size:usize = 8;  //number of bytes in one float
fn name_gen(linesize:usize,num_lines:usize,delinator:&str) -> String {
    (num_lines.to_string() + "_lines_" + linesize.to_string().as_str()+ "_columns_" + delinator + ".txt").to_string()

}

fn main() {

    const linesize:usize = 12000;
    const num_lines:usize = 12000;
    const write_chunk_size:usize = 1;

    let name1 = name_gen(linesize,num_lines,"A");
    let name2 = name_gen(linesize,num_lines,"B");

    println!("Generating files {name1} and {name2}...");

    //linewisedatagen::example(linesize,num_lines,name1.as_str()).unwrap();
  // linewisedatagen::example(linesize,num_lines,name2.as_str()).unwrap();
    println!("adding the files");


    let now = Instant::now();

    let f1 = File::open(name1).unwrap();
    let f1 = BufReader::new(f1);

    let f2 = File::open(name2).unwrap();
    let f2 = BufReader::new(f2);

    let result_file = Writer::from_path("result.txt");
    let mut result_file = match result_file {
        Ok(writer) => writer,
        Err(_err) => panic!(),
    };


    let mut lines2 = f2.lines();

    //let write_counter = 0;
    for line1 in f1.lines(){
        let mut result_line:[f64;linesize]  = [0.0;linesize];

        let line1 = line1.unwrap();
        let line2 = lines2.next().unwrap().unwrap();

        //let line1: Vec<&str> =line1.as_str().split(",").collect::<Vec<&str>>();
       // let line2: Vec<&str> = line2.as_str().split(",").collect::<Vec<&str>>();

       // let line1:[f64;linesize] = line1.iter().map(|x| x.parse().unwrap()).collect::<Vec<f64>>().try_into().unwrap();
        //let line2:[f64;linesize] = line2.iter().map(|x| x.parse().unwrap()).collect::<Vec<f64>>().try_into().unwrap();


        let line1:[f64;linesize] = line1.as_str().split(",").map(|x| x.parse().unwrap()).collect::<Vec<f64>>().try_into().unwrap();
        let line2:[f64;linesize] = line2.as_str().split(",").map(|x| x.parse().unwrap()).collect::<Vec<f64>>().try_into().unwrap();



        for i in 0..linesize {
            let sum = line1[i] + line2[i];
            result_line[i] = sum;
        }
        //println!("The lines and result are \n {:?} \n {:?} \n {:?} ",line1,line2,result_line);
        let result_line:Vec<String> = result_line.iter().map(|x| x.to_string()).collect();

        result_file.write_record(result_line).unwrap()




    }

    println!("completed task in {:?}", now.elapsed().as_secs());








    /*
    let num_pixles:usize = 12000;
    let frequencies = vec![1];


    let result = "/Users/mayabasu/Desktop/data/output.txt";

    File::create(result);


    let qe = SpatialSpectralEffect::initialize("lske1".to_string(), true, num_pixles, frequencies.clone());

    let qe2 = SpatialSpectralEffect::initialize("lske1".to_string(), true, num_pixles, frequencies.clone());

    let qe3 = SpatialSpectralEffect::initialize("lske1".to_string(), true,num_pixles, frequencies.clone());

    let qe4 = SpatialSpectralEffect::initialize("lske1".to_string(), true, num_pixles, frequencies.clone());
    /*
    let qe5 = SpatialSpectralEffect::initialize("lske5".to_string(), true, num_pixles, frequencies.clone());
    let qe6 = SpatialSpectralEffect::initialize("lske6".to_string(), true, num_pixles, frequencies.clone());
    let qe7 = SpatialSpectralEffect::initialize("lske7".to_string(), true,num_pixles, frequencies.clone());
    let qe8 = SpatialSpectralEffect::initialize("lske8".to_string(), true, num_pixles, frequencies.clone());
    let qe9 = SpatialSpectralEffect::initialize("lske12".to_string(), true, num_pixles, frequencies.clone());
    let qe10 = SpatialSpectralEffect::initialize("lske22".to_string(), true, num_pixles, frequencies.clone());
    let qe11 = SpatialSpectralEffect::initialize("lske32".to_string(), true,num_pixles, frequencies.clone());
    let qe12 = SpatialSpectralEffect::initialize("lske42".to_string(), true, num_pixles, frequencies.clone());
    let qe13 = SpatialSpectralEffect::initialize("lske52".to_string(), true, num_pixles, frequencies.clone());
    let qe14 = SpatialSpectralEffect::initialize("lske62".to_string(), true, num_pixles, frequencies.clone());
    let qe15 = SpatialSpectralEffect::initialize("lske72".to_string(), true,num_pixles, frequencies.clone());
    let qe16 = SpatialSpectralEffect::initialize("lske82".to_string(), true, num_pixles, frequencies.clone());

     */


    data_cube_management::wide_compare_quad_unrolled_combine_data_cubes(qe,qe2,qe3,qe4, data_cube_management::ElementWiseCombinationType::ComponentWiseMultiplicative, result);

  //  data_cube_management::oct_unrolled_combine_data_cubes(qe, qe2, qe3, qe4,qe5, qe6, qe7, qe8, data_cube_management::ElementWiseCombinationType::ComponentWiseAddition, result);


    /*
    let fake_light =Array3::random(
        (4000,4000, 10),
        Uniform::new(0., 1.)).to_owned();

     */
    //println!("FAke light is {:?}",fake_light);
    //qe.apply(fake_light)
   // QuantumEfficiency::hallucinate(4000,vec![1000,1500,2000,2500]);
  //  println!("serializing!");

     */

}

//integrate bs checking/data verification and link up gneration/storage


