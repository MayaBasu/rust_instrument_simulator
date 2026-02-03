use rayon::prelude::*;
use std::io::{BufWriter, Write};
use std::time::Instant;
use memmap2::Mmap;
use std::fs::File;
use crate::hallucinate;


pub fn pipline(hallucinate_data:bool){
    pub const LINE_SIZE:usize = 4000; //number of matrix columns
    pub const NUM_LINES:usize = 4000; //number of matrix rows


    let name1 = hallucinate::name_gen(LINE_SIZE, NUM_LINES, "A_bytes"); //standard file names
    let name2 = hallucinate::name_gen(LINE_SIZE, NUM_LINES, "B_bytes");

    println!("Generating files {name1} and {name2}...");
    //generate files if they haven't been made already
    if hallucinate_data {
        hallucinate::byte_version(LINE_SIZE, NUM_LINES, name1.as_str()).expect("TODO: panic message");
        hallucinate::byte_version(LINE_SIZE, NUM_LINES, name2.as_str()).expect("TODO: panic message");
    }
    println!("adding the files");

    let result_file = File::create("result").unwrap(); //file to write the result to
    let mut result_buf = BufWriter::new(result_file);


    let file1 = File::open(name1).unwrap();
    let file2 = File::open(name2).unwrap();
    let mmap1 = unsafe { Mmap::map(&file1).unwrap() };
    let mmap2 = unsafe { Mmap::map(&file2).unwrap() };
    let data1:&[u8] = &mmap1[..];
    let data2:&[u8] = &mmap2[..];
    let data1:&[f64] = bytemuck::cast_slice(data1);
    let data2:&[f64] = bytemuck::cast_slice(data2);

    let now = Instant::now();

    let final_vec: Vec<f64> = data1
        .par_iter()
        .zip(data2.par_iter())
        .map(|(data_point1,data_point2)| data_point1+data_point2).collect();

    result_buf.write_all(bytemuck::cast_slice(&final_vec[..])).expect("failed to write result");

    println!("final sum is {:?} at time {:?}", final_vec.iter().sum::<f64>(), now.elapsed().as_millis());


}




