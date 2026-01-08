use std::fs::File;
use std::io::Write;
use std::time::Instant;
use ndarray::{Array1, Array2, Array3};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

/*
This is a module to aid in prototyping.
It contains functions which allow fabrication of random data cubes with spatial and/or spectral componentes
 */

pub fn hallucinate_spatial_spectral(file_name:&str, number_of_pixels: usize, sample_frequencies_in_nm: Vec<usize>)->File {
    //We initialize an array the size of the data cube
    let now = Instant::now();
    let hallucinated_quantum_efficiency = Array3::random(
        (number_of_pixels, number_of_pixels, sample_frequencies_in_nm.len()),
        Uniform::new(0., 1.)).to_owned();
    println!("Hallucinated Quantum Efficiency in {:?} milliseconds", now.elapsed().as_millis());

    //Converting the quantum efficiency data cube to bytes
    let now = Instant::now();
    let flat_qe = hallucinated_quantum_efficiency.iter().map(|x: &f64| x.to_le_bytes()).flatten().collect::<Vec<u8>>();
    println!("Converted hallucinated data to bytes in {} milliseconds", now.elapsed().as_millis());

    //Writing data to a file
    let now = Instant::now();
    let mut file = File::create(file_name).unwrap();
    file.write_all(flat_qe.as_slice()).unwrap();
    println!("Writing hallucinated data to a file in {} milliseconds", now.elapsed().as_millis());

    file

}

pub fn hallucinate_spatial(file_name:&str, number_of_pixels: usize) ->File {
    //We initialize an array the size of the data cube
    let now = Instant::now();
    let hallucinated_quantum_efficiency = Array2::random(
        (number_of_pixels, number_of_pixels),
        Uniform::new(0., 1.)).to_owned();
    println!("Hallucinated Quantum Efficiency in {:?} milliseconds", now.elapsed().as_millis());

    //Converting the quantum efficiency data cube to bytes
    let now = Instant::now();
    let flat_qe = hallucinated_quantum_efficiency.iter().map(|x: &f64| x.to_le_bytes()).flatten().collect::<Vec<u8>>();
    println!("Converted hallucinated data to bytes in {} milliseconds", now.elapsed().as_millis());

    //Writing data to a file
    let now = Instant::now();
    let mut file = File::create(file_name).unwrap();
    file.write_all(flat_qe.as_slice()).unwrap();
    println!("Writing hallucinated data to a file in {} milliseconds", now.elapsed().as_millis());
    file


}


pub fn hallucinate_spectral(file_name:&str, sample_frequencies_in_nm: Vec<usize>) -> File {
    //We initialize an array the size of the data cube
    let now = Instant::now();
    let hallucinated_quantum_efficiency = Array1::random(
        ( sample_frequencies_in_nm.len()),
        Uniform::new(0., 1.)).to_owned();
    println!("Hallucinated Quantum Efficiency in {:?} milliseconds", now.elapsed().as_millis());

    //Converting the quantum efficiency data cube to bytes
    let now = Instant::now();
    let flat_qe = hallucinated_quantum_efficiency.iter().map(|x: &f64| x.to_le_bytes()).flatten().collect::<Vec<u8>>();
    println!("Converted hallucinated data to bytes in {} milliseconds", now.elapsed().as_millis());

    //Writing data to a file
    let now = Instant::now();
    let mut file = File::create(file_name).unwrap();
    file.write_all(flat_qe.as_slice()).unwrap();
    println!("Writing hallucinated data to a file in {} milliseconds", now.elapsed().as_millis());

    file


}


