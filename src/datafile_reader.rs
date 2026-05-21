use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::num::ParseFloatError;
use std::time::Instant;
use plotpy::{Curve, Plot};
use crate::instrument::{largest_wavelength, smallest_wavelength, spectral_resolution};

pub fn read_data_file(file_name: &str, expected_records:usize, floats_per_record:usize,scale:f32,plot:bool) -> Vec<Vec<f32>>{
    println!("Loading data file {:?}", file_name);
    let start = Instant::now();

    let file = File::open(file_name).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for (i,line_result) in reader.lines().enumerate() {
        let line = line_result.expect("failed to read line");
        let line = line.trim();
        let line:Vec<&str> = line.split("   ").collect();
        let mut valid_line = true;
        let mut parsed_line = Vec::new();
        for (index, element) in line.iter().enumerate(){
            match element.trim().parse::<f32>() {
                Ok(val) => {
                    if index ==0{
                        //println!("pushing {:?}",val*scale);
                        parsed_line.push(val*scale)
                    }else{
                        parsed_line.push(val)
                    }}
                Err(_) => {valid_line = false}
            };
        }
        if !valid_line{
            println!("line {:?} invalid: {:?}",i,parsed_line)
        }else{
            if parsed_line.len() == floats_per_record{
                data.push(parsed_line)
            }else{
                println!("{:?}",parsed_line);
                panic!("Parsed a different number of floats in a line than expected")
            }
        }
    }


    println!("Parsed {:?} records in {:?} ms",data.len(), start.elapsed().as_millis());
    assert_eq!(data.len(),expected_records,"Retrieved a different number of records than expected");

    if plot{
        let mut plot = Plot::new();
        for i in 1..floats_per_record {
            let mut curve = Curve::new();
            curve.set_line_width(2.0);
            curve.points_begin();
            for point in data.clone() {
               // println!("{:?}", point);

                curve.points_add(point[0], point[i]);
            }
            curve.points_end();
            plot.add(&curve).grid_and_labels("x", "y");
        }
        plot.show("ksenf").expect("hHHHHHH");
    }
    println!("The first record is {:?} and the last is {:?}",data[0],data[data.len()-1]);


    data
}

pub fn arrayify<T,P>(vector:Vec<Vec<f32>>){
    let spectral_array = [1.0;spectral_resolution];




}